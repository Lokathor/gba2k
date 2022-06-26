/* RUST_IRQ_HANDLER: Option<extern "C" fn(IrqBits)> = None; */
.global RUST_IRQ_HANDLER

.section .bss.rust_irq_handler_fn_ptr
  .balign 4
  RUST_IRQ_HANDLER:
    .space 4
.previous

/* This fn can only be called by the GBA's BIOS IRQ handling. */
.global rt0_irq_handler

.section .iwram.rt0_irq_handler
  .code 32
  .balign 4
  rt0_irq_handler:
    .L_handle_irq_with_interrupts_off:
    add r12, r0, #0x208 @r12=&IME
    mov r3, #0
    swp r3, r3, [r12]   @IME swap off
    /* Still Important
    * r12, IME
    * r3, ime_previous
    */

    .L_read_update_hardware_flags:
    ldr r0, [r12, #-8]      @r0=IE_IF
    and r0, r0, r0, LSR #16 @r0=IE&IF
    strh r0, [r12, #-6]     @IF=r0
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    .L_read_update_bios_flags:
    sub  r2, r12, #(0x208+8) @r2=&BIOS_IW
    ldrh r1, [r2]            @r1=BIOS_IW
    orr  r1, r1, r0          @r1=r1|r0
    strh r1, [r2]            @BIOS_IW=r0
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r0, irq_flags
    */

    .L_get_rust_fn_ptr:
    ldr r1, =RUST_IRQ_HANDLER
    ldr r1, [r1]       @r1==RUST_IRQ_HANDLER
    cmp r1, #0         @if r1==0
    beq .L_end_of_rt0  @then branch
    /* Still Important
    * r12, IME
    * r3, ime_previous
    * r1, rust_irq_fn
    * r0, irq_flags
    */

    .L_call_rust_fn_in_sys_mode:
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

    .L_end_of_rt0:
    mov r12, #0x04000000
    add r12, r12, #0x208
    swp r3, r3, [r12]  @IME swap previous
    bx lr              @return
  .code 16
.previous