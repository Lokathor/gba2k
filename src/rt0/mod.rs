#![warn(missing_docs)]

//! The `rt0` is the assembly runtime that runs *before* your Rust code.
//!
//! This module also serves as the home of any other handwritten assembly files
//! that the crate adds over time.

use core::arch::global_asm;

use crate::interrupts::{GbaCell, IrqBits};

arm7tdmi_aeabi::generate_fns!(section_prefix = ".iwram");

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
