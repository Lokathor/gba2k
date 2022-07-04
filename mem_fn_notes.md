# Memory Functions

> This whole document, text and code, is released as [Creative Commons 0](https://creativecommons.org/share-your-work/public-domain/cc0/).

## Introduction

The memory functions get called a heck of a lot.
They're normally provided to Rust's standars library by the [compiler-builtins](https://github.com/rust-lang/compiler-builtins) crate.

The problem is that, the versions in that crate aren't particularly fast, they're just kinda normal. Also, since they're part of the standard library they end up in the `.text` section, stored in ROM, which slows things down.
However, the `compiler-builtins` versions are *also* provided via "weak" linkage.
This means that if you define your own version with strong linkage (the default), then it will override the weak linked version and your version will be used.
This means that we can re-define the memory functions for a significant speed gain.

### Function Signature Goals

The memory functions from libc have signatures like this:

```C
void *memcpy(void *dest, const void *src, size_t count);

void *memmove(void *dest, const void *src, size_t count);

void *memset(void *dest, int ch, size_t count);
```

* `memcpy` is for when regions **don't** overlap.
* `memmove` is when the regions **might** overlap.
* `memset` sets all bytes within the region equal to the `u8` value of `ch`. For historical reasons you pass the value as an `int`, but only the `u8` is used.
* In all cases, the return value is the `dest` pointer given. This allows the C version of "chained" calls.

The [ARM EABI](https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting) also mandates the following additional functions be available:

```C
void __aeabi_memcpy8(void *dest, const void *src, size_t n);
void __aeabi_memcpy4(void *dest, const void *src, size_t n);
void __aeabi_memcpy(void *dest, const void *src, size_t n);

void __aeabi_memmove8(void *dest, const void *src, size_t n);
void __aeabi_memmove4(void *dest, const void *src, size_t n);
void __aeabi_memmove(void *dest, const void *src, size_t n);

void __aeabi_memset8(void *dest, size_t n, int c);
void __aeabi_memset4(void *dest, size_t n, int c);
void __aeabi_memset(void *dest, size_t n, int c);

void __aeabi_memclr8(void *dest, size_t n);
void __aeabi_memclr4(void *dest, size_t n);
void __aeabi_memclr(void *dest, size_t n);
```

* Note that these functions return nothing instead of returning the `dest` pointer. This allows an optimization in the common case where the return value isn't used.
* `__aeabi_memclr` works like `__aeabi_memset` but the value to set to is impled to be 0.
* `__aeabi_memset` works like `memset` but the 2nd and 3rd arguments are reversed, allowing `__aeabi_memclr` to easily tail call into `__aeabi_memset` if the implementation desires.
* The functions with 4 or 8 on the end of the name require that the input pointers be aligned to 4 and 8 respectively, allowing some additional optimizations. In this case, the number of bytes does *not* need to be an even multiple of 4 or 8.

### Assumptions

* We want to store our code for these functions in IWRAM for maximum speed. This means that we will need to consider a balance between pure speed and also trying to limit code size (there isn't much IWRAM memory).
* We *do not want* to use DMA for any of these functions. DMA is very fast, but interrupts can't occur when DMA is active, so sometimes interrupts happen late. All uses of DMA should be a conscious decision by the programmer, not something that's sometimes silently inserted by the compiler.

### How We'll Do This

The easiest way to do this is we're gonna have help from <https://rust.godbolt.org/>

We just open up a nightly compiler and set the command line args to this:
```
--target=armv5te-unknown-linux-gnueabi -Copt-level=3
```

And be sure your rust source panel has `#![no_std]` at the top too.

Using ARMv5TE is the oldest ARM that we can easily make Compiler Explorer give us.
It will occasionally do stuff that's not allowed for ARMv4T (the GBA), but we can adjust those bits by hand.

Then we just input some Rust code, check the assembly, and we can hand tune it if we need to.

### Copy Ordering

When copying memory, if there's no overlap in the regions it doesn't matter how you do it, but if there is overlap then you must be very careful. You need to ensure that none of the destination writes you do destroy the data you'll need for a source read later in the copy.

Imagine we've got a memory span like this (using a little-endian chart since the GBA is little-endian):

| 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| H | G | F | E | D | C | B | A |

And we want to copy a 5 byte region with 0 as the source start and 3 as the dest start.

> If we copy from low address to high address within our two regions, then we's first copy A to 3, then B to 4, C to 5, and... oh no. By the time we read address 3 we're seeing the A that we copied at the start, not the D we intended to copy.

Naturally this means we should always copy from the high address to the low address.
If we do that, everything works just according to plan.

Let's try another copy just to be sure.
Now we want to copy a 4 byte region with 2 as the source start and 0 as the destination start.
For clarity, let's assume the memory is reset to match the chart again.

> So first we copy F from 5 to 3, then F from 4 to 2, then... oh no we've done it again. We expected that address 3 would hold a D to be copied, but we've already overwritten it.

If the memory can overlap, we need to check which of the two address is *lower*:
* If the `src` address is lower, we have to copy from high address to low.
* Otherwise we should copy from low address to high.

The direction of our copy has some unfortunate speed implications! Memory has both "sequential" and "non-sequential" access times. The sequential access time is the lowest (for that memory), and the non-sequential access time is *often* slightly higher. Not always, but often. This means that we'd prefer to always do a copy from low to high address so that we can avoid non-sequential accessing.

* We're going to call a copy from low to high, in sequential ordering, the "forward" copy.
* Going from high to low, non-sequential, will be a "reverse" copy.

## The Code

### Going By `u8`

Let's start with functions that copy `u8` at a time. As mentioned, we need both a "forward" and "reverse" form.

This might seem a little silly to start with, but it's useful for SRAM (where all ops must be byte-sized), and we'll use it for the tail portions of the bigger copies, so it's surprisingly useful.

Here's how we might write is in Rust.

```rust
pub unsafe extern "C" fn copy_u8_forward(mut dest: *mut u8, mut src: *const u8, mut count: usize) {
  const SIZE_U8: usize = core::mem::size_of::<u8>();
  while count >= SIZE_U8 {
    *dest = *src;
    dest = dest.add(1);
    src = src.add(1);
    count -= SIZE_U8;
  }
}

pub unsafe extern "C" fn copy_u8_reverse(mut dest: *mut u8, mut src: *const u8, mut count: usize) {
  const SIZE_U8: usize = core::mem::size_of::<u8>();
  dest = dest.add(count);
  src = src.add(count);
  while count >= SIZE_U8 {
    dest = dest.sub(1);
    src = src.sub(1);
    *dest = *src;
    count -= SIZE_U8;
  }
}
```

Now we feed that into Compiler Explorer and start inspecting the assembly.

#### Optimized for Speed

If we set `-Copt-level=3` in Compiler Explorer let's see what we get.

```arm
example::copy_u8_forward:
    cmp     r2, #0
    bxeq    lr
  .LBB0_1:
    ldrb    r3, [r1], #1
    subs    r2, r2, #1
    strb    r3, [r0], #1
    bne     .LBB0_1
    bx      lr

example::copy_u8_reverse:
    cmp     r2, #0
    bxeq    lr
    sub     r1, r1, #1
    sub     r0, r0, #1
  .LBB1_2:
    ldrb    r3, [r1, r2]
    strb    r3, [r0, r2]
    subs    r2, r2, #1
    bne     .LBB1_2
    bx      lr
```

Well, that's a good start.
Basically what you'd expect from a loop this simple.

#### Optimized for Size

If we set Compiler Explorer for `-Copt-level=z` we can optimize for minimum code size instead.

```arm
example::copy_u8_forward:
  .LBB0_1:
    cmp     r2, #0
    bxeq    lr
    ldrb    r3, [r1], #1
    sub     r2, r2, #1
    strb    r3, [r0], #1
    b       .LBB0_1

example::copy_u8_reverse:
    sub     r0, r0, #1
    sub     r1, r1, #1
  .LBB1_1:
    cmp     r2, #0
    bxeq    lr
    ldrb    r3, [r1, r2]
    strb    r3, [r0, r2]
    sub     r2, r2, #1
    b       .LBB1_1
```

Interesting. We're seeing a `sub`, then a branch which leads to a `cmp`. That's a little poor, I think we can adjust that a bit.

#### Tuned By Hand

So with the forward copy, let's start with the small version.

```arm
@ small-optimized version from LLVM
example::copy_u8_forward:
  .LBB0_1:
    cmp     r2, #0
    bxeq    lr
    ldrb    r3, [r1], #1
    sub     r2, r2, #1
    strb    r3, [r0], #1
    b       .LBB0_1
```

First, `::` isn't allowed in actual code.
It's like that in the Compiler Explorer output because I had "demangle" set.
The "real" symbol name was `_ZN7example15copy_u8_forward17h16b4073fb35424cdE`.
We'll just remove the `example::` prefix and call it `copy_u8_forward`.

```arm
copy_u8_forward:
  .LBB0_1:
    cmp     r2, #0
    bxeq    lr
    ldrb    r3, [r1], #1
    sub     r2, r2, #1
    strb    r3, [r0], #1
    b       .LBB0_1
```

Next, it's just kinda bugging me that the sub is between the store and the load.
Maybe LLVM thinks it's better for the CPU pipeline to have it like that?
I know that ARMv5 had a 7-stage pipeline instead of the 3-stage pipeline on the ARMv4.
Either way it's bugging me, so we're gonna move that out from between them.

```arm
copy_u8_forward:
  .LBB0_1:
    cmp     r2, #0
    bxeq    lr
    sub     r2, r2, #1
    ldrb    r3, [r1], #1
    strb    r3, [r0], #1
    b       .LBB0_1
```

Okay, now we've still got two branches. That's kinda the minimum, because no matter how we sort it, one path restarts the loop when there **is** more work, and the other path returns from the function when there **isn't** more work.

But we *don't* need to have both a `cmp` and a `sub` in the same loop. We can use `subs`, which will set the status flags after the subtraction completes.

```arm
copy_u8_forward:
  .LBB0_1:
    subs    r2, r2, #1
    bx<cond> lr
    ldrb    r3, [r1], #1
    strb    r3, [r0], #1
    b       .LBB0_1
```

But now our [conditional code](https://azeria-labs.com/arm-conditional-execution-and-branching-part-6/) has to change.
* Before we had `bxeq`. That's "`bx` when `eq`", so if `r2` (the register we just compared) equals 0, then we did a branch-exchange. Otherwise we did the subtract, the read, and the write.
* Now we're effectively subtracting *before* checking the flags. The flags are set by the outcome of the subtract operation. So if we subtract and *get* 0, the count *was* 1, and we should *keep going* because we have to copy the final byte. We should only stop the loop when we wrap below 0. `subs` sets the Carry flag when you wrap below 0, so we want to use the `cs` ("carry set") condition.

```arm
copy_u8_forward:
  .LBB0_1:
    subs    r2, r2, #1
    bxcs    lr
    ldrb    r3, [r1], #1
    strb    r3, [r0], #1
    b       .LBB0_1
```

Finally, we can just remove the local label and branch to the function entry.
Note that when we put this in an actual file we'll need to take out the `::` part, which isn't allowed in labels in real code.

```arm
copy_u8_forward:
    subs    r2, r2, #1
    bxcs    lr
    ldrb    r3, [r1], #1
    strb    r3, [r0], #1
    b       copy_u8_forward
```

That seems to be about as good as we can get in terms of compact code.
We *could* try unrolling the loop some manually, but this will usually be called for the tail ends of a copy when there's probably less than 8 bytes left to go.
Given that assumption, we'll just keep the code as small as possible.

Now let's see if we can clean up the "reverse" copy.

```arm
@ small-optimized LLVM version
copy_u8_reverse:
    sub     r0, r0, #1
    sub     r1, r1, #1
  .LBB1_1:
    cmp     r2, #0
    bxeq    lr
    ldrb    r3, [r1, r2]
    strb    r3, [r0, r2]
    sub     r2, r2, #1
    b       .LBB1_1
```

Oh, yuck, there's *two* whole lines of setup before our loop!
Also, now it's using `r2` as an offset for the loads and stores, which means that it makes a difference if we change it before or after the load/store part of the loop.

With the forward-copy we were using `ldrb r3, [r1], #1`, which ARM calls "post-indexed addressing" (and "pre-indexed" also exists).
Now LLVM has us using just "offset addressing".
But... I really wonder... why is it using offset addressing?
The instruction takes the same amount of time either way.
We still need to adjust the two pointers at function entry either way.

I don't think that offset addressing fits the loop we want to do.
The problem I have with the offset addressing is that with our copy_u8_forward function we were able to merge the `cmp` and `sub` into a `subs`.
Now we can't do that because `r2` is the counter and the offset at the same time.
We need to change that loop and have `r2` not be the offset, then we can combine the comparison with the subtraction like we want.
If this causes us to use more instructions at all then we'll not get a savings, but I think we can fix things up without any extra cost.

First let's adjust our load and store to not be offset by `r2`.
We'll just do what we literally already did in the rust code: add count to dest (`r0`) and src (`r1`), and then subtract one from each pointer before the read/write.

```arm
copy_u8_reverse:
    @ add count to each pointer
    add     r0, r0, r2
    add     r1, r1, r2
  .LBB1_1:
    cmp     r2, #0
    bxeq    lr
    @ subtract 1 from pointer before use, the ! writes back the changes
    ldrb    r3, [r1, #-1]!
    strb    r3, [r1, #-1]!
    sub     r2, r2, #1
    b       .LBB1_1
```

It didn't cost us any extra instructions after all!

Now we can re-arrange the loop to use `subs` and `bxcs`.

```arm
copy_u8_reverse:
    add     r0, r0, r2
    add     r1, r1, r2
  .LBB1_1:
    subs    r2, r2, #1
    bxcs    lr
    ldrb    r3, [r1, #-1]!
    strb    r3, [r1, #-1]!
    b       .LBB1_1
```

Also I think that, in this case, we should make the label be a numbered label.
The loop is, in a sense, "exceptionally boring", and you're not supposed to jump to this loop from outside the function or anything, so a numbered label is perfect.

```arm
copy_u8_reverse:
    add     r0, r0, r2
    add     r1, r1, r2
  1:
    subs    r2, r2, #1
    bxcs    lr
    ldrb    r3, [r1, #-1]!
    strb    r3, [r1, #-1]!
    b       1b
```

#### Copy `u8` Summary

I guess we're all set for copying `u8` at a time.

```arm
copy_u8_forward:
    subs    r2, r2, #1
    bxcs    lr
    ldrb    r3, [r1], #1
    strb    r3, [r0], #1
    b       copy_u8_forward

copy_u8_reverse:
    add     r0, r0, r2
    add     r1, r1, r2
  1:
    subs    r2, r2, #1
    bxcs    lr
    ldrb    r3, [r1, #-1]!
    strb    r3, [r1, #-1]!
    b       1b
```
