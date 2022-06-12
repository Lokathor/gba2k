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
    /* TODO */
  
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
  unsafe { (0x0400_0000 as *mut u16).write_volatile(0x0403) };
  loop {}
}

#[panic_handler]
fn the_panic_handler(_: &core::panic::PanicInfo) -> ! {
  loop {}
}
