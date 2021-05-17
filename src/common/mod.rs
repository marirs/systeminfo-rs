pub(crate) mod ip;
pub(crate) mod utils;

#[cfg(not(target_os = "linux"))]
mod version;
#[cfg(not(target_os = "linux"))]
pub(crate) use self::version::Version;

mod os_arch;
pub(crate) use self::os_arch::OSArchitecture;

#[cfg(unix)]
pub(crate) mod hostname;
