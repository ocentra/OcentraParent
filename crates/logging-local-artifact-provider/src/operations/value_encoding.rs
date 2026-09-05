use base64::Engine;
use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactDirectoryEntry, LocalArtifactIdentity, LocalArtifactReadSnapshot,
    LocalArtifactStat,
};
use serde_json::json;

use super::super::{FailureDisposition, ProviderError};
use crate::protocol::types::ResponseResult;
use crate::protocol::{self, ReadMaximum};

pub(super) fn wire_identity(identity: LocalArtifactIdentity) -> crate::protocol::WireIdentity {
    crate::protocol::WireIdentity {
        device: identity.volume_serial_number().to_string(),
        inode: u128::from_be_bytes(identity.file_id()).to_string(),
    }
}

pub(super) fn stat_value(stat: LocalArtifactStat) -> Result<ResponseResult, ProviderError> {
    if stat.size() > 9_007_199_254_740_991 {
        return Err(ProviderError::new(
            protocol::text::JSON_SIZE_UNREPRESENTABLE,
            FailureDisposition::Continue,
        ));
    }
    let modified_ms = if stat.modified_ms() < 0 {
        return Err(ProviderError::new(
            protocol::text::MODIFIED_TIME_INVALID,
            FailureDisposition::Terminate,
        ));
    } else {
        // CAST-JUSTIFICATION: the negative branch above proves this timestamp
        // is representable as an unsigned wire value.
        let value = stat.modified_ms() as u64;
        if value > 9_007_199_254_740_991 {
            return Err(ProviderError::new(
                protocol::text::JSON_TIME_UNREPRESENTABLE,
                FailureDisposition::Continue,
            ));
        }
        value
    };
    let identity = wire_identity(stat.identity());
    let identity = protocol::text::object(vec![
        (protocol::text::TextId::DeviceKey, json!(identity.device)),
        (protocol::text::TextId::InodeKey, json!(identity.inode)),
    ]);
    Ok(protocol::text::object(vec![
        (protocol::text::TextId::SizeKey, json!(stat.size())),
        (protocol::text::TextId::ModifiedMsKey, json!(modified_ms)),
        (
            protocol::text::TextId::IsDirectoryKey,
            json!(stat.is_directory()),
        ),
        (protocol::text::TextId::IdentityKey, identity.into_value()),
    ]))
}

pub(super) fn snapshot_value(
    snapshot: &LocalArtifactReadSnapshot,
    maximum_bytes: ReadMaximum,
) -> Result<ResponseResult, ProviderError> {
    let stat = snapshot.stat();
    if stat.is_directory() {
        return Err(ProviderError::new(
            protocol::text::SNAPSHOT_NOT_REGULAR,
            FailureDisposition::Terminate,
        ));
    }
    let byte_length = u64::try_from(snapshot.bytes().len()).map_err(|_error| {
        ProviderError::new(
            protocol::text::SNAPSHOT_LENGTH_UNREPRESENTABLE,
            FailureDisposition::Continue,
        )
    })?;
    if byte_length > maximum_bytes.value() || byte_length != stat.size() {
        return Err(ProviderError::new(
            protocol::text::SNAPSHOT_IDENTITY_MISMATCH,
            FailureDisposition::Terminate,
        ));
    }
    let stat = stat_value(stat)?;
    Ok(protocol::text::object(vec![
        (
            protocol::text::TextId::ContentBase64Key,
            json!(base64::engine::general_purpose::STANDARD.encode(snapshot.bytes())),
        ),
        (protocol::text::TextId::StatKey, stat.into_value()),
    ]))
}

pub(super) fn entry_value(
    entry: &LocalArtifactDirectoryEntry,
) -> Result<ResponseResult, ProviderError> {
    let name = entry.name();
    let dot = protocol::text::TextId::Dot.text();
    let dot_dot = protocol::text::TextId::DotDot.text();
    let utf16_length = name.encode_utf16().count();
    if name.is_empty()
        || utf16_length > 255
        || name == dot
        || name == dot_dot
        || name.contains('/')
        || name.contains('\\')
        || name.contains('\0')
    {
        return Err(ProviderError::new(
            protocol::text::INVALID_DIRECTORY_ENTRY,
            FailureDisposition::Terminate,
        ));
    }
    Ok(protocol::text::object(vec![
        (protocol::text::TextId::NameKey, json!(name)),
        (
            protocol::text::TextId::IsDirectoryKey,
            json!(entry.stat().is_directory()),
        ),
    ]))
}
