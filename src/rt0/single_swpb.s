/*
unsafe extern "C" fn text_single_swpb(new_val: u8, addr: *mut u8) -> u8
*/
.global text_single_swpb

.section .text.single.swpb
  .code 32
  .balign 4
  text_single_swpb:
    swpb r0, r0, [r1]
    bx lr
  .code 16
.previous
