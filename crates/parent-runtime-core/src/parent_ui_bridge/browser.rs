use super::*;

pub(super) fn browser_route_panels_snapshot(
    route: &ParentRouteId,
) -> Option<ParentRouteBrowserPanelsSnapshot> {
    if !matches!(route, ParentRouteId::ProofPanels) {
        return None;
    }

    Some(ParentRouteBrowserPanelsSnapshot {
        browser_parent_explanation: Some(browser_contract_only_panel_snapshot(
            "Browser parent explanations",
            "Schema-backed parent explanations show evidence, model, policy, action, child experience, fallback, and audit sections only when a validated browser AI explanation bundle is present.",
            "0 parent explanation rows",
            "No browser parent explanation bundle has been reported yet.",
            "Rendered parent explanation surface only; runtime service delivery, final policy authority, browser mutation, enforcement, remote AI, and raw page or prompt content remain unclaimed.",
        )),
        social_audit_explanation: Some(browser_contract_only_panel_snapshot(
            "Social explanations",
            "Schema-backed social explanations show parent-visible evidence, policy, approval, memory, connector, native, manual, and audit refs without raw social content.",
            "0 social explanation rows",
            "No social audit explanation snapshot has been reported yet.",
            "Rendered parent explanation surface only; runtime audit-store delivery, notifications, connector authorization, native app control, final policy execution, and enforcement remain unclaimed.",
        )),
        social_alert_report: Some(browser_contract_only_panel_snapshot(
            "Social alerts and reports",
            "Schema-backed social alert and report intents show ref-only local outbox or manual-required rows without provider delivery or enforcement claims.",
            "0 social alert/report rows",
            "No social alert/report read model has been reported yet.",
            "Rendered parent alert/report intent surface only; provider delivery, report delivery, notification UI delivery, final policy execution, and enforcement remain unclaimed.",
        )),
        social_alert_report_parent_surface: Some(browser_contract_only_panel_snapshot(
            "Social parent surface status",
            "Service-backed parent-surface status shows provider and preference handoff state without rendering notification, preference, history, or delivery UI.",
            "0 parent surface rows",
            "No parent-surface status snapshot has been reported yet.",
            "Parent-surface status projection only; notification UI delivery, provider delivery, receipt ingestion, final policy execution, and enforcement remain unclaimed.",
        )),
        social_parent_notification_delivery: Some(browser_contract_only_panel_snapshot(
            "Social parent notification delivery readiness",
            "Service-backed readiness projection shows parent-owned report status and manual gaps without claiming notification UI delivery, provider delivery, final policy execution, or enforcement.",
            "0 parent notification readiness rows",
            "No parent notification delivery readiness snapshot has been reported yet.",
            "Parent report readiness projection only; parent notification UI delivery, external runtime report delivery, provider delivery, final policy execution, and enforcement remain unclaimed.",
        )),
        social_dashboard: Some(browser_contract_only_panel_snapshot(
            "Social dashboard",
            "Schema-backed social rows show parent-review and manual-required status only; runtime fetch, connector, native app, policy execution, and enforcement remain unclaimed.",
            "0 social dashboard rows",
            "No social dashboard snapshot has been reported yet.",
            "Rendered parent surface only; social runtime data fetch, notifications, connector authorization, native app control, policy execution, and enforcement remain unclaimed.",
        )),
        browser_action_intent_stream_status: Some(browser_status_panel_snapshot(
            "Browser action-intent stream status",
            "Rust-owned route snapshot of browser action-intent stream status for the Browser route.",
            "0 action candidates",
            "No browser action-intent stream status has been reported yet.",
            "Browser runtime action-intent stream status only; local outbox handoff, child accepted refs, and parent read-model refs may be visible, while adapter dispatch, child intervention execution, browser mutation, final policy execution, unmanaged exact URL support, and enforcement remain unclaimed.",
        )),
        browser_social_provider_receipt_stream_status: Some(browser_status_panel_snapshot(
            "Social provider receipt stream status",
            "Rust-owned route snapshot of social provider receipt stream status for the Browser route.",
            "0 receipt boundary rows",
            "No social provider receipt stream status has been reported yet.",
            "Browser runtime social provider receipt stream status only; provider delivery, receipt ingestion, parent notification delivery, report delivery, final policy execution, connector/native runtime, and enforcement remain unclaimed.",
        )),
        browser_social_provider_receipt_ingestion_readiness_status: Some(
            browser_status_panel_snapshot(
                "Social provider receipt ingestion readiness",
                "Rust-owned route snapshot of receipt ingestion readiness for the Browser route.",
                "0 readiness rows",
                "No social provider receipt ingestion readiness status has been reported yet.",
                "Browser runtime social provider receipt ingestion readiness status only; provider delivery, receipt ingestion runtime, webhook runtime, credentials, observed provider receipts, parent notification delivery, report delivery, final policy execution, connector/native runtime, browser mutation, child intervention, unmanaged exact URL support, and enforcement remain unclaimed.",
            ),
        ),
    })
}

fn browser_contract_only_panel_snapshot(
    title: &str,
    body: &str,
    summary: &str,
    empty_message: &str,
    product_claim: &str,
) -> ParentBrowserPanelSnapshot {
    ParentBrowserPanelSnapshot {
        eyebrow: BROWSER_PANEL_EYEBROW.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        summary: summary.to_string(),
        summary_details: browser_panel_summary_details(summary, product_claim),
        rows: Vec::new(),
        empty_message: empty_message.to_string(),
        product_claim: product_claim.to_string(),
    }
}

fn browser_status_panel_snapshot(
    title: &str,
    body: &str,
    summary: &str,
    empty_message: &str,
    product_claim: &str,
) -> ParentBrowserPanelSnapshot {
    ParentBrowserPanelSnapshot {
        eyebrow: BROWSER_PANEL_EYEBROW.to_string(),
        title: title.to_string(),
        body: body.to_string(),
        summary: summary.to_string(),
        summary_details: browser_panel_summary_details(summary, product_claim),
        rows: vec![ParentBrowserPanelRowSnapshot {
            key: title.to_string(),
            title: "Status projection".to_string(),
            details: vec![
                browser_panel_detail("Status", "unavailable"),
                browser_panel_detail("Summary", summary),
                browser_panel_detail("Product claim", product_claim),
            ],
        }],
        empty_message: empty_message.to_string(),
        product_claim: product_claim.to_string(),
    }
}

fn browser_panel_summary_details(
    summary: &str,
    product_claim: &str,
) -> Vec<ParentBrowserPanelDetailSnapshot> {
    vec![
        browser_panel_detail("Rows returned", "0"),
        browser_panel_detail("Status", BROWSER_PANEL_NOT_REPORTED),
        browser_panel_detail("Summary", summary),
        browser_panel_detail("Product claim", product_claim),
    ]
}

fn browser_panel_detail(label: &str, value: &str) -> ParentBrowserPanelDetailSnapshot {
    ParentBrowserPanelDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}

const SETUP_FIRST_RUN_PRODUCT_CLAIM: &str = "This panel reports only whether the Start route has a live Rust-owned setup-first-run snapshot. It does not claim live account readiness, signed installer readiness, pairing trust, data-custody execution, or onboarding completion.";

pub(super) fn setup_first_run_panel_snapshot(
    route: &ParentRouteId,
) -> Option<ParentSetupFirstRunPanelSnapshot> {
    if !matches!(route, ParentRouteId::Start) {
        return None;
    }

    Some(ParentSetupFirstRunPanelSnapshot {
        eyebrow: "Setup route".to_string(),
        title: "Setup-first-run boundary status".to_string(),
        body: "The Start route exists, but live setup-first-run runtime state is not yet wired into the Rust parent snapshot. This panel reports that gap honestly instead of inventing onboarding progress.".to_string(),
        summary_card_title: "Current boundary status".to_string(),
        summary: "Portal rendering and the Rust-owned route snapshot exist, but live setup/account/trust/custody state is unavailable here today.".to_string(),
        summary_details: setup_first_run_summary_details(),
        cards: setup_first_run_cards(),
        product_claim: SETUP_FIRST_RUN_PRODUCT_CLAIM.to_string(),
    })
}

fn setup_first_run_summary_details() -> Vec<ParentSetupFirstRunPanelDetailSnapshot> {
    vec![
        setup_first_run_detail("Route", "start"),
        setup_first_run_detail("Runtime state", "unavailable"),
        setup_first_run_detail("Snapshot owner", "Rust parent runtime host bridge"),
        setup_first_run_detail("Product claim", SETUP_FIRST_RUN_PRODUCT_CLAIM),
    ]
}

fn setup_first_run_cards() -> Vec<ParentSetupFirstRunPanelCardSnapshot> {
    vec![
        setup_first_run_current_truth_card(),
        setup_first_run_missing_runtime_card(),
        setup_first_run_ownership_card(),
    ]
}

fn setup_first_run_current_truth_card() -> ParentSetupFirstRunPanelCardSnapshot {
    setup_first_run_card(
        "What is real now",
        "The Start route can render an honest Rust-owned boundary panel without inventing setup progress.",
        vec![
            setup_first_run_detail("Route shell", "Start route is visible in the portal shell"),
            setup_first_run_detail(
                "Snapshot transport",
                "Host bridge snapshot reaches TS presentation",
            ),
            setup_first_run_detail("Evidence boundary", "Route-contract projection only"),
        ],
    )
}

fn setup_first_run_missing_runtime_card() -> ParentSetupFirstRunPanelCardSnapshot {
    setup_first_run_card(
        "What is missing",
        "No live setup-first-run read model is wired here yet, so the panel must stay explicit about the missing runtime state.",
        vec![
            setup_first_run_detail("Account/provider state", "not wired"),
            setup_first_run_detail("Pairing/trust state", "not wired"),
            setup_first_run_detail("Data-custody/readiness state", "not wired"),
            setup_first_run_detail(
                "Completion claim",
                "withheld until a live Rust snapshot exists",
            ),
        ],
    )
}

fn setup_first_run_ownership_card() -> ParentSetupFirstRunPanelCardSnapshot {
    setup_first_run_card(
        "Where it belongs",
        "When first-run becomes live, Rust must own the setup state and TS must remain pure rendering.",
        vec![
            setup_first_run_detail("Rust owner", "parent runtime + setup read model"),
            setup_first_run_detail("TS role", "presentation only"),
            setup_first_run_detail(
                "Proof rule",
                "claim only what the live Rust snapshot can prove",
            ),
        ],
    )
}

fn setup_first_run_card(
    title: &str,
    summary: &str,
    details: Vec<ParentSetupFirstRunPanelDetailSnapshot>,
) -> ParentSetupFirstRunPanelCardSnapshot {
    ParentSetupFirstRunPanelCardSnapshot {
        title: title.to_string(),
        summary: summary.to_string(),
        details,
    }
}

fn setup_first_run_detail(label: &str, value: &str) -> ParentSetupFirstRunPanelDetailSnapshot {
    ParentSetupFirstRunPanelDetailSnapshot {
        label: label.to_string(),
        value: value.to_string(),
    }
}
