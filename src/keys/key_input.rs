use voladdress::*;

/// Reads the hardware key state.
pub const KEYINPUT: VolAddress<KeyInput, Safe, ()> =
  unsafe { VolAddress::new(0x0400_0130) };

/// Key state data.
///
/// Each method uses `true` for *pressed* and `false` for *released*.
/// Internally, the actual bits from the hardware are 0 when pressed and 1 when
/// released, but all the methods will automatically compensate for this. The
/// only time it should come up is if you're converting to/from a `u16` value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeyInput(u16);

impl KeyInput {
  pub_const_fn_new!();
  u16_low_active_bool_field!(0, a, with_a);
  u16_low_active_bool_field!(1, b, with_b);
  u16_low_active_bool_field!(2, select, with_select);
  u16_low_active_bool_field!(3, start, with_start);
  u16_low_active_bool_field!(4, right, with_right);
  u16_low_active_bool_field!(5, left, with_left);
  u16_low_active_bool_field!(6, up, with_up);
  u16_low_active_bool_field!(7, down, with_down);
  u16_low_active_bool_field!(8, r, with_r);
  u16_low_active_bool_field!(9, l, with_l);

  /// Determines what keys have changed since a previous KeyInput.
  #[inline]
  #[must_use]
  pub const fn changes_since(self, previous: KeyInput) -> KeyChanges {
    KeyChanges(self.0 ^ previous.0)
  }
}

impl From<KeyInput> for u16 {
  #[inline]
  #[must_use]
  fn from(low: KeyInput) -> Self {
    low.0
  }
}

impl From<u16> for KeyInput {
  #[inline]
  #[must_use]
  fn from(u: u16) -> Self {
    Self(u)
  }
}

/// A change in the key state data.
///
/// This value only stores that there was a change or not. To know the
/// "direction" of the change (eg: "was pressed" or "was released") you also
/// need to know the previous state.
///
/// Example:
/// * If the L button has changed, and it was previously pressed, then the
///   change is that L became released.
/// * If the L button has changed, and it was previously released, then the
///   change is that L became pressed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeyChanges(u16);
impl KeyChanges {
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
