
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
