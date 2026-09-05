use ocentra_network_core::network_control_catalog::{
    capability_state::capability_state_for,
    card_kind::card_kind_for,
    control_kind::control_kind_for,
    effect_status::effect_status_for,
    fallback::fallback_for,
    layout_hints::layout_hints_for,
    policy::policy_lane_for,
    proof_requirement::proof_requirement_for,
    question::{explicit_option_labels, question_from_source_text},
    requirements::{capability_requirement_for, helper_text_for},
    runtime_owner::runtime_owner_for,
    selection_mode::selection_mode_for,
    source_state::capability_state_from_source_state,
    tokens::{slug_token, title_from_token},
    NetworkControlCapabilityState, NetworkControlCardKind, NetworkControlEffectStatus,
    NetworkControlKind, NetworkControlRuntimeOwner, NetworkControlSelectionMode,
};

#[test]
fn policy_lane_and_runtime_owner_map_rust_first_network_roles() {
    assert_eq!(
        policy_lane_for("Storage", "Retention", "Export audit and cache policy."),
        "audit"
    );
    assert_eq!(
        policy_lane_for(
            "Reports",
            "Visible fields",
            "Retention summary visible to parent sees top history."
        ),
        "reports"
    );
    assert_eq!(
        policy_lane_for("Budget", "Time window", "Network-active time budget."),
        "schedule"
    );
    assert_eq!(
        policy_lane_for("Strict actions", "Adapter", "Firewall rollback router."),
        "enforcement"
    );
    assert_eq!(
        policy_lane_for("Signals", "Evidence", "Flow metadata and DNS attribution."),
        "evidence"
    );
    assert_eq!(
        policy_lane_for("Setup", "Provisioning", "Managed profile admin permission."),
        "setup"
    );
    assert_eq!(
        runtime_owner_for("Rules", "Compiler", "Policy conflict fallback proof."),
        NetworkControlRuntimeOwner::RustParentRuntime
    );
    assert_eq!(
        runtime_owner_for(
            "Adapters",
            "Firewall",
            "DNS adapter through ETW and IP Helper."
        ),
        NetworkControlRuntimeOwner::OsAdapter
    );
    assert_eq!(
        runtime_owner_for(
            "Reports",
            "Visible fields",
            "Portal UI shows parent visible summary."
        ),
        NetworkControlRuntimeOwner::PortalOnly
    );
    assert_eq!(
        runtime_owner_for("Retention", "Custody", "Local-first storage export audit."),
        NetworkControlRuntimeOwner::ParentOwnedStorage
    );
}

#[test]
fn control_kind_selection_card_and_layout_cover_catalog_shape_rules() {
    assert_eq!(
        control_kind_for("Enable this control?", Some("boolean")),
        NetworkControlKind::Toggle
    );
    assert_eq!(
        control_kind_for(
            "Capability matrix row | Capability=Router | State=Ready",
            None
        ),
        NetworkControlKind::ReadOnlyStatus
    );
    assert_eq!(
        control_kind_for("Bandwidth budget minutes", None),
        NetworkControlKind::Number
    );
    assert_eq!(
        control_kind_for("Retention days", None),
        NetworkControlKind::Retention
    );
    assert_eq!(
        control_kind_for("Allow or block actions", None),
        NetworkControlKind::ActionList
    );
    assert_eq!(
        control_kind_for("Target domains: gaming, streaming or chat", None),
        NetworkControlKind::MultiChoice
    );

    let selection = selection_mode_for(NetworkControlKind::ActionList, 6);
    assert_eq!(selection, NetworkControlSelectionMode::Multi);
    assert_eq!(
        card_kind_for(NetworkControlKind::ActionList, selection, 6),
        NetworkControlCardKind::MultiChoiceMany
    );

    let status_hints = layout_hints_for(
        NetworkControlKind::ReadOnlyStatus,
        NetworkControlSelectionMode::Single,
        2,
    );
    assert_eq!(status_hints.preferred_column_span, 2);
    assert!(status_hints.collapsible);
    assert!(!status_hints.searchable_options);
    assert_eq!(status_hints.option_group_count, 1);

    let multi_hints = layout_hints_for(
        NetworkControlKind::ActionList,
        NetworkControlSelectionMode::Multi,
        8,
    );
    assert_eq!(multi_hints.preferred_column_span, 2);
    assert!(multi_hints.collapsible);
    assert!(multi_hints.searchable_options);
    assert_eq!(multi_hints.option_group_count, 2);
    assert!(multi_hints.show_as_matrix_when_large);
    assert!(multi_hints.show_selected_count);
}

#[test]
fn effect_and_capability_helpers_keep_proof_states_explicit() {
    assert_eq!(
        effect_status_for(
            "Strict actions",
            "Firewall",
            "Exact URL strict enforcement requires managed browser proof."
        ),
        NetworkControlEffectStatus::ProofRequired
    );
    assert_eq!(
        effect_status_for(
            "Setup",
            "Provisioning",
            "Driver installation and admin privilege are manual required."
        ),
        NetworkControlEffectStatus::ManualRequired
    );
    assert_eq!(
        effect_status_for(
            "Permissions",
            "Protected",
            "TCC review and user setup permission flow."
        ),
        NetworkControlEffectStatus::PermissionRequired
    );
    assert_eq!(
        effect_status_for(
            "Storage",
            "Reports",
            "Retention summary and redact local-first report."
        ),
        NetworkControlEffectStatus::AlreadyRepresented
    );
    assert_eq!(
        effect_status_for("Gaps", "Planning", "Future gap not yet implemented."),
        NetworkControlEffectStatus::FutureGap
    );

    assert_eq!(
        capability_state_for(NetworkControlEffectStatus::ProofRequired),
        NetworkControlCapabilityState::Protected
    );
    assert_eq!(
        capability_state_for(NetworkControlEffectStatus::ManualRequired),
        NetworkControlCapabilityState::ManualRequired
    );
    assert_eq!(
        capability_state_for(NetworkControlEffectStatus::PermissionRequired),
        NetworkControlCapabilityState::PermissionRequired
    );
    assert_eq!(
        capability_state_for(NetworkControlEffectStatus::NeedsEffectWiring),
        NetworkControlCapabilityState::Degraded
    );
    assert_eq!(
        capability_state_from_source_state("authoring-only"),
        NetworkControlCapabilityState::Disabled
    );
    assert_eq!(
        capability_state_from_source_state("ready-if-browser-capability-ready"),
        NetworkControlCapabilityState::Protected
    );
    assert_eq!(
        capability_state_from_source_state("unknown-state"),
        NetworkControlCapabilityState::Degraded
    );
}

#[test]
fn requirements_fallbacks_and_helper_text_preserve_network_non_claims() {
    assert_eq!(
        capability_requirement_for(
            "Evidence",
            "DNS",
            "Domain resolver confidence and DNS evidence."
        ),
        "dns-or-domain-attribution-source-with-confidence"
    );
    assert_eq!(
        capability_requirement_for(
            "Strict actions",
            "Firewall",
            "Block traffic through packet filter enforcement."
        ),
        "real-platform-network-adapter-proof"
    );
    assert_eq!(
        capability_requirement_for("Signals", "Flow", "IP port protocol flow metadata."),
        "local-network-flow-metadata-evidence"
    );
    assert_eq!(
        proof_requirement_for(
            "Evidence",
            "URLs",
            "Exact URL and page title proof requirement."
        ),
        Some(
            "Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough."
        )
    );
    assert_eq!(
        proof_requirement_for(
            "Strict actions",
            "Firewall",
            "Strict firewall rollback and always-on lockdown."
        ),
        Some(
            "Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state."
        )
    );
    assert_eq!(
        helper_text_for("Signals", "Flow", "IP port protocol flow metadata."),
        "Network claims require stored metadata evidence with source, freshness, confidence, and custody labels."
    );
    assert_eq!(
        fallback_for(
            NetworkControlEffectStatus::ManualRequired,
            "Router adapter requires manual setup."
        ),
        "Show manual-required until setup, privileges, and adapter proof exist; compile observe or unavailable fallback."
    );
    assert_eq!(
        fallback_for(
            NetworkControlEffectStatus::ProofRequired,
            "Exact URL strict action."
        ),
        "Hide or disable exact URL controls unless managed browser, explicit URL filter, or proxy proof exists."
    );
}

#[test]
fn question_option_and_slug_helpers_normalize_catalog_source_text() {
    assert_eq!(
        question_from_source_text("Parent review actions?", None),
        "Parent review actions?"
    );
    assert_eq!(
        question_from_source_text("Runtime owner: Rust parent runtime.", None),
        "Configure runtime owner."
    );
    assert_eq!(
        question_from_source_text(
            "Capability matrix row | Capability=Router protection | Status=Ready",
            None
        ),
        "Represent Router protection capability status."
    );
    assert_eq!(
        question_from_source_text("Represent fallback state.", Some("Keep custom question.")),
        "Keep custom question."
    );
    assert_eq!(
        explicit_option_labels("Target domains: gaming, streaming or chat"),
        vec![
            "Gaming".to_owned(),
            "Streaming".to_owned(),
            "Chat".to_owned()
        ]
    );
    assert_eq!(
        explicit_option_labels("Capability matrix row | Capability=Router | Status=Ready"),
        vec!["Capability: Router".to_owned(), "Status: Ready".to_owned()]
    );
    assert_eq!(slug_token("Strict Action / Router"), "strict-action-router");
    assert_eq!(slug_token("###"), "item");
    assert_eq!(
        title_from_token("strict-action-router"),
        "Strict Action Router"
    );
}
