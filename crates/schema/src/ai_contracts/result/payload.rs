use std::collections::HashSet;

use super::AiResultPayload;
use crate::ai_contracts::context::{
    AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference,
};
use crate::ai_contracts::identity::AiFamilyId;
use crate::ai_contracts::memory::{AiGraphReference, AiMemoryReference};
use crate::ai_contracts::AiSafeText;

impl AiResultPayload {
    pub(crate) fn new(
        family_id: AiFamilyId,
        claims: Vec<super::AiClaim>,
        summary: Option<AiSafeText>,
        evidence: Vec<AiEvidenceReference>,
        memory: Vec<AiMemoryReference>,
        graph: Vec<AiGraphReference>,
        rules: Vec<AiRuleReference>,
        prompt: AiPromptReference,
        runtime: Option<AiRuntimeReference>,
    ) -> Result<Self, &'static str> {
        let evidence_ids = evidence
            .iter()
            .map(|item| item.evidence_reference_id())
            .collect::<HashSet<_>>();
        let memory_ids = memory
            .iter()
            .map(|item| item.memory_reference_id())
            .collect::<HashSet<_>>();
        let graph_ids = graph
            .iter()
            .map(|item| item.graph_reference_id())
            .collect::<HashSet<_>>();
        let rule_ids = rules
            .iter()
            .map(|rule| rule.rule_id())
            .collect::<HashSet<_>>();
        let claims_grounded = claims.iter().all(|claim| {
            claim
                .evidence_reference_ids()
                .iter()
                .all(|id| evidence_ids.contains(id))
                && claim
                    .memory_reference_ids()
                    .iter()
                    .all(|id| memory_ids.contains(id))
                && claim
                    .graph_reference_ids()
                    .iter()
                    .all(|id| graph_ids.contains(id))
                && claim
                    .rule_reference_ids()
                    .iter()
                    .all(|id| rule_ids.contains(id))
        });
        if evidence.iter().any(|item| item.family_id() != &family_id)
            || memory.iter().any(|item| item.family_id() != &family_id)
            || graph.iter().any(|item| item.family_id() != &family_id)
            || rules.iter().any(|rule| rule.family_id() != &family_id)
            || claims
                .iter()
                .any(|claim| claim.subject().family_id() != &family_id)
            || !claims_grounded
        {
            return Err("AI result payload contains a family-mismatched identity");
        }
        Ok(Self {
            family_id,
            claims,
            summary,
            evidence,
            memory,
            graph,
            rules,
            prompt,
            runtime,
        })
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn claims(&self) -> &[super::AiClaim] {
        &self.claims
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
}
