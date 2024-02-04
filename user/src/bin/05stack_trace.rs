//裸机应用程序，实现打印栈
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use core::{arch::asm, ptr};

#[no_mangle]
fn main() {
    let mut fp: *const usize;
    println!("== Begin stack trace ==");
    unsafe {
        asm!("mv {}, fp", out(reg) fp);
        while fp != ptr::null() {
            let saved_ra = *fp.sub(1);
            let saved_fp = *fp.sub(2);
            println!("0x{:016x}, fp = 0x{:016x}", saved_ra, saved_fp);
            fp = saved_fp as *const usize;
        }
    }
    println!("== End stack trace ==");
}
