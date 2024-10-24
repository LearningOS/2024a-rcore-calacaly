//! Process management syscalls
use core::{mem:: size_of, panic};

use crate::{
    config::MAX_SYSCALL_NUM, mm::translated_byte_buffer, task::{
        change_program_brk, current_syscall_stats, current_time_total, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus
    }
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl TimeVal {
    pub fn as_bytes(&self) -> &[u8] {
        let len = size_of::<Self>();
        let ptr = self as *const _ as *const u8;
        unsafe {
            core::slice::from_raw_parts(ptr, len)
        }
    }
}

/// Task information
#[allow(dead_code)]
#[derive(Debug)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

impl TaskInfo {
    pub fn as_bytes(&self) -> &[u8] {
        let len = size_of::<Self>();
        let ptr = self as *const _ as *const u8;
        unsafe {
            core::slice::from_raw_parts(ptr, len)
        }
    }
    
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let current_time_us = crate::timer::get_time_us();

    let temp = TimeVal {
        sec: current_time_us / 1_000_000,
        usec: current_time_us % 1_000_000,
    };
    let bytes = temp.as_bytes();

    let buffers = translated_byte_buffer(current_user_token(), ts as *const u8, size_of::<TimeVal>());

    let mut offset = 0;

    for buffer in buffers {
        buffer.copy_from_slice(&bytes[offset..offset + buffer.len()]);
        offset += buffer.len();
    }

    0

}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let buffers = translated_byte_buffer(current_user_token(), ti as *const u8, size_of::<TaskInfo>());
    

    let temp = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: current_syscall_stats(),
        time: current_time_total(),
    };

    let bytes = temp.as_bytes();


    let mut offset = 0;

    for buffer in buffers {
        buffer.copy_from_slice(&bytes[offset..offset + buffer.len()]);
        offset += buffer.len();
    }
    
    
    0
    
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
