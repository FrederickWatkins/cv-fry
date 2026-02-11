// Define the base address for verification (adjust to your linker script)
#define TEST_RESULTS_BASE 0x00002000 

void main() {
    volatile int* results = (volatile int*)TEST_RESULTS_BASE;
    int i = 0;

    // --- 1. Computational & Logical (Testing ADDI, SLTI, SLTIU, ANDI, ORI, XORI) ---
    int a = 0x12345678;
    int b = 0xFEDCBA98;
    
    results[i++] = a + 0x7FF;          // 1. ADDI (max positive immediate)
    results[i++] = a + (-0x800);       // 2. ADDI (max negative immediate)
    results[i++] = (a < 0x100);        // 3. SLTI
    results[i++] = ((unsigned int)a < 0x7FFFFFFF); // 4. SLTIU
    results[i++] = a ^ 0xFFF;          // 5. XORI
    results[i++] = a | 0x001;          // 6. ORI
    results[i++] = a & 0xAAA;          // 7. ANDI

    asm volatile("nop");

    // --- 2. Shift Operations (Testing SLLI, SRLI, SRAI) ---
    int s = 0x8000000F;
    results[i++] = s << 4;             // 8. SLLI
    results[i++] = (unsigned int)s >> 4; // 9. SRLI (Logical)
    results[i++] = s >> 4;             // 10. SRAI (Arithmetic - should preserve sign)

    asm volatile("nop");

    // --- 3. Register-Register Ops (Testing ADD, SUB, SLL, SLT, SLTU, XOR, SRL, SRA, OR, AND) ---
    int reg_a = 0x55555555;
    int reg_b = 0xAAAAAAAA;
    results[i++] = reg_a + reg_b;      // 11. ADD
    results[i++] = reg_a - reg_b;      // 12. SUB
    results[i++] = reg_a ^ reg_b;      // 13. XOR
    results[i++] = reg_a | reg_b;      // 14. OR
    results[i++] = reg_a & reg_b;      // 15. AND
    results[i++] = (reg_a < reg_b);    // 16. SLT (Signed)

    asm volatile("nop");

    // --- 4. Branching & Control Flow (Testing BEQ, BNE, BLT, BGE, BLTU, BGEU) ---
    // We use a simple checksum to verify branches were taken correctly
    int branch_check = 0;
    if (reg_a != reg_b) branch_check += 1;  // BNE
    if (reg_a == reg_a) branch_check += 2;  // BEQ
    if (reg_a < reg_b)  branch_check += 4;  // BLT (Should be false for signed)
    if ((unsigned int)reg_b > (unsigned int)reg_a) branch_check += 8; // BGEU/BLTU
    
    results[i++] = branch_check; // 17.

    asm volatile("nop");

    // --- 5. Memory Access (Testing LB, LH, LW, LBU, LHU, SB, SH, SW) ---
    // We'll write to a local buffer and read back with different widths
    volatile unsigned char data_buf[8] = {0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88};
    
    results[i++] = *((volatile int*)&data_buf[0]);  // 18. LW
    results[i++] = *((volatile short*)&data_buf[4]); // 19. LH
    results[i++] = *((volatile signed char*)&data_buf[7]); // 20. LB (Sign extended)
    results[i++] = *((volatile unsigned char*)&data_buf[7]); // 21. LBU (Zero extended)

    asm volatile("nop");

    // --- 6. Upper Immediates (Testing LUI, AUIPC) ---
    unsigned int upper = 0xABCDE000;   // LUI
    results[i++] = upper; // 22.

    asm volatile("nop");
    
    // Final marker to signal completion
    results[i++] = 0xDEADBEEF; //23.

    return;
}