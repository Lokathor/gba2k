#![no_std]
#![no_main]

use gba2k::{
  interrupts::{set_rust_irq_handler, GbaCell, IrqBits},
  video::display_control::{DisplayControl, DisplayMode, DISPCNT},
};

/// This ends up in `bss`
#[no_mangle]
pub static FOO: GbaCell<i32> = GbaCell::new_i32(0_i32);

/// This ends up in `data`
#[no_mangle]
pub static BAR: GbaCell<i32> = GbaCell::new_i32(7);

#[no_mangle]
extern "C" fn main() -> ! {
  DISPCNT.write(
    DisplayControl::new()
      .with_display_mode(DisplayMode::_0)
      .with_display_bg2(true),
  );
  set_rust_irq_handler(Some(my_irq_handler));
  FOO.swap(5);
  BAR.write(8);
  loop {}
}

extern "C" fn my_irq_handler(_: IrqBits) {
  //
}
