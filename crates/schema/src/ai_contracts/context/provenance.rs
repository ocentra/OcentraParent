use super::{
    AiEvidenceProvenance, AiProvenanceKind, AiRuleId, AiSchemaVersion, AiSourceId, AiTimestamp,
};
use crate::ai_contracts::identity::{AiAdapterId, AiEvidenceReferenceId, AiFamilyId, AiResultId};
use crate::ai_contracts::validate_contract_schema_version;

impl AiEvidenceProvenance {
    pub(crate) fn new(
        provenance_kind: AiProvenanceKind,
        family_id: AiFamilyId,
        source_id: AiSourceId,
        adapter_id: AiAdapterId,
        source_schema_version: AiSchemaVersion,
        observed_at: AiTimestamp,
        ingested_at: Option<AiTimestamp>,
        source_evidence_reference_id: Option<AiEvidenceReferenceId>,
        source_result_id: Option<AiResultId>,
        source_rule_id: Option<AiRuleId>,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&source_schema_version)?;
        let source_count = usize::from(source_evidence_reference_id.is_some())
            + usize::from(source_result_id.is_some())
            + usize::from(source_rule_id.is_some());
        let valid_sources = match provenance_kind {
            AiProvenanceKind::DirectObservation => source_count == 0,
            AiProvenanceKind::DerivedFromEvidence => {
                source_evidence_reference_id.is_some() && source_count == 1
            }
            AiProvenanceKind::DerivedFromResult => source_result_id.is_some() && source_count == 1,
            AiProvenanceKind::ParentAuthoredRule => source_rule_id.is_some() && source_count == 1,
        };
        if !valid_sources {
            return Err("AI provenance source identity does not match its provenance kind");
        }
        if !observed_at.is_well_formed()
            || ingested_at
                .as_ref()
                .is_some_and(|ingested| !ingested.is_well_formed())
        {
            return Err("AI provenance timestamp is not well formed");
        }
        if let Some(ingested_at) = &ingested_at {
            if !observed_at.precedes(ingested_at) && observed_at != *ingested_at {
                return Err("AI provenance ingestion time precedes observation time");
            }
        }
        Ok(Self {
            provenance_kind,
            family_id,
            source_id,
            adapter_id,
            source_schema_version,
            observed_at,
            ingested_at,
            source_evidence_reference_id,
            source_result_id,
            source_rule_id,
        })
    }

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
