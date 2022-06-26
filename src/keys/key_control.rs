use voladdress::*;

/// Key interrupt control.
///
/// See [KeyControl].
pub const KEYCNT: VolAddress<KeyControl, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0000) };

/// Configures when the keypad should send an interrupt.
///
/// If you'd like an interrupt when a certain keypress happens, set the
/// appropriate buttons as well as the `irq_enable` field.
///
/// If you set the `all_required` field then **all** keys must be pressed at the
/// same time to trigger the interrupt. Otherwise **any** selected key can
/// trigger the interrupt.
///
/// They key interrupt is advised only for breaking out of the low-power
/// [`Halt`](crate::bios::Halt) state. You should not use it as a way to read
/// regular user input.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct KeyControl(u16);

impl KeyControl {
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
  u16_bool_field!(14, irq_enable, with_irq_enable);
  u16_bool_field!(15, all_required, with_all_required);
}
