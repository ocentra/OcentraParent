use std::collections::HashSet;
use std::hash::Hash;

use super::context::{AiEvidenceReference, AiRuleReference};
use super::identity::{
    AiDigest, AiEvidenceReferenceId, AiGraphReferenceId, AiMemoryReferenceId, AiResultId, AiRuleId,
};
use super::memory::{AiGraphReference, AiMemoryReference};

pub(super) struct AiReferenceInventory<'a> {
    evidence_ids: HashSet<&'a AiEvidenceReferenceId>,
    memory_ids: HashSet<&'a AiMemoryReferenceId>,
    graph_ids: HashSet<&'a AiGraphReferenceId>,
    rule_ids: HashSet<&'a AiRuleId>,
}

fn unique_set<'a, T>(values: impl Iterator<Item = &'a T>, expected: usize) -> Option<HashSet<&'a T>>
where
    T: Eq + Hash + 'a,
{
    let values = values.collect::<HashSet<_>>();
    (values.len() == expected).then_some(values)
}

fn known_unique_evidence(
    ids: &[AiEvidenceReferenceId],
    inventory: &HashSet<&AiEvidenceReferenceId>,
) -> bool {
    unique_set(ids.iter(), ids.len()).is_some() && ids.iter().all(|id| inventory.contains(id))
}

fn valid_result_binding(result_id: Option<&AiResultId>, digest: Option<&AiDigest>) -> bool {
    result_id.is_some() == digest.is_some() && digest.is_none_or(AiDigest::is_canonical)
}

fn valid_grounding(
    evidence_ids: &[AiEvidenceReferenceId],
    evidence_inventory: &HashSet<&AiEvidenceReferenceId>,
    result_id: Option<&AiResultId>,
    digest: Option<&AiDigest>,
) -> bool {
    known_unique_evidence(evidence_ids, evidence_inventory)
        && valid_result_binding(result_id, digest)
        && (!evidence_ids.is_empty() || result_id.is_some())
}

impl<'a> AiReferenceInventory<'a> {
    pub(super) fn new(
        evidence: &'a [AiEvidenceReference],
        memory: &'a [AiMemoryReference],
        graph: &'a [AiGraphReference],
        rules: &'a [AiRuleReference],
    ) -> Result<Self, &'static str> {
        let evidence_ids = unique_set(
            evidence
                .iter()
                .map(AiEvidenceReference::evidence_reference_id),
            evidence.len(),
        )
        .ok_or("AI reference inventory contains duplicate evidence identity")?;
        let memory_ids = unique_set(
            memory.iter().map(AiMemoryReference::memory_reference_id),
            memory.len(),
        )
        .ok_or("AI reference inventory contains duplicate memory identity")?;
        let graph_ids = unique_set(
            graph.iter().map(AiGraphReference::graph_reference_id),
            graph.len(),
        )
        .ok_or("AI reference inventory contains duplicate graph identity")?;
        let rule_ids = unique_set(rules.iter().map(AiRuleReference::rule_id), rules.len())
            .ok_or("AI reference inventory contains duplicate rule identity")?;
        unique_set(
            rules.iter().map(AiRuleReference::policy_reference_id),
            rules.len(),
        )
        .ok_or("AI reference inventory contains duplicate policy identity")?;

        if evidence.iter().any(|item| !item.is_grounding_safe())
            || memory.iter().any(|item| {
                !item.is_grounding_safe()
                    || !valid_grounding(
                        item.provenance().source_evidence_reference_ids(),
                        &evidence_ids,
                        item.provenance().source_result_id(),
                        item.provenance().source_digest(),
                    )
            })
            || graph.iter().any(|item| {
                !item.is_grounding_safe()
                    || !valid_grounding(
                        item.source_evidence_reference_ids(),
                        &evidence_ids,
                        item.source_result_id(),
                        item.source_result_digest(),
                    )
                    || item
                        .source_memory_reference_id()
                        .is_some_and(|id| !memory_ids.contains(id))
            })
            || rules
                .iter()
                .any(|rule| !evidence_ids.contains(rule.source_evidence_reference_id()))
        {
            return Err("AI reference inventory contains unsafe or unbound provenance");
        }

        Ok(Self {
            evidence_ids,
            memory_ids,
            graph_ids,
            rule_ids,
        })
    }

    pub(super) fn contains_evidence(&self, id: &AiEvidenceReferenceId) -> bool {
        self.evidence_ids.contains(id)
    }

    pub(super) fn contains_memory(&self, id: &AiMemoryReferenceId) -> bool {
        self.memory_ids.contains(id)
    }

    pub(super) fn contains_graph(&self, id: &AiGraphReferenceId) -> bool {
        self.graph_ids.contains(id)
    }

    pub(super) fn contains_rule(&self, id: &AiRuleId) -> bool {
        self.rule_ids.contains(id)
    }
}
