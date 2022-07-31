#![no_std]
//#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_abi)]
#![feature(isa_attribute)]
#![feature(naked_functions)]

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

#[inline]
pub fn swp(word: u32, addr: &mut u32) -> u32 {
  unsafe { a32_swp_r0_r0_r1(word, addr) }
}

#[naked]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn a32_swpb_r0_r0_r1(byte: u8, addr: *mut u8) -> u8 {
  core::arch::asm! {
    "
    swpb r0, r0, [r1]
    bx lr
    "
    ,options(noreturn, raw)
  }
}

#[naked]
#[instruction_set(arm::a32)]
pub unsafe extern "C" fn a32_swp_r0_r0_r1(word: u32, addr: *mut u32) -> u32 {
  core::arch::asm! {
    "
    swp r0, r0, [r1]
    bx lr
    "
    ,options(noreturn, raw)
  }
}

#[naked]
pub unsafe extern "C" fn t32_bx_r3<A, B, C, R>(
  a: A, b: B, c: C, f: unsafe extern "C" fn(A, B, C) -> R,
) -> R {
  core::arch::asm! {
    "
    bx r3
    ",
    options(noreturn, raw)
  }
}
