use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub enum Version {
    /// Unknown version.
    #[default]
    Unknown,
    /// Semantic version (major.minor.patch).
    Semantic(u64, u64, u64),
    /// Rolling version. Optionally contains the release date in the string format.
    Rolling(Option<String>),
    /// Custom version format.
    Custom(String),
}

#[cfg(target_os = "macos")]
impl Version {
    pub fn from_string<S: Into<String> + AsRef<str>>(s: S) -> Self {
        if s.as_ref().is_empty() {
            Self::Unknown
        } else if let Some((major, minor, patch)) = parse_version_str(s.as_ref()) {
            Self::Semantic(major, minor, patch)
        } else {
            Self::Custom(s.into())
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Self::Unknown => f.write_str("Unknown"),
            Self::Semantic(major, minor, patch) => write!(f, "{}.{}.{}", major, minor, patch),
            Self::Rolling(ref date) => {
                let date = match date {
                    Some(date) => format!(" ({})", date),
                    None => "".to_owned(),
                };
                write!(f, "Rolling Release{}", date)
            }
            Self::Custom(ref version) => write!(f, "{}", version),
        }
    }
}

#[cfg(target_os = "macos")]
pub fn parse_version_str(s: &str) -> Option<(u64, u64, u64)> {
    //! Parse a version number string and return (major, minor, patch) tuple
    let mut iter = s.trim().split_terminator('.').fuse();

    let major = iter.next().and_then(|s| s.parse().ok())?;
    let minor = iter.next().unwrap_or("0").parse().ok()?;
    let patch = iter.next().unwrap_or("0").parse().ok()?;

    if iter.next().is_some() {
        return None;
    }

    Some((major, minor, patch))
}
