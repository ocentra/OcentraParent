use std::io;

use super::{StorageCustodyEffectKind, StorageCustodyEffectStatus, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub(in crate::service::storage_custody_runtime) fn mark_applied(
        &self,
        operation_ref: &str,
        lease_id: &str,
    ) -> io::Result<()> {
        if lease_id.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local custody apply requires a non-empty owner lease",
            ));
        }
        self.update(operation_ref, |record| {
            mark_applied_status(record, lease_id)
        })
    }
}

fn mark_applied_status(
    record: &mut super::StorageCustodyEffectRecord,
    lease_id: &str,
) -> io::Result<()> {
    if record.effect_kind != StorageCustodyEffectKind::LocalDelete {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only local delete effects may be marked applied",
        ));
    }
    match record.status {
        StorageCustodyEffectStatus::Applied => Ok(()),
        StorageCustodyEffectStatus::Applying
            if record.apply_lease_id.as_deref() == Some(lease_id) =>
        {
            record.status = StorageCustodyEffectStatus::Applied;
            record.apply_lease_id = None;
            record.manual_required_reason = None;
            Ok(())
        }
        StorageCustodyEffectStatus::Applying => Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "local delete effect is owned by a different apply lease",
        )),
        StorageCustodyEffectStatus::Prepared
        | StorageCustodyEffectStatus::Journaled
        | StorageCustodyEffectStatus::ManualRequired => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "local delete effect was not applying",
        )),
    }
}
