use std::{any::Any, sync::PoisonError};

use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Thread panicked: {}", 0)]
    ThreadPanicked(Box<dyn Any + Send + 'static>),

    // Because what happens to a resource when a lock is obtained must appear atomic
    // to the rest of the world, a panic while holding a lock *could* expose
    // intermediate state.  The lock is considered `Poisoned` if this happens.
    // If the app knows how to detect and fix this situation for whatever it is doing,
    // it can unpoison the lock.  Otherwise, the app cannot continue correctly and
    // should terminate.
    #[error("Thread panicked while holding a lock: {}", 0)]
    PoisonedLock(String),
}

// This allows `?` to automatically coerce the error type returned from
// `std::thread::JoinHandle` into our error type.  Note the lack of
// boilerplate at the error handling sites (ll.39, 73)
impl From<Box<dyn Any + Send + 'static>> for Error {
    fn from(error: Box<dyn Any + Send + 'static>) -> Self {
        Self::ThreadPanicked(error)
    }
}

impl<G> From<PoisonError<G>> for Error {
    fn from(error: PoisonError<G>) -> Self {
        Self::PoisonedLock(error.to_string())
    }
}
