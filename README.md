# limits-rs

[![Build Status](https://travis-ci.com/aesedepece/limits-rs.svg?branch=master)](https://travis-ci.com/aesedepece/limits-rs)
[![Crate](https://img.shields.io/crates/v/limits-rs.svg)](https://crates.io/crates/limits-rs)
[![Docs](https://docs.rs/limits-rs/badge.svg)](https://docs.rs/limits-rs)
![License](https://img.shields.io/crates/l/limits-rs.svg)

**A Rust library for determining the limits that an operating system enforces on a given particular
process.**

## Operating systems support

In its current implementation, this crate allows convenient read of the `/proc/<pid>/limits`
file on GNU/Linux. On any other platform, the provided methods will return an error so that the
user can decide what to do in the absence of information about limits.

Support for other operating systems and platforms may be added on demand.
Feel free to [file an issue][issues] or make a PR!

## Examples

### Checking the limits for a specific PID
```rust
use limits_rs::get_pid_limits;

// Let's check what the CPU time hard limit is for process `1`.
let limits = get_pid_limits(1).unwrap();
let max_cpu_time_hard_limit = limits.max_cpu_time.hard;

// This will print either:
// - "Some(x)" if there is a limit, where `x` is the limit itself.
// - "None" if it is "unlimited".
println!("{}", max_cpu_time_hard_limit);
```

### Checking the limits for our own PID
```rust
use limits_rs::get_pid_limits;

// Let's check what the open files soft limit is for our own process.
let limits = get_own_limits().unwrap();
let max_open_files_soft_limit = limits.max_open_files.soft;

// This will print either:
// - "Some(x)" if there is a limit, where `x` is the limit itself.
// - "None" if it is "unlimited".
println!("{}", max_open_files_soft_limit);
```

## Supported "limitable" properties

### GNU/Linux
The properties currently tracked by `limits-rs::linux::Limits` are:

- `max_cpu_time`
- `max_file_size`
- `max_data_size`
- `max_stack_size`
- `max_core_file_size`
- `max_resident_set`
- `max_processes`
- `max_open_files`
- `max_locked_memory`
- `max_address_space`
- `max_file_locks`
- `max_pending_signals`
- `max_msgqueue_size`
- `max_nice_priority`
- `max_realtime_priority`
- `max_realtime_timeout`

### Other operating systems

As said before, support for other operating systems and platforms may be added on demand.
Feel free to [file an issue][issues] or make a PR!

## License

Scriptful is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE] and [LICENSE-MIT], and [COPYRIGHT] for details.

[issues]: https://github.com/aesedepece/limits-rs/issues
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
[COPYRIGHT]: COPYRIGHT
