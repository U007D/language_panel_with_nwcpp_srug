use std::any::Any;

use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Thread panicked: {}", 0)]
    ThreadPanicked(Box<dyn Any + Send + 'static>),
}

// This allows `?` to automatically coerce the error type returned from
// `std::thread::JoinHandle` into our error type.  Note the lack of
// boilerplate at the error handling sites (ll.39, 73)
impl From<Box<dyn Any + Send + 'static>> for Error {
    fn from(error: Box<dyn Any + Send + 'static>) -> Self {
        Self::ThreadPanicked(error)
    }
}
