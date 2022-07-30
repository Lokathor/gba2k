#![no_std]
//#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_abi)]
#![feature(isa_attribute)]

//! A GBA development project.
//!
//! ## Safety
//!
//! * All of the crate's safety calculations assume that you're using our
//!   provided linker script, and using the `thumbv4t-none-eabi` target.
//! * On any other target you *can* still render the crate's docs, but you
//!   generally won't be able to compile the crate. Even if you can, all MMIO
//!   and inline assembly would be incorrect to run.

#[macro_use]
mod macros;

pub mod bios;
pub mod interrupts;
pub mod keys;
pub mod rt0;
pub mod video;
