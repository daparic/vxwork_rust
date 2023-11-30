use std::os::raw::{c_char, c_int, c_void};

pub type FUNCPTR = unsafe extern "C" fn(*mut c_void);

extern "C" {
    pub fn taskIdSelf() -> c_int;

    pub fn taskSpawn(
        name: *mut c_char,
        priority: c_int,
        options: c_int,
        stackSize: c_int,
        entryPt: FUNCPTR,
        arg1: *mut c_void,
        arg2: c_int,
        arg3: c_int,
        arg4: c_int,
        arg5: c_int,
        arg6: c_int,
        arg7: c_int,
        arg8: c_int,
        arg9: c_int,
        arg10: c_int,
    ) -> c_int;

    pub fn taskDelay(tick: c_int) -> c_int;

    pub fn taskPrioritySet(tid: c_int, priority: c_int) -> c_int;
    pub fn taskPriorityGet(tid: c_int, priority: *mut c_int) -> c_int;

    pub fn errnoGet() -> c_int;

    pub fn semBCreate(option: i32, init: i32) -> *mut c_void;
    pub fn semTake(sid: *mut c_void, timeout: i32) -> i32;
    pub fn semGive(sid: *mut c_void) -> i32;
}
