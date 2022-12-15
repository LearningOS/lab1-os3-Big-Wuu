//! Types related to task management

use super::TaskContext;
use crate::timer::get_time_us;
use crate::config::MAX_SYSCALL_NUM;

use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub syscall_times: Vec<u32>, // [id, times]
    first_scheduled: bool,
    start_time: usize, // in us
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskControlBlock {
    pub fn new() -> TaskControlBlock {
        TaskControlBlock {
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            syscall_times: vec![0; MAX_SYSCALL_NUM],
            first_scheduled: true,
            start_time: usize::MAX,
        }
    }
    pub fn update_when_scheduled(&mut self) {
        if self.first_scheduled {
            self.first_scheduled = false;
            self.start_time = get_time_us();
        }
    }
    /// us
    pub fn running_time(&self) -> usize {
        get_time_us() - self.start_time
    }
}