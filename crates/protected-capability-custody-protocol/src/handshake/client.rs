use std::fmt;

use crate::constants;
use crate::types::{CorrelationId, Nonce, ProtocolError, ProtocolGeneration, ProtocolVersion};

use super::UntrustedClientHello;

impl UntrustedClientHello {
    pub fn try_new(
        nonce: Nonce,
        correlation: CorrelationId,
        client_process_id: u32,
        client_process_epoch: u64,
        client_session_id: u32,
    ) -> Result<Self, ProtocolError> {
        if client_process_id == 0 || client_session_id == 0 {
            return Err(ProtocolError::InvalidProcessId);
        }
        if client_process_epoch == 0 {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(Self {
            version: ProtocolVersion::CURRENT,
            protocol_generation: ProtocolGeneration::CURRENT,
            nonce,
            correlation,
            client_process_id,
            client_process_epoch,
            client_session_id,
        })
    }

    pub fn version(&self) -> ProtocolVersion {
        self.version
    }

    pub fn protocol_generation(&self) -> ProtocolGeneration {
        self.protocol_generation
    }

    pub fn nonce(&self) -> Nonce {
        self.nonce
    }

    pub fn correlation(&self) -> CorrelationId {
        self.correlation
    }

    pub fn client_process_id(&self) -> u32 {
        self.client_process_id
    }

    pub fn client_process_epoch(&self) -> u64 {
        self.client_process_epoch
    }

    pub fn client_session_id(&self) -> u32 {
        self.client_session_id
    }
}

impl fmt::Debug for UntrustedClientHello {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct(constants::DEBUG_CLIENT_HELLO)
            .field(constants::DEBUG_FIELD_VERSION, &self.version)
            .field(
                constants::DEBUG_FIELD_PROTOCOL_GENERATION,
                &self.protocol_generation,
            )
            .field(constants::DEBUG_FIELD_NONCE, &self.nonce)
            .field(constants::DEBUG_FIELD_CORRELATION, &self.correlation)
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_ID,
                &self.client_process_id,
            )
            .field(
                constants::DEBUG_FIELD_CLIENT_PROCESS_EPOCH,
                &self.client_process_epoch,
            )
            .field(
                constants::DEBUG_FIELD_CLIENT_SESSION_ID,
                &self.client_session_id,
            )
            .finish()
    }
}
