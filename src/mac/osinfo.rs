use crate::{
    common::{
        hostname,
        ip::get_local_ip,
        utils::{exec_command_with_args, to_hashmap},
        OSArchitecture, Version,
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
    let (version, os, kernel) = version();
    SystemOS {
        os,
        kernel,
        edition: edition(Version::from_string(&version)).to_string(),
        version,
        architecture: OSArchitecture::get_arch().to_string(),
        hostname: hostname::get().unwrap_or_default(),
        ip_address: get_local_ip().unwrap_or_default(),
    }
}

fn version() -> (String, String, String) {
    //! Returns Version, OS_Name, Kernel_Version
    let v = vec![
        std::thread::spawn(get_sw_vers),
        std::thread::spawn(get_kernel),
    ];
    let mut it = v.into_iter().filter_map(|x| x.join().ok());

    let sw = it.next().unwrap().unwrap_or_default();
    let kern = it.next().unwrap().unwrap_or_default();
    (
        sw.get("ProductVersion").cloned().unwrap_or_default(),
        sw.get("ProductName").cloned().unwrap_or_default(),
        kern.get("Kernel").cloned().unwrap_or_default(),
    )
}

// Based on the Version semantic, determine the macOS edition
// https://support.apple.com/en-in/HT201260
fn edition<'a>(v: Version) -> &'a str {
    if let Version::Semantic(major, minor, patch) = v {
        match (major, minor, patch) {
            (15, _, _) => "macOS Sequoia",
            (14, _, _) => "macOS Sonoma",
            (13, _, _) => "macOS Ventura",
            (12, _, _) => "macOS Monterey",
            (11, _, _) => "macOS Big Sur",
            (10, 15, _) => "macOS Catalina",
            (10, 14, _) => "macOS Mojave",
            (10, 13, _) => "macOS High Sierra",
            (10, 12, _) => "macOS Sierra",
            (10, 11, _) => "OS X El Capitan",
            (10, 10, _) => "OS X Yosemite",
            (10, 9, _) => "OS X Mavericks",
            (10, 8, _) => "OS X Mountain Lion",
            (10, 7, _) => "OS X Lion",
            (10, 6, _) => "Mac OS X Snow Leopard",
            (10, 5, _) => "Mac OS X Leopard",
            (10, 4, _) => "Mac OS X Tiger",
            (10, 3, _) => "Mac OS X Panther",
            (10, 2, _) => "Mac OS X Jaguar",
            (10, 1, _) => "Mac OS X Puma",
            (10, 0, _) => "Mac OS X Cheetah",
            (_, _, _) => "Unknown",
        }
    } else {
        ""
    }
}

fn get_sw_vers() -> Option<HashMap<String, String>> {
    let output = exec_command_with_args("sw_vers", &[]).unwrap_or_default();
    if output.is_empty() {
        None
    } else {
        Some(to_hashmap(output))
    }
}

fn get_kernel() -> Option<HashMap<String, String>> {
    let output = exec_command_with_args("uname", &["-r"]).unwrap_or_default();
    if output.is_empty() {
        None
    } else {
        Some(
            vec![("Kernel".to_string(), output.trim().to_string())]
                .into_iter()
                .collect(),
        )
    }
}
