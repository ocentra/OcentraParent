use super::{score, NetworkCascadeSource, NetworkCascadeSignalStrength};

pub(super) fn strongest_source(sources: &[NetworkCascadeSource]) -> Option<&NetworkCascadeSource> {
    sources
        .iter()
        .filter(|source| source.signal_strength != NetworkCascadeSignalStrength::Unavailable)
        .max_by_key(|source| score::source_score(source))
}
