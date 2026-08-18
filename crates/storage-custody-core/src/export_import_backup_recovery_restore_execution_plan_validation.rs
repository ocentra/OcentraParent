use ocentra_schema::export_import_backup_recovery as contracts;

use super::{RestoreExecutionPlan, RestoreExecutionPlanError};

pub(super) fn preflight_is_safe(preflight: &contracts::ExportImportImportPreflight) -> bool {
    matches!(
        preflight.state,
        contracts::ExportImportPreflightState::AcceptedPreview
            | contracts::ExportImportPreflightState::PartialPreview
    ) && preflight.schema_version_supported
        && preflight.household_binding_match
        && preflight.key_available
        && preflight.manifest_integrity_verified
        && preflight.payload_integrity_verified
        && !preflight.local_truth_mutated
        && preflight.tombstones_preserved
        && !preflight.duplicate_device_detected
}

fn sections_are_safe(
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> bool {
    let mut seen = Vec::new();
    for section in applied_sections.iter().chain(rejected_sections.iter()) {
        if section.state != contracts::ExportImportSectionDecisionState::Accepted
            && applied_sections.iter().any(|candidate| {
                candidate.data_class == section.data_class
                    && candidate.state == contracts::ExportImportSectionDecisionState::Accepted
            })
        {
            return false;
        }
        if seen.contains(&section.data_class) {
            return false;
        }
        seen.push(section.data_class);
    }
    rejected_sections
        .iter()
        .all(|section| section.state != contracts::ExportImportSectionDecisionState::Accepted)
}

pub(super) fn sections_match_plan(
    plan: &RestoreExecutionPlan,
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> bool {
    if !sections_are_safe(applied_sections, rejected_sections)
        || applied_sections.iter().any(|section| {
            section.state != contracts::ExportImportSectionDecisionState::Accepted
                || !plan.accepted_sections().contains(section)
        })
        || rejected_sections
            .iter()
            .any(|section| !plan.rejected_sections().contains(section))
        || plan
            .accepted_sections()
            .iter()
            .any(|section| !applied_sections.contains(section))
        || plan
            .rejected_sections()
            .iter()
            .any(|section| !rejected_sections.contains(section))
    {
        return false;
    }
    state != contracts::ExportImportRestoreApplyState::Applied
        || (applied_sections == plan.accepted_sections()
            && rejected_sections == plan.rejected_sections())
}

pub(super) fn validate_restore_execution_observation(
    plan: &RestoreExecutionPlan,
    state: contracts::ExportImportRestoreApplyState,
    applied_sections: &[contracts::ExportImportSectionDecision],
    rejected_sections: &[contracts::ExportImportSectionDecision],
) -> Result<(), RestoreExecutionPlanError> {
    if !plan.no_resurrection()
        || !sections_match_plan(plan, state, applied_sections, rejected_sections)
    {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    if state == contracts::ExportImportRestoreApplyState::ApplyPending {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    if state == contracts::ExportImportRestoreApplyState::Applied && plan.is_partial() {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    if state == contracts::ExportImportRestoreApplyState::Partial && applied_sections.is_empty() {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    if matches!(
        state,
        contracts::ExportImportRestoreApplyState::NotApplied
            | contracts::ExportImportRestoreApplyState::WrongHousehold
            | contracts::ExportImportRestoreApplyState::WrongKey
            | contracts::ExportImportRestoreApplyState::Corrupt
            | contracts::ExportImportRestoreApplyState::Blocked
    ) && !applied_sections.is_empty()
    {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    if state == contracts::ExportImportRestoreApplyState::Partial
        && !plan.is_partial()
        && applied_sections == plan.accepted_sections()
        && rejected_sections == plan.rejected_sections()
    {
        return Err(RestoreExecutionPlanError::UnsafeSectionDecision);
    }
    Ok(())
}
