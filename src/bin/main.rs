#![no_std]
#![no_main]

use gba2k::{set_rust_irq_handler, DISPCNT};

#[no_mangle]
pub static mut FOO: i32 = 0;
#[no_mangle]
pub static mut BAR: i32 = 7;

#[no_mangle]
extern "C" fn main() -> ! {
  DISPCNT.write(0x0403);
  set_rust_irq_handler(Some(my_irq_handler));
  unsafe { FOO = 5 };
  unsafe { BAR = 8 };
  loop {}
}

extern "C" fn my_irq_handler(_: u16) {
  //
}
