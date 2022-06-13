.code 32 /* paired with ".code 16" at the end of the file */

.section .text.gba_rom_header
  .global __start
  __start:
    b asm_init
    /* This space can be overwritten later with actual header data by `gbafix`.
    mgba doesn't actually run the header checksum that happens on hardware, so
    just having sufficient blank space is fine for development builds. */
    .space 0xE0

  asm_init:
    mov r12, #0x04000000

    set_waitcnt:
      /* This is the best default WAITCNT setting for most GBA carts.*/
      add r0, r12, #0x204
      ldr r1, =0x4317
      strh r1, [r0]

    iwram_copy:
      /* Check the number of bytes in (__iwram_end - __iwram_start), and skip
      this part if it's zero. */
      ldr r0, =__iwram_start
      ldr r1, =__iwram_end
      subs r4, r1, r0
      beq 1f
      /* Uses DMA3 to copy the required number of words from ROM to IWRAM. */
      add r3, r12, #0xD4 /* DMA3_BASE */
      ldr r2, =__iwram_position_in_rom
      str r2, [r3] /* set source */
      str r0, [r3, #4] /* set destination */
      lsr r4, r4, #2 /* Convert bytes to words */
      strh r4, [r3, #8] /* set word count */
      mov r5, #0x8400 /* 32-bit transfers, DMA Enabled */
      strh r5, [r3, #10] /* set control bits */
      1:
      /* After the DMA is set it takes 2 cycles to actually start, but we won't
      touch DMA again for at least that much time so it's fine. */
    
    bss_zeroing:
      /* Check the number of bytes in (__bss_end - __bss_start), and skip
      this part if it's zero. */
      ldr r0, =__bss_start
      ldr r1, =__bss_end
      subs r4, r1, r0
      beq 1f
      /* Zero out one word at a time until we're done. TODO: this should probably
      also use DMA3 to go faster. */
      mov r2, #0
      lsr r4, r4, #2 /* Convert bytes to words */
      .L_write_loop:
      str r2, [r0], #4
      subs r4, r4, #1
      bne .L_write_loop
      1:

    set_rt0_interrupt_handler:
      ldr r1, =rt0_irq_handler
      str r1, [r12, #-4]

  call_to_rust_main:
    ldr r0, =main
    bx r0

  /* Having this label just makes the `objdump` output look better. All of the
  constants inserted by the assembler when we use "ldr reg,=label" and similar
  will end up going just after this label, and then objdump will list all those
  constants in their own paragraph. */
  end_of_init_code:
.previous

.section .iwram
  .align 4
  rt0_irq_handler:
    /*
    This function is entered in Supervisor mode
    r0 == 0x04000000
    we are free to use ?? and must preserve all other registers
    */
    update_if_bits:
      /* This acknowledges all interrupts to the hardware. To do this
      need to write a `1` bit to any bit that's set in both IE and IF. For
      efficiency we 32-bit read from 0x04000200, then we bitand the top and
      bottom half of that, and write back to IF. */
      add   r2, r0, #0x200
      ldr   r3, [r2]
      and   r3, r0, r0, lsr #16
      strh  r3, [r2, #2]
    
    update_bios_if_bits:
      /* This acknowledges all interrupts to the BIOS. To so this we need to
      read BIOS_IF, then bitor any interrupt bits we just handled above, and
      then write it back. */
      ldrh  r1, [r0, #-8]
      orr   r1, r1, r3
      strh  r1, [r0, #-8]
    
    load_rust_handler_fn:
      ldr   r1, =RUST_IRQ_HANDLER
      ldr   r1, [r1]
      cmp   r1, #0
      beq   end_of_handler

    switch_to_sys_mode_and_call_fn:
      add   r2, r2, #8
      mov   r12, #0
      swp   r12, r12, [r2]
      mrs   r3, SPSR
      push  {r3}
      mov   r3, #0b11111
      msr   CPSR_cf, r3
      mov   r3, sp
      bic   sp, sp, #7
      push  {r2, r3, r12, lr}
      adr   lr, restore_state_from_fn_call
      bx    r1
    
    restore_state_from_fn_call:
      pop   {r2, r3, r12, lr}
      mov   sp, r3
      mov   r3, #0b10010010
      msr   CPSR_cf, r3
      pop   {r3}
      msr   SPSR, r3
      swp   r12, r12, [r2]
    
    end_of_handler:
      bx    lr
.previous

.section .bss
  .align 4
  .global RUST_IRQ_HANDLER
  /* RUST_IRQ_HANDLER: Option<extern "C" fn(IrqBits)> = None; */
  RUST_IRQ_HANDLER:
    .space 4
.previous

.code 16
