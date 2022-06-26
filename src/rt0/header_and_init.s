/* import */
.global rt0_irq_handler

/* unsafe extern "C" fn() -> ! */
.global __start

.section .text.gba_rom_header
  .code 32
  .balign 4
  __start:
    b system_init
    /* This space can be overwritten later with actual header data by `gbafix`.
    mgba doesn't actually run the header checksum that happens on hardware, so
    just having sufficient blank space is fine for development builds. */
    .space 0xE0

  system_init:
    /* We're gonna "pin" r12 to be MMIO_BASE during our initialization. */
    mov r12, #0x04000000

    .L_set_waitcnt:
      /* This is the best default WAITCNT setting for most GBA carts.*/
      add r0, r12, #0x204
      ldr r1, =0x4317
      strh r1, [r0]

    .L_iwram_copy:
      /* If our iwram copy is 0 words we skip this bit. But, it's almost never
      gonna be zero words. */
      ldr r4, =__iwram_word_copy_count
      cmp r4, #0
      beq 1f
      ldr r0, =__iwram_start
      add r3, r12, #0xD4 /* DMA3_BASE */
      ldr r2, =__iwram_position_in_rom
      str r2, [r3] /* set source */
      str r0, [r3, #4] /* set destination */
      strh r4, [r3, #8] /* set word count */
      mov r5, #0x8400 /* 32-bit transfers, DMA Enabled */
      strh r5, [r3, #10] /* set control bits */
      1:
      /* After the DMA is set it takes 2 cycles to actually start, but we won't
      touch DMA again for at least that much time so it's fine. */
    
    .L_bss_zeroing:
      /* Similarly to iwram, we skip this block if there's no words to clear. */
      ldr r4, =__bss_word_clear_count
      cmp r4, #0
      beq 1f
      ldr r0, =__bss_start
      /* Zero out one word at a time. TODO: use DMA? */
      mov r2, #0
      .L_write_loop:
      str r2, [r0], #4
      subs r4, r4, #1
      bne .L_write_loop
      1:

    .L_set_rt0_interrupt_handler:
      ldr r1, =rt0_irq_handler
      str r1, [r12, #-4]

    .L_call_to_rust_main:
      /* The `main` function should *not* return, but we'll set the link register
      anyway so that if the programmer defines a main that returns on accident we
      can just go back to the start of the program. */
      adr lr, system_init
      ldr r0, =main
      bx r0

  /* Having this extra label just makes the `objdump` output look better. All of
  the constants inserted by the assembler when we use "ldr reg,=label" and
  similar will end up going just after this label, and then objdump will list
  all those constants in their own paragraph. */
  system_init_literal_pool:
  .code 16
.previous
