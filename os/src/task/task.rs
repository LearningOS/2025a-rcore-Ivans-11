//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task syscall number
    pub task_syscall_num: TaskSyscallNum,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// The syscall number of a task
#[derive(Copy, Clone)]
pub struct TaskSyscallNum {
    /// The number of syscall write
    pub write_num: usize,
    /// The number of syscall exit
    pub exit_num: usize,
    /// The number of syscall yield
    pub yield_num: usize,
    /// The number of syscall get_time
    pub get_time_num: usize,
    /// The number of syscall trace
    pub trace_num: usize,

}

impl TaskSyscallNum {
    /// Create a new task syscall number
    pub fn new() -> Self {
        TaskSyscallNum {
            write_num: 0,
            exit_num: 0,
            yield_num: 0,
            get_time_num: 0,
            trace_num: 0,
        }
    }

    /// Add a syscall number to the task syscall number
    pub fn add_syscall_num(&mut self, syscall_id: usize) {
        match syscall_id {
            64 => self.write_num += 1,
            93 => self.exit_num += 1,
            124 => self.yield_num += 1,
            169 => self.get_time_num += 1,
            410 => self.trace_num += 1,
            _ => {}
        }
    }

    /// Get the syscall number of the task
    pub fn get_syscall_num(&self, syscall_id: usize) -> usize {
        match syscall_id {
            64 => self.write_num,
            93 => self.exit_num,
            124 => self.yield_num,
            169 => self.get_time_num,
            410 => self.trace_num,
            _ => 0,
        }
    }
}