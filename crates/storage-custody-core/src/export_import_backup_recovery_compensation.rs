use ocentra_schema::export_import_backup_recovery as contracts;

use super::export_import_backup_recovery_restore_execution_plan::RestoreExecutionPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartialWriteCompensation {
    NotRequired,
    Required,
    Applied,
    ManualRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialWriteObservation {
    pub applied_sections: Vec<contracts::ExportImportSectionDecision>,
    pub rejected_sections: Vec<contracts::ExportImportSectionDecision>,
}

/// Decides whether a partial restore requires rollback/compensation. The
/// decision is derived from the owner-bound plan and observed section results;
/// no caller-provided success or integrity flag can bypass tombstone safety.
pub fn decide_partial_write_compensation(
    plan: &RestoreExecutionPlan,
    observation: &PartialWriteObservation,
) -> PartialWriteCompensation {
    if !plan.no_resurrection()
        || observation
            .applied_sections
            .iter()
            .any(|section| section.state != contracts::ExportImportSectionDecisionState::Accepted)
    {
        return PartialWriteCompensation::ManualRequired;
    }
    if observation.applied_sections.is_empty() {
        return PartialWriteCompensation::NotRequired;
    }
    if observation.applied_sections == plan.accepted_sections()
        && observation.rejected_sections == plan.rejected_sections()
    {
        PartialWriteCompensation::NotRequired
    } else {
        PartialWriteCompensation::Required
    }
}
