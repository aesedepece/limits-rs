/// A placeholder for the `struct Limit` type of unsupported operating systems.
pub struct Limits();

/// Always return an `UnsupportedOS` error for unsupported operating systems.
pub fn get_pid_limits(pid: u32) -> Result<Limits, crate::Error> {
    Err(crate::Error::UnsupportedOS)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unimplemented() {
        let result = crate::get_pid_limits(0).unwrap_err();
        let expected = crate::Error::UnsupportedOS;

        assert_eq!(result, expected);
    }
}
