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
    // A V3 entry can be visible before its separately written activation. Do
    // not checkpoint past that entry: an incremental caller would otherwise
    // never see it after the activation reaches the journal.
    let mut last_sequence = filter.cursor.next_sequence.saturating_sub(1);
    let mut records = Vec::new();
    for entry in entries {
        let completed = entry.append.hash_version != JournalHashVersion::V3
            || completions.iter().any(|completion| {
                verify_synchronization_completion(&entry, completion)
                    && activations
                        .iter()
                        .any(|activation| verify_synchronization_activation(completion, activation))
            });
        if entry.append.sequence >= filter.cursor.next_sequence && !completed {
            skipped_count += 1;
            break;
        }
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
        if entry.append.sequence >= filter.cursor.next_sequence {
            last_sequence = entry.append.sequence;
        }
    }

    Ok(ReplayReadReport {
        mode,
        cursor: ReplayCursor::after(last_sequence),
        records,
        skipped_count,
    })
}
