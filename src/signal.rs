use crate::{
    binding::{sigaction, sigaction__bindgen_ty_1, sigaction_t, sigemptyset},
    error::Error,
    Nullable,
};

#[repr(i32)]
pub enum Signal {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGTRAP = 5,
    SIGABRT = 6,
    SIGEMT = 7,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGBUS = 10,
    SIGSEGV = 11,
    SIGFMT = 12,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    SIGCNCL = 16,
    SIGSTOP = 17,
    SIGTSTP = 18,
    SIGCONT = 19,
    SIGCHLD = 20,
    SIGTTIN = 21,
    SIGTTOU = 22,
    SIGRES1 = 23,
    SIGRES2 = 24,
    SIGRES3 = 25,
    SIGRES4 = 26,
    SIGRES5 = 27,
    SIGRES6 = 28,
    SIGRES7 = 29,
    SIGUSR1 = 30,
    SIGUSR2 = 31,
    SIGPOLL = 32,
    SIGPROF = 33,
    SIGSYS = 34,
    SIGURG = 35,
    SIGVTALRM = 36,
    SIGXCPU = 37,
    SIGXFSZ = 38,
    SIGEVTS = 39,
    SIGEVTD = 40,
    SIGRTMIN = 48,
    SIGRTMAX = 63,
}

pub fn signal_action(sig: Signal, flag: i32, action: extern "C" fn(i32)) -> Result<i32, Error> {
    let mut signal = sigaction_t {
        sa_u: sigaction__bindgen_ty_1 {
            __sa_handler: Some(action),
        },
        sa_mask: 0,
        sa_flags: flag,
    };
    unsafe { sigemptyset(&mut signal.sa_mask as *mut _) };
    unsafe { sigaction(sig as i32, &signal as *const _, std::ptr::null_mut()) }.if_error()
}

pub fn kill(tid: i32, sig: Signal) -> Result<i32, Error> {
    unsafe { crate::binding::taskKill(tid, sig as i32) }.if_error()
}
