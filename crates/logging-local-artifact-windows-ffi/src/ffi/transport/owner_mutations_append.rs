use super::*;
use crate::constants::APPEND_OPERATION;

#[path = "owner_mutations_append_create.rs"]
mod create;
#[path = "owner_mutations_append_prepare.rs"]
mod prepare;
#[path = "owner_mutations_append_write.rs"]
mod write;

pub(super) struct AppendPreparation {
    pub(super) chain: DirectoryChain,
    pub(super) target_path: PathBuf,
    pub(super) leaf: String,
    pub(super) target: Option<OwnedFile>,
    pub(super) prior_length: u64,
    pub(super) target_identity: Option<IdentityRecord>,
    pub(super) created: bool,
    pub(super) intent: IntentRecord,
    pub(super) descriptor: String,
    pub(super) payload_length: u64,
}

impl<'a> MutationSession<'a> {
    pub fn append(
        &mut self,
        request_id: &str,
        relative_path: &str,
        payload: &[u8],
    ) -> Result<MutationReceipt, ArtifactError> {
        validate_request_id(request_id)?;
        validate_relative(relative_path)?;
        bounded_payload(payload)?;
        let descriptor = request_descriptor(APPEND_OPERATION, relative_path, Some(payload));
        if let Some(receipt) = read_receipt(
            &self.owner.root_path,
            request_id,
            APPEND_OPERATION,
            relative_path,
            &descriptor,
        )? {
            return Ok(receipt);
        }
        let prepared = prepare::prepare(self, request_id, relative_path, payload, descriptor)?;
        write::complete(self, request_id, relative_path, payload, prepared)
    }
}
