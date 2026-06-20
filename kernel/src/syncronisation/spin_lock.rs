use crate::syncronisation::Mutex;
use core::{
    cell::UnsafeCell,
    hint::{spin_loop, unlikely},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T>
where
    T: ?Sized,
{
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for SpinLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for SpinLock<T> where T: ?Sized + Send {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> SpinLock<T> {
        SpinLock {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> Mutex for SpinLock<T> {
    type Data<'a> = &'a mut T where T: 'a;
    fn lock<'a, R>(&'a self, f: impl FnOnce(Self::Data<'a>) -> R) -> R {
        loop {
            if unlikely(
                self.lock
                    .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok(),
            ) {
                let data = unsafe { &mut *self.data.get() };
                let r = f(data);
                self.lock.store(false, Ordering::Release);
                break r;
            }
            while self.lock.load(Ordering::Relaxed) {
                // while locked
                spin_loop();
            }
        }
        // locked now
    }
}
