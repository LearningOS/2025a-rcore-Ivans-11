//! Process management syscalls
use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, current_user_token, get_syscall_num, insert_framed_area, remove_framed_area};
use crate::timer::get_time_us;
use crate::mm::{translated_byte_buffer, is_readable, is_writable, is_empty, VirtAddr, frame_remain_num, VPNRange};
use core::ptr::copy;

#[repr(C)]
#[derive(Debug)]
/// time value in second and microsecond
pub struct TimeVal {
    /// second
    pub sec: usize,
    /// microsecond
    pub usec: usize,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let src = &time_val as *const TimeVal as *const u8;
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, core::mem::size_of::<TimeVal>());
    let mut start = 0;
    unsafe {
        for buffer in buffers {
            copy(src.add(start), buffer.as_mut_ptr(), buffer.len());
            start += buffer.len();
        }
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            let ptr = _id as *const u8;
            if !is_readable(current_user_token(), ptr) {
                return -1;
            }
            let buffers = translated_byte_buffer(current_user_token(), ptr, 1);
            buffers[0][0] as isize
        }
        1 => {
            let ptr = _id as *const u8;
            if !is_writable(current_user_token(), ptr) {
                return -1;
            }
            let mut buffers = translated_byte_buffer(current_user_token(), ptr, 1);
            buffers[0][0] = _data as u8;
            0
        }
        2 => {
            get_syscall_num(_id) as isize
        }
        _ => {
            -1
        }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start = VirtAddr::from(_start);
    if !start.aligned() {
        return -1;
    }
    if _port & !7 != 0 || _port & 7 == 0 {
        return -1;
    }
    let end = VirtAddr::from(_start + _len);
    let start_vpn = start.floor();
    let end_vpn = end.ceil();

    for vpn in VPNRange::new(start_vpn, end_vpn) {
        if !is_empty(current_user_token(), vpn.into()) {
            return -1;
        }
    }
    if frame_remain_num() < (end_vpn.0 - start_vpn.0) {
        return -1;
    }
    insert_framed_area(start, end, _port << 1 | 16);
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start = VirtAddr::from(_start);
    if!start.aligned() {
        return -1;
    }
    let end = VirtAddr::from(_start + _len);
    let start_vpn = start.floor();
    let end_vpn = end.ceil();

    for vpn in VPNRange::new(start_vpn, end_vpn) {
        if is_empty(current_user_token(), vpn.into()) {
            return -1;
        }
    }
    if remove_framed_area(start_vpn, end_vpn) {
        0
    } else {
        -1
    }
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
