use std::{cell::UnsafeCell, os::windows::thread, sync::atomic::AtomicBool};
use core::sync::atomic::Ordering::{Acquire, Release};
use std::ops::{Deref, DerefMut};
use std::thread::scope;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        //unsafe { &mut *self.value.get() }
        Guard { lock: self }
        //self.locked.compare_exchange_weak(false, true, Acquire, Relaxed).is_err();
    }

    pub fn unlock(&self) {
        self.locked.store(false, Release);
    }
}


pub struct Guard<'b, T> {
    lock: &'b SpinLock<T>,
}

unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}

fn main() {
    let x = SpinLock::new(Vec::<i32>::new());
}
