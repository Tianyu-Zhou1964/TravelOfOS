OUTPUT_ARCH(riscv)
ENTRY(_start)
/* 场务总管 RustSBI 规定，操作系统必须放在 0x80200000 这个地址 */
BASE_ADDRESS = 0x80200000; /* 内核加载到这个物理地址 */

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry) /* 把 _start 这扇门放在最前面！*/
        *(.text .text.*)
    }
    .rodata : { *(.rodata .rodata.*) } /* 只读数据，比如字符串字面量 */
    .data : { *(.data .data.*) } /* 可读写数据 */
    .bss : { /* 未初始化数据（包括你的栈） */
        *(.bss.stack)
        *(.bss .bss.*)
    }
}