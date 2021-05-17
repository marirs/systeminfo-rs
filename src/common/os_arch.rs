#[cfg(not(windows))]
use crate::common::utils::exec_command_with_args;
use core::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

/// OS Architecture
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
// #[non_exhaustive]
pub enum OSArchitecture {
    /// Unknown architecture (unable to determine).
    Unknown,
    /// 32-bit Operating System
    X32,
    /// 64-bit Operating System
    X64,
}

impl Display for OSArchitecture {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            OSArchitecture::Unknown => write!(f, "unknown architecture"),
            OSArchitecture::X32 => write!(f, "32-bit"),
            OSArchitecture::X64 => write!(f, "64-bit"),
        }
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "macos"
))]
impl OSArchitecture {
    pub fn get_arch() -> Self {
        let output = exec_command_with_args("getconf", &["LONG_BIT"]).unwrap_or_default();
        if output.contains("32") {
            Self::X32
        } else if output.contains("64") {
            Self::X64
        } else {
            Self::Unknown
        }
    }
}
