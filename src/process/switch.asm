// TODO: figure out wtf this file is talking about

.equ WSIZE, 8 // Word size

// LOAD value at {sp + 8 * a2} into a1
.macro LOAD a1, a2
    ld \a1, \a2 * WSIZE(sp)
.endm

// STORE a1 into address {sp + 8 * a2}
.macro STORE a1, a2
    sd \a1, \a2 * WSIZE(sp)
.endm

    // Allocate space for saving context
    addi sp, sp, -14 * WSIZE
    // Switch to a new stack top
    sd sp, 0(a0)
    // Save registers
    STORE ra, 0
    STORE s0, 2
    STORE s1, 3
    STORE s2, 4
    STORE s3, 5
    STORE s4, 6
    STORE s5, 7
    STORE s6, 8
    STORE s7, 9
    STORE s8, 10
    STORE s9, 11
    STORE s10, 12
    STORE s11, 13
    csrr s11, satp
    STORE s11, 1

    // Switch
    ld sp, 0(a1)
    // Restore
    LOAD s11, 1
    csrw satp, s11
    // TLB Refresh
    sfence.vma
    LOAD ra, 0
    LOAD s0, 2
    LOAD s1, 3
    LOAD s2, 4
    LOAD s3, 5
    LOAD s4, 6
    LOAD s5, 7
    LOAD s6, 8
    LOAD s7, 9
    LOAD s8, 10
    LOAD s9, 11
    LOAD s10, 12
    LOAD s11, 13
    addi sp, sp, 14 * WSIZE

    // Start running
    sd zero, 0(a1)
    ret