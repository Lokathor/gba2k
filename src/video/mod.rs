use voladdress::*;

pub mod mode3;

mod color;
pub use color::*;

mod display_control;
pub use display_control::*;

mod display_status;
pub use display_status::*;

const BG_PALETTE_BASE: usize = 0x0500_0000;
const OBJ_PALETTE_BASE: usize = 0x0500_0200;
const VRAM_BASE: usize = 0x0600_0000;

pub const VCOUNT: VolAddress<u8, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0006) };

pub const BACKDROP_COLOR: VolAddress<Color, Safe, Safe> =
  unsafe { VolAddress::new(BG_PALETTE_BASE) };

pub const BG_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(BG_PALETTE_BASE) };

pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(OBJ_PALETTE_BASE) };

pub type PalBank = VolBlock<Color, Safe, Safe, 16>;

#[inline]
#[must_use]
pub const fn bg_palbank(x: usize) -> PalBank {
  assert!(x < 16);
  unsafe { VolBlock::new(BG_PALETTE_BASE + x * size_of!([Color; 16])) }
}

#[inline]
#[must_use]
pub const fn obj_palbank(x: usize) -> PalBank {
  assert!(x < 16);
  unsafe { VolBlock::new(OBJ_PALETTE_BASE + x * size_of!([Color; 16])) }
}
