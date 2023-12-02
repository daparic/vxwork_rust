use std::os::raw::{c_char, c_int, c_uint, c_ulonglong, c_ushort, c_void};

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
    pub fn taskUnsafe() -> c_int;
    pub fn taskSafe() -> c_int;

    pub fn errnoGet() -> c_int;

    // semLib.h
    pub fn semBCreate(option: c_int, init: c_int) -> *mut c_void;
    pub fn semMCreate(option: c_int) -> *mut c_void;
    pub fn semDelete(sid: *mut c_void) -> c_int;
    pub fn semTake(sid: *mut c_void, timeout: c_int) -> c_int;
    pub fn semGive(sid: *mut c_void) -> c_int;

    // msgQLib.h
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

    // sysLib.h
    pub fn sysClkRateGet() -> c_int;

    // kernelLib.h
    // pub fn kernelTimeSlice(tick: c_int) -> c_int;

    // signal.h
    pub fn sigaction(
        signo: c_int,
        new_handler: *const sigaction_t,
        old_handler: *mut sigaction_t,
    ) -> c_int;
    pub fn sigemptyset(signal: *mut sigset_t) -> c_int;
    pub fn taskKill(tid: c_int, signo: c_int) -> c_int;
}

#[allow(non_camel_case_types)]
pub type uid_t = c_ushort;
#[allow(non_camel_case_types)]
pub type pid_t = c_int;
#[allow(non_camel_case_types)]
pub type sigset_t = c_ulonglong;

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: c_int,
    pub sival_ptr: *mut c_void,
}

impl std::fmt::Debug for sigval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("sigval").finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct _Siginfo {
    pub si_signo: c_int,
    pub si_code: c_int,
    pub si_value: sigval,
    pub si_errno: c_int,
    pub si_status: c_int,
    pub si_addr: *mut c_void,
    pub si_uid: uid_t,
    pub si_pid: pid_t,
}

#[allow(non_camel_case_types)]
pub type siginfo_t = _Siginfo;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct sigaction_t {
    pub sa_u: sigaction__bindgen_ty_1,
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union sigaction__bindgen_ty_1 {
    pub __sa_handler: Option<unsafe extern "C" fn(arg1: c_int)>,
    pub __sa_sigaction:
        Option<unsafe extern "C" fn(arg1: c_int, arg2: *mut siginfo_t, arg3: *mut c_void)>,
}
