// Define the base address for verification
#define TEST_RESULTS_BASE 0x00002000 

void main() {
    // RV64: We write 64-bit results (long long)
    volatile unsigned long long* results = (volatile unsigned long long*)TEST_RESULTS_BASE;
    int i = 0;

    // --- 1. 64-bit Computational (ADDI, SLTI, ANDI, ORI, XORI) ---
    // Using 'long long' forces 64-bit registers/operations
    long long a = 0x1122334455667788; 
    
    results[i++] = a + 0x111;           // [0] ADDI (64-bit add)
    results[i++] = (a < 0x2222222222222222); // [1] SLTI (True)
    results[i++] = a ^ 0xF0F0F0F0F0F0F0F0;   // [2] XORI
    results[i++] = a | 0x00000000FFFFFFFF;   // [3] ORI
    results[i++] = a & 0xFFFF00000000FFFF;   // [4] ANDI

    // --- 2. 64-bit Shift Operations (SLLI, SRLI, SRAI) ---
    long long s = 0x800000000000000F; // Negative signed 64-bit
    results[i++] = s << 4;              // [5] SLLI
    results[i++] = (unsigned long long)s >> 4; // [6] SRLI (Logical)
    results[i++] = s >> 4;              // [7] SRAI (Arithmetic - sign preserved)

    // --- 3. Word (32-bit) Operations (ADDW, SUBW, SLLW, etc.) ---
    // These instructions operate on lower 32-bits and SIGN-EXTEND the result to 64-bits.
    // We explicitly cast to int to ensure W-ops, then cast back to long long to check extension.
    int w1 = 0x40000000;
    int w2 = 0x40000000;
    
    // 0x40000000 + 0x40000000 = 0x80000000 (-2147483648 in 32-bit signed)
    // In RV64, this MUST become 0xFFFFFFFF80000000
    results[i++] = (long long)(w1 + w2); // [8] ADDW Check (Sign Extension)
    
    int w3 = 0x12345678;
    results[i++] = (long long)(w3 << 4); // [9] SLLW (Sign extends 32-bit result)
    
    int w4 = 0xF0000000;
    results[i++] = (long long)(w4 >> 4); // [10] SRAW (Arithmetic Right Word)

    // --- 4. M Extension: Multiplication & Division (MUL, DIV, REM) ---
    long long m1 = 100;
    long long m2 = -10;
    
    results[i++] = m1 * m2;    // [11] MUL (64-bit) -> -1000
    results[i++] = m1 / m2;    // [12] DIV (64-bit) -> -10
    results[i++] = m1 % 30;    // [13] REM (64-bit) -> 10

    // M Extension Word Operations (MULW, DIVW)
    int mw1 = 0x00010000;
    int mw2 = 0x00010000;
    // 0x10000 * 0x10000 = 0x100000000. 
    // 32-bit MULW truncates to 0x00000000.
    results[i++] = (long long)(mw1 * mw2); // [14] MULW

    // --- 5. Register-Register Ops (ADD, SUB, XOR, OR, AND) ---
    long long reg_a = 0x00000000AAAAAAAA;
    long long reg_b = 0x0000000055555555;
    results[i++] = reg_a + reg_b; // [15] ADD -> 0xFFFFFFFF
    results[i++] = reg_a - reg_b; // [16] SUB -> 0x55555555
    results[i++] = reg_a | reg_b; // [17] OR  -> 0xFFFFFFFF
    results[i++] = reg_a & reg_b; // [18] AND -> 0x00000000

    // --- 6. Memory Access (LD, LWU, LW, LH, LB) ---
    // Buffer: 11 22 33 44 55 66 77 88
    volatile unsigned char data_buf[8] = {0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88};
    
    // LD (Load Double - 64 bits)
    results[i++] = *((volatile unsigned long long*)&data_buf[0]); // [19] LD
    
    // LWU (Load Word Unsigned - 32 bits zero extended)
    // data_buf[0..3] is 0x44332211. LWU -> 0x0000000044332211
    results[i++] = *((volatile unsigned int*)&data_buf[0]); // [20] LWU
    
    // LW (Load Word Signed - 32 bits sign extended)
    // We use 0x88... to force negative check.
    volatile unsigned char neg_buf[4] = {0xFF, 0xFF, 0xFF, 0xFF};
    results[i++] = *((volatile int*)&neg_buf[0]); // [21] LW (-1 sign extended to 64-bit -1)

    // Final marker
    results[i++] = 0xDEADBEEFDEADBEEF; // [22]

    return;
}