.section .text.entry
.globl _start

_start:
    la sp, boot_stack_top
    call kernel_entry

    .section .bss.stack
    .align 12
    .global boot_stack

boot_stack:
    .space 4096 * 4
    .global boot_stack_top

boot_stack_top: