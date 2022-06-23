#![no_std]
#![no_main]

use gba2k::{
  bios::VBlankIntrWait,
  interrupts::{set_rust_irq_handler, GbaCell, IrqBits, IE, IME},
  keyinput::KEYINPUT,
  video::{
    Color, DisplayControl, DisplayStatus, BACKDROP_COLOR, DISPCNT, DISPSTAT,
  },
};

pub static THE_COLOR: GbaCell<Color> = GbaCell::new(Color::WHITE);

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
