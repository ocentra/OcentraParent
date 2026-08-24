use serde::{Deserialize, Serialize};

use super::{
    identity::{
        AiAdapterId, AiCapabilityId, AiChildProfileId, AiDeviceId, AiEvidenceReferenceId,
        AiFamilyId, AiModelId, AiPolicyReferenceId, AiPromptTemplateId, AiPromptVersion,
        AiProviderId, AiRequestId, AiResultId, AiRuleId, AiRuntimeReferenceId, AiSchemaIdentity,
        AiSchemaVersion, AiSourceId, AiTimestamp,
    },
    memory::{AiGraphReference, AiMemoryReference},
    validate_contract_schema_version, AiAuthorityBoundary, AiConfidence, AiCustodyState,
    AiDegradedState, AiRedactionState, AiRetentionState, AiSafeText, AiValidationState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiEvidenceKind {
    Browser,
    App,
    Game,
    Network,
    ScreenSummary,
    Activity,
    ParentRule,
    Audit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiProvenanceKind {
    DirectObservation,
    DerivedFromEvidence,
    DerivedFromResult,
    ParentAuthoredRule,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiContextBuildState {
    Ready,
    Partial,
    Rejected,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiReferenceValidationState {
    Validated,
    MissingSource,
    CustodyBlocked,
    Stale,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceProvenance {
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
}

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

/// Owner-issued evidence custody and validation metadata. It is serialized
/// for read models but intentionally has no public wire deserializer.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceReference {
    evidence_reference_id: AiEvidenceReferenceId,
    family_id: AiFamilyId,
    evidence_kind: AiEvidenceKind,
    provenance: AiEvidenceProvenance,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    confidence: Option<AiConfidence>,
    validation: AiReferenceValidationState,
}

impl AiEvidenceReference {
    pub(crate) fn new(
        evidence_reference_id: AiEvidenceReferenceId,
        family_id: AiFamilyId,
        evidence_kind: AiEvidenceKind,
        provenance: AiEvidenceProvenance,
        custody: AiCustodyState,
        retention: AiRetentionState,
        redaction: AiRedactionState,
        confidence: Option<AiConfidence>,
        validation: AiReferenceValidationState,
    ) -> Result<Self, &'static str> {
        if provenance.family_id() != &family_id {
            return Err("AI evidence provenance family does not match its reference");
        }
        let blocked = matches!(
            custody,
            AiCustodyState::Deleted | AiCustodyState::Unavailable
        ) || matches!(
            retention,
            AiRetentionState::Deleted | AiRetentionState::Tombstoned
        ) || matches!(redaction, AiRedactionState::RejectedPrivatePayload);
        if matches!(validation, AiReferenceValidationState::Validated) && blocked {
            return Err("AI evidence cannot be validated while custody or retention is blocked");
        }
        if matches!(validation, AiReferenceValidationState::Rejected)
            && !matches!(redaction, AiRedactionState::RejectedPrivatePayload)
            && !blocked
        {
            return Err("AI rejected evidence must carry a blocked or rejected state");
        }
        Ok(Self {
            evidence_reference_id,
            family_id,
            evidence_kind,
            provenance,
            custody,
            retention,
            redaction,
            confidence,
            validation,
        })
    }

    pub fn evidence_reference_id(&self) -> &AiEvidenceReferenceId {
        &self.evidence_reference_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn evidence_kind(&self) -> AiEvidenceKind {
        self.evidence_kind
    }

    pub fn provenance(&self) -> &AiEvidenceProvenance {
        &self.provenance
    }

    pub fn custody(&self) -> AiCustodyState {
        self.custody
    }

    pub fn retention(&self) -> AiRetentionState {
        self.retention
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }

    pub fn validation(&self) -> AiReferenceValidationState {
        self.validation
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleReference {
    policy_reference_id: AiPolicyReferenceId,
    family_id: AiFamilyId,
    rule_id: AiRuleId,
    rule_version: AiSchemaVersion,
    source_evidence_reference_id: AiEvidenceReferenceId,
}

impl AiRuleReference {
    pub(crate) fn new(
        policy_reference_id: AiPolicyReferenceId,
        family_id: AiFamilyId,
        rule_id: AiRuleId,
        rule_version: AiSchemaVersion,
        source_evidence_reference_id: AiEvidenceReferenceId,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&rule_version)?;
        Ok(Self {
            policy_reference_id,
            family_id,
            rule_id,
            rule_version,
            source_evidence_reference_id,
        })
    }

    pub fn policy_reference_id(&self) -> &AiPolicyReferenceId {
        &self.policy_reference_id
    }

    pub fn rule_id(&self) -> &AiRuleId {
        &self.rule_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn source_evidence_reference_id(&self) -> &AiEvidenceReferenceId {
        &self.source_evidence_reference_id
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPromptReference {
    template_id: AiPromptTemplateId,
    version: AiPromptVersion,
    task: AiSafeText,
}

impl AiPromptReference {
    pub(crate) fn new(
        template_id: AiPromptTemplateId,
        version: AiPromptVersion,
        task: AiSafeText,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            template_id,
            version,
            task,
        })
    }

    pub fn template_id(&self) -> &AiPromptTemplateId {
        &self.template_id
    }

    pub fn version(&self) -> &AiPromptVersion {
        &self.version
    }

    pub fn task(&self) -> &AiSafeText {
        &self.task
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRuntimeReference {
    runtime_reference_id: AiRuntimeReferenceId,
    provider_id: AiProviderId,
    model_id: AiModelId,
    capability_ids: Vec<AiCapabilityId>,
    observed_at: AiTimestamp,
}

impl AiRuntimeReference {
    pub fn new(
        runtime_reference_id: AiRuntimeReferenceId,
        provider_id: AiProviderId,
        model_id: AiModelId,
        capability_ids: Vec<AiCapabilityId>,
        observed_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        if capability_ids.is_empty() || !observed_at.is_well_formed() {
            return Err("AI runtime reference requires capabilities and a well-formed timestamp");
        }
        Ok(Self {
            runtime_reference_id,
            provider_id,
            model_id,
            capability_ids,
            observed_at,
        })
    }

    pub fn runtime_reference_id(&self) -> &AiRuntimeReferenceId {
        &self.runtime_reference_id
    }

    pub fn provider_id(&self) -> &AiProviderId {
        &self.provider_id
    }

    pub fn model_id(&self) -> &AiModelId {
        &self.model_id
    }

    pub fn capability_ids(&self) -> &[AiCapabilityId] {
        &self.capability_ids
    }

    pub fn observed_at(&self) -> &AiTimestamp {
        &self.observed_at
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRuntimeReferenceFields {
    runtime_reference_id: AiRuntimeReferenceId,
    provider_id: AiProviderId,
    model_id: AiModelId,
    capability_ids: Vec<AiCapabilityId>,
    observed_at: AiTimestamp,
}

impl<'de> Deserialize<'de> for AiRuntimeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiRuntimeReferenceFields::deserialize(deserializer)?;
        Self::new(
            fields.runtime_reference_id,
            fields.provider_id,
            fields.model_id,
            fields.capability_ids,
            fields.observed_at,
        )
        .map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContextRequest {
    identity: AiSchemaIdentity,
    requested_evaluation: AiSafeText,
    requested_at: AiTimestamp,
    required_evidence: Vec<AiEvidenceKind>,
    allowed_custody: Vec<AiCustodyState>,
    parent_rules: Vec<AiRuleReference>,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
}

impl AiEvidenceContextRequest {
    pub(crate) fn new(
        identity: AiSchemaIdentity,
        requested_evaluation: AiSafeText,
        requested_at: AiTimestamp,
        required_evidence: Vec<AiEvidenceKind>,
        allowed_custody: Vec<AiCustodyState>,
        parent_rules: Vec<AiRuleReference>,
        prompt: AiPromptReference,
        runtime: Option<AiRuntimeReference>,
    ) -> Result<Self, &'static str> {
        if required_evidence.is_empty()
            || allowed_custody.is_empty()
            || !requested_at.is_well_formed()
            || parent_rules
                .iter()
                .any(|rule| rule.family_id() != identity.family())
        {
            return Err(
                "AI evidence context request has mismatched family or missing required data",
            );
        }
        Ok(Self {
            identity,
            requested_evaluation,
            requested_at,
            required_evidence,
            allowed_custody,
            parent_rules,
            prompt,
            runtime,
        })
    }

    pub fn identity(&self) -> &AiSchemaIdentity {
        &self.identity
    }

    pub fn requested_evaluation(&self) -> &AiSafeText {
        &self.requested_evaluation
    }

    pub fn requested_at(&self) -> &AiTimestamp {
        &self.requested_at
    }

    pub fn required_evidence(&self) -> &[AiEvidenceKind] {
        &self.required_evidence
    }

    pub fn allowed_custody(&self) -> &[AiCustodyState] {
        &self.allowed_custody
    }

    pub fn parent_rules(&self) -> &[AiRuleReference] {
        &self.parent_rules
    }

    pub fn prompt(&self) -> &AiPromptReference {
        &self.prompt
    }

    pub fn runtime(&self) -> Option<&AiRuntimeReference> {
        self.runtime.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContext {
    schema_version: AiSchemaVersion,
    request_id: AiRequestId,
    family_id: AiFamilyId,
    child_profile_id: Option<AiChildProfileId>,
    device_id: Option<AiDeviceId>,
    evidence: Vec<AiEvidenceReference>,
    parent_rules: Vec<AiRuleReference>,
    memory: Vec<AiMemoryReference>,
    graph: Vec<AiGraphReference>,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
    custody: Vec<AiCustodyState>,
    authority_boundary: AiAuthorityBoundary,
    degraded_state: AiDegradedState,
}

impl AiEvidenceContext {
    pub(crate) fn new(
        schema_version: AiSchemaVersion,
        request_id: AiRequestId,
        family_id: AiFamilyId,
        child_profile_id: Option<AiChildProfileId>,
        device_id: Option<AiDeviceId>,
        evidence: Vec<AiEvidenceReference>,
        parent_rules: Vec<AiRuleReference>,
        memory: Vec<AiMemoryReference>,
        graph: Vec<AiGraphReference>,
        prompt: AiPromptReference,
        runtime: Option<AiRuntimeReference>,
        custody: Vec<AiCustodyState>,
        authority_boundary: AiAuthorityBoundary,
        degraded_state: AiDegradedState,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if evidence.iter().any(|item| item.family_id() != &family_id)
            || parent_rules
                .iter()
                .any(|rule| rule.family_id() != &family_id)
            || memory.iter().any(|item| item.family_id() != &family_id)
            || graph.iter().any(|item| item.family_id() != &family_id)
        {
            return Err("AI evidence context contains a family-mismatched reference");
        }
        if !matches!(authority_boundary, AiAuthorityBoundary::EvidenceOnly) {
            return Err("AI evidence context cannot mint policy authority");
        }
        if custody.is_empty() {
            return Err("AI evidence context must declare custody states");
        }
        Ok(Self {
            schema_version,
            request_id,
            family_id,
            child_profile_id,
            device_id,
            evidence,
            parent_rules,
            memory,
            graph,
            prompt,
            runtime,
            custody,
            authority_boundary,
            degraded_state,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn evidence(&self) -> &[AiEvidenceReference] {
        &self.evidence
    }

    pub fn memory(&self) -> &[AiMemoryReference] {
        &self.memory
    }

    pub fn graph(&self) -> &[AiGraphReference] {
        &self.graph
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContextBuildResult {
    request_id: AiRequestId,
    state: AiContextBuildState,
    validation: AiValidationState,
    context: Option<AiEvidenceContext>,
    rejected_references: Vec<AiEvidenceReferenceId>,
    missing_evidence: Vec<AiEvidenceKind>,
    degraded_state: AiDegradedState,
}

impl AiEvidenceContextBuildResult {
    pub(crate) fn new(
        request_id: AiRequestId,
        state: AiContextBuildState,
        validation: AiValidationState,
        context: Option<AiEvidenceContext>,
        rejected_references: Vec<AiEvidenceReferenceId>,
        missing_evidence: Vec<AiEvidenceKind>,
        degraded_state: AiDegradedState,
    ) -> Result<Self, &'static str> {
        if context
            .as_ref()
            .is_some_and(|context| context.request_id() != &request_id)
        {
            return Err("AI context build result request identity does not match its context");
        }
        match (state, validation, context.is_some()) {
            (AiContextBuildState::Ready, AiValidationState::Accepted, true)
            | (AiContextBuildState::Partial, AiValidationState::ManualRequired, true)
            | (AiContextBuildState::Rejected, AiValidationState::Rejected, false)
            | (AiContextBuildState::ManualRequired, AiValidationState::ManualRequired, _) => {}
            _ => return Err("AI context build state and validation are inconsistent"),
        }
        Ok(Self {
            request_id,
            state,
            validation,
            context,
            rejected_references,
            missing_evidence,
            degraded_state,
        })
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn state(&self) -> AiContextBuildState {
        self.state
    }

    pub fn validation(&self) -> AiValidationState {
        self.validation
    }

    pub fn context(&self) -> Option<&AiEvidenceContext> {
        self.context.as_ref()
    }
}
