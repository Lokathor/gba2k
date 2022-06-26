#![warn(missing_docs)]

//! Allows reading the GBA's key inputs.
//!
//! The GBA has:
//! * Two primary buttons (A and B)
//! * Two secondary buttons (Start and Select)
//! * Two shoulder buttons (L and R)
//! * A 4-way directional pad
//!
//! The [`KEYINPUT`] value will update every single CPU cycle. Minor variations
//! in pressure can make a button seem to "bounce" up and down quite rapidly.
//! Because of this, you should usually read the key state just once per frame
//! (usually at v-blank) and then use that data for the entire frame. Otherwise,
//! the user can get inconsistent behavior when an early part of the frame's
//! computation thinks a button is pressed while later on in the same frame it's
//! released.

mod keys_low_active;
pub use keys_low_active::*;

mod key_control;
pub use key_control::*;

/// Holds data for the keys using a high-active convention.
///
/// High-active means that a button's bit is set ("high") when the button is
/// *pressed*, and then cleared ("low") when the button is *released*.
///
/// This doesn't match the raw data from the hardware (see [KeysLowActive]), but
/// it better matches how bit sets types normally work. `From` and `Into` impls
/// are provided for conversions between the two formats.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Keys(u16);

impl_bitops_for!(Keys);

impl Keys {
  pub_const_fn_new!();
  u16_bool_field!(0, a, with_a);
  u16_bool_field!(1, b, with_b);
  u16_bool_field!(2, select, with_select);
  u16_bool_field!(3, start, with_start);
  u16_bool_field!(4, right, with_right);
  u16_bool_field!(5, left, with_left);
  u16_bool_field!(6, up, with_up);
  u16_bool_field!(7, down, with_down);
  u16_bool_field!(8, r, with_r);
  u16_bool_field!(9, l, with_l);
}

impl From<KeysLowActive> for Keys {
  #[inline]
  #[must_use]
  fn from(low: KeysLowActive) -> Self {
    Self(u16::from(low) ^ 0b11_1111_1111)
  }
}

impl From<Keys> for u16 {
  #[inline]
  #[must_use]
  fn from(k: Keys) -> Self {
    k.0
  }
}

impl From<Keys> for KeysLowActive {
  #[inline]
  #[must_use]
  fn from(k: Keys) -> Self {
    Self::from(k.0 ^ 0b11_1111_1111)
  }
}

/// Reads the current key state
///
/// Shorthand for
/// ```no_run
/// KEYINPUT.read().into()
/// ```
#[inline]
#[must_use]
pub fn get_keys() -> Keys {
  KEYINPUT.read().into()
}
