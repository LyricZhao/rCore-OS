/*
Physical memory status:
    OpenSBI: [0x80000000, 0x80200000)
    Kernel:  [0x80200000, ..)   <-> vaddr (0xffffffffc0200000)
*/

    .section .text.entry
    .globl _start

_start:
    // %hi(addr) for top 20 bits
    // boot_page_table_sv39 is 12 aligned (20 bits + 12 zeros)
    lui t0, %hi(boot_page_table_sv39)
    // t1 = offset (mapping from vaddr to paddr)
    li t1, 0xffffffffc0000000 - 0x80000000
    // t0 = paddr of page table
    sub t0, t0, t1
    // t0 = ppn of page table
    srli t0, t0, 12

    // satp (register) mode = Sv39
    li t1, 8 << 60
    // Add info of ppn into stap
    or t0, t0, t1
    // Write satp
    csrw satp, t0
    // TLB refresh
    sfence.vma

    // Now set sp as vaddr
    lui sp, %hi(boot_stack_top)

    // Jump into kernel entry
    lui t0, %hi(kernel_entry)
    addi t0, t0, %lo(kernel_entry)
    jr t0

    .section .bss.stack
    .align 12
    .global boot_stack

boot_stack:
    .space 4096 * 4
    .global boot_stack_top
boot_stack_top:

    // The page table must be in one physical page (12 aligned)
    // Mapping 0xffffffff_c0000000 to 0x80000000 (1G)
    // 1G has 512 physical page, 511 zeros
    // The last is (PPN: 0x80000, flags: VRWXAD (0xcf))
    .section .data
    .align 12
boot_page_table_sv39:
    .zero 8 * 511
    .quad (0x80000 << 10) | 0xcf