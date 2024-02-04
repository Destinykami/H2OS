//和进程有关的系统调用

use crate::batch::run_next_app;

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    run_next_app()
}