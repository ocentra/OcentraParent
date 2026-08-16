use ocentra_schema::export_import_backup_recovery as contracts;

use super::ImportBundleContext;

#[path = "export_import_backup_recovery_import_logic.rs"]
mod export_import_backup_recovery_import_logic;

pub(super) fn run_import_preflight(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportImportPreflight {
    export_import_backup_recovery_import_logic::run_import_preflight(bundle, context)
}
