# TravelOfOS

> 🚀 从零开始编写一个 RISC-V 架构的操作系统内核 —— 用 Rust 探索操作系统的奥秘

## 📖 项目简介

**TravelOfOS**（简称 TOOS）是一个基于 **RISC-V 架构** 的入门级操作系统内核项目，使用 **Rust** 语言编写。

本项目旨在帮助初学者理解操作系统底层的工作原理，包括：
- 操作系统如何启动
- 内存布局与链接器脚本
- 如何通过 SBI（Supervisor Binary Interface）与底层固件交互
- 如何在无标准库（`no_std`）环境下实现基本的控制台输出

项目代码注释详尽，使用生动的比喻解释复杂的底层概念，非常适合操作系统初学者学习。

---

## 🛠️ 技术栈

| 技术 | 说明 |
|------|------|
| **Rust** | 编程语言，Edition 2021 |
| **RISC-V** | 目标 CPU 架构 |
| **RustSBI** | RISC-V 启动标准固件接口，提供底层服务 |
| **no_std** | 无标准库环境，完全从零构建 |
| **内联汇编** | 通过 `core::arch::asm!` 实现底层硬件交互 |

---

## 📁 项目结构

```
TravelOfOS/
├── README.md                 # 项目说明文档（本文件）
└── toos/                     # 操作系统内核项目
    ├── Cargo.toml            # Rust 项目配置文件
    ├── .gitignore            # Git 忽略配置
    └── src/
        ├── main.rs           # 内核主程序入口
        ├── console.rs        # 控制台输出模块
        ├── entry.asm         # 汇编启动代码
        └── linker.ld         # 链接器脚本（定义内存布局）
```

---

## 🔑 核心文件说明

### `src/main.rs` — 内核主程序

操作系统的核心入口文件，使用了两个重要的编译器属性：

- **`#![no_std]`** — 不使用 Rust 标准库，意味着没有 `std::io`、`std::fs` 等常用库
- **`#![no_main]`** — 不使用标准的 `main` 函数入口，而是自定义启动流程

主要功能：
- 通过 `global_asm!` 宏引入汇编启动代码
- 定义 `rust_main()` 函数作为 Rust 代码的入口点
- 实现 `panic_handler` 处理程序崩溃情况

### `src/console.rs` — 控制台输出模块

实现了类似标准库中 `print!` 和 `println!` 宏的功能，底层通过 **SBI（Supervisor Binary Interface）** 调用实现。

核心组件：
- **`console_putchar()`** — 最底层的字符输出函数，通过 `ecall` 指令触发 SBI 调用
- **`Stdout` 结构体** — 实现了 `core::fmt::Write` trait，使格式化输出成为可能
- **`print!` / `println!` 宏** — 导出到全局，可在其他模块中直接使用

### `src/entry.asm` — 汇编启动代码

操作系统启动的第一段代码，负责：
1. **设置栈指针（`sp`）** — 从 `boot_stack_top` 加载栈顶地址
2. **分配栈空间** — 在 `.bss.stack` 区域预留 64KB 栈空间（4096 × 16 字节）
3. **跳转到 Rust 入口** — 调用 `rust_main()` 函数

### `src/linker.ld` — 链接器脚本

定义了内核在内存中的布局：

| 段（Section） | 说明 |
|--------------|------|
| `.text` | 代码段，包含可执行指令（`.text.entry` 优先放置） |
| `.rodata` | 只读数据段，如字符串字面量 |
| `.data` | 可读写数据段 |
| `.bss` | 未初始化数据段，包含栈空间 |

**内核加载基地址**: `0x80200000`（由 RustSBI 规范规定）

---

## 🔄 启动流程

```
┌─────────────────────────────────────────────────────────────┐
│                    系统启动流程                              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   1. CPU 上电，跳转到入口地址 0x80200000                     │
│         ↓                                                   │
│   2. 执行 entry.asm 中的 _start 标签                         │
│         ↓                                                   │
│   3. 设置栈指针 sp = boot_stack_top                          │
│         ↓                                                   │
│   4. 调用 rust_main() 函数                                  │
│         ↓                                                   │
│   5. 执行 Rust 代码（打印信息、初始化等）                      │
│         ↓                                                   │
│   6. 进入无限循环，操作系统持续运行                           │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔬 核心概念

### SBI（Supervisor Binary Interface）

SBI 是 RISC-V 架构定义的一套"办事协议"，类似于操作系统的"菜单系统"：

- **`a7` 寄存器**：存放"业务 ID"（服务代号）
  - `1` → 打印字符（`console_putchar`）
  - `8` → 关机
- **`a0` ~ `a2` 寄存器**：存放参数
- **`ecall` 指令**：触发环境调用，跳转到 RustSBI 执行对应服务

### `no_std` 环境

在 `no_std` 环境下：
- ❌ 没有 `std::io::println`
- ❌ 没有 `std::fs::File`
- ❌ 没有 `std::vec::Vec`
- ✅ 仍然可以使用 `core` 中的基础功能（如 `fmt`、`arch`）

### 链接器脚本

链接器脚本告诉链接器如何将代码和数据放置在内存中。关键指令：

- **`OUTPUT_ARCH(riscv)`** — 指定目标架构
- **`ENTRY(_start)`** — 指定程序入口点
- **`SECTIONS { ... }`** — 定义内存段布局

---

## 📝 编译与运行

### 前置要求

- [Rust](https://www.rust-lang.org/)（Edition 2021）
- [QEMU](https://www.qemu.org/)（RISC-V 模拟器）
- Rust RISC-V 目标支持

### 安装 RISC-V 目标

```bash
rustup target add riscv64gc-unknown-none-elf
```

### 编译

```bash
cd toos
cargo build --target riscv64gc-unknown-none-elf
```

### 运行（使用 QEMU）

```bash
qemu-system-riscv64 \
  -machine virt \
  -bios default \
  -kernel target/riscv64gc-unknown-none-elf/debug/toos \
  -nographic
```

---

## 📚 学习资源

- [RISC-V 官方文档](https://riscv.org/technical/specifications/)
- [RustSBI 规范](https://github.com/riscv-non-isa/riscv-sbi-doc)
- [Rust for Embedded](https://rust-embedded.org/)
- [小白的操作系统入门](https://github.com/rcore-os/learning-os)

---

## 📄 许可证

本项目仅供学习使用。

---

## 🙏 致谢

感谢 Rust 社区和 RISC-V 社区提供的丰富资源，让操作系统学习变得更加容易。