use std::os::raw::{c_char, c_int, c_uint, c_void};

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

    pub fn semBCreate(option: c_int, init: c_int) -> *mut c_void;
    pub fn semTake(sid: *mut c_void, timeout: c_int) -> c_int;
    pub fn semGive(sid: *mut c_void) -> c_int;

    pub fn msgQCreate(max_message: c_int, length: c_int, option: c_int) -> *mut c_void;
    pub fn msgQSend(
        mqid: *mut c_void,
        buffer: *mut c_char,
        length: c_uint,
        timeout: c_int,
        priority: c_int,
    ) -> c_int;
    pub fn msgQReceive(
        mqid: *mut c_void,
        buffer: *mut c_char,
        length: c_uint,
        timeout: c_int,
    ) -> c_int;
    pub fn msgQDelete(mqid: *mut c_void) -> c_int;
}
