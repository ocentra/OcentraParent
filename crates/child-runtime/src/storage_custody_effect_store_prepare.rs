use std::io;

use super::{
    storage_custody_effect_store_io, StorageCustodyEffectRecord, StorageCustodyEffectStore,
};

impl StorageCustodyEffectStore {
    pub(super) fn prepare(&self, record: StorageCustodyEffectRecord) -> io::Result<()> {
        validate_record_for_prepare(&record)?;
        let lock = self.lock()?;
        fs2::FileExt::lock_exclusive(&lock)?;
        let mut records = self.read_records()?;
        let result = match records
            .iter()
            .find(|existing| existing.operation_ref == record.operation_ref)
        {
            Some(existing) => existing_record_is_same(existing, &record),
            None => {
                records.push(record);
                self.write_records(&records)
            }
        };
        storage_custody_effect_store_io::unlock(&lock)?;
        result
    }
}

fn validate_record_for_prepare(record: &StorageCustodyEffectRecord) -> io::Result<()> {
    record.validate_loaded()?;
    if record.status != super::StorageCustodyEffectStatus::Prepared
        || record.apply_lease_id.is_some()
        || record.manual_required_reason.is_some()
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "new custody effect must be Prepared with no lease or manual reason",
        ));
    }
    Ok(())
}

fn existing_record_is_same(
    existing: &StorageCustodyEffectRecord,
    record: &StorageCustodyEffectRecord,
) -> io::Result<()> {
    if existing.schema_version != record.schema_version
        || existing.effect_kind != record.effect_kind
        || existing.effect_ref != record.effect_ref
        || existing.relative_path != record.relative_path
        || existing.household_id != record.household_id
        || existing.child_profile_id != record.child_profile_id
        || existing.target_device_id != record.target_device_id
        || existing.authority_generation != record.authority_generation
        || existing.session_generation != record.session_generation
        || existing.custody_input != record.custody_input
        || existing.action != record.action
        || existing.envelope != record.envelope
    {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "custody effect operation reference was reused with different data",
        ));
    }
    Ok(())
}
