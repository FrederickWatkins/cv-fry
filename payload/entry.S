# Machine Mode Entry Point for RV32I
.section .text.init
.global _start

_start:
    nop
    nop
    nop
    nop
    nop
    /* * 1. Initialize the Stack Pointer (sp)
     * _stack_top is defined in the linker script.
     * We use the 'la' pseudo-instruction which expands to LUI/ADDI
     */
    la sp, _stack_top

    /* * 2. Initialize the Global Pointer (gp) 
     * (Optional but recommended for linker relaxation optimization)
     * The compiler/linker calculates __global_pointer$
     */
    .option push
    .option norelax
    la gp, __global_pointer$
    .option pop

    /* * 3. Clear the BSS section (Uninitialized variables)
     * C standards require uninitialized globals to be zero.
     * We loop from _bss_start to _bss_end and store 0.
     */
    la t0, _bss_start
    la t1, _bss_end
    
    /* If bss_start == bss_end, skip the loop */
    beq t0, t1, call_main

bss_clear_loop:
    sw zero, 0(t0)      # Store 0 at current address
    addi t0, t0, 4      # Increment address by 4 bytes
    bltu t0, t1, bss_clear_loop # Continue if t0 < t1

call_main:
    /* * 4. Call the main C function
     * We use JAL (Jump and Link) to call 'main'
     */
    jal ra, main

    /* * 5. Trap / Infinite Loop
     * If main returns, we have nowhere to go.
     * We loop infinitely to stop the PC from wandering.
     */
end_loop:
    j end_loop