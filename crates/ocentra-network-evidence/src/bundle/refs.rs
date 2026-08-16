use super::normalize::normalize_ref;
use super::NetworkCrossSliceEvidenceSource;
use crate::cascade::NetworkCascadeSourceKind;

pub(super) fn unique_evidence_refs(sources: &[NetworkCrossSliceEvidenceSource]) -> Vec<String> {
    let mut refs = Vec::new();
    for source in sources {
        if let Some(evidence_ref) = normalize_ref(&source.evidence_ref) {
            if !refs.contains(&evidence_ref) {
                refs.push(evidence_ref);
            }
        }
    }
    refs
}

pub(super) fn exact_url_evidence_refs(sources: &[NetworkCrossSliceEvidenceSource]) -> Vec<String> {
    sources
        .iter()
        .filter(|source| {
            source.source_kind == NetworkCascadeSourceKind::ManagedBrowserExactUrl
                && source.exact_url_available
        })
        .filter_map(|source| normalize_ref(&source.evidence_ref))
        .collect()
}
