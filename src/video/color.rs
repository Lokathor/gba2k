#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Color(u16);

impl Color {
  pub const WHITE: Self = Self(0b11111_11111_11111);
  pub const BLACK: Self = Self(0);
  pub const RED: Self = Self(0b00000_00000_11111);
  pub const GREEN: Self = Self(0b00000_11111_00000);
  pub const BLUE: Self = Self(0b11111_00000_00000);

  pub_const_fn_new!();
  u16_val_field!(0 - 4, red, with_red);
  u16_val_field!(5 - 9, green, with_green);
  u16_val_field!(10 - 14, blue, with_blue);
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
