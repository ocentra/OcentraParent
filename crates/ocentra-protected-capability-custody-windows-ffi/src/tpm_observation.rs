//! Accessors for bounded TPM NV metadata.

use super::NvPublic;

impl NvPublic {
    pub fn nv_index(&self) -> u32 {
        self.nv_index
    }

    pub fn name_algorithm(&self) -> u16 {
        self.name_algorithm
    }

    pub fn attributes(&self) -> u32 {
        self.attributes
    }

    pub fn auth_policy(&self) -> &[u8] {
        &self.auth_policy
    }

    pub fn data_size(&self) -> u16 {
        self.data_size
    }
}
