use voladdress::*;

/// Reads the hardware key state, provided using the low-active convention.
///
/// See [KeysLowActive].
pub const KEYINPUT: VolAddress<KeysLowActive, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };

/// Holds data for the keys using a low-active convention.
///
/// Low-active means that a button's bit is set ("high") when the button is
/// *released*, and then cleared ("low") when the button is *pressed*.
///
/// This can be odd to think about, so if you'd instead like to have the data in
/// a form where bits are set when the matching button is pressed try using
/// the [Keys](crate::keys::Keys) struct instead:
/// ```no_run
/// # use crate::keys::Keys;
/// Keys::from(KeysLowActive::new());
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeysLowActive(u16);

//impl_bitops_for!(KeysLowActive);

impl KeysLowActive {
  pub_const_fn_new!();
  u16_bool_field!(0, a_released, with_a_released);
  u16_bool_field!(1, b_released, with_b_released);
  u16_bool_field!(2, select_released, with_select_released);
  u16_bool_field!(3, start_released, with_start_released);
  u16_bool_field!(4, right_released, with_right_released);
  u16_bool_field!(5, left_released, with_left_released);
  u16_bool_field!(6, up_released, with_up_released);
  u16_bool_field!(7, down_released, with_down_released);
  u16_bool_field!(8, r_released, with_r_released);
  u16_bool_field!(9, l_released, with_l_released);
}

impl From<KeysLowActive> for u16 {
  #[inline]
  #[must_use]
  fn from(low: KeysLowActive) -> Self {
    low.0
  }
}

impl From<u16> for KeysLowActive {
  #[inline]
  #[must_use]
  fn from(u: u16) -> Self {
    Self(u)
  }
}
