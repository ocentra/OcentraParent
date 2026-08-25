use std::collections::HashSet;

use super::AiResultPayload;
use crate::ai_contracts::context::{
    AiEvidenceReference, AiOwnerResolvedRuntime, AiPromptReference, AiRuleReference,
};
use crate::ai_contracts::identity::AiFamilyId;
use crate::ai_contracts::memory::{AiGraphReference, AiMemoryReference};
use crate::ai_contracts::reference_inventory::AiReferenceInventory;
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
        runtime: AiOwnerResolvedRuntime,
    ) -> Result<Self, &'static str> {
        let inventory = AiReferenceInventory::new(&evidence, &memory, &graph, &rules)?;
        let claim_ids = claims
            .iter()
            .map(super::AiClaim::claim_id)
            .collect::<HashSet<_>>();
        let claims_grounded = claims.iter().all(|claim| {
            claim.has_unique_reference_ids()
                && claim
                    .evidence_reference_ids()
                    .iter()
                    .all(|id| inventory.contains_evidence(id))
                && claim
                    .memory_reference_ids()
                    .iter()
                    .all(|id| inventory.contains_memory(id))
                && claim
                    .graph_reference_ids()
                    .iter()
                    .all(|id| inventory.contains_graph(id))
                && claim
                    .rule_reference_ids()
                    .iter()
                    .all(|id| inventory.contains_rule(id))
        });
        if evidence.iter().any(|item| item.family_id() != &family_id)
            || memory.iter().any(|item| item.family_id() != &family_id)
            || graph.iter().any(|item| item.family_id() != &family_id)
            || rules.iter().any(|rule| rule.family_id() != &family_id)
            || claims
                .iter()
                .any(|claim| claim.subject().family_id() != &family_id)
            || claim_ids.len() != claims.len()
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
            runtime: runtime.into_runtime(),
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
