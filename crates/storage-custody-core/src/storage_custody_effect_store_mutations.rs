use std::io;

use super::{StorageCustodyEffectKind, StorageCustodyEffectStatus, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub fn mark_journaled(&self, operation_ref: &str) -> io::Result<()> {
        self.update(operation_ref, |record| {
            if matches!(
                record.status,
                StorageCustodyEffectStatus::Prepared | StorageCustodyEffectStatus::Journaled
            ) {
                record.status = StorageCustodyEffectStatus::Journaled;
            }
            Ok(())
        })
    }

    pub fn begin_local_apply(&self, operation_ref: &str) -> io::Result<()> {
        self.update(operation_ref, begin_local_apply_status)
    }

    pub fn mark_applied(&self, operation_ref: &str) -> io::Result<()> {
        self.update(operation_ref, |record| {
            if record.effect_kind != StorageCustodyEffectKind::LocalDelete {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "only local delete effects may be marked applied",
                ));
            }
            if !matches!(
                record.status,
                StorageCustodyEffectStatus::Applying | StorageCustodyEffectStatus::Applied
            ) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "local delete effect was not applying",
                ));
            }
            record.status = StorageCustodyEffectStatus::Applied;
            record.manual_required_reason = None;
            Ok(())
        })
    }

    pub fn mark_manual_required(
        &self,
        operation_ref: &str,
        reason: impl Into<String>,
    ) -> io::Result<()> {
        let reason = reason.into();
        if reason.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "manual-required custody effect needs a reason",
            ));
        }
        self.update(operation_ref, |record| {
            record.status = StorageCustodyEffectStatus::ManualRequired;
            record.manual_required_reason = Some(reason.clone());
            Ok(())
        })
    }
}

fn begin_local_apply_status(record: &mut super::StorageCustodyEffectRecord) -> io::Result<()> {
    if record.effect_kind != StorageCustodyEffectKind::LocalDelete {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "only local delete effects may enter Applying",
        ));
    }
    match record.status {
        StorageCustodyEffectStatus::Journaled | StorageCustodyEffectStatus::Applying => {
            record.status = StorageCustodyEffectStatus::Applying;
            Ok(())
        }
        StorageCustodyEffectStatus::Applied => Ok(()),
        StorageCustodyEffectStatus::Prepared | StorageCustodyEffectStatus::ManualRequired => {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "local delete effect must be journaled before apply",
            ))
        }
    }
}
