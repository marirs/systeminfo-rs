systeminfo
============
[![Build Status](https://travis-ci.com/marirs/systeminfo-rs.svg?branch=main)](https://travis-ci.com/marirs/systeminfo-rs)

Get the System Hardware & Operating System information.

## Usage
```toml
[dependencies]
systeminfo = { git = "https://github.com/marirs/systeminfo-rs", branch = "main" }
```
and then
```rust
use systeminfo;

fn main() {
    let sysinfo = systeminfo::from_system_hardware();
    println!("{:#?}", sysinfo);

    let osinfo = systeminfo::from_system_os();
    println!("{:#?}", osinfo);
}
```

If you want to return the object:
```rust
use systeminfo::{
    consts::SystemHardware,
    from_system_hardware
};

fn get_hw_info() -> SystemHardware {
    from_system_hardware()
}

fn main() {
    println!("{:#?}", get_hw_info())
}
```

## Running the example
```bash
cargo b --example info
```

## Example Output
- macOS
```bash
SystemHardware {
    system_manufacturer: "Apple",
    system_model: "MacBookPro15,2 (MacBook Pro)",
    serial_number: "C02Z90HQLVDL",
    bios: "1554.60.15.0.0 (iBridge: 18.16.13030.0.0,0)",
    physical_memory: "16 GB",
    processor: "Intel(R) Core(TM) i7-8569U CPU @ 2.80GHz",
    architecture: "x86_64",
    processor_vendor: "GenuineIntel",
    processor_physical_cpus: "4",
    processor_logical_cpus: "8",
    processor_features: [
        "FPU",
        ...
    ],
}
SystemOS {
    os: "macOS",
    kernel: "20.2.0",
    edition: "macOS Big Sur",
    version: "11.1",
    architecture: "64-bit",
    hostname: "chocolate",
    ip_address: "192.168.0.115",
}
```

- Linux
```bash
SystemHardware {
    system_manufacturer: "Raspberry Pi 4 Model B Rev 1.4",
    system_model: "",
    serial_number: "10000000f62c0f0d",
    bios: "",
    physical_memory: "8 GB",
    processor: "Cortex-A72",
    architecture: "aarch64",
    processor_vendor: "ARM",
    processor_physical_cpus: "4",
    processor_logical_cpus: "4",
    processor_features: [
        "FP",
        ...
    ],
}
SystemOS {
    os: "Ubuntu",
    kernel: "Linux 5.11.0-1008-raspi",
    edition: "HIRSUTE",
    version: "21.04 (Hirsute Hippo)",
    architecture: "64-bit",
    hostname: "chocolate",
    ip_address: "192.168.0.116",
}
```
- Windows
```bash
SystemHardware {
    system_manufacturer: "VMware, Inc.",
    system_model: "VMware Virtual Platform",
    serial_number: "43 2e 97 b3 5e",
    bios: "PhoenixBIOS 4.0 Release 6.0 (Wed, 22 Jul 2020 00:00:00 +0000)",
    physical_memory: "2.15 GB",
    processor: "Intel(R) Core(TM) i7-8569U CPU @ 2.80GHz",
    architecture: "X86-based PC",
    processor_vendor: "GenuineIntel",
    processor_physical_cpus: "1",
    processor_logical_cpus: "2",
    processor_features: [],
}
SystemOS {
    os: "Microsoft Windows",
    kernel: "19041",
    edition: "Windows 10 Pro",
    version: "10.0.19041",
    architecture: "32-bit",
    hostname: "DESKTOP-G089JUF",
    ip_address: "192.168.0.114",
}
```

## Requirements
- Rust 1.52+

## Supports & Tested on
- macOS
- Windows
- Linux

---
License: MIT