#![no_std]
//#![warn(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_abi)]
#![feature(isa_attribute)]

core::arch::global_asm! {
  include_str!("rt0.s"),
  options(raw)
}
extern "C" {
  /// A `SWP` instruction, stored in the ROM.
  ///
  /// ## Safety
  /// * The pointer must be valid for a read.
  /// * The pointer must be valid for a write.
  pub fn rt0_rom_swp(new_val: u32, addr: *mut u32) -> u32;

  /// A `SWPB` instruction, stored in the ROM.
  ///
  /// ## Safety
  /// * The pointer must be valid for a read.
  /// * The pointer must be valid for a write.
  pub fn rt0_rom_swpb(new_val: u8, addr: *mut u8) -> u8;
}

#[macro_use]
mod macros;
mod bit_utils;

pub mod bios;
pub mod interrupts;
pub mod keyinput;
pub mod video;

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}

/// Reads the stack pointer value.
///
/// The stack pointer doesn't usually move by much on the GBA.
#[macro_export]
macro_rules! read_sp {
  () => {{
    let sp_output: u32;
    unsafe {
      // * T32: `mov` between high and low registers doesn't set flags
      // * A32: `mov` without `s` on the end doesn't set flags.
      core::arch::asm! {
        "mov {sp_val}, sp",
        sp_val = out(reg) sp_output,
        options(nomem, nostack, preserves_flags)
      }
    }
    sp_output
  }};
}

/// Reads the link register value.
///
/// This is the "return address" of the current function.
#[macro_export]
macro_rules! read_lr {
  () => {{
    let lr_output: u32;
    unsafe {
      // * T32: `mov` between high and low registers doesn't set flags
      // * A32: `mov` without `s` on the end doesn't set flags.
      core::arch::asm! {
        "mov {lr_val}, lr",
        lr_val = out(reg) lr_output,
        options(nomem, nostack, preserves_flags)
      }
    }
    lr_output
  }};
}

/// Reads the program counter value.
///
/// Because of the CPU's pipeline, the program counter will hold the address of
/// the instruction that's *two* instructions ahead of the current instruction.
#[macro_export]
macro_rules! read_pc {
  () => {{
    let pc_output: u32;
    unsafe {
      // * T32: `mov` between high and low registers doesn't set flags
      // * A32: `mov` without `s` on the end doesn't set flags.
      core::arch::asm! {
        "mov {pc_val}, pc",
        pc_val = out(reg) pc_output,
        options(nomem, nostack, preserves_flags)
      }
    }
    pc_output
  }};
}
