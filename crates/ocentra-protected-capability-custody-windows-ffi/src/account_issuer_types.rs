//! Public mechanical types for the fixed AccountIssuer P-256 seam.

use crate::{SecurityDescriptorObservation, WindowsText};

/// The fixed AccountIssuer P-256 public observation returned by the Windows
/// CNG boundary. This is mechanical platform evidence only; it is not an
/// enrollment, authority, or caller capability.
#[cfg(windows)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIssuerP256Observation {
    pub(crate) key_name: WindowsText,
    pub(crate) algorithm: WindowsText,
    pub(crate) algorithm_group: WindowsText,
    pub(crate) unique_name: WindowsText,
    pub(crate) implementation_type: u32,
    pub(crate) provider_version: u32,
    pub(crate) export_policy: u32,
    pub(crate) key_usage: u32,
    pub(crate) pcp_key_usage_policy: u32,
    pub(crate) key_length_bits: u32,
    pub(crate) platform_type: WindowsText,
    pub(crate) ek_public: Vec<u8>,
    pub(crate) tpm2b_name: Vec<u8>,
    pub(crate) public_key_sec1: [u8; 65],
    pub(crate) security: SecurityDescriptorObservation,
}

#[cfg(windows)]
impl AccountIssuerP256Observation {
    pub fn key_name(&self) -> &WindowsText {
        &self.key_name
    }

    pub fn algorithm(&self) -> &WindowsText {
        &self.algorithm
    }

    pub fn algorithm_group(&self) -> &WindowsText {
        &self.algorithm_group
    }

    pub fn unique_name(&self) -> &WindowsText {
        &self.unique_name
    }

    pub fn implementation_type(&self) -> u32 {
        self.implementation_type
    }

    pub fn provider_version(&self) -> u32 {
        self.provider_version
    }

    pub fn export_policy(&self) -> u32 {
        self.export_policy
    }

    pub fn key_usage(&self) -> u32 {
        self.key_usage
    }

    pub fn pcp_key_usage_policy(&self) -> u32 {
        self.pcp_key_usage_policy
    }

    pub fn key_length_bits(&self) -> u32 {
        self.key_length_bits
    }

    pub fn platform_type(&self) -> &WindowsText {
        &self.platform_type
    }

    pub fn ek_public(&self) -> &[u8] {
        &self.ek_public
    }

    pub fn tpm2b_name(&self) -> &[u8] {
        &self.tpm2b_name
    }

    pub fn public_key_sec1(&self) -> &[u8; 65] {
        &self.public_key_sec1
    }

    pub fn security(&self) -> &SecurityDescriptorObservation {
        &self.security
    }

    pub(crate) fn same_immutable_identity(&self, other: &Self) -> bool {
        self.key_name == other.key_name
            && self.algorithm == other.algorithm
            && self.algorithm_group == other.algorithm_group
            && self.unique_name == other.unique_name
            && self.implementation_type == other.implementation_type
            && self.provider_version == other.provider_version
            && self.export_policy == other.export_policy
            && self.key_usage == other.key_usage
            && self.pcp_key_usage_policy == other.pcp_key_usage_policy
            && self.key_length_bits == other.key_length_bits
            && self.platform_type == other.platform_type
            && self.ek_public == other.ek_public
            && self.tpm2b_name == other.tpm2b_name
            && self.public_key_sec1 == other.public_key_sec1
    }
}

/// One canonical low-S ECDSA P-256 signature in fixed-width P1363 form.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountIssuerP256Signature(pub(crate) [u8; 64]);

#[cfg(windows)]
impl AccountIssuerP256Signature {
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}
