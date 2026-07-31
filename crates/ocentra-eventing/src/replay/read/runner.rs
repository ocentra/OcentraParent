use crate::{
    journal::{
        hash_chain::{verify_synchronization_activation, verify_synchronization_completion},
        JournalHashVersion,
    },
    EventingError, NdjsonEventJournal, ReplayCursor, ReplayFilter, ReplayMode, ReplayReadReport,
    ReplayRecord,
};

pub(super) async fn read(
    journal: &NdjsonEventJournal,
    filter: ReplayFilter,
    mode: ReplayMode,
) -> Result<ReplayReadReport, EventingError> {
    let (entries, completions, activations, mut skipped_count) =
        super::records::collect(journal).await?;
    let last_sequence = entries
        .last()
        .map_or(filter.cursor.next_sequence.saturating_sub(1), |entry| {
            entry.append.sequence
        });
    let mut records = Vec::new();
    for entry in entries {
        let completed = entry.append.hash_version != JournalHashVersion::V3
            || completions.iter().any(|completion| {
                verify_synchronization_completion(&entry, completion)
                    && activations
                        .iter()
                        .any(|activation| verify_synchronization_activation(completion, activation))
            });
        let skip = !completed
            || (mode == ReplayMode::ActionHandlersAllowed
                && entry.phase != crate::JournalDispatchPhase::AfterDispatch)
            || !filter.matches(&entry);
        if skip {
            skipped_count += 1;
        } else {
            records.push(ReplayRecord {
                sequence: entry.append.sequence,
                envelope: entry.envelope,
            });
        }
    }

    Ok(ReplayReadReport {
        mode,
        cursor: ReplayCursor::after(last_sequence),
        records,
        skipped_count,
    })
}
