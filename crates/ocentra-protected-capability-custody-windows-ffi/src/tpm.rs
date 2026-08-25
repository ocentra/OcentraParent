//! Strict TPM 2.0 command codecs used by the Windows TBS wrapper.
//!
//! The child modules encode and decode wire mechanics only. They do not
//! decide which NV index is enrolled or whether a generation may advance.

#[path = "tpm_command.rs"]
pub(crate) mod command;
#[path = "tpm_cursor.rs"]
mod cursor;
#[path = "tpm_response.rs"]
pub(crate) mod response;

const TPM_HEADER_BYTES: usize = 10;
const TPM_ST_NO_SESSIONS: u16 = 0x8001;
const TPM_RC_SUCCESS: u32 = 0;
const TPM_CC_NV_READ_PUBLIC: u32 = 0x0000_0169;
