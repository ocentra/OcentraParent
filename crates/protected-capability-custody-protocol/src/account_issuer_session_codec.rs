use crate::account_issuer_contract::ACCOUNT_ISSUER_MAX_WIRE_BYTES;
use crate::account_issuer_session::{
    AuthenticatedAccountIssuerReceipt, AuthenticatedAccountIssuerRequest, SessionBinding,
    UntrustedAccountIssuerReceipt, UntrustedAccountIssuerRequest, ACCOUNT_ISSUER_SESSION_DOMAIN,
    RECEIPT_TAG, REQUEST_TAG,
};
use crate::constants::{FRAME_PREFIX_BYTES, MAX_FRAME_BYTES};
use crate::types::{
    AuthenticationTag, CorrelationId, Nonce, ProtocolError, ProtocolGeneration, ProtocolVersion,
    SessionHandle, SessionTranscriptDigest,
};

pub(crate) fn encode_request(
    request: &AuthenticatedAccountIssuerRequest,
) -> Result<Vec<u8>, ProtocolError> {
    let request_wire = crate::account_issuer_v2_codec::encode_request(&request.request)?;
    let mut payload = Vec::with_capacity(512 + request_wire.len());
    append_binding(&mut payload, &request.binding);
    append_field(&mut payload, &request_wire, ACCOUNT_ISSUER_MAX_WIRE_BYTES)?;
    payload.extend_from_slice(&request.request_digest);
    payload.extend_from_slice(request.authentication_tag.as_bytes());
    encode_frame(REQUEST_TAG, &payload)
}

pub(crate) fn decode_request(frame: &[u8]) -> Result<UntrustedAccountIssuerRequest, ProtocolError> {
    let mut cursor = Cursor::new(frame)?;
    cursor.take_domain(REQUEST_TAG)?;
    let binding = cursor.take_binding()?;
    let request_wire = cursor.take_field(ACCOUNT_ISSUER_MAX_WIRE_BYTES)?;
    let request_digest = cursor.take_digest()?;
    let authentication_tag = cursor.take_tag()?;
    cursor.finish()?;
    Ok(UntrustedAccountIssuerRequest {
        binding,
        request: crate::account_issuer_v2_codec::decode_request(&request_wire)?,
        request_digest,
        authentication_tag,
    })
}

pub(crate) fn encode_receipt(
    receipt: &AuthenticatedAccountIssuerReceipt,
) -> Result<Vec<u8>, ProtocolError> {
    let receipt_wire = crate::account_issuer_v2_codec::encode_receipt(&receipt.receipt)?;
    let mut payload = Vec::with_capacity(512 + receipt_wire.len());
    append_binding(&mut payload, &receipt.binding);
    payload.extend_from_slice(&receipt.request_digest);
    append_field(&mut payload, &receipt_wire, ACCOUNT_ISSUER_MAX_WIRE_BYTES)?;
    payload.extend_from_slice(receipt.authentication_tag.as_bytes());
    encode_frame(RECEIPT_TAG, &payload)
}

pub(crate) fn decode_receipt(frame: &[u8]) -> Result<UntrustedAccountIssuerReceipt, ProtocolError> {
    let mut cursor = Cursor::new(frame)?;
    cursor.take_domain(RECEIPT_TAG)?;
    let binding = cursor.take_binding()?;
    let request_digest = cursor.take_digest()?;
    let receipt_wire = cursor.take_field(ACCOUNT_ISSUER_MAX_WIRE_BYTES)?;
    let authentication_tag = cursor.take_tag()?;
    cursor.finish()?;
    Ok(UntrustedAccountIssuerReceipt {
        binding,
        request_digest,
        receipt: crate::account_issuer_v2_codec::decode_receipt(&receipt_wire)?,
        authentication_tag,
    })
}

fn append_binding(payload: &mut Vec<u8>, binding: &SessionBinding) {
    payload.extend_from_slice(&binding.version.value().to_be_bytes());
    payload.extend_from_slice(&binding.protocol_generation.value().to_be_bytes());
    payload.extend_from_slice(binding.client_nonce.as_bytes());
    payload.extend_from_slice(binding.broker_nonce.as_bytes());
    payload.extend_from_slice(binding.correlation.as_bytes());
    payload.extend_from_slice(&binding.client_process_id.to_be_bytes());
    payload.extend_from_slice(&binding.client_process_epoch.to_be_bytes());
    payload.extend_from_slice(&binding.client_session_id.to_be_bytes());
    payload.extend_from_slice(&binding.broker_process_id.to_be_bytes());
    payload.extend_from_slice(&binding.broker_session_id.to_be_bytes());
    payload.extend_from_slice(&binding.broker_epoch.to_be_bytes());
    payload.extend_from_slice(&binding.broker_key_epoch.to_be_bytes());
    payload.extend_from_slice(&binding.writer_lease_epoch.to_be_bytes());
    payload.extend_from_slice(&binding.watermark.to_be_bytes());
    payload.extend_from_slice(binding.session_handle.as_bytes());
    payload.extend_from_slice(binding.transcript_digest.as_bytes());
    payload.extend_from_slice(&binding.sequence.to_be_bytes());
    payload.extend_from_slice(&binding.expires_at_unix_millis.to_be_bytes());
}

fn encode_frame(tag: u8, payload: &[u8]) -> Result<Vec<u8>, ProtocolError> {
    let payload_len = ACCOUNT_ISSUER_SESSION_DOMAIN
        .len()
        .checked_add(1)
        .and_then(|length| length.checked_add(payload.len()))
        .ok_or(ProtocolError::FrameTooLarge)?;
    let frame_len = FRAME_PREFIX_BYTES
        .checked_add(payload_len)
        .ok_or(ProtocolError::FrameTooLarge)?;
    if frame_len > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    let payload_len = u32::try_from(payload_len).map_err(|_| ProtocolError::FrameTooLarge)?;
    let mut frame = Vec::with_capacity(frame_len);
    frame.extend_from_slice(&payload_len.to_be_bytes());
    frame.extend_from_slice(ACCOUNT_ISSUER_SESSION_DOMAIN);
    frame.push(tag);
    frame.extend_from_slice(payload);
    Ok(frame)
}

struct Cursor<'a> {
    frame: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(frame: &'a [u8]) -> Result<Self, ProtocolError> {
        if frame.len() < FRAME_PREFIX_BYTES {
            return Err(ProtocolError::InvalidFrameLength);
        }
        let prefix: [u8; FRAME_PREFIX_BYTES] = frame[..FRAME_PREFIX_BYTES]
            .try_into()
            .map_err(|_| ProtocolError::InvalidFrameLength)?;
        let declared = u32::from_be_bytes(prefix) as usize;
        if declared == 0
            || declared > MAX_FRAME_BYTES - FRAME_PREFIX_BYTES
            || declared + FRAME_PREFIX_BYTES != frame.len()
        {
            return Err(ProtocolError::InvalidFrameLength);
        }
        Ok(Self {
            frame,
            offset: FRAME_PREFIX_BYTES,
        })
    }

    fn take_domain(&mut self, expected_tag: u8) -> Result<(), ProtocolError> {
        if self.take_exact(ACCOUNT_ISSUER_SESSION_DOMAIN.len())? != ACCOUNT_ISSUER_SESSION_DOMAIN {
            return Err(ProtocolError::InvalidDomain);
        }
        if self.take_exact(1)?[0] != expected_tag {
            return Err(ProtocolError::InvalidDiscriminant(expected_tag));
        }
        Ok(())
    }

    fn take_binding(&mut self) -> Result<SessionBinding, ProtocolError> {
        Ok(SessionBinding {
            version: ProtocolVersion::decode(self.take_u16()?)?,
            protocol_generation: ProtocolGeneration::decode(self.take_u64()?)?,
            client_nonce: Nonce::try_from_bytes(self.take_exact(crate::constants::NONCE_BYTES)?)?,
            broker_nonce: Nonce::try_from_bytes(self.take_exact(crate::constants::NONCE_BYTES)?)?,
            correlation: CorrelationId::try_from_bytes(
                self.take_exact(crate::constants::CORRELATION_BYTES)?,
            )?,
            client_process_id: self.take_u32()?,
            client_process_epoch: self.take_u64()?,
            client_session_id: self.take_u32()?,
            broker_process_id: self.take_u32()?,
            broker_session_id: self.take_u32()?,
            broker_epoch: self.take_u64()?,
            broker_key_epoch: self.take_u64()?,
            writer_lease_epoch: self.take_u64()?,
            watermark: self.take_u64()?,
            session_handle: SessionHandle::try_from_untrusted_bytes(
                self.take_exact(crate::constants::SESSION_HANDLE_BYTES)?,
            )?,
            transcript_digest: SessionTranscriptDigest::try_from_untrusted_bytes(
                self.take_exact(crate::constants::TRANSCRIPT_DIGEST_BYTES)?,
            )?,
            sequence: self.take_u64()?,
            expires_at_unix_millis: self.take_u64()?,
        })
    }

    fn take_field(&mut self, maximum: usize) -> Result<Vec<u8>, ProtocolError> {
        let length = self.take_u32()? as usize;
        if length == 0 {
            return Err(ProtocolError::EmptyField);
        }
        if length > maximum {
            return Err(ProtocolError::FieldTooLarge);
        }
        Ok(self.take_exact(length)?.to_vec())
    }

    fn take_digest(
        &mut self,
    ) -> Result<[u8; crate::constants::REQUEST_DIGEST_BYTES], ProtocolError> {
        self.take_exact(crate::constants::REQUEST_DIGEST_BYTES)?
            .try_into()
            .map_err(|_| ProtocolError::InvalidFrameLength)
    }

    fn take_tag(&mut self) -> Result<AuthenticationTag, ProtocolError> {
        AuthenticationTag::try_from_untrusted_bytes(
            self.take_exact(crate::constants::AUTHENTICATION_TAG_BYTES)?,
        )
    }

    fn take_u16(&mut self) -> Result<u16, ProtocolError> {
        Ok(u16::from_be_bytes(
            self.take_exact(2)?
                .try_into()
                .map_err(|_| ProtocolError::Truncated)?,
        ))
    }

    fn take_u32(&mut self) -> Result<u32, ProtocolError> {
        Ok(u32::from_be_bytes(
            self.take_exact(4)?
                .try_into()
                .map_err(|_| ProtocolError::Truncated)?,
        ))
    }

    fn take_u64(&mut self) -> Result<u64, ProtocolError> {
        Ok(u64::from_be_bytes(
            self.take_exact(8)?
                .try_into()
                .map_err(|_| ProtocolError::Truncated)?,
        ))
    }

    fn take_exact(&mut self, length: usize) -> Result<&'a [u8], ProtocolError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ProtocolError::Truncated)?;
        let value = self
            .frame
            .get(self.offset..end)
            .ok_or(ProtocolError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), ProtocolError> {
        if self.offset == self.frame.len() {
            Ok(())
        } else {
            Err(ProtocolError::TrailingBytes)
        }
    }
}

fn append_field(payload: &mut Vec<u8>, value: &[u8], maximum: usize) -> Result<(), ProtocolError> {
    if value.is_empty() {
        return Err(ProtocolError::EmptyField);
    }
    if value.len() > maximum {
        return Err(ProtocolError::FieldTooLarge);
    }
    let length = u32::try_from(value.len()).map_err(|_| ProtocolError::FieldTooLarge)?;
    payload.extend_from_slice(&length.to_be_bytes());
    payload.extend_from_slice(value);
    Ok(())
}
