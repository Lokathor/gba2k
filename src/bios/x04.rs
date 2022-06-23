use crate::interrupts::IrqBits;

/// `swi #0x04`: Performs an "interrupt wait".
///
/// This function:
/// * Forces [`IME`](crate::interrupts::IME) to be enabled.
/// * Halts the CPU (until an interrupt).
/// * Checks if `target_irqs & IntrWaitFlags` has any bits set. If so, all bits
///   set in `target_irqs` are cleared from the `IntrWaitFlags` value and the
///   function returns. Otherwise the CPU will loop and halt again.
///
/// If you want the main program to wait until after a specific type of
/// interrupt has occurred, using this function is significantly more efficient
/// then repeatedly calling [`Halt`](crate::bios::Halt) yourself.
///
/// If the `clear_old_flags` value is `true` then all `target_irqs` bits in
/// `IntrWaitFlags` will be cleared before the halt loop begins, ensuring that
/// the function only returns once a *new* interrupt of the desired type(s) has
/// occurred.
///
/// The `IME` register is left enabled even after the function returns.
///
/// Note: The `IntrWaitFlags` are automatically updated by the assembly runtime
/// whenever an interrupt occurs. Your own interrupt handler does not (and
/// should not) need to update the value itself.
///
/// Note: If you attempt to wait on an interrupt that is not correctly enabled
/// to occur the function won't ever return.
#[inline]
#[instruction_set(arm::t32)]
pub fn IntrWait(clear_old_flags: bool, target_irqs: IrqBits) {
  unsafe {
    core::arch::asm! {
      "swi #0x04",
      inout("r0") clear_old_flags as u32 => _,
      inout("r1") target_irqs.0 => _,
      out("r3") _,
      options(preserves_flags)
    }
  }
}
