use std::io;

use super::{storage_custody_effect_store_io, StorageCustodyEffectStore};

impl StorageCustodyEffectStore {
    pub(super) fn update(
        &self,
        operation_ref: &str,
        update: impl FnOnce(&mut super::StorageCustodyEffectRecord) -> io::Result<()>,
    ) -> io::Result<()> {
        let lock = self.lock()?;
        fs2::FileExt::lock_exclusive(&lock)?;
        let mut records = self.read_records()?;
        let Some(record) = records
            .iter_mut()
            .find(|record| record.operation_ref == operation_ref)
        else {
            storage_custody_effect_store_io::unlock(&lock)?;
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("unknown custody effect operation: {operation_ref}"),
            ));
        };
        let result = update(record).and_then(|()| self.write_records(&records));
        storage_custody_effect_store_io::unlock(&lock)?;
        result
    }
}
