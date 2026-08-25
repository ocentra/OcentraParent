use sha2::{Digest, Sha256};

use super::constants;
use super::error::ProvisioningError;

const MAGIC: [u8; 4] = [79, 67, 80, 69];
const VERSION: u16 = 1;
const CHECKSUM_DOMAIN: &str = "ocentra.pcc.enrollment-envelope.v1";
const REGISTRY_ID_DOMAIN: &str = "ocentra.pcc.enrollment-registry-id.v1";

const BROAD_SIDS: &[&[u8]] = &[
    &[1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    &[1, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0],
    &[1, 1, 0, 0, 0, 0, 0, 5, 4, 0, 0, 0],
    &[1, 1, 0, 0, 0, 0, 0, 5, 6, 0, 0, 0],
    &[1, 1, 0, 0, 0, 0, 0, 5, 11, 0, 0, 0],
    constants::SYSTEM_SID,
    &[1, 1, 0, 0, 0, 0, 0, 5, 19, 0, 0, 0],
    &[1, 1, 0, 0, 0, 0, 0, 5, 20, 0, 0, 0],
    &[1, 2, 0, 0, 0, 0, 0, 5, 32, 0, 0, 0, 32, 2, 0, 0],
];

#[derive(Clone, Eq, PartialEq)]
pub(super) struct EnrollmentSnapshot {
    pub(super) registry_security_digest: [u8; 32],
    pub(super) broker_image_digest: [u8; 32],
    pub(super) client_image_digest: [u8; 32],
    pub(super) service_digest: [u8; 32],
    pub(super) client_sid: Vec<u8>,
    pub(super) client_integrity: u32,
    pub(super) client_session: u32,
    pub(super) tpm: TpmEnrollment,
}

#[derive(Clone, Eq, PartialEq)]
pub(super) struct TpmEnrollment {
    pub(super) index: u32,
    pub(super) name_algorithm: u16,
    pub(super) attributes: u32,
    pub(super) data_size: u16,
    pub(super) policy_digest: [u8; 32],
}

pub(super) fn parse(
    bytes: &[u8],
    registry_security_digest: [u8; 32],
) -> Result<EnrollmentSnapshot, ProvisioningError> {
    if bytes.len() < 4 + 2 + 4 + 5 * 32 + 2 + 4 + 4 + 4 + 2 + 4 + 2 + 32 + 32
        || bytes.len() > 64 * 1024
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    let checksum_offset = bytes
        .len()
        .checked_sub(32)
        .ok_or(ProvisioningError::ExistingStateRejected)?;
    if domain_hash(CHECKSUM_DOMAIN.as_bytes(), &bytes[..checksum_offset])
        != bytes[checksum_offset..]
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }

    let mut cursor = Cursor::new(&bytes[..checksum_offset]);
    if cursor.take_array::<4>()? != MAGIC
        || cursor.take_u16()? != VERSION
        || usize::try_from(cursor.take_u32()?)
            .map_err(|_| ProvisioningError::ExistingStateRejected)?
            != bytes.len()
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    let registry_id_digest = cursor.take_array::<32>()?;
    let stored_security_digest = cursor.take_array::<32>()?;
    let broker_image_digest = cursor.take_array::<32>()?;
    let client_image_digest = cursor.take_array::<32>()?;
    let service_digest = cursor.take_array::<32>()?;
    let client_sid = cursor.take_sized_sid()?;
    let client_integrity = cursor.take_u32()?;
    let client_session = cursor.take_u32()?;
    let tpm = TpmEnrollment {
        index: cursor.take_u32()?,
        name_algorithm: cursor.take_u16()?,
        attributes: cursor.take_u32()?,
        data_size: cursor.take_u16()?,
        policy_digest: cursor.take_array::<32>()?,
    };

    let fixed_registry_id =
        constants::fixed_registry_id().map_err(|_| ProvisioningError::ExistingStateRejected)?;
    if !cursor.is_empty()
        || registry_id_digest
            != domain_hash(
                REGISTRY_ID_DOMAIN.as_bytes(),
                fixed_registry_id.as_str().as_bytes(),
            )
        || stored_security_digest != registry_security_digest
        || any_zero(&broker_image_digest)
        || any_zero(&client_image_digest)
        || any_zero(&service_digest)
        || client_integrity == 0
        || client_session == 0
        || tpm.index != constants::TPM_NV_INDEX
        || tpm.name_algorithm != constants::TPM_ALG_SHA256
        || tpm.attributes != constants::TPM_COUNTER_ATTRIBUTES
        || tpm.data_size != constants::TPM_COUNTER_BYTES
        || any_zero(&tpm.policy_digest)
    {
        return Err(ProvisioningError::ExistingStateRejected);
    }

    Ok(EnrollmentSnapshot {
        registry_security_digest,
        broker_image_digest,
        client_image_digest,
        service_digest,
        client_sid,
        client_integrity,
        client_session,
        tpm,
    })
}

fn any_zero(value: &[u8]) -> bool {
    value.iter().all(|byte| *byte == 0)
}

pub(super) fn domain_hash(domain: &[u8], value: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update((domain.len() as u32).to_be_bytes());
    digest.update(domain);
    digest.update((value.len() as u32).to_be_bytes());
    digest.update(value);
    digest.finalize().into()
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], ProvisioningError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(ProvisioningError::ExistingStateRejected)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(ProvisioningError::ExistingStateRejected)?;
        self.position = end;
        Ok(value)
    }

    fn take_array<const N: usize>(&mut self) -> Result<[u8; N], ProvisioningError> {
        self.take(N)?
            .try_into()
            .map_err(|_| ProvisioningError::ExistingStateRejected)
    }

    fn take_u16(&mut self) -> Result<u16, ProvisioningError> {
        Ok(u16::from_be_bytes(self.take_array()?))
    }

    fn take_u32(&mut self) -> Result<u32, ProvisioningError> {
        Ok(u32::from_be_bytes(self.take_array()?))
    }

    fn take_sized_sid(&mut self) -> Result<Vec<u8>, ProvisioningError> {
        let length = usize::from(self.take_u16()?);
        if !(8..=184).contains(&length) {
            return Err(ProvisioningError::ExistingStateRejected);
        }
        let sid = self.take(length)?.to_vec();
        validate_sid(&sid)?;
        Ok(sid)
    }

    fn is_empty(&self) -> bool {
        self.position == self.bytes.len()
    }
}

fn validate_sid(sid: &[u8]) -> Result<(), ProvisioningError> {
    if sid.len() < 8 || sid[0] != 1 || usize::from(sid[1]) > 15 {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    if sid.len() != 8 + usize::from(sid[1]) * 4 {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    if BROAD_SIDS.contains(&sid) {
        return Err(ProvisioningError::ExistingStateRejected);
    }
    Ok(())
}
