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
    /* We're gonna "pin" r12 to be MMIO_BASE during our initialization. */
    mov r12, #0x04000000

    set_waitcnt:
      /* This is the best default WAITCNT setting for most GBA carts.*/
      add r0, r12, #0x204
      ldr r1, =0x4317
      strh r1, [r0]

    iwram_copy:
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
    
    bss_zeroing:
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

    set_rt0_interrupt_handler:
      ldr r1, =rt0_irq_handler
      str r1, [r12, #-4]

  call_to_rust_main:
    /* The `main` function should *not* return, but we'll set the link register
    anyway so that if the programmer defines a main that returns on accident we
    can just go back to the start of the program. */
    adr lr, asm_init
    ldr r0, =main
    bx r0

  /* Having this label just makes the `objdump` output look better. All of the
  constants inserted by the assembler when we use "ldr reg,=label" and similar
  will end up going just after this label, and then objdump will list all those
  constants in their own paragraph. */
  end_of_init_code:
.previous

.section .text.single_instructions.swp
  /*
  unsafe extern "C" fn rt0_rom_swp(new_val: u32, addr: *mut u32) -> u32
  */
  .align 4
  .global rt0_rom_swp
  rt0_rom_swp:
    swp r0, r0, [r1]
    bx lr
.previous

.section .text.single_instructions.swpb
  /*
  unsafe extern "C" fn rt0_rom_swpb(new_val: u8, addr: *mut u8) -> u8
  */
  .align 4
  .global rt0_rom_swpb
  rt0_rom_swpb:
    swpb r0, r0, [r1]
    bx lr
.previous

.section .iwram.rt0_irq_handler
  .align 4
  rt0_irq_handler:
    handle_irq_with_interrupts_off:
    add r12, r0, #0x208 @r12=&IME
    mov r3, #0
    swp r3, r3, [r12]   @IME swap off
    /* Still Important
    * r12, IME
    * r3, ime_previous
    */

    read_update_hardware_flags:
    ldr r0, [r12, #-8]      @r0=IE_IF
    and r0, r0, r0, LSR #16 @r0=IE&IF
    strh r0, [r12, #-6]     @IF=r0
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    read_update_bios_flags:
    sub  r2, r12, #(0x208+8) @r2=&BIOS_IW
    ldrh r1, [r2]            @r1=BIOS_IW
    orr  r1, r1, r0          @r1=r1|r0
    strh r1, [r2]            @BIOS_IW=r0
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    get_rust_fn_ptr:
    ldr r1, =RUST_IRQ_HANDLER
    ldr r1, [r1]       @r1==RUST_IRQ_HANDLER
    cmp r1, #0         @if r1==0
    beq end_of_rt0     @then branch
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r1, rust_irq_fn
    * r0, irq_flags
    */

    call_rust_fn_in_sys_mode:
    mrs r2, SPSR      @save SPSR
    push {r0, r2}     @push SPSR (SVC)

    mov r2, #0b00011111
    msr CPSR_cf, r2   @set SYS mode

    /* We need to push an even number of registers here. We also need to save,
    at minimum, r3 (ime_previous) and lr (return_address). We could also save
    r12 and any junk register, but that costs +2 cycles before *and* after the
    call, and just rebuilding the r12 value after is only 2 cycles.
    */
    push {r3, lr} @push regs (SYS)

    adr lr, 1f
    bx r1
    1:

    pop {r3, lr} @pop regs (SYS)

    mov r2, #0b10010010
    msr CPSR_cf, r2   @set SVC mode

    pop {r0, r2}      @pop SPSR (SVC)
    msr SPSR, r2      @restore SPSR
    /* Still Important
    * r3, ime_previous
    */

    end_of_rt0:
    mov r12, #0x04000000
    add r12, r12, #0x208
    swp r3, r3, [r12]  @IME swap previous
    bx lr              @return
.previous

.section .bss.rust_irq_handler_fn_ptr
  .align 4
  .global RUST_IRQ_HANDLER
  /* RUST_IRQ_HANDLER: Option<extern "C" fn(IrqBits)> = None; */
  RUST_IRQ_HANDLER:
    .space 4
.previous

.code 16
