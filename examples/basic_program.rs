#![no_std]
#![no_main]

use gba2k::{
  bios::VBlankIntrWait,
  interrupts::{set_rust_irq_handler, GbaCell, IrqBits, IE, IME},
  keys::KEYINPUT,
  video::{
    color::{Color, BACKDROP_COLOR},
    display_control::{DisplayControl, DISPCNT},
    display_status::{DisplayStatus, DISPSTAT},
  },
};

/// This ends up in `bss`
pub static THE_COLOR: GbaCell<Color> = GbaCell::new_color(Color::WHITE);

/// This ends up in `data`
pub static BAR: GbaCell<u32> = GbaCell::new_u32(7);

#[no_mangle]
extern "C" fn main() -> ! {
  set_rust_irq_handler(Some(the_rust_irq_handler));
  DISPSTAT.write(DisplayStatus::new().with_vblank_irq(true));
  IE.write(IrqBits::V_BLANK);
  IME.write(true);

  DISPCNT.write(DisplayControl::new().with_display_bg0(true));
  loop {
    VBlankIntrWait();
    BACKDROP_COLOR.write(THE_COLOR.read());
  }
}

#[link_section = ".iwram"]
extern "C" fn the_rust_irq_handler(_: IrqBits) {
  THE_COLOR.write(Color::from(u16::from(KEYINPUT.read())));
}
