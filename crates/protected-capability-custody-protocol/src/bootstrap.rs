use crate::types::{Nonce, ProtocolError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootstrapIdentity {
    client_process_id: u32,
    client_process_epoch: u64,
    client_session_id: u32,
    pipe_nonce: Nonce,
}

impl BootstrapIdentity {
    pub fn try_new(
        client_process_id: u32,
        client_process_epoch: u64,
        client_session_id: u32,
        pipe_nonce: Nonce,
    ) -> Result<Self, ProtocolError> {
        if client_process_id == 0 || client_session_id == 0 {
            return Err(ProtocolError::InvalidProcessId);
        }
        if client_process_epoch == 0 {
            return Err(ProtocolError::InvalidEpoch);
        }
        Ok(Self {
            client_process_id,
            client_process_epoch,
            client_session_id,
            pipe_nonce,
        })
    }

    pub fn client_process_id(self) -> u32 {
        self.client_process_id
    }

    pub fn client_process_epoch(self) -> u64 {
        self.client_process_epoch
    }

    pub fn client_session_id(self) -> u32 {
        self.client_session_id
    }

    pub fn pipe_nonce(self) -> Nonce {
        self.pipe_nonce
    }
}

pub struct BootstrapPacket {
    identity: BootstrapIdentity,
}

impl BootstrapPacket {
    pub fn generate(
        client_process_id: u32,
        client_process_epoch: u64,
        client_session_id: u32,
    ) -> Result<Self, ProtocolError> {
        let identity = BootstrapIdentity::try_new(
            client_process_id,
            client_process_epoch,
            client_session_id,
            Nonce::generate()?,
        )?;
        Ok(Self { identity })
    }

    pub fn identity(&self) -> BootstrapIdentity {
        self.identity
    }

    pub fn into_identity(self) -> BootstrapIdentity {
        self.identity
    }

    pub(crate) fn from_decoded(identity: BootstrapIdentity) -> Self {
        Self { identity }
    }
}

impl std::fmt::Debug for BootstrapPacket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct(crate::constants::DEBUG_BOOTSTRAP_PACKET)
            .field(crate::constants::DEBUG_FIELD_IDENTITY, &self.identity)
            .finish()
    }
}
