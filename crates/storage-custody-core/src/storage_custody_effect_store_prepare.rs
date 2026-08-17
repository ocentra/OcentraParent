use std::io;

use super::{
    storage_custody_effect_store_io, StorageCustodyEffectRecord, StorageCustodyEffectStore,
};

impl StorageCustodyEffectStore {
    pub fn prepare(&self, record: StorageCustodyEffectRecord) -> io::Result<()> {
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
    if record.schema_version != 1
        || record.operation_ref.trim().is_empty()
        || record.effect_ref.trim().is_empty()
        || record.household_id.trim().is_empty()
        || record.child_profile_id.trim().is_empty()
        || record.target_device_id.trim().is_empty()
        || record.authority_generation == 0
        || record.session_generation == 0
        || record.apply_lease_id.as_deref().is_some_and(str::is_empty)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "custody effect intent has an invalid binding",
        ));
    }
    if record.effect_kind == super::StorageCustodyEffectKind::LocalDelete
        && record.relative_path.as_deref().map_or(true, str::is_empty)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "local delete effect requires a relative payload path",
        ));
    }
    if record.effect_kind != super::StorageCustodyEffectKind::LocalDelete
        && record.relative_path.is_some()
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "non-local custody effects must not carry a filesystem path",
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
