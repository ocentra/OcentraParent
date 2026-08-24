use super::*;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    AppGameLinuxDockerHostPreflight, AppGamePlatformProofStatusRow,
};

const DOCKER_PREFLIGHT_LABEL: &str = "Docker preflight";
const DOCKER_CLI_LABEL: &str = "Docker CLI";
const DOCKER_DAEMON_LABEL: &str = "Docker daemon";
const DOCKER_CONTEXTS_LABEL: &str = "Docker contexts";
const DOCKER_IMAGES_LABEL: &str = "Docker images";
const DOCKER_CONTAINERS_LABEL: &str = "Docker containers";
const DOCKER_IDENTIFIERS_REDACTED_LABEL: &str = "Docker identifiers redacted";
const DOCKER_PROOF_REFS_LABEL: &str = "Docker proof refs";
const DOCKER_OPEN_GAPS_LABEL: &str = "Docker open gaps";
const INVENTORY_UNAVAILABLE_LABEL: &str = "unavailable";

pub(super) fn app_game_platform_proof_status_panel_snapshot(
    read_model: Option<&AppGamePlatformProofStatusReadModel>,
) -> ParentAppGamePanelSnapshot {
    let product_claim = "Windows, Android, Linux, macOS, and iOS platform proof rows are parent-visible evidence only. Native enforcement, broad blocking, rollback, audit, Apple CI artifacts, and child delivery remain unclaimed until platform authority proof is attached.".to_string();
    match read_model {
        None => app_game_panel_unavailable(
            "Runtime reference",
            "App/game platform proof status",
            "Parent-safe platform proof status for Windows, Android, Linux, macOS, and iOS evidence and CI-required proof rows.",
            "No app/game platform proof status has been reported yet.",
            product_claim.as_str(),
        ),
        Some(read_model) => ParentAppGamePanelSnapshot {
            eyebrow: "Runtime reference".to_string(),
            title: "App/game platform proof status".to_string(),
            body: "Parent-safe platform proof status for Windows, Android, Linux, macOS, and iOS evidence and CI-required proof rows.".to_string(),
            load_state: if read_model.enforcement_ready_count > 0 {
                "ready".to_string()
            } else {
                "warn".to_string()
            },
            summary_details: app_game_platform_summary_details(read_model, &product_claim),
            rows: app_game_platform_rows(read_model, &product_claim),
            empty_message: "No app/game platform proof rows were returned.".to_string(),
            product_claim,
        },
    }
}

fn app_game_platform_summary_details(
    read_model: &AppGamePlatformProofStatusReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    let status = if read_model.enforcement_ready_count > 0 {
        "ready"
    } else {
        "warn"
    };
    vec![
        app_game_detail("Status", status),
        app_game_detail("Generated at", read_model.generated_at.as_str()),
        app_game_detail("Platform proofs", read_model.returned.to_string()),
        app_game_detail(
            "Host-visible rows",
            read_model.host_visible_count.to_string(),
        ),
        app_game_detail(
            "Host not-detected rows",
            read_model.host_not_detected_count.to_string(),
        ),
        app_game_detail(
            "Not-applicable rows",
            read_model.local_runtime_not_applicable_count.to_string(),
        ),
        app_game_detail(
            "Enforcement-ready rows",
            read_model.enforcement_ready_count.to_string(),
        ),
        app_game_detail("Open gaps", read_model.open_gap_count.to_string()),
        app_game_detail("Product claim", product_claim),
    ]
}

fn app_game_platform_rows(
    read_model: &AppGamePlatformProofStatusReadModel,
    product_claim: &str,
) -> Vec<ParentAppGamePanelRowSnapshot> {
    read_model
        .rows
        .iter()
        .map(|row| app_game_platform_row_snapshot(row, product_claim))
        .collect()
}

fn app_game_platform_row_snapshot(
    row: &AppGamePlatformProofStatusRow,
    product_claim: &str,
) -> ParentAppGamePanelRowSnapshot {
    let mut details = vec![
        app_game_detail("Platform", row.platform.as_str()),
        app_game_detail("Status", row.proof_state.as_str()),
        app_game_detail("Authority state", row.authority_state.as_str()),
        app_game_detail("Host capability", row.host_capability_state.as_str()),
        app_game_detail(
            "Host capability evidence",
            app_game_join_strings(&row.host_capability_evidence_refs),
        ),
        app_game_detail(
            "Host capability probes",
            app_game_join_strings(&row.host_capability_probe_refs),
        ),
    ];
    if let Some(preflight) = row.linux_docker_host_preflight.as_ref() {
        details.extend(linux_docker_host_preflight_details(preflight));
    }
    details.extend([
        app_game_detail(
            "Evidence references",
            app_game_join_strings(&row.proof_refs),
        ),
        app_game_detail("Open gaps", app_game_join_strings(&row.open_gaps)),
        app_game_detail(
            "Adapter dispatch",
            app_game_claimed_value(row.adapter_dispatch_claimed),
        ),
        app_game_detail(
            "Broad blocking",
            app_game_claimed_value(row.broad_installed_app_blocking_claimed),
        ),
        app_game_detail(
            "Platform enforcement",
            app_game_claimed_value(row.platform_enforcement_claimed),
        ),
        app_game_detail(
            "Provider delivery",
            app_game_claimed_value(row.provider_delivery_claimed),
        ),
        app_game_detail(
            "Child delivery",
            app_game_claimed_value(row.child_device_delivery_claimed),
        ),
        app_game_detail(
            "Private diagnostics",
            app_game_claimed_value(row.private_diagnostics_claimed),
        ),
        app_game_detail("Product claim", product_claim),
    ]);
    app_game_panel_row(row.platform.clone(), details)
}

fn linux_docker_host_preflight_details(
    preflight: &AppGameLinuxDockerHostPreflight,
) -> Vec<ParentAppGamePanelDetailSnapshot> {
    vec![
        app_game_detail(DOCKER_PREFLIGHT_LABEL, preflight.state.as_str()),
        app_game_detail(
            DOCKER_CLI_LABEL,
            app_game_ready_warn_value(preflight.cli_visible),
        ),
        app_game_detail(
            DOCKER_DAEMON_LABEL,
            app_game_ready_warn_value(preflight.daemon_visible),
        ),
        app_game_detail(
            DOCKER_CONTEXTS_LABEL,
            inventory_count(preflight.context_inventory_visible, preflight.context_count),
        ),
        app_game_detail(
            DOCKER_IMAGES_LABEL,
            inventory_count(preflight.image_inventory_visible, preflight.image_count),
        ),
        app_game_detail(
            DOCKER_CONTAINERS_LABEL,
            inventory_count(
                preflight.container_inventory_visible,
                preflight.container_count,
            ),
        ),
        app_game_detail(
            DOCKER_IDENTIFIERS_REDACTED_LABEL,
            app_game_ready_warn_value(preflight.identifiers_redacted),
        ),
        app_game_detail(
            DOCKER_PROOF_REFS_LABEL,
            app_game_join_strings(&preflight.proof_refs),
        ),
        app_game_detail(
            DOCKER_OPEN_GAPS_LABEL,
            app_game_join_strings(&preflight.open_gaps),
        ),
    ]
}

fn inventory_count(visible: bool, count: u64) -> String {
    if visible {
        count.to_string()
    } else {
        INVENTORY_UNAVAILABLE_LABEL.to_string()
    }
}
