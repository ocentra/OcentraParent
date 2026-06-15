use std::sync::{Mutex, MutexGuard};

pub(crate) fn lock_unpoison<'a, T>(mutex: &'a Mutex<T>) -> MutexGuard<'a, T> {
    mutex.lock().unwrap_or_else(|error| error.into_inner())
}
