/*
Content in this file is based on the memcpy routines from
https://github.com/felixjones/agbabi
Copyright (C) 2021-2022 agbabi contributors
Used under the Zlib license: https://opensource.org/licenses/Zlib
*/

/*
All these functions are variants of

unsafe extern "C" fn name(dest: *mut u8, src: *mut u8, count: usize);

The `memcpy` and `memmove` functions are as per libc. Also, unlike the above
signature they do return the initial `dest` pointer.

The functions with 4 or 8 on the end require that the input pointers are aligned
to either 4 or 8.

The "copy" operations assume no overlap, the "move" operations don't.

The rest of the docs are in the `mod.rs` file in the rustdoc.
*/

.global memcpy
.global memcpy_sram
.global __aeabi_memcpy
.global __aeabi_memcpy4
.global __aeabi_memcpy8
.global memmove
.global __aeabi_memmove
.global __aeabi_memmove4
.global __aeabi_memmove8

/*
"JoaoBapt carry & sign bit test": In a few places there's a left shift by 31
bits. The trick here is that it checks two bits in a single step. The N flag is
set to the lowest bit, and the C flag is set to the next lowest bit. In other
words, with N set (mi) the value is aligned to 1, else if C is set (cs) the
value is aligned to 2, else the value is aligned to at least 4 (which is the
most we care about for memcpy).
*/

/*
Register And Stack Usage: In this file the register usage is generally as follows:
* r0 = dest pointer
* r1 = src pointer
* r2 = bytes to copy
* r3 and r12 = scratch

The libc style memcpy will put 2 words on the stack and call to `__aeabi_memcpy`.

The aeabi style memcpy functions will put 7 words on the stack for word aligned
copies of 32 bytes or more. This means that if a large memcpy is interrupted the
call back into rust would have a stack aligned to only 4, not 8, but on the GBA
this can't make a difference since no instruction needs align 8.
*/

.code 32

.section .iwram.memcpy
  .balign 4

  memcpy:
    push    {r0, lr}
    bl      __aeabi_memcpy
    pop     {r0, lr}
    bx      lr

.previous

.section .iwram.__aeabi_memcpy
  .balign 4

  __aeabi_memcpy:
    eor     r3, r1, r0
    movs    r3, r3, lsl #31
    bmi     memcpy_sram
    bcs     .L_memcpy_mostly_u16
    cmp     r2, #2
    ble     memcpy_sram
    rsb     r3, r0, #4
    movs    r3, r3, lsl #31
    ldrbmi  r3, [r1], #1
    strbmi  r3, [r0], #1
    submi   r2, r2, #1
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    subcs   r2, r2, #2
    @ Fallthrough
  __aeabi_memcpy8:
  __aeabi_memcpy4:
    movs    r12, r2, lsr #5
    beq     .L_after_block_copy
    lsl     r3, r12, #5
    sub     r2, r2, r3
    push    {r4-r10}
    .L_block_copy:
    ldmia   r1!, {r3-r10}
    stmia   r0!, {r3-r10}
    subs    r12, r12, #1
    bne     .L_block_copy
    pop     {r4-r10}
    .L_after_block_copy:
    movs    r12, r2, lsr #2
    .L_word_copy:
    subs    r12, r12, #1
    ldrhs   r3, [r1], #4
    strhs   r3, [r0], #4
    bhs     .L_word_copy
    movs    r3, r2, lsl #31
    ldrhcs  r3, [r1], #2
    strhcs  r3, [r0], #2
    ldrbmi  r3, [r1]
    strbmi  r3, [r0]
    bx      lr

  .L_memcpy_mostly_u16:
    tst     r0, #1
    cmpne   r2, #0
    ldrbne  r3, [r1], #1
    strbne  r3, [r0], #1
    subne   r2, r2, #1
    movs    r12, r2, lsr #1
    .L_halfword_copy:
    subs    r12, r12, #1
    ldrhhs  r3, [r1], #2
    strhhs  r3, [r0], #2
    bhs     .L_halfword_copy
    tst     r2, #1
    ldrbne  r3, [r1]
    strbne  r3, [r0]
    bx      lr

  memcpy_sram:
    subs    r2, r2, #1
    ldrbhs  r3, [r1], #1
    strbhs  r3, [r0], #1
    bhs     memcpy_sram
    bx      lr

.previous

.section .iwram.memmove
  .balign 4

  memmove:
    push    {r0, lr}
    bl      __aeabi_memmove
    pop     {r0, lr}
    bx      lr

.previous

.section .iwram.__aeabi_memmove

  __aeabi_memmove:
    cmp     r0, r1
    bgt     __agbabi_rmemcpy
    b       __aeabi_memcpy

  __aeabi_memmove8:
  __aeabi_memmove4:
    cmp     r0, r1
    bgt     __agbabi_rmemcpy4
    b       __aeabi_memcpy4

  __agbabi_rmemcpy:
    add     r0, r0, r2
    add     r1, r1, r2
    eor     r3, r1, r0
    movs    r3, r3, lsl #31
    bmi     .L_rmemcpy_by_u8
    bcs     .L_rmemcpy_by_mostly_u16
    cmp     r2, #2
    ble     .L_rmemcpy_by_u8
    movs    r3, r0, lsl #31
    ldrbmi  r3, [r1, #-1]!
    strbmi  r3, [r0, #-1]!
    submi   r2, r2, #1
    ldrhcs  r3, [r1, #-2]!
    strhcs  r3, [r0, #-2]!
    subcs   r2, r2, #2
    b       .L_rmemcpy4_skip_adjustment
  __agbabi_rmemcpy4:
    add     r0, r0, r2
    add     r1, r1, r2
    .L_rmemcpy4_skip_adjustment:
    movs    r12, r2, lsr #5
    beq     .L_rmemcpy_skip_bulk_work
    lsl     r3, r12, #5
    sub     r2, r2, r3
    push    {r4-r10}
    .L_rmemcpy_8word_block:
    ldmdb   r1!, {r3-r10}
    stmdb   r0!, {r3-r10}
    subs    r12, r12, #1
    bne     .L_rmemcpy_8word_block
    pop     {r4-r10}
    .L_rmemcpy_skip_bulk_work:
    movs    r12, r2, lsr #2
    .L_rmemcpy_by_word:
    subs    r12, r12, #1
    ldrhs   r3, [r1, #-4]!
    strhs   r3, [r0, #-4]!
    bhs     .L_rmemcpy_by_word
    movs    r3, r2, lsl #31
    ldrhcs  r3, [r1, #-2]!
    strhcs  r3, [r0, #-2]!
    ldrbmi  r3, [r1, #-1]
    strbmi  r3, [r0, #-1]
    bx      lr

  .L_rmemcpy_by_mostly_u16:
    tst     r0, #1
    cmpne   r2, #0
    ldrbne  r3, [r1, #-1]!
    strbne  r3, [r0, #-1]!
    subne   r2, r2, #1
    movs    r12, r2, lsr #1
    .L_work_by_u16_units:
    subs    r12, r12, #1
    ldrhhs  r3, [r1, #-2]!
    strhhs  r3, [r0, #-2]!
    bhs     .L_work_by_u16_units
    tst     r2, #1
    ldrbne  r3, [r1, #-1]
    strbne  r3, [r0, #-1]
    bx      lr

  .L_rmemcpy_by_u8:
    subs    r2, r2, #1
    ldrbhs  r3, [r1, #-1]!
    strbhs  r3, [r0, #-1]!
    bhs     .L_rmemcpy_by_u8
    bx      lr

.previous

.code 16
