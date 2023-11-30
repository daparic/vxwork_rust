use std::{
    marker::PhantomData,
    os::raw::{c_char, c_void},
    sync::Arc,
};

use crate::{
    binding::{msgQCreate, msgQDelete, msgQReceive, msgQSend},
    error::Error,
    Nullable, SIZE,
};

struct InnerMQ {
    handle: *mut c_void,
}

impl InnerMQ {
    fn new(handle: *mut c_void) -> Self {
        Self { handle }
    }

    fn send(&self, buffer: *mut c_char, timeout: i32, priority: i32) -> Result<i32, Error> {
        unsafe { msgQSend(self.handle, buffer, SIZE as u32, timeout, priority) }.if_error()
    }

    fn receive(&self, timeout: i32) -> Result<usize, Error> {
        let mut buf = 0usize;
        let p = &mut buf as *mut usize as *mut i8;
        unsafe { msgQReceive(self.handle, p, SIZE as u32, timeout) }
            .if_error()
            .map(|_| buf)
    }
}

#[derive(Clone)]
pub struct MessageQueue<T> {
    inner: Arc<InnerMQ>,
    _marker: PhantomData<fn() -> T>,
}

unsafe impl<T: Send> Send for MessageQueue<T> {}
unsafe impl<T: Sync> Sync for MessageQueue<T> {}

impl<T> MessageQueue<T> {
    pub fn new(buffer: i32) -> Result<Self, Error> {
        unsafe { msgQCreate(buffer, SIZE as i32, 0x00) }
            .if_error()
            .map(|handle| Self {
                inner: Arc::new(InnerMQ::new(handle)),
                _marker: Default::default(),
            })
    }

    pub fn send(&self, t: T, timeout: i32, priority: i32) -> Result<i32, Error> {
        let boxed = Box::new(t);
        let p = Box::into_raw(Box::new(boxed)) as *mut i8;
        self.inner.send(p, timeout, priority)
    }

    pub fn receive(&self, timeout: i32) -> Result<T, Error> {
        self.inner
            .receive(timeout)
            .map(|buf| unsafe { *Box::from_raw(buf as *mut T) })
    }
}

impl<T> Drop for MessageQueue<T> {
    fn drop(&mut self) {
        if Arc::strong_count(&self.inner) == 1 {
            unsafe {
                msgQDelete(self.inner.handle);
            }
        }
    }
}
