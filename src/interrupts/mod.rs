use voladdress::*;

pub const IE: VolAddress<IrqBits, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0200) };
pub const IF: VolAddress<IrqBits, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0202) };
pub const IME: VolAddress<bool, Safe, Safe> =
  unsafe { VolAddress::new(0x0400_0208) };

mod gba_cell;
pub use gba_cell::*;

extern "C" {
  static RUST_IRQ_HANDLER: GbaCell<Option<extern "C" fn(IrqBits)>>;
}
pub fn set_rust_irq_handler(opt_f: Option<extern "C" fn(IrqBits)>) {
  unsafe { RUST_IRQ_HANDLER.write(opt_f) };
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct IrqBits(pub(crate) u16);

impl_bitops_for!(IrqBits);

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

/// Reads the address given, then writes a new value, then returns the old
/// value.
///
/// The entire operation happens as a single action, an interrupt cannot fire in
/// between the read and the write.
///
/// ## Safety
/// * The pointer must be valid for a standard [`read`](core::ptr::read)
/// * The pointer must be valid for a standard [`write`](core::ptr::write)
#[link_section = ".iwram"]
#[instruction_set(arm::a32)]
pub unsafe fn a32_swp_raw(new_val: u32, addr: *mut u32) -> u32 {
  let old_val: u32;
  core::arch::asm! {
    "swp {old_val}, {new_val}, [{addr}]",
    old_val = lateout(reg) old_val,
    new_val = in(reg) new_val,
    addr = in(reg) addr,
    options(nostack)
  }
  old_val
}

/// Works like [`a32_swp_raw`], but with `u8`.
#[link_section = ".iwram"]
#[instruction_set(arm::a32)]
pub unsafe fn a32_swpb_raw(new_val: u8, addr: *mut u8) -> u8 {
  let old_val: u8;
  core::arch::asm! {
    "swp {old_val}, {new_val}, [{addr}]",
    old_val = lateout(reg) old_val,
    new_val = in(reg) new_val,
    addr = in(reg) addr,
    options(nostack)
  }
  old_val
}
