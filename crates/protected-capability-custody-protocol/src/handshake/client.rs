use std::fmt;

use crate::constants;
use crate::types::ProtocolError;

use super::ClientHello;

impl ClientHello {
    pub fn try_new(
        nonce: super::super::types::Nonce,
        correlation: super::super::types::CorrelationId,
        client_process_epoch: u64,
    ) -> Result<Self, ProtocolError> {
        if client_process_epoch == 0 {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(Self {
            version: super::super::types::ProtocolVersion::CURRENT,
            nonce,
            correlation,
            client_process_epoch,
        })
    }

    pub fn version(&self) -> super::super::types::ProtocolVersion {
        self.version
    }

    pub fn nonce(&self) -> super::super::types::Nonce {
        self.nonce
    }

    pub fn correlation(&self) -> super::super::types::CorrelationId {
        self.correlation
    }

    pub fn client_process_epoch(&self) -> u64 {
        self.client_process_epoch
    }
}

impl fmt::Debug for ClientHello {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_CLIENT_HELLO)
            .field(constants::DEBUG_FIELD_VERSION, &self.version)
            .field(constants::DEBUG_FIELD_NONCE, &self.nonce)
            .field(constants::DEBUG_FIELD_CORRELATION, &self.correlation)
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_EPOCH,
                &self.client_process_epoch,
            )
            .finish()
    }
}
