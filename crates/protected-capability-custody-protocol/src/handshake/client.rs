use std::fmt;

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
            .debug_struct("ClientHello")
            .field("version", &self.version)
            .field("nonce", &self.nonce)
            .field("correlation", &self.correlation)
            .field("client_process_epoch", &self.client_process_epoch)
            .finish()
    }
}
