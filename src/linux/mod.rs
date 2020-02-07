/// A limit for a GNU/Linux specific limitable property.
///
/// Any given limit always contain a _soft_ and a _hard_ limit.
///
/// A soft or hard limited whose value is `None` here means there is no actual limit, i.e. the value
/// found in `/proc/<pid>/limits` is `unlimited`.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Limit {
    pub soft: Option<u32>,
    pub hard: Option<u32>,
}

/// A structure containing all possible properties that can be limited by a GNU/Linux operating
/// system.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Limits {
    pub max_cpu_time: Limit,
    pub max_file_size: Limit,
    pub max_data_size: Limit,
    pub max_stack_size: Limit,
    pub max_core_file_size: Limit,
    pub max_resident_set: Limit,
    pub max_processes: Limit,
    pub max_open_files: Limit,
    pub max_locked_memory: Limit,
    pub max_address_space: Limit,
    pub max_file_locks: Limit,
    pub max_pending_signals: Limit,
    pub max_msgqueue_size: Limit,
    pub max_nice_priority: Limit,
    pub max_realtime_priority: Limit,
    pub max_realtime_timeout: Limit,
}

impl Limits {
    /// Set properties on a `Limit` structure, as read from strings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use limits_rs::{Limit, Limits};
    ///
    /// // Create a new limits structure
    /// let mut limits = Limits::default();
    ///
    /// // Trying to set a non-existing property should do nothing
    /// limits.set_property_from_strings("Does_not_exist", "123", "456");
    /// assert_eq!(limits, Limits::default());
    ///
    /// // Let's set a limit for a existing property and assert that the limit is actually stored in
    /// // the structure
    /// limits.set_property_from_strings("Max file locks", "123", "456");
    /// assert_eq!(limits.max_file_locks, Limit { soft: Some(123), hard: Some(456) })
    ///
    /// ```
    pub fn set_property_from_strings(&mut self, name: &str, soft_string: &str, hard_string: &str) {
        use std::str::FromStr;

        let lower_case = name.to_lowercase();

        let soft = if soft_string == "unlimited" {
            None
        } else {
            u32::from_str(soft_string).ok()
        };

        let hard = if hard_string == "unlimited" {
            None
        } else {
            u32::from_str(hard_string).ok()
        };

        let new_limit = Limit { soft, hard };

        match lower_case.as_str() {
            "max cpu time" => self.max_cpu_time = new_limit,
            "max file_size" => self.max_file_size = new_limit,
            "max data size" => self.max_data_size = new_limit,
            "max stack size" => self.max_stack_size = new_limit,
            "max core file size" => self.max_core_file_size = new_limit,
            "max resident set" => self.max_resident_set = new_limit,
            "max processes" => self.max_processes = new_limit,
            "max open files" => self.max_open_files = new_limit,
            "max locked memory" => self.max_locked_memory = new_limit,
            "max address space" => self.max_address_space = new_limit,
            "max file locks" => self.max_file_locks = new_limit,
            "max pending signals" => self.max_pending_signals = new_limit,
            "max msgqueue size" => self.max_msgqueue_size = new_limit,
            "max nice priority" => self.max_nice_priority = new_limit,
            "max realtime priority" => self.max_realtime_priority = new_limit,
            "max realtime timeout" => self.max_realtime_timeout = new_limit,
            _ => (),
        }
    }
}

/// Get the limits for a specific process identifier.
///
/// Along `get_own_limits`, this method provides the core functionality of this crate.
///
/// # Examples
///
/// ```rust
/// use limits_rs::get_pid_limits;
///
/// // Let's check what the CPU time hard limit is for process `1`.
/// let limits = get_pid_limits(1).unwrap();
/// let max_cpu_time_hard_limit = limits.max_cpu_time.hard;
/// ```
pub fn get_pid_limits(pid: u32) -> Result<Limits, crate::Error> {
    // Rad the limits file for the process, and put all the lines into an iterator.
    let file_path = format!("/proc/{}/limits", pid);
    let file = std::fs::File::open(&file_path)
        .map_err(|io_error| crate::Error::ProcFileNotFound(file_path, io_error))?;
    let reader = std::io::BufReader::new(file);

    get_limits_from_reader(reader)
}

/// Read limits from any type that implements the `std::io::BufRead` crate, such as
/// `std::io::BufReader` or `std::io::Cursor`.
fn get_limits_from_reader<T>(reader: T) -> Result<Limits, crate::Error>
where
    T: std::io::BufRead,
{
    let mut limits = Limits::default();
    let mut lines = std::io::BufRead::lines(reader).filter_map(Result::ok);

    // Skip first line, which always contains the table header.
    lines.next();

    for line in lines {
        // Separate the name of the property from the rest of the table, which is padded to 27
        // characters, i.e. the soft limits always start at character 27.
        let (property, values) = line.split_at(26);
        let property = property.trim();
        let values: Vec<&str> = values.split_whitespace().collect();
        limits.set_property_from_strings(property, values[0], values[1]);
    }

    Ok(limits)
}

#[cfg(test)]
mod tests {
    use crate::{Limit, Limits};

    #[test]
    fn test_own_limits_does_not_panic() {
        crate::get_own_limits().unwrap();
    }

    #[test]
    fn test_pid_limits_does_not_panic() {
        crate::get_pid_limits(1).unwrap();
    }

    #[test]
    fn test_proc_file_not_found() {
        let error = format!("{:?}", super::get_pid_limits(std::u32::MAX).unwrap_err());
        let expected_error = String::from(r#"ProcFileNotFound("/proc/4294967295/limits", Os { code: 2, kind: NotFound, message: "No such file or directory" })"#);

        assert_eq!(error, expected_error);
    }

    #[test]
    fn test_from_empty_string() {
        let reader = std::io::Cursor::new("");
        let limits = super::get_limits_from_reader(reader).unwrap();

        let expected_limits = Limits::default();

        assert_eq!(limits, expected_limits);
    }

    #[test]
    fn test_from_correct_string() {
        let reader = std::io::Cursor::new(
            r#"Limit                     Soft Limit           Hard Limit           Units
Max cpu time              unlimited            unlimited            seconds
Max file size             unlimited            unlimited            bytes
Max data size             unlimited            unlimited            bytes
Max stack size            8388608              unlimited            bytes
Max core file size        unlimited            unlimited            bytes
Max resident set          unlimited            unlimited            bytes
Max processes             62935                62935                processes
Max open files            1024                 524288               files
Max locked memory         65536                65536                bytes
Max address space         unlimited            unlimited            bytes
Max file locks            unlimited            unlimited            locks
Max pending signals       62935                62935                signals
Max msgqueue size         819200               819200               bytes
Max nice priority         0                    0
Max realtime priority     99                   99
Max realtime timeout      unlimited            unlimited            us"#,
        );
        let limits = super::get_limits_from_reader(reader).unwrap();

        let expected_limits = Limits {
            max_cpu_time: Default::default(),
            max_file_size: Default::default(),
            max_data_size: Default::default(),
            max_stack_size: Limit {
                soft: Some(8388608),
                hard: None,
            },
            max_core_file_size: Default::default(),
            max_resident_set: Default::default(),
            max_processes: Limit {
                soft: Some(62935),
                hard: Some(62935),
            },
            max_open_files: Limit {
                soft: Some(1024),
                hard: Some(524288),
            },
            max_locked_memory: Limit {
                soft: Some(65536),
                hard: Some(65536),
            },
            max_address_space: Default::default(),
            max_file_locks: Default::default(),
            max_pending_signals: Limit {
                soft: Some(62935),
                hard: Some(62935),
            },
            max_msgqueue_size: Limit {
                soft: Some(819200),
                hard: Some(819200),
            },
            max_nice_priority: Limit {
                soft: Some(0),
                hard: Some(0),
            },
            max_realtime_priority: Limit {
                soft: Some(99),
                hard: Some(99),
            },
            max_realtime_timeout: Default::default(),
        };

        assert_eq!(limits, expected_limits);
    }
}
