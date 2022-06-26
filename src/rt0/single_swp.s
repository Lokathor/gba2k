/*
unsafe extern "C" fn text_single_swp(new_val: u32, addr: *mut u32) -> u32
*/
.global text_single_swp

.section .text.single.swp
  .code 32
  .balign 4
  text_single_swp:
    swp r0, r0, [r1]
    bx lr
  .code 16
.previous
