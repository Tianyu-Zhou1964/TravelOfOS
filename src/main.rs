#![no_std]
#![no_main]

core::arch::global_asm!(include_str!("entry.asm"));
// 引入我们刚才写的模块（假设文件名是 console.rs）
mod console;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // 🎉 见证奇迹的时刻！
    println!("Hello, RISC-V OS!");
    println!("My OS is booting...");
    println!("1 + 1 = {}", 1 + 1); // 格式化也完美支持！
    
    // 操作系统不能随便退出，让它在这里无限循环休息
    loop {}
}

use core::panic::PanicInfo;

// 这是一个宏，告诉编译器：如果程序崩溃了，就来执行这个函数
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // 崩溃了也没什么能做的，就让它在这里无限循环死机吧
    loop {}
}