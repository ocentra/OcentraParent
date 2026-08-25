//! Private typed TPM handles and protected command codes.

use super::super::{TPM_HT_HMAC_SESSION, TPM_HT_POLICY_SESSION, TPM_HT_TRANSIENT};
use crate::{Error, InputFault, Result};

const TPM_HR_NV_INDEX_MASK: u32 = 0x00ff_ffff;
const TPM_HR_NV_INDEX: u32 = 0x0100_0000;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct NvIndex(u32);

impl NvIndex {
    pub(crate) fn from_enrollment(raw: u32) -> Result<Self> {
        if raw & !TPM_HR_NV_INDEX_MASK != TPM_HR_NV_INDEX || raw == TPM_HR_NV_INDEX {
            return Err(Error::InvalidInput(InputFault::TpmNvIndexInvalid));
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NonNullHandle(u32);

impl NonNullHandle {
    pub(crate) fn from_raw(raw: u32) -> Result<Self> {
        if raw == 0 || raw == u32::MAX {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct SessionHandle(u32);

impl SessionHandle {
    pub(crate) fn from_response(raw: u32) -> Result<Self> {
        let kind = raw & 0xff00_0000;
        if kind != TPM_HT_HMAC_SESSION && kind != TPM_HT_POLICY_SESSION {
            return Err(Error::MalformedTpm);
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TransientHandle(u32);

impl TransientHandle {
    pub(crate) fn from_response(raw: u32) -> Result<Self> {
        if raw & 0xff00_0000 != TPM_HT_TRANSIENT {
            return Err(Error::MalformedTpm);
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CommandCode(u32);

impl CommandCode {
    pub(crate) fn from_enrollment(raw: u32) -> Result<Self> {
        let allowed = matches!(
            raw,
            super::super::TPM_CC_NV_INCREMENT
                | super::super::TPM_CC_NV_READ
                | super::super::TPM_CC_NV_UNDEFINE_SPACE
                | super::super::TPM_CC_NV_DEFINE_SPACE
        );
        if !allowed {
            return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
        }
        Ok(Self(raw))
    }

    pub(crate) fn raw(self) -> u32 {
        self.0
    }
}
