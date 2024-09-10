use chrono::{prelude::DateTime, Utc};
use std::{
    cmp,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub trait Humanize {
    fn humanize(&self) -> String;
    fn humanize_into(&self, _: &str) -> String;
}

impl Humanize for SystemTime {
    fn humanize(&self) -> String {
        //! Convert Unix Epoch to Human Readable Time
        //!
        //! ## Example usage
        //! ```
        //! use std::time::{UNIX_EPOCH, Duration};
        //! use humanize::Humanize;
        //!
        //! let epoch_time = UNIX_EPOCH + Duration::from_secs(1610859829);
        //! let human_time = String::from("Sun Jan 17 2021, 05:03:49");
        //! assert_eq!(epoch_time.humanize(), human_time);
        //! println!("Time: {}", epoch_time.humanize());
        //! ```
        self.humanize_into("%a %b %e %Y, %T")
    }

    fn humanize_into(&self, fmt: &str) -> String {
        //! Convert Unix Epoch to Human Readable Time with a
        //! custom date/time format
        //!
        //! ## Example usage
        //! ```
        //! use std::time::{UNIX_EPOCH, Duration};
        //! use humanize::Humanize;
        //!
        //! let epoch_time = UNIX_EPOCH + Duration::from_secs(1610859829);
        //! let human_time = String::from("Sun Jan 17 2021, 05:03:49");
        //! assert_eq!(epoch_time.humanize_into("%a %b %e %Y, %T"), human_time);
        //! println!("Time: {}", epoch_time.humanize_into("%a %b %e %Y, %T"));
        //! ```
        let secs = self.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let d = UNIX_EPOCH + Duration::from_secs(secs);
        let datetime = DateTime::<Utc>::from(d);
        let human_time = datetime.format(fmt.as_ref()).to_string();
        human_time
    }
}

impl Humanize for f64 {
    fn humanize(&self) -> String {
        //! Convert a file size of type [f64] into human readable format
        //!
        //! ## Example usage
        //! ```
        //! use humanize::Humanize;
        //!
        //! let file_size = 1028 as f64;
        //! println!("Size: {}", file_size.humanize());
        //! ```
        let negative = if self.is_sign_positive() { "" } else { "-" };
        let size = self.abs();
        let units: [&str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
        if size < 1_f64 {
            format!("{}{} {}", negative, size, "B")
        } else {
            let delim = 1000_f64;
            let exponent = cmp::min(
                (size.ln() / delim.ln()).floor() as i32,
                (units.len() - 1) as i32,
            );
            let pretty_bytes = format!("{:.2}", size / delim.powi(exponent))
                .parse::<f64>()
                .unwrap()
                * 1_f64;
            let unit = units[exponent as usize];
            format!("{}{} {}", negative, pretty_bytes, unit)
        }
    }

    fn humanize_into(&self, _: &str) -> String {
        //! ## Not allowed on f64. Will result in panic!
        panic!("Not allowed for f64")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_humanize_time() {
        let epoch_time = UNIX_EPOCH + Duration::from_secs(1610859829);
        let human_time = String::from("Sun Jan 17 2021, 05:03:49");
        assert_eq!(epoch_time.humanize(), human_time);
    }

    #[test]
    fn test_humanize_into_time() {
        let epoch_time = UNIX_EPOCH + Duration::from_secs(1610859829);
        let human_time = String::from("2021-01-17");
        assert_eq!(epoch_time.humanize_into("%Y-%m-%d"), human_time);
    }

    #[test]
    fn test_humanize_file_size() {
        let file_size = 1000_f64;
        assert_eq!(file_size.humanize(), "1 kB");
    }

    #[test]
    #[should_panic(expected = "Not allowed for f64")]
    fn test_humanize_into_f64_panic() {
        let file_size = 1000_f64;
        file_size.humanize_into("");
    }
}
