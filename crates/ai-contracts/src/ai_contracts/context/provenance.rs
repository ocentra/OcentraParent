use super::{
    AiEvidenceProvenance, AiProvenanceKind, AiRuleId, AiSchemaVersion, AiSourceId, AiTimestamp,
};
use crate::ai_contracts::identity::{AiAdapterId, AiEvidenceReferenceId, AiFamilyId, AiResultId};

impl AiEvidenceProvenance {
    pub fn provenance_kind(&self) -> AiProvenanceKind {
        self.provenance_kind
    }

    pub fn source_evidence_reference_id(&self) -> Option<&AiEvidenceReferenceId> {
        self.source_evidence_reference_id.as_ref()
    }

    pub fn source_result_id(&self) -> Option<&AiResultId> {
        self.source_result_id.as_ref()
    }

    pub fn source_rule_id(&self) -> Option<&AiRuleId> {
        self.source_rule_id.as_ref()
    }

    pub fn source_id(&self) -> &AiSourceId {
        &self.source_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn adapter_id(&self) -> &AiAdapterId {
        &self.adapter_id
    }

    pub fn source_schema_version(&self) -> &AiSchemaVersion {
        &self.source_schema_version
    }

    pub fn observed_at(&self) -> &AiTimestamp {
        &self.observed_at
    }

    pub fn ingested_at(&self) -> Option<&AiTimestamp> {
        self.ingested_at.as_ref()
    }
}
