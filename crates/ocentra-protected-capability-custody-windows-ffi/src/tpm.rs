//! Strict TPM 2.0 command codecs used by the Windows TBS wrapper.
//!
//! The child modules encode and decode wire mechanics only. They do not
//! decide which NV index is enrolled or whether a generation may advance.

#[path = "tpm_codec_types.rs"]
pub(crate) mod codec_types;
#[path = "tpm_command.rs"]
pub(crate) mod command;
#[path = "tpm_cursor.rs"]
mod cursor;
#[path = "tpm_response.rs"]
pub(crate) mod response;
#[path = "tpm_session.rs"]
pub(crate) mod session;

pub(crate) const TPM_HEADER_BYTES: usize = 10;
pub(crate) const TPM_ST_NO_SESSIONS: u16 = 0x8001;
pub(crate) const TPM_ST_SESSIONS: u16 = 0x8002;
pub(crate) const TPM_RC_SUCCESS: u32 = 0;

pub(crate) const TPM_ALG_SHA256: u16 = 0x000b;
pub(crate) const TPM_ALG_NULL: u16 = 0x0010;
pub(crate) const TPM_ALG_RSA: u16 = 0x0001;
pub(crate) const TPM_ALG_RSAPSS: u16 = 0x0016;
pub(crate) const TPM_SE_POLICY: u8 = 0x01;
pub(crate) const TPM_SHA256_BYTES: usize = 32;

pub(crate) const TPM_CC_NV_INCREMENT: u32 = 0x0000_0134;
pub(crate) const TPM_CC_NV_READ: u32 = 0x0000_014e;
pub(crate) const TPM_CC_POLICY_SIGNED: u32 = 0x0000_0160;
pub(crate) const TPM_CC_LOAD_EXTERNAL: u32 = 0x0000_0167;
pub(crate) const TPM_CC_NV_READ_PUBLIC: u32 = 0x0000_0169;
pub(crate) const TPM_CC_POLICY_COMMAND_CODE: u32 = 0x0000_016c;
pub(crate) const TPM_CC_POLICY_OR: u32 = 0x0000_0171;
pub(crate) const TPM_CC_START_AUTH_SESSION: u32 = 0x0000_0176;
pub(crate) const TPM_CC_FLUSH_CONTEXT: u32 = 0x0000_0165;

pub(crate) const TPM_RH_NULL: u32 = 0x4000_0007;
pub(crate) const TPM_HT_TRANSIENT: u32 = 0x8000_0000;
pub(crate) const TPM_HT_POLICY_SESSION: u32 = 0x0300_0000;
pub(crate) const TPM_ST_AUTH_SIGNED: u16 = 0x8025;
pub(crate) const TPM_SESSION_CONTINUE: u8 = 0x01;
pub(crate) const TPM_MAX_POLICY_OR_DIGESTS: usize = 8;

pub(crate) const FIXED_COUNTER_INDEX: u32 = 0x0180_f001;
pub(crate) const FIXED_COUNTER_BYTES: u16 = 8;
pub(crate) const FIXED_COUNTER_DEFINED_ATTRIBUTES: u32 = 0x4208_0018;
pub(crate) const FIXED_COUNTER_OBSERVED_ATTRIBUTES: u32 = 0x6208_0018;
pub(crate) const FIXED_RSA_BITS: u16 = 3072;
pub(crate) const FIXED_RSA_BYTES: usize = 384;
