use voladdress::*;

/// "Display Status"
pub const DISPSTAT: VolAddress<DisplayStatus, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0004) };

/// The display status lets you get info about the display and configure display
/// interrupts.
///
/// You can determine if the display is currently in v-blank, h-blank, or if the
/// current [`VCOUNT`] value matches the vertical counter setting you've
/// assigned to the display status. These are read-only, and they're ignored by
/// the hardware when you write a `DisplayStatus` value to `DISPSTAT`.
///
/// You can set if you want an interrupt to be sent when each of the above
/// states (v-blank, h-blank, v-match) becomes true. Remember that for an
/// interrupt to occur you have to also enable the interrupt to be received in
/// [`IE`] and you have to have [`IME`] enabled.
///
/// Finally, you can set what vertical counter value you watch to match on.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DisplayStatus(u16);

impl DisplayStatus {
  pub_const_fn_new!();
  u16_bool_field!(0, is_vblank, with_is_vblank);
  u16_bool_field!(1, is_hblank, with_is_hblank);
  u16_bool_field!(2, is_vcounter, with_is_vcounter);
  u16_bool_field!(3, vblank_irq, with_vblank_irq);
  u16_bool_field!(4, hblank_irq, with_hblank_irq);
  u16_bool_field!(5, vcounter_irq, with_vcounter_irq);
  u16_val_field!(8 - 15, vcounter, with_vcounter);
}
