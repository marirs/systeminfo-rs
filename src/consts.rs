use serde::{Deserialize, Serialize};

/// System Hardware
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SystemHardware {
    pub system_manufacturer: String,
    pub system_model: String,
    pub serial_number: String,
    pub bios: String,
    pub physical_memory: String,
    pub processor: String,
    pub architecture: String,
    pub processor_vendor: String,
    pub processor_physical_cpus: String,
    pub processor_logical_cpus: String,
    pub processor_features: Vec<String>,
}

/// Operating System
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SystemOS {
    pub os: String,
    pub kernel: String,
    pub edition: String,
    pub version: String,
    pub architecture: String,
    pub hostname: String,
    pub ip_address: String,
}
