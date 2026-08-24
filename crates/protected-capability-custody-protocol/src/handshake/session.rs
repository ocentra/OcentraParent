use crate::types::ProtocolError;

use super::{BrokerSessionWireValues, UntrustedClientHello};

impl BrokerSessionWireValues {
    pub fn try_new(self, now_unix_millis: u64) -> Result<Self, ProtocolError> {
        // SCM services run in Windows session 0. The broker session is bound
        // to the authenticated server PID and executable, so zero is a valid
        // service session identifier; client sessions remain nonzero.
        if self.broker_process_id == 0 {
            return Err(ProtocolError::InvalidProcessId);
        }
        if self.broker_epoch == 0
            || self.broker_key_epoch == 0
            || self.writer_lease_epoch == 0
            || self.watermark == 0
        {
            return Err(ProtocolError::InvalidEpoch);
        }
        if now_unix_millis == 0
            || self.session_expires_at_unix_millis <= now_unix_millis
            || self
                .session_expires_at_unix_millis
                .saturating_sub(now_unix_millis)
                > crate::constants::SESSION_TTL_MILLIS
        {
            return Err(ProtocolError::InvalidExpiry);
        }
        Ok(self)
    }

    pub(crate) fn append_attestation_message(
        &self,
        client: &UntrustedClientHello,
        canonical: &mut Vec<u8>,
    ) {
        canonical.extend_from_slice(&client.version().value().to_be_bytes());
        canonical.extend_from_slice(&client.protocol_generation().value().to_be_bytes());
        canonical.extend_from_slice(client.nonce().as_bytes());
        canonical.extend_from_slice(self.broker_nonce.as_bytes());
        canonical.extend_from_slice(client.correlation().as_bytes());
        canonical.extend_from_slice(&client.client_process_id().to_be_bytes());
        canonical.extend_from_slice(&client.client_process_epoch().to_be_bytes());
        canonical.extend_from_slice(&client.client_session_id().to_be_bytes());
        canonical.extend_from_slice(&self.broker_process_id.to_be_bytes());
        canonical.extend_from_slice(&self.broker_session_id.to_be_bytes());
        canonical.extend_from_slice(&self.broker_epoch.to_be_bytes());
        canonical.extend_from_slice(&self.broker_key_epoch.to_be_bytes());
        canonical.extend_from_slice(&self.writer_lease_epoch.to_be_bytes());
        canonical.extend_from_slice(&self.watermark.to_be_bytes());
        canonical.extend_from_slice(self.session_handle.as_bytes());
        canonical.extend_from_slice(&self.session_expires_at_unix_millis.to_be_bytes());
    }
}
