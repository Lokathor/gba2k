#![no_std]
#![no_main]

use gba2k::{
  bios::VBlankIntrWait,
  interrupts::{set_rust_irq_handler, GbaCell, IrqBits, IE, IME},
  keys::{get_keys, Keys},
  video::{
    display_control::{DisplayControl, DisplayMode, DISPCNT},
    display_status::{DisplayStatus, DISPSTAT},
  },
};

/// This ends up in `bss`
pub static CURRENT_KEYS: GbaCell<Keys> =
  unsafe { GbaCell::new_unchecked(Keys::new()) };

/// This ends up in `data`
pub static BAR: GbaCell<i32> = GbaCell::new_i32(7);

#[no_mangle]
extern "C" fn main() -> ! {
  set_rust_irq_handler(Some(my_irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::V_BLANK);
  IME.write(true);

  DISPCNT.write(
    DisplayControl::new()
      .with_display_mode(DisplayMode::_0)
      .with_display_bg2(true),
  );
  loop {
    VBlankIntrWait();
    let ck = CURRENT_KEYS.read();
    BAR.write(u16::from(ck) as i32);
  }
}

#[link_section = ".iwram"]
extern "C" fn my_irq_handler(_: IrqBits) {
  let k = get_keys();
  CURRENT_KEYS.write(k);
}
