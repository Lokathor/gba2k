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
    /* The `main` function should *not* return, but we'll set the link register
    anyway so that if the programmer defines a main that returns on accident we
    can just go back to the start of the program. */
    ldr lr, =asm_init
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
    When a hardware interrupt occurs the CPU switches into Supervisor mode and
    execution goes into the BIOS, which does the following steps:

    * push r0-r3, r12, and lr
    * set r0 to 0x04000000
    * set lr to after the call to the program handler
    * load the program's IRQ handler (at 0x04000000 - 4)
    * call the program handler
    * pop all the pushed registers
    * return from the hardware interrupt

    Our program's handler function is supposed to acknowledge any flagged
    interrupts before returning. Any flags that aren't acknowledged will trigger
    another hardware interrupt after the current one returns, so it's most
    efficient to just acknowledge all of them at once.

    Also, to work with the IntrWait functions, the BIOS_IF value should be
    updated as well.

    Most programs want to do something during the interrupt besides just
    acknowledge it. To make this part easily configurable from Rust the rt0 code
    provides a `RUST_IRQ_HANDLER` variable which the program can set. When it's
    non-null the rt0 handler will call that function with the flags of what
    interrupts were just acknowledged before returning to the BIOS.
    */
    swap_ime_off:
      /* We don't want IME to be active during our handler. However, we can't
      strictly assume that IME is on when our handler is running. There's a 2
      cycle delay between an interrupt triggering and it actually changing over
      the CPU. It's possible for IME to be turned off during this window. For
      maximum robustness we need to swap IME to 0 and then swap it back later.
      This ensures that we always restore the correct IME value. */
      add   r2, r0, #0x208
      mov   r12, #0
      swp   r12, r12, [r2]

    update_if_bits:
      /* This acknowledges all interrupts to the hardware. To do this need to
      write a `1` bit to any bit that's set in both IE and IF. For efficiency we
      do a 32-bit read from 0x04000200, then we bitand the top and bottom half
      of that, and write back to IF. */
      ldr   r3, [r2, #-8]
      and   r3, r0, r0, lsr #16
      strh  r3, [r2, #-6]
    
    update_bios_if_bits:
      /* This acknowledges all interrupts to the BIOS_IF. This is a normal
      variable, not an MMIO. We need to read BIOS_IF, then bitor that with the
      new interrupt bits we just got from above, and then write it back. */
      ldrh  r1, [r0, #-8]
      orr   r1, r1, r3
      strh  r1, [r0, #-8]
    
    load_rust_handler_fn:
      /* Read the `RUST_IRQ_HANDLER` value and skip past calling the function if
      it's null. */
      ldr   r1, =RUST_IRQ_HANDLER
      ldr   r1, [r1]
      cmp   r1, #0
      beq   end_of_handler

    switch_to_sys_mode_and_call_fn:
      /* Now we need to switch the CPU from Supervisor mode to System mode. The
      old flags are in SPSR, and the switching will overwrite it, so first we
      need to read the current SPSR and push it onto the Supervisor stack. Then
      we can write to the "CPSR control flag" to change to System mode. */
      mrs   r3, SPSR
      push  {r3}
      mov   r3, #0b00011111
      msr   CPSR_cf, r3
      /* Now we're in System mode, but it's possible the stack might not be
      aligned to 8 (it might only be aligned to 4). This shouldn't ever matter,
      but that's what the ABI calls for, and it's a very small step to conform
      to the ABI, so we might as well. To do this, we first save SP into a temp
      register and then clear the lowest 3 bits of SP. This effectively "pushes"
      sufficient stack junk so that SP is aligned. */
      mov   r3, sp
      bic   sp, sp, #7
      /* Now we're almost ready to do the call, but there's a few registers we
      need to save for after the call. Also, now that our stack is aligned to 8,
      we need to push an even number of registers to the stack so that it stays
      aligned to 8. The register we want to save are as follows:

      * lr: the return address for our rt0 function
      * r12: the swapped IME value
      * r3: the old SP value
      * r2: This holds the IME address, which we could regenerate if we needed
        to, but we need to save an even number of registers and we'll use the
        IME address, so we might as well save this one.

      After we've saved our registers we can set lr to return here and make the
      call to the Rust function.
      */
      push  {r2, r3, r12, lr}
      adr   lr, restore_state_from_fn_call
      bx    r1
    
    restore_state_from_fn_call:
      /* After the Rust function returns we undo all the steps we did to get
      here:

      * pop all of our saved registers off of the System stack
      * go back to Supervisor mode
      * pop the saved SPSR off of the Supervisor stack
      * swap IME back to its previous state
      */
      pop   {r2, r3, r12, lr}
      mov   sp, r3
      mov   r3, #0b10010010
      msr   CPSR_cf, r3
      pop   {r3}
      msr   SPSR, r3
    
    end_of_handler:
      swp   r12, r12, [r2]
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
