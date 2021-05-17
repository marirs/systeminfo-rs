use crate::common::utils::{exec_command_with_args, to_hashmap};
use crate::consts::SystemHardware;
use std::collections::HashMap;

fn get_arch() -> Option<HashMap<String, String>> {
    let arch = exec_command_with_args("uname", &["-m"]).unwrap_or_default();
    if arch.is_empty() {
        None
    } else {
        Some(
            vec![("arch".to_string(), arch.trim().to_string())]
                .into_iter()
                .collect(),
        )
    }
}

fn from_systemprofiler() -> Option<HashMap<String, String>> {
    let command = "system_profiler";
    let args = ["SPHardwareDataType"];
    let output = exec_command_with_args(command, &args).unwrap_or_default();
    if output.is_empty() {
        None
    } else {
        Some(to_hashmap(output))
    }
}

fn from_sysctl() -> Option<HashMap<String, String>> {
    let command = "sysctl";
    let args = ["machdep.cpu", "hw"];
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
        std::thread::spawn(from_systemprofiler),
        std::thread::spawn(from_sysctl),
        std::thread::spawn(get_arch),
    ];
    let mut it = v.into_iter().filter_map(|x| x.join().ok());

    let sys_info = it.next().unwrap().unwrap_or_default();
    let sysctl = it.next().unwrap().unwrap_or_default();
    let arch = it.next().unwrap().unwrap_or_default();

    // from systemprofiler
    let system_model = match (sys_info.get("Model Identifier"), sys_info.get("Model Name")) {
        (Some(model), Some(name)) => format!("{} ({})", model, name),
        _ => String::new(),
    };
    let mem = sys_info.get("Memory");
    let sn = sys_info.get("Serial Number (system)");
    let boot_rom = sys_info.get("System Firmware Version");

    // from sysctl hw machdep.cpu
    let cpu_brand = sysctl.get("machdep.cpu.brand_string");
    let cpu = sysctl.get("hw.physicalcpu");
    let cores = sysctl.get("hw.logicalcpu");
    let vendor = sysctl.get("machdep.cpu.vendor");

    let leaf7_features = sysctl.get("machdep.cpu.leaf7_features");
    let cpu_features = sysctl.get("machdep.cpu.features");
    let ext_features = sysctl.get("machdep.cpu.extfeatures");
    let processor_features = format!(
        "{} {} {}",
        cpu_features.cloned().unwrap_or_default(),
        ext_features.cloned().unwrap_or_default(),
        leaf7_features.cloned().unwrap_or_default()
    )
    .split_whitespace()
    .into_iter()
    .map(|l| l.trim().to_ascii_uppercase())
    .collect();

    SystemHardware {
        system_manufacturer: "Apple".into(),
        system_model,
        serial_number: sn.cloned().unwrap_or_default(),
        bios: boot_rom.cloned().unwrap_or_default(),
        physical_memory: mem.cloned().unwrap_or_default(),
        processor: cpu_brand.cloned().unwrap_or_default(),
        architecture: arch.get("arch").cloned().unwrap_or_default(),
        processor_vendor: vendor.cloned().unwrap_or_default(),
        processor_physical_cpus: cpu.cloned().unwrap_or_default(),
        processor_logical_cpus: cores.cloned().unwrap_or_default(),
        processor_features,
    }
}
