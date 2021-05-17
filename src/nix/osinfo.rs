use crate::{
    common::{
        hostname,
        ip::get_local_ip,
        utils::{exec_command_with_args, to_hashmap_with_delim},
        OSArchitecture,
    },
    consts::SystemOS,
};
use std::collections::HashMap;

pub fn from_system_os() -> SystemOS {
    //! Get Operating System Information
    //! ## Example Usage:
    //! ```ignore
    //! use systeminfo;
    //! let os_info = systeminfo::from_system_os();
    //! println!("{:#?}", os_info);
    //! ```
    let v = vec![
        std::thread::spawn(from_hostnamectl),
        std::thread::spawn(from_os_release),
    ];
    let mut it = v.into_iter().filter_map(|x| x.join().ok());

    let hostnamectl = it.next().unwrap().unwrap_or_default();
    let os_release = it.next().unwrap().unwrap_or_default();

    let os = os_release.get("NAME").cloned().unwrap_or_else(|| {
        hostnamectl
            .get("Operating System")
            .cloned()
            .unwrap_or_default()
    });

    let edition = os_release
        .get("VERSION_CODENAME")
        .cloned()
        .and_then(|x|if x.is_empty() {None} else {Some(x)})
        .unwrap_or_else(|| os_release.get("PRETTY_NAME").cloned().unwrap_or_default())
        .to_uppercase();

    let version = os_release
        .get("VERSION")
        .cloned()
        .unwrap_or_else(|| os_release.get("VERSION_ID").cloned().unwrap_or_default());

    SystemOS {
        os,
        kernel: hostnamectl.get("Kernel").cloned().unwrap_or_default(),
        architecture: OSArchitecture::get_arch().to_string(),
        hostname: hostname::get().unwrap_or_default(),
        version,
        edition,
        ip_address: get_local_ip().unwrap_or_default(),
    }
}

fn from_hostnamectl() -> Option<HashMap<String, String>> {
    let output = exec_command_with_args("hostnamectl", &[]).unwrap_or_default();
    if output.is_empty() || output.contains("command not found") {
        None
    } else {
        Some(to_hashmap_with_delim(output, ':'))
    }
}

fn from_os_release() -> Option<HashMap<String, String>> {
    let output = exec_command_with_args("cat", &["/etc/os-release"]).unwrap_or_default();
    if output.is_empty() || output.contains("command not found") {
        None
    } else {
        Some(to_hashmap_with_delim(output, '='))
    }
}
