use ocentra_schema::export_import_backup_recovery as contracts;

use super::{RestoreExecutionBinding, RestoreExecutionCapability};

impl RestoreExecutionBinding {
    pub fn is_same_capability(&self, other: &Self) -> bool {
        std::ptr::eq(self.capability.as_ref(), other.capability.as_ref())
    }

    pub fn capability(&self) -> &dyn RestoreExecutionCapability {
        self.capability.as_ref()
    }

    pub fn migration_ref(&self) -> Option<&contracts::ExportImportMigrationRef> {
        self.migration_ref.as_ref()
    }
}
