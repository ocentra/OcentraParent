use std::io;

use super::{StorageCustodyEffectStatus, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub fn mark_manual_required(
        &self,
        operation_ref: &str,
        reason: impl Into<String>,
    ) -> io::Result<()> {
        let reason = validated_reason(reason)?;
        self.update(operation_ref, |record| {
            if record.status == StorageCustodyEffectStatus::Applied {
                return Ok(());
            }
            if record.status == StorageCustodyEffectStatus::Applying {
                return Err(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "an in-flight local delete requires its owner lease for recovery",
                ));
            }
            record.status = StorageCustodyEffectStatus::ManualRequired;
            record.apply_lease_id = None;
            record.manual_required_reason = Some(reason.clone());
            Ok(())
        })
    }

    pub fn mark_manual_required_with_lease(
        &self,
        operation_ref: &str,
        lease_id: &str,
        reason: impl Into<String>,
    ) -> io::Result<()> {
        if lease_id.trim().is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "manual custody recovery requires a non-empty owner lease",
            ));
        }
        let reason = validated_reason(reason)?;
        self.update(operation_ref, |record| {
            if record.status == StorageCustodyEffectStatus::Applied {
                return Ok(());
            }
            if record.status != StorageCustodyEffectStatus::Applying
                || record.apply_lease_id.as_deref() != Some(lease_id)
            {
                return Err(io::Error::new(
                    io::ErrorKind::WouldBlock,
                    "manual recovery does not own the local apply lease",
                ));
            }
            record.status = StorageCustodyEffectStatus::ManualRequired;
            record.apply_lease_id = None;
            record.manual_required_reason = Some(reason.clone());
            Ok(())
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
