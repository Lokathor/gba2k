#![no_std]
#![feature(isa_attribute)]

core::arch::global_asm! {
  include_str!("rt0.s"),
  options(raw)
}

#[macro_use]
mod macros;
mod bit_utils;

pub mod bios;
pub mod interrupts;
pub mod video;

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
