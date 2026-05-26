use core::arch::asm;
// arch: 代表 Architecture（架构）。因为不同的 CPU（比如 RISC-V、x86、ARM）
// 的汇编指令完全不同，所以这个功能被放在 arch 包下。
// asm!: 这是 Rust 提供的一个“传送门”，允许你直接在 Rust 代码里写汇编指令。

// 这是一个极其底层的函数，每次只能往管子里塞一个字节（字母）
pub fn console_putchar(c: usize) {
// pub是让这个函数公开，使得 main.rs 可以直接mod 这个函数
    unsafe {
    // unsafe操作是因为接下来我们要写汇编代码了，编译器无法检查汇编代码的安全性
        asm!(
        // asm!的小括号里面的内容就是汇编代码了
            "ecall",
            // ecall 就是环境调用的意思，此时 CPU 立刻停下手头任务，
            // 并且跳转到 RustSBI 的代码去执行
            // CPU 内部有一些极其快速的临时存储单元，叫寄存器。RISC-V 规定了一套“办事协议”（SBI 标准）：
            // a7 寄存器：用来放“业务 ID”。你往 a7 里放 1，RustSBI 就知道你想调用的业务是“打印字符”；如果你放 8，它就知道你想“关机”。
            // a0 寄存器：用来放“参数”。你想打印哪个字符？把它塞进 a0 寄过去。
            // a1, a2 放 0：这是为了遵循协议规范，确保这些寄存器里没有脏数据干扰。

            in("a7") 1, // SBI 的 console_putchar 服务代号，相当于菜单
            in("a0") c, // 填入具体内容
            in("a1") 0,
            in("a2") 0,
        );
    }
}

use core::fmt::{self, Write};
// 这行就是在导包。它从 Rust 的核心库（core）里把格式化相关的工具（fmt）拿了过来。
// fmt 就是 formatting 的意思，格式化
// fmt 的作用是填补了我输入的内容和 CPU 能听懂的 ASCII 码之间的 gap

// 造一个名叫 Stdout 的空结构体，代表我们的“标准输出屏幕”
struct Stdout;

// 让 Stdout 拥有“写”的能力
impl Write for Stdout {
    // 必须实现 write_str 方法
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 把传进来的字符串打散成一个个字节
        for c in s.bytes() {
            // 调用上面写好的底层函数，一个一个挤过管子
            console_putchar(c as usize);
        }
        Ok(())
    }
}

// 包装一个函数，让后面写宏的时候更方便调用
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// 导出这个宏，让别的文件也能用
#[macro_export]
macro_rules! print {
    // 把接收到的参数原封不动地传给刚才写的 print 函数
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    // 如果什么参数都不传，就只打印一个换行符 \n
    () => { $crate::print!("\n") };
    // match 模式匹配，空元组则创建一个换行符
    // 如果传了参数，就在结尾自动加上换行符 \n
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
        // 这里有点像正则，$(, $($arg)+)?表示整体可选，就是逗号加一个字符串，有或没有都行，如果有就 match Some，就是逗号加一串字符串
        // $($arg: tt)+ 中 tt = token tree，能匹配任何东西， + 表示一个或多个
        // 比如println!("hello");           // 没有后半段，? 匹配空
        // println!("x = {}", x);      // 有后半段，, $($arg:tt)+ 匹配 ", x"
    }
}