use ocentra_schema::export_import_backup_recovery as contracts;

use super::ImportBundleContext;

pub(super) fn import_preflight_section_decisions(
    bundle: &contracts::ExportImportRecoveryBundle,
    context: &ImportBundleContext,
) -> (
    contracts::ExportImportPreflightState,
    Vec<contracts::ExportImportSectionDecision>,
    Vec<contracts::ExportImportSectionDecision>,
) {
    let mut accepted_sections = Vec::new();
    let mut rejected_sections = Vec::new();

    for section in &bundle.sections {
        let blocked_by_tombstone = context
            .blocked_restore_data_classes
            .iter()
            .any(|data_class| data_class == &section.data_class)
            || section.retention_state == contracts::ExportImportSectionRetentionState::Tombstoned;

        if section.retention_state == contracts::ExportImportSectionRetentionState::Expired {
            rejected_sections.push(contracts::ExportImportSectionDecision {
                data_class: section.data_class,
                state: contracts::ExportImportSectionDecisionState::RetentionExpired,
                reason: "Retention expired before restore preview.".to_string(),
            });
            continue;
        }

        if blocked_by_tombstone {
            rejected_sections.push(contracts::ExportImportSectionDecision {
                data_class: section.data_class,
                state: contracts::ExportImportSectionDecisionState::TombstonePreserved,
                reason: "Local tombstone ordering blocks section resurrection.".to_string(),
            });
            continue;
        }

        accepted_sections.push(contracts::ExportImportSectionDecision {
            data_class: section.data_class,
            state: contracts::ExportImportSectionDecisionState::Accepted,
            reason: "Section passed household, key, integrity, and retention preflight."
                .to_string(),
        });
    }

    let state = if accepted_sections.is_empty() {
        if rejected_sections.iter().all(|decision| {
            decision.state == contracts::ExportImportSectionDecisionState::RetentionExpired
        }) {
            contracts::ExportImportPreflightState::RetentionExpired
        } else {
            contracts::ExportImportPreflightState::TombstoneConflict
        }
    } else if rejected_sections.is_empty() {
        contracts::ExportImportPreflightState::AcceptedPreview
    } else {
        contracts::ExportImportPreflightState::PartialPreview
    };

    (state, accepted_sections, rejected_sections)
}
