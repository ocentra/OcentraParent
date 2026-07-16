mod read_model;

use self::read_model::policy_preview_read_model_from_payload_impl;
use super::*;
use ocentra_parent_agent_protocol::policy_constants as policy;

pub(super) fn policy_preview_read_model_from_payload(
    payload: &LogFields,
) -> Result<ParentPolicyPreviewReadModelSnapshot, String> {
    policy_preview_read_model_from_payload_impl(payload)
}
