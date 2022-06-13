#![no_std]
#![no_main]

use gba2k::DISPCNT;

#[no_mangle]
fn main() -> ! {
  DISPCNT.write(0x0403);
  loop {}
}
