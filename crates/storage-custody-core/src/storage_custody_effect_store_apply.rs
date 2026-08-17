use std::io;

use super::{StorageCustodyEffectKind, StorageCustodyEffectStatus, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub fn begin_local_apply(&self, operation_ref: &str, lease_id: &str) -> io::Result<()> {
        if lease_id.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local custody apply requires a non-empty owner lease",
            ));
        }
        self.update(operation_ref, |record| {
            begin_local_apply_status(record, lease_id)
        })
    }
}

fn begin_local_apply_status(
    record: &mut super::StorageCustodyEffectRecord,
    lease_id: &str,
) -> io::Result<()> {
    if record.effect_kind != StorageCustodyEffectKind::LocalDelete {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only local delete effects may enter Applying",
        ));
    }
    match record.status {
        StorageCustodyEffectStatus::Journaled => {
            record.status = StorageCustodyEffectStatus::Applying;
            record.apply_lease_id = Some(lease_id.to_owned());
            Ok(())
        }
        StorageCustodyEffectStatus::Applying => Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "local delete effect already has an apply owner",
        )),
        StorageCustodyEffectStatus::Applied => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "local delete effect is already terminal",
        )),
        StorageCustodyEffectStatus::Prepared | StorageCustodyEffectStatus::ManualRequired => {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local delete effect must be journaled before apply",
            ))
        }
    }
}
