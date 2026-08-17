use crate::retention_delete_tombstone_store::RetentionDeleteOutboxRecord;

pub(super) fn is_completed_terminal_marker(
    record: &RetentionDeleteOutboxRecord,
) -> std::io::Result<bool> {
    if record.terminal_pending {
        return Ok(false);
    }
    (record.version == 3 && record.typed_action_and_envelope().is_none())
        .then_some(true)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "child-runtime tombstone recovery requires a version 3 terminal marker for a completed tombstone",
            )
        })
}
