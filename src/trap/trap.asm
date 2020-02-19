.equ WSIZE, 8 // Word size

// Load value at {sp + 8 * a2} into a1
.macro LOAD a1, a2
    ld \a1, \a2 * WSIZE(sp)
.endm

// Store a1 into address {sp + 8 * a2}
.macro STORE a1, a2
    sd \a1, \a2 * WSIZE(sp)
.endm

// Save the status
.macro SAVE_ALL
    // Atomically swap sp and sscratch
    // CSR: Control and Status Registers
    // csrrw x0, csr, rs1: swap (csr, rs1), write csr into x0
    // sscratch: kernel stack (U mode), 0 (S mode)
    csrrw sp, sscratch, sp

    // If sp = 0, which means sscratch = 0 (before swapping)
    //  => trapped in S mode
    //  => now: sp, sscratch all for kernel stack
    // Else
    //  => trapped in U mode
    //  => now: sp for kernel stack, sscratch for user stack
    //  => skip the csrr instruction
    bnez sp, trap_from_user

trap_from_kernel:
    // csrr rd, csr (read csr into rd)
    csrr sp, sscratch

trap_from_user:
    // Allocate frame stack
    addi sp, sp, -36 * WSIZE

    // Store registers except x0 (always 0), x2 (sp)
    STORE x1, 1
	STORE x3, 3
	STORE x4, 4
	STORE x5, 5
	STORE x6, 6
	STORE x7, 7
	STORE x8, 8
	STORE x9, 9
	STORE x10, 10
	STORE x11, 11
    STORE x12, 12
    STORE x13, 13
    STORE x14, 14
    STORE x15, 15
    STORE x16, 16
    STORE x17, 17
    STORE x18, 18
    STORE x19, 19
    STORE x20, 20
    STORE x21, 21
    STORE x22, 22
    STORE x23, 23
    STORE x24, 24
    STORE x25, 25
    STORE x26, 26
    STORE x27, 27
    STORE x28, 28
    STORE x29, 29
    STORE x30, 30
    STORE x31, 31

    // sscratch (now): kernel stack (from S mode), user stack (from U mode)
    // save sscratch into s0, let sccratch = 0
    csrrw s0, sscratch, x0

    // read another 4 registers into s[1-4]
    csrr s1, sstatus
    csrr s2, sepc
    csrr s3, stval
    csrr s4, scause

    // store s[0-4] into stack
    STORE s0, 2
    STORE s1, 32
    STORE s2, 33
    STORE s3, 34
    STORE s4, 35
.endm

// Restore the status
.macro RESTORE_ALL
    // s1 = sstatus
    LOAD s1, 32

    // s2 = sepc
    LOAD s2, 33

    // Judge whether entered from S mode (or U mode)
    //  => sstatus.SPP = 1 (S mode)
    andi s0, s1, 1 << 8

    // Whether SPP != 0
    bnez s0, to_kernel

to_user:
    // Release the space on stack
    addi s0, sp, 36 * WSIZE

    // let sscratch = s0 (kernel stack)
    csrw sscratch, s0

to_kernel:
    // Restore sstatus, sepc
    csrw sstatus, s1
    csrw sepc, s2

    // Restore registers except x0 and x2
    LOAD x1, 1
    LOAD x3, 3
    LOAD x4, 4
    LOAD x5, 5
    LOAD x6, 6
    LOAD x7, 7
    LOAD x8, 8
    LOAD x9, 9
    LOAD x10, 10
    LOAD x11, 11
    LOAD x12, 12
    LOAD x13, 13
    LOAD x14, 14
    LOAD x15, 15
    LOAD x16, 16
    LOAD x17, 17
    LOAD x18, 18
    LOAD x19, 19
    LOAD x20, 20
    LOAD x21, 21
    LOAD x22, 22
    LOAD x23, 23
    LOAD x24, 24
    LOAD x25, 25
    LOAD x26, 26
    LOAD x27, 27
    LOAD x28, 28
    LOAD x29, 29
    LOAD x30, 30
    LOAD x31, 31

    // sp = kernel stack or user stack
    LOAD x2, 2
.endm

    .section .text
    .globl __traps_entry

__trap_entry:
    SAVE_ALL
    mv a0, sp
    jal trap_handler

    .globl __trap_ret

__trap_ret:
    RESTORE_ALL
    sret