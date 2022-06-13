#![no_std]
#![no_main]

core::arch::global_asm! {
  "
  .arm
  .section .text.gba_rom_header
  
  .global __start
  __start:
    b asm_init
    .space 0xE0

  asm_init:
    /* TODO: a full program would need a proper init sequence */
  
  call_to_rust_main:
    ldr r0, =main
    bx r0
  end_of_init_code:
  
  .previous
  .thumb
  ",
  options(raw)
}

#[no_mangle]
fn main() -> ! {
  // With this line active, the screen will be black. If you comment out this
  // line and then rebuild/rerun the screen will be white instead. That's proof
  // enough that we've made a binary that's doing *something*.
  unsafe { (0x0400_0000 as *mut u16).write_volatile(0x0403) };
  loop {}
}

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
