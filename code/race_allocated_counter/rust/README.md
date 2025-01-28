# concurrent_inc_1  (concurrent increment v1)
## Purpose
* Explain the purpose of locks (concurrency synchronization primitive)
* Demonstrate memory management (managing of allocations)
* Properly handle errors
* Use idiomatic Rust

- v1 disables Rust's data race safety guarantees to illustrate the problem.
- v2 uses locks to eliminate the data race data loss introduced in v2.
- v3 uses `Atomic`s to achieve the same effect with less code to maintain.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
