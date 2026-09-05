use std::io;

use super::{StorageCustodyEffectStatus, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub(in crate::service::storage_custody_runtime) fn mark_manual_required(
        &self,
        operation_ref: &str,
        reason: impl Into<String>,
    ) -> io::Result<()> {
        let reason = validated_reason(reason)?;
        self.update(operation_ref, |record| match record.status {
            StorageCustodyEffectStatus::Applied | StorageCustodyEffectStatus::ManualRequired => {
                Ok(())
            }
            StorageCustodyEffectStatus::Applying => Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "an in-flight local delete requires its owner lease for recovery",
            )),
            StorageCustodyEffectStatus::Prepared | StorageCustodyEffectStatus::Journaled => {
                record.status = StorageCustodyEffectStatus::ManualRequired;
                record.apply_lease_id = None;
                record.manual_required_reason = Some(reason.clone());
                Ok(())
            }
        })
    }
}

fn validated_reason(reason: impl Into<String>) -> io::Result<String> {
    let reason = reason.into();
    if reason.trim().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "manual-required custody effect needs a reason",
        ));
    }
    Ok(reason)
}
