//! Canonical TPM2 NV enrollment parsing and exact live comparison.

use crate::{Error, InputFault, Result, TpmNvEnrollment, TpmNvPublic};

const ENROLLMENT_MAGIC: [u8; 4] = [0x4f, 0x43, 0x4e, 0x56];
const ENROLLMENT_VERSION: u16 = 1;
const ENROLLMENT_FIXED_BYTES: usize = 22;
const MAX_AUTH_POLICY_BYTES: usize = 64;
const NV_HANDLE_PREFIX: u32 = 0x0100_0000;
const HANDLE_TYPE_MASK: u32 = 0xff00_0000;
const TPM_ALG_SHA256: u16 = 0x000b;

impl TpmNvEnrollment {
    /// Parse one canonical, versioned installer enrollment record.
    ///
    /// Encoding is big-endian: magic, version, total length, NV index, name
    /// algorithm, attributes, data size, policy length, then exact policy.
    pub fn from_canonical_bytes(record: &[u8]) -> Result<Self> {
        if record.len() < ENROLLMENT_FIXED_BYTES
            || record.len() > ENROLLMENT_FIXED_BYTES + MAX_AUTH_POLICY_BYTES
            || record[..4] != ENROLLMENT_MAGIC
        {
            return Err(invalid_record());
        }
        let version = take_u16(record, 4)?;
        let declared_length = usize::from(take_u16(record, 6)?);
        let nv_index = take_u32(record, 8)?;
        let name_algorithm = take_u16(record, 12)?;
        let attributes = take_u32(record, 14)?;
        let data_size = take_u16(record, 18)?;
        let policy_length = usize::from(take_u16(record, 20)?);
        let expected_length = ENROLLMENT_FIXED_BYTES
            .checked_add(policy_length)
            .ok_or_else(invalid_record)?;
        if version != ENROLLMENT_VERSION
            || declared_length != record.len()
            || expected_length != record.len()
            || policy_length > MAX_AUTH_POLICY_BYTES
            || nv_index & HANDLE_TYPE_MASK != NV_HANDLE_PREFIX
            || name_algorithm != TPM_ALG_SHA256
            || data_size == 0
        {
            return Err(invalid_record());
        }
        Ok(Self {
            nv_index,
            name_algorithm,
            attributes,
            auth_policy: record[ENROLLMENT_FIXED_BYTES..].to_vec(),
            data_size,
        })
    }

    pub(crate) fn verify_public(&self, public: &TpmNvPublic) -> Result<()> {
        if self.nv_index != public.nv_index
            || self.name_algorithm != public.name_algorithm
            || self.attributes != public.attributes
            || self.auth_policy != public.auth_policy
            || self.data_size != public.data_size
        {
            return Err(Error::InvalidInput(InputFault::TpmEnrollmentPublicMismatch));
        }
        Ok(())
    }

    pub(crate) fn validate_read_range(&self, size: u16, offset: u16) -> Result<()> {
        let end = offset
            .checked_add(size)
            .ok_or(Error::InvalidInput(InputFault::TpmNvReadRangeInvalid))?;
        if size == 0 || end > self.data_size {
            return Err(Error::InvalidInput(InputFault::TpmNvReadRangeInvalid));
        }
        Ok(())
    }

    pub(crate) fn validate_increment_shape(&self) -> Result<()> {
        const TPMA_NV_NT_MASK: u32 = 0x0f << 4;
        const TPMA_NV_COUNTER: u32 = 1 << 4;
        const COUNTER_BYTES: u16 = 8;
        if self.attributes & TPMA_NV_NT_MASK != TPMA_NV_COUNTER || self.data_size != COUNTER_BYTES {
            return Err(Error::InvalidInput(InputFault::TpmNvTypeInvalid));
        }
        Ok(())
    }
}

fn take_u16(record: &[u8], offset: usize) -> Result<u16> {
    let bytes = record.get(offset..offset + 2).ok_or_else(invalid_record)?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn take_u32(record: &[u8], offset: usize) -> Result<u32> {
    let bytes = record.get(offset..offset + 4).ok_or_else(invalid_record)?;
    Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn invalid_record() -> Error {
    Error::InvalidInput(InputFault::TpmEnrollmentRecordInvalid)
}
