use core::mem::size_of;

use voladdress::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(u16);

impl Color {
  pub const WHITE: Self = Self(0b11111_11111_11111);

  pub_const_fn_new!();
  unsafe_u16_val_field!(0 - 4, red, with_red);
  unsafe_u16_val_field!(5 - 9, green, with_green);
  unsafe_u16_val_field!(10 - 14, blue, with_blue);
}

impl From<u16> for Color {
  #[inline]
  #[must_use]
  fn from(u: u16) -> Self {
    Self(u)
  }
}
impl From<Color> for u16 {
  #[inline]
  #[must_use]
  fn from(c: Color) -> Self {
    c.0
  }
}

pub const BACKDROP_COLOR: VolAddress<Color, Safe, Safe> =
  unsafe { VolAddress::new(0x0500_0000) };

pub const BG_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(0x0500_0000) };

pub const OBJ_PALETTE: VolBlock<Color, Safe, Safe, 256> =
  unsafe { VolBlock::new(0x0500_0200) };

pub type PalBank = VolBlock<Color, Safe, Safe, 16>;

#[inline]
#[must_use]
pub const fn bg_palbank(x: usize) -> PalBank {
  assert!(x < 16);
  unsafe { VolBlock::new(0x0500_0000 + x * size_of::<[Color; 16]>()) }
}

#[inline]
#[must_use]
pub const fn obj_palbank(x: usize) -> PalBank {
  assert!(x < 16);
  unsafe { VolBlock::new(0x0500_0200 + x * size_of::<[Color; 16]>()) }
}
