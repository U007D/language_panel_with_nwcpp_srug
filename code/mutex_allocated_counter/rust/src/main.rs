// Suppress (good) suggestion to use `AtomicU64` instead of `Mutex<u64>`.
#![allow(clippy::mutex_integer)]

// This module just defines a bespoke error type for our app
mod error;

use std::{
    sync::Mutex,
    thread,
};

use error::Result;

const THREADS: usize = 100;
const INCS_PER_THREAD: usize = 10_000;

fn main() -> Result<()> {
    // `Mutex` provides synchronization (safety).
    let counter = Mutex::new(0);

    println!(
        "Spawning {THREADS} threads to increment heap-allocated `count` {INCS_PER_THREAD} \
              times each..."
    );

    // Do the counting
    concurrent_count(&counter)?;

    // Accessing a synchronized (i.e. via a lock) shared resource is safe.
    println!(
        "Expected total count: {}; Actual count: {}",
        THREADS * INCS_PER_THREAD,
        *counter.lock()?
    );

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn concurrent_count(counter_ref: &Mutex<u64>) -> Result<()> {
    fn increment(counter_ref: &Mutex<u64>) -> Result<()> {
        (0..INCS_PER_THREAD).try_for_each(|_i| {
            // Equivalent to:
            // let mut guard = counter_ref.lock()?;
            // *guard += 1; // warning: temporary with significant `Drop` can be early dropped;
            //              //          merge the temporary construction with its single usage
            *counter_ref.lock()? += 1;
            Ok(())
        })
    }

    thread::scope(|scope| {
        // Create a slice of handles to track each of the threads we create.  Use them to wait until all
        // of them have completed.
        let join_handles = (0..THREADS).fold(Vec::with_capacity(THREADS), |mut handles_acc, _i| {
            #[allow(unsafe_code)]
            let handle = scope.spawn(|| increment(counter_ref));
            handles_acc.push(handle);
            handles_acc
        });

        // Block until every spawned thread indicates it has finished (or one panics or was poisoned)
        join_handles
            .into_iter()
            .try_for_each(|join_handle| join_handle.join()?)
    })
}
