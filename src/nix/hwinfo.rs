#![allow(clippy::needless_collect)]
use crate::{
    common::utils::{exec_command_with_args, to_hashmap, ToVecString},
    consts::SystemHardware,
};
use humanize::Humanize;
use std::collections::HashMap;

fn get_mem_info() -> Option<HashMap<String, String>> {
    let command = "grep";
    let args = ["-i", "memtotal:", "/proc/meminfo"];
    let output = exec_command_with_args(command, &args).unwrap_or_default();
    if output.is_empty() || output.contains("No such file") {
        None
    } else {
        let total_mem = output.replace(" kB", "").trim().to_string();
        Some(to_hashmap(total_mem))
    }
}

fn dmidecode_bios() -> Option<HashMap<String, String>> {
    let command = "dmidecode";
    let args = ["-qt", "bios"];
    let output = exec_command_with_args(command, &args).unwrap_or_default();
    if output.is_empty() || output.contains("No SMBIOS nor DMI entry point found") {
        None
    } else {
        Some(
            output
                .to_vec_string(true)
                .into_iter()
                .filter(|l| {
                    l.starts_with("Vendor:")
                        || l.starts_with("Version")
                        || l.starts_with("Release Date:")
                })
                .flat_map(to_hashmap)
                .collect(),
        )
    }
}

fn dmidecode_system() -> Option<HashMap<String, String>> {
    let command = "dmidecode";
    let args = ["-qt", "system"];
    let output = exec_command_with_args(command, &args).unwrap_or_default();
    if output.is_empty() || output.contains("No SMBIOS nor DMI entry point found") {
        None
    } else {
        Some(
            output
                .to_vec_string(true)
                .into_iter()
                .filter(|l| {
                    l.starts_with("Manufacturer:")
                        || l.starts_with("Product Name:")
                        || l.starts_with("Serial Number:")
                })
                .flat_map(to_hashmap)
                .collect(),
        )
    }
}

fn from_dmidecode() -> Option<HashMap<String, String>> {
    let system = dmidecode_system();
    let bios = dmidecode_bios();
    match (system, bios) {
        (Some(mut s), Some(b)) => {
            s.extend(b);
            Some(s)
        }
        (Some(x), None) | (None, Some(x)) => Some(x),
        _ => None,
    }
}

fn from_lshw() -> Option<HashMap<String, String>> {
    let command = "lshw";
    let args = ["-quiet", "-C", "system"];
    let output = exec_command_with_args(command, &args).unwrap_or_default();
    if output.is_empty() || output.contains("No such file") {
        None
    } else {
        Some(
            output
                .to_vec_string(true)
                .into_iter()
                .filter(|l| l.starts_with("product") || l.starts_with("serial"))
                .flat_map(to_hashmap)
                .collect(),
        )
    }
}

fn from_lscpu() -> Option<HashMap<String, String>> {
    let command = "lscpu";
    let args = [];
    let output = exec_command_with_args(command, &args).unwrap_or_default();
    if output.is_empty() {
        None
    } else {
        Some(to_hashmap(output))
    }
}

pub fn from_system_hardware() -> SystemHardware {
    //! Get hardware information of the system
    //! ## Example Usage:
    //! ```ignore
    //! use systeminfo;
    //! let hw_info = systeminfo::from_system_hardware();
    //! println!("{:#?}", hw_info);
    //! ```
    let v = vec![
        std::thread::spawn(get_mem_info),
        std::thread::spawn(move || from_dmidecode().or_else(from_lshw)),
        std::thread::spawn(from_lscpu),
    ];

    let mut it = v.into_iter().filter_map(|x| x.join().ok());

    let mem_info = it.next().unwrap();
    let sys_info = it.next().unwrap().unwrap_or_default();
    let cpu_info = it.next().unwrap().unwrap_or_default();

    // get_mem_info
    let mem = mem_info
        .and_then(|m| m.get("MemTotal").cloned())
        .and_then(|m| m.parse::<f64>().ok())
        .map(|x| (x * 1000.).humanize());

    let manufacturer = sys_info
        .get("Manufacturer")
        .or_else(|| sys_info.get("product"));

    let model = sys_info.get("Product Name");

    let sn = sys_info
        .get("Serial Number")
        .or_else(|| sys_info.get("serial"));

    let bios_vendor = sys_info.get("Vendor");

    let bios_version = sys_info.get("Version");

    let bios_date = sys_info.get("Release Date");

    let bios = match (bios_vendor, bios_version, bios_date) {
        (Some(vend), Some(vers), Some(date)) => format!("{} v{} ({})", vend, vers, date),
        _ => String::new(),
    };

    SystemHardware {
        system_manufacturer: manufacturer.cloned().unwrap_or_default(),
        system_model: model.cloned().unwrap_or_default(),
        serial_number: sn.cloned().unwrap_or_default(),
        bios,
        physical_memory: mem.unwrap_or_default(),
        processor: cpu_info.get("Model name").cloned().unwrap_or_default(),
        architecture: cpu_info.get("Architecture").cloned().unwrap_or_default(),
        processor_vendor: cpu_info.get("Vendor ID").cloned().unwrap_or_default(),
        processor_physical_cpus: cpu_info
            .get("Core(s) per socket")
            .cloned()
            .unwrap_or_default(),
        processor_logical_cpus: cpu_info.get("CPU(s)").cloned().unwrap_or_default(),
        processor_features: cpu_info
            .get("Flags")
            .map(|l| l.to_ascii_uppercase())
            .map(|l| l.split_whitespace().map(Into::into).collect())
            .unwrap_or_default(),
    }
}
