// This module just defines a bespoke error type for our app
mod error;

use std::{sync::atomic::{AtomicU64, Ordering::SeqCst},
          thread::{self, ScopedJoinHandle}};

use error::Result;

const THREADS: usize = 100;
const INCS_PER_THREAD: usize = 10_000;

fn main() -> Result<()> {
    // `Atomic` provides "built-in" synchronization (safety).
    let counter = AtomicU64::new(0);

    println!("Spawning {THREADS} threads to increment heap-allocated `count` {INCS_PER_THREAD} \
              times each...");

    // Do the counting
    concurrent_count(&counter)?;

    // Accessing a synchronized (i.e. via a lock) shared resource is safe.
    println!("Expected total count: {}; Actual count: {}",
             THREADS * INCS_PER_THREAD,
             counter.load(SeqCst));

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn concurrent_count(counter_ref: &AtomicU64) -> Result<()> {
    fn increment(counter_ref: &AtomicU64) {
        (0..INCS_PER_THREAD).for_each(|_i| {
                                counter_ref.fetch_add(1, SeqCst);
                            })
    }

    // Limits the lifetime of all (scoped) threads spawned within to the scope (closing brace) of
    // this statement.
    thread::scope(|scope| {
        // Create a slice of handles to track each of the threads we create.  Use them to wait until
        // all of them have completed.
        let join_handles =
            (0..THREADS).fold(Vec::with_capacity(THREADS), |mut handles_acc, _i| {
               #[allow(unsafe_code)]
               let handle = scope.spawn(move || increment(counter_ref));
               handles_acc.push(handle);
               handles_acc
           });

        // Block until every spawned thread indicates it has finished (or one panics)
        join_handles.into_iter().try_for_each(ScopedJoinHandle::join)?;
        Ok(())
    })
}
