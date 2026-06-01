use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPairingDeviceHardwareProfile {
    #[serde(default)]
    pub manufacturer: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub cpu_model: Option<String>,
    #[serde(default)]
    pub cpu_cores: Option<String>,
    #[serde(default)]
    pub memory_total: Option<String>,
    #[serde(default)]
    pub gpu_model: Option<String>,
    #[serde(default)]
    pub gpu_driver: Option<String>,
    #[serde(default)]
    pub gpu_memory: Option<String>,
    #[serde(default)]
    pub nvidia_smi: Option<String>,
}
