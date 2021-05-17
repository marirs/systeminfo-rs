#![allow(non_camel_case_types, non_snake_case)]
use crate::consts::SystemHardware;
use humanize::Humanize;
use serde::{Deserialize, Deserializer, Serialize};
use wmi::{COMLibrary, WMIConnection, WMIDateTime};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Win32_Bios {
    Caption: String,
    SerialNumber: String,
    #[serde(deserialize_with = "wmidatetime_to_string")]
    ReleaseDate: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Win32_Computersystem {
    #[serde(deserialize_with = "i8_to_string")]
    NumberOfLogicalProcessors: String,
    #[serde(deserialize_with = "i8_to_string")]
    NumberOfProcessors: String,
    SystemType: String,
    Manufacturer: String,
    Model: String,
    #[serde(deserialize_with = "to_human")]
    TotalPhysicalMemory: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Win32_Processor {
    Name: String,
    Manufacturer: String,
    #[serde(deserialize_with = "i8_to_string")]
    NumberOfLogicalProcessors: String,
    #[serde(deserialize_with = "i8_to_string")]
    NumberOfCores: String,
}

fn i8_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    i8::deserialize(deserializer).map(|x| x.to_string())
}

fn to_human<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|x| {
        let v = x.parse::<f64>().ok().unwrap_or_default();
        v.humanize()
    })
}

fn wmidatetime_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    WMIDateTime::deserialize(deserializer).map(|x| x.0.to_rfc2822())
}

fn from_bios(wmi_con: &WMIConnection) -> Win32_Bios {
    let bios_info: Vec<Win32_Bios> = wmi_con.query().unwrap();
    bios_info.iter().next().cloned().unwrap_or_default()
}

fn from_computer_system(wmi_con: &WMIConnection) -> Win32_Computersystem {
    let sysinfo: Vec<Win32_Computersystem> = wmi_con.query().unwrap();
    sysinfo.iter().next().cloned().unwrap_or_default()
}

fn from_processor(wmi_con: &WMIConnection) -> Win32_Processor {
    let processor_info: Vec<Win32_Processor> = wmi_con.query().unwrap();
    processor_info.iter().next().cloned().unwrap_or_default()
}

pub fn from_system_hardware() -> SystemHardware {
    //! Get hardware information of the system
    //! ## Example Usage:
    //! ```ignore
    //! use systeminfo;
    //! let hw_info = systeminfo::from_system_hardware();
    //! println!("{:#?}", hw_info);
    //! ```
    let com_con = COMLibrary::new().unwrap();
    let wmi_conn = WMIConnection::new(com_con.into()).unwrap();

    let cpu_info = from_processor(&wmi_conn);
    let sys_info = from_computer_system(&wmi_conn);
    let bios_info = from_bios(&wmi_conn);

    SystemHardware {
        system_manufacturer: sys_info.Manufacturer,
        system_model: sys_info.Model,
        serial_number: bios_info.SerialNumber,
        bios: format!("{} ({})", bios_info.Caption.trim(), bios_info.ReleaseDate),
        physical_memory: sys_info.TotalPhysicalMemory,
        processor: cpu_info.Name,
        architecture: sys_info.SystemType,
        processor_vendor: cpu_info.Manufacturer,
        processor_physical_cpus: sys_info.NumberOfProcessors,
        processor_logical_cpus: sys_info.NumberOfLogicalProcessors,
        processor_features: vec![],
    }
}
