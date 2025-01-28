#![feature(sync_unsafe_cell)]

// This module just defines a bespoke error type for our app
mod error;

use std::{cell::SyncUnsafeCell,
          thread::{self, ScopedJoinHandle}};

use error::Result;

const THREADS: usize = 100;
const INCS_PER_THREAD: usize = 10_000;

fn main() -> Result<()> {
    let counter = SyncUnsafeCell::new(0);

    println!("Spawning {THREADS} threads to increment heap-allocated `count` {INCS_PER_THREAD} \
              times each...");

    // Do the counting
    concurrent_count(&counter)?;

    // Every `counter` access (even "just" a read) is `unsafe`
    #[allow(static_mut_refs, unsafe_code)]
    unsafe {
        println!("Expected total count: {}; Actual count: {}",
                 THREADS * INCS_PER_THREAD,
                 *counter.get());
    };

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn concurrent_count(counter_ref: &SyncUnsafeCell<u64>) -> Result<()> {
    fn increment(counter_ref: &SyncUnsafeCell<u64>) {
        (0..INCS_PER_THREAD).for_each(|_i| {
                                #[allow(unsafe_code)]
                                unsafe {
                                    *counter_ref.get() += 1
                                };
                            })
    }

    thread::scope(|scope| {

        // Create a slice of handles to track each of the threads we create.  Use them to wait until all
        // of them have completed.
        let join_handles =
            (0..THREADS).fold(Vec::with_capacity(THREADS), |mut handles_acc, _i| {
                #[allow(unsafe_code)]
                let handle =
                    scope.spawn(move || increment(counter_ref));
                handles_acc.push(handle);
                handles_acc
            });

        // Block until every spawned thread indicates it has finished (or one panics)
        join_handles.into_iter().try_for_each(ScopedJoinHandle::join)?;

        Ok(())
    })
}
