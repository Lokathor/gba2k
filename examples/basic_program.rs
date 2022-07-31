#![no_std]
#![no_main]

use gba2k::{
  bios::VBlankIntrWait,
  interrupts::{GbaCell, IrqBits, IE, IME},
  keys::KEYINPUT,
  rt0::set_rust_irq_handler,
  swp, t32_bx_r3,
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

  let mut dest = [0; 4];
  let src = [1, 2, 3, 4];
  unsafe {
    t32_bx_r3(dest.as_mut_ptr(), src.as_ptr(), 4, gba2k::rt0::gba_memcpy_sram)
  };

  let mut x = 0_u32;
  DISPCNT.write(DisplayControl::new().with_display_bg0(true));
  loop {
    VBlankIntrWait();
    swp(x.wrapping_add(1), &mut x);
    //unsafe { text_single_swp(x.wrapping_add(1), &mut x) };
    let keys = KEYINPUT.read();
    if keys.start() {
      BACKDROP_COLOR.write(Color::BLUE);
    } else {
      BACKDROP_COLOR.write(THE_COLOR.read());
    }
  }
}

#[link_section = ".iwram"]
extern "C" fn the_rust_irq_handler(_: IrqBits) {
  THE_COLOR.write(Color::from(u16::from(KEYINPUT.read())));
}

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
