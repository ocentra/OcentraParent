use ocentra_protected_capability_custody_windows_ffi::TpmNvPublicObservation;
use sha2::{Digest, Sha256};

use crate::platform::PlatformError;

const ENROLLMENT_MAGIC: [u8; 4] = *b"OCPE";
const ENROLLMENT_VERSION: u16 = 1;
const ENROLLMENT_CHECKSUM_DOMAIN: &[u8] = b"ocentra.pcc.enrollment-envelope.v1";
const SYSTEM_SID: &str = "S-1-5-18";
const TPM_NV_INDEX: u32 = 0x0180_f001;
const TPM_ALG_SHA256: u16 = 0x000b;
const TPM_COUNTER_ATTRIBUTES: u32 = 0x6208_0018;
const TPM_COUNTER_BYTES: u16 = 8;

pub(super) struct TpmPublicEnrollment {
    index: u32,
    name_algorithm: u16,
    attributes: u32,
    data_size: u16,
    policy_digest: [u8; 32],
}

pub(super) struct EnrollmentRecord {
    pub(super) registry_id_digest: [u8; 32],
    pub(super) registry_security_digest: [u8; 32],
    pub(super) broker_image_digest: [u8; 32],
    pub(super) client_image_digest: [u8; 32],
    pub(super) service_digest: [u8; 32],
    pub(super) client_sid: Vec<u8>,
    pub(super) client_sid_sddl: String,
    pub(super) client_integrity: u32,
    pub(super) client_session: u32,
    pub(super) tpm: TpmPublicEnrollment,
}

impl TpmPublicEnrollment {
    pub(super) fn verify(&self, observed: &TpmNvPublicObservation) -> Result<(), PlatformError> {
        let expected_name = tpm_name(observed);
        if observed.nv_index() != self.index
            || observed.name_algorithm() != self.name_algorithm
            || observed.attributes() != self.attributes
            || observed.data_size() != self.data_size
            || observed.auth_policy() != self.policy_digest
            || expected_name.as_slice() != observed.name()
        {
            return Err(PlatformError::Tampered);
        }
        Ok(())
    }
}

impl EnrollmentRecord {
    pub(super) fn parse(bytes: &[u8]) -> Result<Self, PlatformError> {
        const FIXED_WITHOUT_SID_AND_CHECKSUM: usize =
            4 + 2 + 4 + (5 * 32) + 2 + 4 + 4 + 4 + 2 + 4 + 2 + 32;
        if bytes.len() < FIXED_WITHOUT_SID_AND_CHECKSUM + 32
            || bytes.len()
                > ocentra_protected_capability_custody_protocol::constants::MAX_REGISTRY_VALUE_BYTES
        {
            return Err(PlatformError::Tampered);
        }
        let checksum_offset = bytes.len().checked_sub(32).ok_or(PlatformError::Tampered)?;
        let expected_checksum = domain_hash(ENROLLMENT_CHECKSUM_DOMAIN, &bytes[..checksum_offset]);
        if bytes[checksum_offset..] != expected_checksum {
            return Err(PlatformError::Tampered);
        }

        let mut cursor = Cursor::new(&bytes[..checksum_offset]);
        if cursor.take_array::<4>()? != ENROLLMENT_MAGIC
            || cursor.take_u16()? != ENROLLMENT_VERSION
            || usize::try_from(cursor.take_u32()?).map_err(|_error| PlatformError::Tampered)?
                != bytes.len()
        {
            return Err(PlatformError::Tampered);
        }
        let registry_id_digest = cursor.take_array()?;
        let registry_security_digest = cursor.take_array()?;
        let broker_image_digest = cursor.take_array()?;
        let client_image_digest = cursor.take_array()?;
        let service_digest = cursor.take_array()?;
        let client_sid = cursor.take_sized()?;
        let client_sid_sddl = sid_to_sddl(&client_sid)?;
        reject_broad_client_sid(&client_sid_sddl)?;
        let client_integrity = cursor.take_u32()?;
        let client_session = cursor.take_u32()?;
        let tpm = TpmPublicEnrollment {
            index: cursor.take_u32()?,
            name_algorithm: cursor.take_u16()?,
            attributes: cursor.take_u32()?,
            data_size: cursor.take_u16()?,
            policy_digest: cursor.take_array()?,
        };
        if !cursor.is_empty()
            || client_integrity == 0
            || client_session == 0
            || tpm.index != TPM_NV_INDEX
            || tpm.name_algorithm != TPM_ALG_SHA256
            || tpm.attributes != TPM_COUNTER_ATTRIBUTES
            || tpm.data_size != TPM_COUNTER_BYTES
            || tpm.policy_digest == [0_u8; 32]
        {
            return Err(PlatformError::Tampered);
        }
        Ok(Self {
            registry_id_digest,
            registry_security_digest,
            broker_image_digest,
            client_image_digest,
            service_digest,
            client_sid,
            client_sid_sddl,
            client_integrity,
            client_session,
            tpm,
        })
    }
}

fn tpm_name(observed: &TpmNvPublicObservation) -> Vec<u8> {
    let mut public = Vec::new();
    public.extend_from_slice(&observed.nv_index().to_be_bytes());
    public.extend_from_slice(&observed.name_algorithm().to_be_bytes());
    public.extend_from_slice(&observed.attributes().to_be_bytes());
    public.extend_from_slice(&(observed.auth_policy().len() as u16).to_be_bytes());
    public.extend_from_slice(observed.auth_policy());
    public.extend_from_slice(&observed.data_size().to_be_bytes());
    let public_digest = Sha256::digest(&public);
    let mut name = Vec::with_capacity(2 + public_digest.len());
    name.extend_from_slice(&observed.name_algorithm().to_be_bytes());
    name.extend_from_slice(&public_digest);
    name
}

pub(super) fn domain_hash(domain: &[u8], value: &[u8]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update((domain.len() as u32).to_be_bytes());
    digest.update(domain);
    digest.update((value.len() as u32).to_be_bytes());
    digest.update(value);
    digest.finalize().into()
}

fn reject_broad_client_sid(sid: &str) -> Result<(), PlatformError> {
    const BROAD: [&str; 9] = [
        "S-1-1-0",
        "S-1-3-0",
        "S-1-5-4",
        "S-1-5-6",
        "S-1-5-11",
        SYSTEM_SID,
        "S-1-5-19",
        "S-1-5-20",
        "S-1-5-32-544",
    ];
    if BROAD.contains(&sid) {
        Err(PlatformError::Tampered)
    } else {
        Ok(())
    }
}

fn sid_to_sddl(bytes: &[u8]) -> Result<String, PlatformError> {
    if bytes.len() < 8 || bytes[0] != 1 || usize::from(bytes[1]) > 15 {
        return Err(PlatformError::Tampered);
    }
    let count = usize::from(bytes[1]);
    if bytes.len() != 8 + count * 4 {
        return Err(PlatformError::Tampered);
    }
    let authority = bytes[2..8]
        .iter()
        .fold(0_u64, |value, byte| (value << 8) | u64::from(*byte));
    let mut value = format!("S-1-{authority}");
    for chunk in bytes[8..].chunks_exact(4) {
        let sub = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        value.push('-');
        value.push_str(&sub.to_string());
    }
    Ok(value)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], PlatformError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(PlatformError::Tampered)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(PlatformError::Tampered)?;
        self.position = end;
        Ok(value)
    }

    fn take_array<const N: usize>(&mut self) -> Result<[u8; N], PlatformError> {
        self.take(N)?
            .try_into()
            .map_err(|_error| PlatformError::Tampered)
    }

    fn take_u16(&mut self) -> Result<u16, PlatformError> {
        self.take_array().map(u16::from_be_bytes)
    }

    fn take_u32(&mut self) -> Result<u32, PlatformError> {
        self.take_array().map(u32::from_be_bytes)
    }

    fn take_sized(&mut self) -> Result<Vec<u8>, PlatformError> {
        let length = usize::from(self.take_u16()?);
        if length == 0 || length > 184 {
            return Err(PlatformError::Tampered);
        }
        self.take(length).map(<[u8]>::to_vec)
    }

    fn is_empty(&self) -> bool {
        self.position == self.bytes.len()
    }
}
