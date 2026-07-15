use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(super) struct SocialAuditExplanationFieldPair(
    pub(super) &'static str,
    pub(super) LogFieldValue,
);

pub(super) struct SocialAuditExplanationFieldKey(pub(super) &'static str);

pub(super) struct SocialAuditExplanationTextRef(pub(super) &'static str);

pub(super) fn social_audit_explanation_fields_from_pairs(
    pairs: Vec<SocialAuditExplanationFieldPair>,
) -> LogFields {
    crate::fields::fields_from_pairs(pairs.into_iter().map(|pair| (pair.0, pair.1)).collect())
}

pub(super) fn field_pair(
    key: &SocialAuditExplanationFieldKey,
    value: LogFieldValue,
) -> SocialAuditExplanationFieldPair {
    SocialAuditExplanationFieldPair(key.0, value)
}
