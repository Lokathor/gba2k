//! Module for interacting with the GBA's hardware interrupt system.

use voladdress::*;

/// "Interrupts Enabled"
pub const IE: VolAddress<IrqBits, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0200) };

/// "Interrupts Flagged"
///
/// You do not normally need to use this at all. It's handled for you by the
/// assembly runtime.
pub const IF: VolAddress<IrqBits, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0202) };

/// "Interrupt Master Enable"
///
/// Obscure Note: there's a 2 cycle delay between an interrupt being triggered
/// and the CPU actually switching over and running the handler code, so it's
/// *possible* for `IME` to be written to false during this 2 cycle gap and you
/// end up having an interrupt request happen while `IME` is off.
pub const IME: VolAddress<bool, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0208) };

mod gba_cell;
pub use gba_cell::*;

/// A bit set where each bit is a particular interrupt source.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqBits(u16);

impl From<IrqBits> for u16 {
  #[inline]
  #[must_use]
  fn from(i: IrqBits) -> Self {
    i.0
  }
}

impl From<u16> for IrqBits {
  #[inline]
  #[must_use]
  fn from(u: u16) -> Self {
    Self(u)
  }
}

impl_bitops_for!(IrqBits);

#[allow(missing_docs)]
impl IrqBits {
  pub_const_fn_new!();
  u16_bool_field!(0, vblank, with_vblank);
  u16_bool_field!(1, hblank, with_hblank);
  u16_bool_field!(2, vcounter, with_vcounter);
  u16_bool_field!(3, timer0, with_timer0);
  u16_bool_field!(4, timer1, with_timer1);
  u16_bool_field!(5, timer2, with_timer2);
  u16_bool_field!(6, timer3, with_timer3);
  u16_bool_field!(7, serial, with_serial);
  u16_bool_field!(8, dma0, with_dma0);
  u16_bool_field!(9, dma1, with_dma1);
  u16_bool_field!(10, dma2, with_dma2);
  u16_bool_field!(11, dma3, with_dma3);
  u16_bool_field!(12, keypad, with_keypad);
  u16_bool_field!(13, gamepak, with_gamepak);

  pub const V_BLANK: Self = Self::new().with_vblank(true);
  pub const H_BLANK: Self = Self::new().with_hblank(true);
  pub const V_COUNTER: Self = Self::new().with_vcounter(true);
  pub const TIMER0: Self = Self::new().with_timer0(true);
  pub const TIMER1: Self = Self::new().with_timer1(true);
  pub const TIMER2: Self = Self::new().with_timer2(true);
  pub const TIMER3: Self = Self::new().with_timer3(true);
  pub const SERIAL: Self = Self::new().with_serial(true);
  pub const DMA0: Self = Self::new().with_dma0(true);
  pub const DMA1: Self = Self::new().with_dma1(true);
  pub const DMA2: Self = Self::new().with_dma2(true);
  pub const DMA3: Self = Self::new().with_dma3(true);
  pub const KEYPAD: Self = Self::new().with_keypad(true);
  pub const GAMEPAK: Self = Self::new().with_gamepak(true);
}
