.section .text.entry
# 告诉链接器，这段代码要放在一个叫 .text.entry 的“抽屉”里。
    .globl _start
    # _start 是一个标签（类似地图上的地标）。.globl 是向外界大声宣布：“嘿！我是整个程序的起点！”
_start:
    # 1. 准备好化妆间（设置栈指针 sp，操作系统必须有栈才能运行）
    la sp, boot_stack_top
    # la (Load Address) 是加载地址，是把一个标签（符号）的地址加载进入寄存器，方便后续跳转或取值；
    # 此时还没有内存管理单元，逻辑地址就等于物理地址。加载的是内存条上实打实的物理位置
    #（如果你有一条 8GB 的内存条，你可以把它想象成一排从 0 排到 8,589,934,591 的超长座位。）。
    # sp (Stack Pointer) 是栈指针寄存器，它就是你从内存里“划”出来的一块普通空间

    # 总结一下：la (Load Address)：它的意思是“把 boot_stack_top 这个标签对应的物理地址数值（比如 0x80220000）算出来，塞进 sp 寄存器里”。

    # 2. 呼叫巨星上场！（跳转到我们写的 rust_main 函数）
    call rust_main

    # 划分一块 64KB 的内存作为化妆间（栈）
    .section .bss.stack
    # 这是在告诉链接器：“我要在内存的 .bss 区域（专门放初始化为 0 的数据的地盘）里，单独开辟一个叫 stack 的小房间。

    .globl boot_stack_lower_bound
    # 告诉项目里的其他文件（比如你的 Rust 代码），boot_stack_lower_bound 这个名字是公开的，大家都可以引用它。

boot_stack_lower_bound:
# 这是地基的起点（低地址）。它只是一个刻在内存地址上的“记号”。假设它的地址是 0x80210000
    .space 4096 * 16
    # 这行指令让编译器在内存里结结实实地空出 64KB 的位置。这块空地就是你给 rust_main 准备的“活动空间”。
    .globl boot_stack_top
    # 这是地基的终点（高地址）。
boot_stack_top: