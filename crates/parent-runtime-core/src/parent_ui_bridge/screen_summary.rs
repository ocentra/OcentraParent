use super::*;
use ocentra_parent_agent_protocol::screen_child_disclosure::ActivityScreenChildDisclosure;

pub(super) fn screen_summary_panel_snapshot(
    read_model: Option<&ActivityScreenReadModel>,
) -> ParentScreenSummaryPanelSnapshot {
    let eyebrow = "Activity kind".to_string();
    let title = "Screen analysis".to_string();
    let body = "Stored activity".to_string();
    let empty_message = "No recent activity is available yet.".to_string();
    let product_claim = "No family setting is configured for this area yet.".to_string();

    match read_model {
        None => ParentScreenSummaryPanelSnapshot {
            eyebrow,
            title,
            body,
            load_state: SCREEN_SUMMARY_UNAVAILABLE.to_string(),
            summary_details: unavailable_screen_summary_details(&product_claim),
            rows: Vec::new(),
            empty_message,
            product_claim,
        },
        Some(read_model) => {
            let latest_row = read_model.rows.first();
            let child_disclosure =
                ActivityScreenChildDisclosure::unavailable(read_model.schema_version);
            ParentScreenSummaryPanelSnapshot {
                eyebrow,
                title,
                body,
                load_state: screen_summary_state_label(read_model.state),
                summary_details: screen_summary_details(
                    read_model,
                    latest_row,
                    &child_disclosure,
                    &product_claim,
                ),
                rows: read_model
                    .rows
                    .iter()
                    .map(|row| screen_summary_panel_row_snapshot(row, &product_claim))
                    .collect(),
                empty_message,
                product_claim,
            }
        }
    }
}

fn unavailable_screen_summary_details(
    product_claim: &str,
) -> Vec<ParentScreenSummaryPanelDetailSnapshot> {
    let child_disclosure = ActivityScreenChildDisclosure::unavailable(
        ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION,
    );
    vec![
        screen_summary_detail("Status", SCREEN_SUMMARY_UNAVAILABLE.to_string()),
        screen_summary_detail(
            "Child disclosure (diagnostic/proposed; not delivered)",
            screen_summary_child_disclosure_state(&child_disclosure),
        ),
        screen_summary_detail(
            "Child disclosure copy (diagnostic/proposed; not delivered)",
            child_disclosure.message().to_string(),
        ),
        screen_summary_detail("Product claim", product_claim.to_string()),
    ]
}

fn screen_summary_details(
    read_model: &ActivityScreenReadModel,
    latest_row: Option<&ActivityScreenReadModelRow>,
    child_disclosure: &ActivityScreenChildDisclosure,
    product_claim: &str,
) -> Vec<ParentScreenSummaryPanelDetailSnapshot> {
    vec![
        screen_summary_detail("Status", screen_summary_state_label(read_model.state)),
        screen_summary_detail("Generated at", read_model.generated_at.clone()),
        screen_summary_detail("Rows returned", read_model.rows.len().to_string()),
        screen_summary_detail(
            "Capability",
            latest_row
                .map(|row| screen_summary_readable_label(&row.capability_status))
                .unwrap_or_else(|| SCREEN_SUMMARY_UNAVAILABLE.to_string()),
        ),
        screen_summary_detail(
            "Custody",
            latest_row
                .map(|row| screen_summary_readable_label(&row.custody_state))
                .unwrap_or_else(|| SCREEN_SUMMARY_UNAVAILABLE.to_string()),
        ),
        screen_summary_detail(
            "Deleted evidence",
            latest_row
                .map(|row| screen_summary_readable_label(&row.image_deletion_state))
                .unwrap_or_else(|| SCREEN_SUMMARY_UNAVAILABLE.to_string()),
        ),
        screen_summary_detail(
            "Model",
            latest_row
                .map(screen_summary_model_summary)
                .unwrap_or_else(|| SCREEN_SUMMARY_NOT_REPORTED.to_string()),
        ),
        screen_summary_detail(
            "Child disclosure (diagnostic/proposed; not delivered)",
            screen_summary_child_disclosure_state(child_disclosure),
        ),
        screen_summary_detail(
            "Child disclosure copy (diagnostic/proposed; not delivered)",
            child_disclosure.message().to_string(),
        ),
        screen_summary_detail(
            "Child surface requirement",
            screen_summary_yes_no(child_disclosure.child_surface_required()),
        ),
        screen_summary_detail(
            "Hidden capture claim",
            screen_summary_yes_no(child_disclosure.hidden_capture_claimed()),
        ),
        screen_summary_detail(
            "Raw screenshot representation",
            screen_summary_yes_no(child_disclosure.raw_screenshot_shown()),
        ),
        screen_summary_detail(
            "Child-agent delivery",
            if child_disclosure.child_agent_delivery_claimed() {
                "Claimed".to_string()
            } else {
                "Not claimed".to_string()
            },
        ),
        screen_summary_detail("Product claim", product_claim.to_string()),
    ]
}

fn screen_summary_child_disclosure_state(disclosure: &ActivityScreenChildDisclosure) -> String {
    screen_summary_readable_label(&serialized_enum_label(&disclosure.state()))
}

fn screen_summary_panel_row_snapshot(
    row: &ActivityScreenReadModelRow,
    product_claim: &str,
) -> ParentScreenSummaryPanelRowSnapshot {
    ParentScreenSummaryPanelRowSnapshot {
        title: row.label.clone(),
        details: vec![
            screen_summary_detail("Status", screen_summary_state_label(row.state)),
            screen_summary_detail("Event ID", row.row_id.clone()),
            screen_summary_detail("Source", row.capture_reason.clone()),
            screen_summary_detail(
                "Capability",
                screen_summary_readable_label(&row.capability_status),
            ),
            screen_summary_detail("Runtime reference", row.model_runtime_ref.clone()),
            screen_summary_detail("Model", screen_summary_model_summary(row)),
            screen_summary_detail("Provider", row.provider_kind.clone()),
            screen_summary_detail("Level", row.confidence.to_string()),
            screen_summary_detail(
                "Activity kind",
                row.primary_category
                    .clone()
                    .unwrap_or_else(|| SCREEN_SUMMARY_NOT_REPORTED.to_string()),
            ),
            screen_summary_detail("Custody", screen_summary_readable_label(&row.custody_state)),
            screen_summary_detail(
                "Deleted evidence",
                screen_summary_readable_label(&row.image_deletion_state),
            ),
            screen_summary_detail(
                "Policy check",
                row.policy_decision_ref
                    .clone()
                    .unwrap_or_else(|| SCREEN_SUMMARY_NOT_REPORTED.to_string()),
            ),
            screen_summary_detail(
                "Decision action",
                row.policy_action
                    .clone()
                    .unwrap_or_else(|| SCREEN_SUMMARY_NOT_REPORTED.to_string()),
            ),
            screen_summary_detail("Enforcement handoff", "Not claimed".to_string()),
            screen_summary_detail(
                "Evidence references",
                screen_summary_evidence_references(row),
            ),
            screen_summary_detail(
                "Reason codes",
                screen_summary_reference_list(&row.policy_reason_codes),
            ),
            screen_summary_detail(
                "Parent rule context references",
                screen_summary_reference_list(&row.parent_rule_refs),
            ),
            screen_summary_detail(
                "Reason",
                screen_summary_reference_list(&row.explanation_reasons),
            ),
            screen_summary_detail(
                "OCR snippets",
                screen_summary_reference_list(&row.ocr_text_snippets),
            ),
            screen_summary_detail(
                "Redaction notes",
                screen_summary_reference_list(&row.redaction_notes),
            ),
            screen_summary_detail(
                "Parent explanation refs",
                screen_summary_reference_list(&row.parent_explanation_refs),
            ),
            screen_summary_detail("Product claim", product_claim.to_string()),
        ],
    }
}

fn screen_summary_state_label(state: ActivityReadModelState) -> String {
    screen_summary_readable_label(&serialized_enum_label(&state))
}

fn screen_summary_model_summary(row: &ActivityScreenReadModelRow) -> String {
    [
        row.model_id.as_str(),
        row.prompt_or_template_version.as_str(),
        row.queue_job_id.as_str(),
    ]
    .join(SCREEN_SUMMARY_DETAIL_SEPARATOR)
}

fn screen_summary_evidence_references(row: &ActivityScreenReadModelRow) -> String {
    let references = row
        .evidence
        .iter()
        .map(|evidence| evidence.evidence_id.clone())
        .collect::<Vec<_>>();
    screen_summary_reference_list(&references)
}

fn screen_summary_reference_list(references: &[String]) -> String {
    let unique_references = references.iter().fold(Vec::new(), |mut unique, reference| {
        if !reference.trim().is_empty() && !unique.iter().any(|value| value == reference) {
            unique.push(reference.clone());
        }
        unique
    });
    if unique_references.is_empty() {
        SCREEN_SUMMARY_NOT_REPORTED.to_string()
    } else {
        unique_references.join(SCREEN_SUMMARY_DETAIL_SEPARATOR)
    }
}

fn screen_summary_readable_label(value: &str) -> String {
    if value.trim().is_empty() {
        return SCREEN_SUMMARY_NOT_REPORTED.to_string();
    }
    value
        .split('-')
        .map(screen_summary_title_case_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn screen_summary_title_case_word(word: &str) -> String {
    let mut chars = word.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    let mut title = first.to_uppercase().collect::<String>();
    title.push_str(chars.as_str());
    title
}

fn screen_summary_detail(label: &str, value: String) -> ParentScreenSummaryPanelDetailSnapshot {
    ParentScreenSummaryPanelDetailSnapshot {
        label: label.to_string(),
        value,
    }
}

fn screen_summary_yes_no(value: bool) -> String {
    if value {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}
