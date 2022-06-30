#![warn(missing_docs)]

//! The `rt0` is the assembly runtime that runs *before* your Rust code.
//!
//! This module also serves as the home of any other handwritten assembly files
//! that the crate adds over time.

use core::arch::global_asm;

use crate::interrupts::{GbaCell, IrqBits};

global_asm! {
  include_str!("header_and_init.s"),
  options(raw)
}

global_asm! {
  include_str!("irq_handler.s"),
  options(raw)
}
extern "C" {
  pub(crate) static RUST_IRQ_HANDLER: GbaCell<Option<extern "C" fn(IrqBits)>>;
}
/// Sets the rust function to run when a hardware interrupt occurs.
///
/// When a hardware interrupt happens, control transfers to the BIOS, which
/// calls the rt0 interrupt handler, which will acknowledge the interrupt(s) and
/// then call your handler with the [IrqBits] of what interrupt(s) just
/// occurred.
///
/// If you set the handler to `None` the rt0 handler will still acknowledge any
/// interrupt, but will just skip calling back into rust.
#[inline]
pub fn set_rust_irq_handler(opt_f: Option<extern "C" fn(IrqBits)>) {
  unsafe { RUST_IRQ_HANDLER.write(opt_f) };
}

global_asm!(include_str!("single_swp.s"), options(raw));
extern "C" {
  /// A `swp` instruction, stored in the `.text` section.
  ///
  /// Reads `addr` and then writes `new_val` to `addr` as a single, atomic
  /// action.
  ///
  /// * **Returns:** the previous value at `addr`.
  ///
  /// ## Safety
  /// * The pointer must be aligned and valid for a read.
  /// * The pointer must be aligned and valid for a write.
  pub fn text_single_swp(new_val: u32, addr: *mut u32) -> u32;
}

global_asm!(include_str!("single_swpb.s"), options(raw));
extern "C" {
  /// A `swpb` instruction, stored in the `.text` section.
  ///
  /// Reads `addr` and then writes `new_val` to `addr` as a single, atomic
  /// action.
  ///
  /// * **Returns:** the previous value at `addr`.
  ///
  /// ## Safety
  /// * The pointer must be aligned and valid for a read.
  /// * The pointer must be aligned and valid for a write.
  pub fn text_single_swpb(new_val: u8, addr: *mut u8) -> u8;
}

#[cfg(feature = "memcpy")]
global_asm!(include_str!("memcpy.s"), options(raw));
#[cfg(feature = "memcpy")]
extern "C" {
  /// Provides libc-style [memory copy][man-memcpy].
  ///
  /// Copies `count` bytes from `src` to `dest`, then returns the `dest` value.
  ///
  /// This function requires slightly more stack usage than [__aeabi_memcpy],
  /// and thus also runs a few cycles slower. If you don't specifically need the
  /// `dest` pointer to be returned back to you, then use `__aeabi_memcpy`
  /// instead.
  ///
  /// [man-memcpy]: https://man7.org/linux/man-pages/man3/memcpy.3.html
  ///
  /// ## Safety
  /// * The `src` and `dest` regions must not overlap.
  /// * `src` must be valid to read for `count` bytes.
  /// * `dest` must be valid to write for `count` bytes.
  pub fn memcpy(dest: *mut u8, src: *mut u8, count: usize) -> *mut u8;

  /// A memory copy operation guaranteed to work correctly with SRAM.
  ///
  /// This copy function's code is stored in IWRAM and it will **always** do
  /// work one byte at a time. Normally a byte by byte copy would be not so
  /// great, but SRAM is *required* to be accessed only one byte at a time, and
  /// using code not stored in ROM.
  ///
  /// If you need to copy a buffer of data into or out of SRAM, use this
  /// function.
  ///
  /// ## Safety
  /// * The `src` and `dest` regions must not overlap.
  /// * `src` must be valid to read for `count` bytes.
  /// * `dest` must be valid to write for `count` bytes.
  pub fn memcpy_sram(dest: *mut u8, src: *mut u8, count: usize);

  /// Provides ARM-style [memory copy][arm-memcpy].
  ///
  /// This works just like [memcpy] from libc except that there's no return
  /// value. This allows a small saving on stack space, as well as saving a few
  /// cycles required to push/pop two words.
  ///
  /// Prefer this function over `memcpy` whenever you don't need the return
  /// value.
  ///
  /// If you know that both your pointers are aligned to 4, prefer
  /// [__aeabi_memcpy4].
  ///
  /// [arm-memcpy]:
  ///     https://github.com/ARM-software/abi-aa/blob/main/rtabi32/rtabi32.rst#memory-copying-clearing-and-setting
  ///
  /// ## Safety
  /// * The `src` and `dest` regions must not overlap.
  /// * `src` must be valid to read for `count` bytes.
  /// * `dest` must be valid to write for `count` bytes.
  pub fn __aeabi_memcpy(dest: *mut u8, src: *mut u8, count: usize);

  /// Works like [__aeabi_memcpy] but optimized for pointers aligned to 4.
  ///
  /// While the pointers must both be aligned to 4, the number of bytes to copy
  /// *does not* need to be an exact multiple of 4.
  ///
  /// ## Safety
  /// * The `src` and `dest` regions must not overlap.
  /// * `src` must be aligned to 4 and valid to read for `count` bytes.
  /// * `dest` must be aligned to 4 and valid to write for `count` bytes.
  pub fn __aeabi_memcpy4(dest: *mut u8, src: *mut u8, count: usize);

  /// Works like [__aeabi_memcpy] but optimized for pointers aligned to 8.
  ///
  /// In this implementation, this is simply an alias for [__aeabi_memcpy4].
  /// On the GBA, pointers with a known alignment of 8 instead of just 4 don't
  /// confer any additional advantage.
  pub fn __aeabi_memcpy8(dest: *mut u8, src: *mut u8, count: usize);
}
