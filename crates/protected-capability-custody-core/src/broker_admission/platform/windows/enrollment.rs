use ocentra_protected_capability_custody_windows_ffi::{
    OwnedRegistryChain, RegistryPath, RegistryValueName, RegistryValueObservation,
};

use crate::platform::PlatformError;

use super::enrollment_record::{domain_hash, EnrollmentRecord, TpmPublicEnrollment};
use super::enrollment_security::{registry_security_digest, validate_leaf_security};
use super::map_ffi_error;

const REGISTRY_ROOT: &str = "Software\\Ocentra\\ProtectedCapabilityCustody";
const ENROLLMENT_SUBKEY: &str = "Enrollment";
const ENROLLMENT_VALUE_NAME: &str = "authority-v1";
const REG_BINARY: u32 = 3;
const REGISTRY_ID_DOMAIN: &[u8] = b"ocentra.pcc.enrollment-registry-id.v1";

pub(super) struct VerifiedEnrollment {
    path: RegistryPath,
    value_name: RegistryValueName,
    chain: OwnedRegistryChain,
    value: RegistryValueObservation,
    record: EnrollmentRecord,
}

impl VerifiedEnrollment {
    pub(super) fn open(registry_id: &str) -> Result<Self, PlatformError> {
        validate_registry_id(registry_id)?;
        let path = RegistryPath::try_from_str(&format!(
            "{REGISTRY_ROOT}\\{registry_id}\\{ENROLLMENT_SUBKEY}"
        ))
        .map_err(map_ffi_error)?;
        let value_name =
            RegistryValueName::try_from_str(ENROLLMENT_VALUE_NAME).map_err(map_ffi_error)?;
        let chain = OwnedRegistryChain::open_hklm(&path).map_err(map_ffi_error)?;
        validate_leaf_security(&chain)?;
        let value = chain.observe_value(&value_name).map_err(map_ffi_error)?;
        if value.value().value_type() != REG_BINARY {
            return Err(PlatformError::Tampered);
        }
        let record = EnrollmentRecord::parse(value.value().data())?;
        if record.registry_id_digest != domain_hash(REGISTRY_ID_DOMAIN, registry_id.as_bytes())
            || record.registry_security_digest != registry_security_digest(&chain)?
        {
            return Err(PlatformError::Tampered);
        }
        let enrollment = Self {
            path,
            value_name,
            chain,
            value,
            record,
        };
        enrollment.revalidate()?;
        Ok(enrollment)
    }

    pub(super) fn revalidate(&self) -> Result<(), PlatformError> {
        self.chain.revalidate().map_err(map_ffi_error)?;
        let current = self
            .chain
            .reobserve_value(&self.value)
            .map_err(map_ffi_error)?;
        if current != self.value {
            return Err(PlatformError::Tampered);
        }

        // Retained handles prove the originally opened objects did not drift;
        // this fresh open additionally proves the fixed path still resolves to
        // the same exact chain and value rather than a replacement key tree.
        let reopened = OwnedRegistryChain::open_hklm(&self.path).map_err(map_ffi_error)?;
        validate_leaf_security(&reopened)?;
        if reopened.observations().map_err(map_ffi_error)?
            != self.chain.observations().map_err(map_ffi_error)?
            || reopened
                .observe_value(&self.value_name)
                .map_err(map_ffi_error)?
                != self.value
        {
            return Err(PlatformError::Tampered);
        }
        reopened.revalidate().map_err(map_ffi_error)
    }

    pub(super) fn client_sid(&self) -> &[u8] {
        &self.record.client_sid
    }

    pub(super) fn client_sid_sddl(&self) -> &str {
        &self.record.client_sid_sddl
    }

    pub(super) fn client_integrity(&self) -> u32 {
        self.record.client_integrity
    }

    pub(super) fn client_session(&self) -> u32 {
        self.record.client_session
    }

    pub(super) fn broker_image_digest(&self) -> &[u8; 32] {
        &self.record.broker_image_digest
    }

    pub(super) fn client_image_digest(&self) -> &[u8; 32] {
        &self.record.client_image_digest
    }

    pub(super) fn service_digest(&self) -> &[u8; 32] {
        &self.record.service_digest
    }

    pub(super) fn tpm(&self) -> &TpmPublicEnrollment {
        &self.record.tpm
    }
}

fn validate_registry_id(value: &str) -> Result<(), PlatformError> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(PlatformError::InvalidAttestation);
    }
    Ok(())
}
