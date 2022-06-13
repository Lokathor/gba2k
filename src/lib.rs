#![no_std]

use voladdress::*;

core::arch::global_asm! {
  include_str!("rt0.s"),
  options(raw)
}

extern "C" {
  static mut RUST_IRQ_HANDLER: Option<extern "C" fn(u16)>;
}
pub fn set_rust_irq_handler(opt_f: Option<extern "C" fn(u16)>) {
  unsafe { RUST_IRQ_HANDLER = opt_f };
}

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

pub const DISPCNT: VolAddress<u16, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };
