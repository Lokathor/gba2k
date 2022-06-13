#![no_std]

use voladdress::*;

core::arch::global_asm! {
  include_str!("rt0.s"),
  options(raw)
}

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

pub const DISPCNT: VolAddress<u16, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };
