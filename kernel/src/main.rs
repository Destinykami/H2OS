#![no_std]
#![no_main]
#![feature(panic_info_message)] //通过 PanicInfo::message 获取报错信息

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod logging;


global_asm!(include_str!("entry.asm"));
#[no_mangle] //避免编译器对它的名字进行混淆
pub fn rust_main() -> ! {
    clear_bss(); //对.bss段清零
    println!("[kernel] Hello world!");
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    logging::init();
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);







    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
    }
    // (sbss as usize..ebss as usize).for_each(|a|{
    //     unsafe{(a as *mut u8).write_volatile(0)}
    // }); //迭代器与闭包
    for a in sbss as usize..ebss as usize {
        unsafe { (a as *mut u8).write_volatile(0) }
    } //改为等效的for循环实现
}
