use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
};

use binding::*;
use bitflags::bitflags;
use error::Error;

mod binding;
pub mod error;
pub mod mq;

pub const WAIT_FOREVER: i32 = -1;
pub const SIZE: usize = std::mem::size_of::<usize>();

bitflags! {
    pub struct SemaphoreOption: i32 {
        const Q_FIFO = 0x000000;
        const Q_PRIORITY = 0x000001;
        const DELETE_SAFE = 0x000004;
        const INVERSION_SAFE = 0x000008;
        const EVENTSEND_ERR_NOTIFY = 0x000010;
        const INTERRUPTIBLE = 0x000020;
        const NO_EVENT_SEND = 0x000100;
        const NO_SYSTEM_VIEWER = 0x000200;
        const NO_RECURSE = 0x000400;
        const NO_ID_VALIDATE = 0x000800;
        const NO_ERROR_CHECK = 0x001000;
        const TASK_DELETION_WAKEUP = 0x002000;
    }
}

pub trait Nullable: Sized {
    fn null(&self) -> bool;

    fn if_error(self) -> Result<Self, Error> {
        if self.null() {
            Err(errno().into())
        } else {
            Ok(self)
        }
    }
}

impl Nullable for i32 {
    fn null(&self) -> bool {
        *self < 0
    }
}

impl<T> Nullable for *mut T {
    fn null(&self) -> bool {
        self.is_null()
    }
}

pub fn errno() -> i32 {
    unsafe { errnoGet() }
}

pub fn task_id_self() -> i32 {
    unsafe { taskIdSelf() }
}

pub fn sys_clock_rate() -> i32 {
    unsafe { sysClkRateGet() }
}

// pub fn kernel_time_slice(tick: i32) -> Result<i32, Error> {
//     unsafe { kernelTimeSliceWrapper(tick) }.if_error()
// }

// Safety: task must be send and be inbound
pub unsafe fn task_spawn_unchecked(
    name: &str,
    priority: i32,
    task: unsafe extern "C" fn(*mut c_void),
    value: *mut c_void,
) -> Result<i32, Error> {
    let name = if let Some((front, _)) = name.split_once('\0') { front } else { name };

    // Unwrap is ok here because there would be no null bytes in name
    let c_string = CString::new(name).unwrap();
    let name: *mut c_char = c_string.into_raw();

    taskSpawn(name, priority, 0x100, 2000, task, value, 0, 0, 0, 0, 0, 0, 0, 0, 0).if_error()
}

pub fn task_spawn<F>(name: &str, priority: i32, task: F) -> Result<i32, Error>
where
    F: FnOnce(),
    F: Send + 'static,
{
    let main: Box<dyn FnOnce()> = Box::new(task);
    let main = Box::into_raw(Box::new(main));

    unsafe {
        return task_spawn_unchecked(name, priority, task_start, main as *mut _);
    }

    extern "C" fn task_start(main: *mut c_void) {
        unsafe {
            Box::from_raw(main as *mut Box<dyn FnOnce()>)();
        }
    }
}

pub fn task_delay(tick: i32) -> Result<i32, Error> {
    unsafe { taskDelay(tick) }.if_error()
}

pub fn task_priority_set(tid: i32, priority: u8) -> Result<i32, Error> {
    unsafe { taskPrioritySet(tid, priority as i32).if_error() }
}

pub fn task_priority_get(tid: i32) -> Result<i32, Error> {
    let buf: i32 = 0;
    let ptr = buf as *mut i32;
    unsafe { taskPriorityGet(tid, ptr).if_error().map(|_| *ptr) }
}

#[derive(Clone, Copy)]
pub struct Semaphore {
    sid: *mut c_void,
}

unsafe impl Send for Semaphore {}
unsafe impl Sync for Semaphore {}

impl Semaphore {
    pub fn new(option: SemaphoreOption, initial: bool) -> Result<Self, Error> {
        let initial = if initial { 1 } else { 0 };
        unsafe { semBCreate(option.bits(), initial) }.if_error().map(|sid| Self { sid })
    }

    pub fn take(self, timeout: i32) -> Result<i32, Error> {
        unsafe { semTake(self.sid, timeout) }.if_error()
    }

    pub fn release(self) -> Result<i32, Error> {
        unsafe { semGive(self.sid) }.if_error()
    }
}
