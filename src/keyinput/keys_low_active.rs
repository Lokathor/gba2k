use super::Keys;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeysLowActive(u16);

impl From<Keys> for KeysLowActive {
  #[inline]
  #[must_use]
  fn from(k: Keys) -> Self {
    Self(u16::from(k) ^ 0b11_1111_1111)
  }
}

impl From<KeysLowActive> for u16 {
  #[inline]
  #[must_use]
  fn from(low: KeysLowActive) -> Self {
    low.0
  }
}
