use std::any::Any;

#[expect(clippy::wildcard_imports, reason = "Ok to use wildcard import on `shared_consts`.")]
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
    fn from(value: Box<dyn Any + Send + 'static>) -> Self {
        Self::ThreadPanicked(value)
    }
}
