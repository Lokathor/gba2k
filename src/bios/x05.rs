/// `swi #0x05`: Performs an "interrupt wait" for a new Vertical-blank
/// Interrupt.
///
/// This is effectively just an alternate way to write
/// ```no_run
/// # gba2k::bios::IntrWait;
/// # gba2k::interrupts::IrqBits;
/// IntrWait(true, IrqBits::V_BLANK);
/// ```
///
/// * See: [`IntrWait`](crate::bios::IntrWait)
#[inline]
#[instruction_set(arm::t32)]
pub fn VBlankIntrWait() {
  unsafe {
    core::arch::asm! {
      "swi #0x05",
      out("r0") _,
      out("r1") _,
      out("r3") _,
      options(preserves_flags),
    }
  };
}
