use ocentra_schema::export_import_backup_recovery as contracts;

use super::ImportBundleContext;

#[path = "export_import_backup_recovery_import_rejection.rs"]
mod export_import_backup_recovery_import_rejection;
#[path = "export_import_backup_recovery_import_response.rs"]
mod export_import_backup_recovery_import_response;
#[path = "export_import_backup_recovery_import_sections.rs"]
mod export_import_backup_recovery_import_sections;

pub(super) fn run_import_preflight(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> contracts::ExportImportImportPreflight {
    if let Some(rejected) =
        export_import_backup_recovery_import_rejection::import_preflight_rejection(bundle, context)
    {
        return export_import_backup_recovery_import_response::rejected_preflight(rejected);
    }

    let migration_state =
        export_import_backup_recovery_import_rejection::import_preflight_migration_state(
            bundle, context,
        );
    let (state, accepted_sections, rejected_sections) =
        export_import_backup_recovery_import_sections::import_preflight_section_decisions(
            bundle, context,
        );

    export_import_backup_recovery_import_response::accepted_preflight(
        state,
        migration_state,
        accepted_sections,
        rejected_sections,
    )
}
