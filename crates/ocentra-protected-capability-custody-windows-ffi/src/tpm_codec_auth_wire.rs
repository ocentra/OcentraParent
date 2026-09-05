//! Fixed one-session authorization encoding.

use super::SecretNonce;
use crate::tpm::codec_types::handles::SessionHandle;
use crate::tpm::TPM_SHA256_BYTES;
use crate::{Error, InputFault, Result};

pub(super) fn encode_policy_authorization(
    session: &SessionHandle,
    nonce_caller: &SecretNonce,
    attributes: u8,
) -> Result<Vec<u8>> {
    if attributes & !crate::tpm::TPM_SESSION_CONTINUE != 0 {
        return Err(Error::InvalidInput(InputFault::TpmCommandShapeInvalid));
    }
    let mut wire = Vec::with_capacity(4 + 2 + TPM_SHA256_BYTES + 1 + 2);
    wire.extend_from_slice(&session.raw().to_be_bytes());
    push_tpm2b(&mut wire, nonce_caller.as_bytes())?;
    wire.push(attributes);
    // A plain TPM_SE_POLICY session with no PolicyAuthValue/PolicyPassword has
    // no authorization HMAC. PolicySigned supplies the command authority.
    push_tpm2b(&mut wire, &[])?;
    Ok(wire)
}

fn push_tpm2b(output: &mut Vec<u8>, bytes: &[u8]) -> Result<()> {
    output.extend_from_slice(&u16::try_from(bytes.len())?.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}
