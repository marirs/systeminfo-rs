use std::{ffi::OsString, os::unix::ffi::OsStringExt};

type Error = String;

fn wrap_buffer(mut bytes: Vec<u8>) -> OsString {
    //! Returned name might be truncated if it does not fit
    //! and `buffer` will not contain the trailing \0 in that case.
    //! Manually capping the buffer length here.
    let end = bytes
        .iter()
        .position(|&byte| byte == 0x00)
        .unwrap_or(bytes.len());
    bytes.resize(end, 0x00);

    OsString::from_vec(bytes)
}

/// Gets the hostname of the system
pub(crate) fn get() -> Result<String, Error> {
    // According to the POSIX specification,
    // host names are limited to `HOST_NAME_MAX` bytes
    // https://pubs.opengroup.org/onlinepubs/9699919799/functions/gethostname.html
    let size = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) as libc::size_t };

    let mut buffer = vec![0u8; size];
    let result = unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, size) };

    if result == 0 {
        Ok(wrap_buffer(buffer).to_str().unwrap_or_default().to_string())
    } else {
        Err(std::io::Error::last_os_error().to_string())
    }
}
