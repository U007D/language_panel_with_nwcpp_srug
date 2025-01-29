// This module just defines a bespoke error type for our app
mod error;

use std::thread::{self, JoinHandle};

use error::Result;

const N_THREADS: usize = 100;
const INCS_PER_THREAD: usize = 10_000;

// A mutable global is a giant red flag for idiomatic Rust--don't do this!
static mut COUNT: u64 = 0;

fn main() -> Result<()> {
    println!("Spawning {N_THREADS} threads to increment `COUNT` {INCS_PER_THREAD} times each...");

    // Do the counting
    concurrent_count()?;

    // Every access (even a read) to a mutable static is `unsafe`
    #[allow(global_mut_refs, unsafe_code)]
    unsafe {
        println!(
            "Expected total count: {}; Actual count: {COUNT}",
            N_THREADS * INCS_PER_THREAD
        )
    };

    Ok(())
}

fn concurrent_count() -> Result<()> {
    // Create a vector of handles for the threads we create.  Wait until
    // all of them have completed.
    let mut join_handles = Vec::<JoinHandle<()>>::with_capacity(N_THREADS);

    (0..N_THREADS).for_each(|_thread| {
        let join_handle = thread::spawn(move || {
            (0..INCS_PER_THREAD).for_each(|_i| {
                #[allow(unsafe_code)]
                unsafe {
                    COUNT += 1
                };
            })
        });
        join_handles.push(join_handle);
    });

    // Block until every spawned thread indicates it has finished (or Panic)
    join_handles
        .into_iter()
        .try_for_each(JoinHandle::join)?;

    Ok(())
}
