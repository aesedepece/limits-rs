//! Utilities for determining the limits that an operating system enforces on a given particular
//! process.
//!
//! In its current implementation, this crate allows convenient read of the `/proc/<pid>/limits`
//! file on GNU/Linux. On any other platform, the provided methods will return an error so that the
//! user can decide what to do in the absence of information about limits.
//!
//! Support for other operating systems and platforms may be added on demand.

use thiserror::Error;

// Support for GNU/Linux
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use crate::linux::*;

// Placeholder for all other platforms
#[cfg(not(target_os = "linux"))]
mod default;
#[cfg(not(target_os = "linux"))]
pub use crate::default::*;

/// All methods that can fail in this crate should return `Result<_, Error>`. That is, one of the
/// variants herein.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unsupported OS. Could not get process limits.")]
    UnsupportedOS,
    #[error("Proc file not found at `{}`: {}", .0, .1)]
    ProcFileNotFound(String, #[source] std::io::Error),
}

/// Get the limits for the process in which we are running (our own process id).
pub fn get_own_limits() -> Result<Limits, crate::Error> {
    let own_pid = std::process::id();

    get_pid_limits(own_pid)
}
