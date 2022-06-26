/// `swi #0x02`: Halts the CPU until an interrupt request occurs.
///
/// The CPU is placed into low-power mode, while other parts (video, sound,
/// timers, serial, keypad) continue to operate.
///
/// This low-power mode only terminates when one of the interrupts set in
/// [`IE`](crate::interrupts::IE) occurs.
///
/// If [`IME`](crate::interrupts::IME) is set then the interrupt handler will be
/// called when the CPU wakes and before this function returns. Otherwise the
/// CPU will wake up when the interrupt occurs without calling the interrupt
/// handler.
#[inline]
#[instruction_set(arm::t32)]
pub fn Halt() {
  // Note(Lokathor): I checked and double checked, `Halt` doesn't trash any of
  // the registers.
  unsafe {
    core::arch::asm! {
      "swi #0x02",
      options(preserves_flags)
    }
  }
}
