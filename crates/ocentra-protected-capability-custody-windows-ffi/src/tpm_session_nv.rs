//! NV operations bound to one live, owned policy session.

use super::super::codec_types::handles::{NonNullHandle, NvIndex};
use super::super::{command, response};
use super::OwnedTpmSession;
use crate::Result;

impl OwnedTpmSession<'_> {
    pub(crate) fn nv_read(
        &self,
        auth_handle: NonNullHandle,
        index: NvIndex,
        size: u16,
        offset: u16,
        nonce: &[u8],
        attributes: u8,
        hmac: &[u8],
    ) -> Result<Vec<u8>> {
        let authorization = self.authorization_area(nonce, attributes, hmac)?;
        let command =
            command::nv::encode_nv_read(auth_handle, index, &authorization, size, offset)?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_nv_read(&response)
    }

    pub(crate) fn nv_increment(
        &self,
        auth_handle: NonNullHandle,
        index: NvIndex,
        nonce: &[u8],
        attributes: u8,
        hmac: &[u8],
    ) -> Result<()> {
        let authorization = self.authorization_area(nonce, attributes, hmac)?;
        let command = command::nv::encode_nv_increment(auth_handle, index, &authorization)?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_success_with_sessions(&response)
    }

    pub(crate) fn nv_undefine_space(
        &self,
        auth_handle: NonNullHandle,
        index: NvIndex,
        nonce: &[u8],
        attributes: u8,
        hmac: &[u8],
    ) -> Result<()> {
        let authorization = self.authorization_area(nonce, attributes, hmac)?;
        let command = command::nv::encode_nv_undefine_space(auth_handle, index, &authorization)?;
        let response = self.context.submit(&command)?;
        response::sessions::decode_success_with_sessions(&response)
    }
}
