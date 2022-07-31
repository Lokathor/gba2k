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

arm7tdmi_aeabi::generate_fns!(section_prefix = ".iwram");
