use ocentra_schema::export_import_backup_recovery as contracts;
use ocentra_storage_custody_core::export_import_backup_recovery::
    export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan;

pub(super) fn matches_plan(
    plan: &RestoreExecutionPlan,
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> bool {
    let mut seen = Vec::new();
    for section in applied_sections {
        if section.state != contracts::ExportImportSectionDecisionState::Accepted
            || !plan.accepted_sections().contains(section)
            || seen.contains(&section.data_class)
        {
            return false;
        }
        seen.push(section.data_class);
    }
    for section in rejected_sections {
        if !plan.rejected_sections().contains(section) || seen.contains(&section.data_class) {
            return false;
        }
        seen.push(section.data_class);
    }
    plan.rejected_sections()
        .iter()
        .all(|section| rejected_sections.contains(section))
}
