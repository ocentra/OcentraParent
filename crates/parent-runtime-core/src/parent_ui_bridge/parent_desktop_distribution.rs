use ocentra_parent_agent_protocol::constants;
use ocentra_schema::parent_ui_bridge::{ParentDesktopDistributionSnapshot, ParentRouteId};

pub(super) fn parent_desktop_distribution_snapshot_for_route(
    route: &ParentRouteId,
) -> Option<ParentDesktopDistributionSnapshot> {
    matches!(
        route,
        ParentRouteId::PlatformsInstall | ParentRouteId::InstallUpdates
    )
    .then(parent_desktop_distribution_snapshot)
}

fn parent_desktop_distribution_snapshot() -> ParentDesktopDistributionSnapshot {
    ParentDesktopDistributionSnapshot {
        payload_source:
            constants::value::PARENT_DESKTOP_DISTRIBUTION_PAYLOAD_SOURCE_RUST_PARENT_RUNTIME
                .to_string(),
        source_custody_state: constants::value::PARENT_DESKTOP_SOURCE_CUSTODY_MANUAL_REQUIRED
            .to_string(),
        product_claim_state: constants::value::PARENT_DESKTOP_DISTRIBUTION_PRODUCT_CLAIM_READ_ONLY
            .to_string(),
        no_claim: constants::value::PARENT_DESKTOP_DISTRIBUTION_NO_EXECUTION_CLAIM.to_string(),
        package_frontend_state: constants::value::PARENT_DESKTOP_FRONTEND_BUILT_PORTAL_DIST
            .to_string(),
        package_service_manager_state: constants::value::PARENT_DESKTOP_PACKAGE_SERVICE_AUTO_START
            .to_string(),
        package_health_probe_state: constants::value::PARENT_DESKTOP_PACKAGE_HEALTH_PROBE_REQUIRED
            .to_string(),
        package_preview_state: constants::value::PARENT_DESKTOP_PACKAGE_PREVIEW_UNSIGNED
            .to_string(),
        update_channel_state: constants::value::PARENT_DESKTOP_UPDATE_CHANNEL_SCAFFOLD.to_string(),
        rollback_state: constants::value::PARENT_DESKTOP_ROLLBACK_UNAVAILABLE.to_string(),
        signing_state: constants::value::PARENT_DESKTOP_SIGNING_MANUAL_REQUIRED.to_string(),
        notarization_state: constants::value::PARENT_DESKTOP_NOTARIZATION_MANUAL_REQUIRED
            .to_string(),
        store_distribution_state:
            constants::value::PARENT_DESKTOP_STORE_DISTRIBUTION_MANUAL_REQUIRED.to_string(),
        platform_matrix_state: constants::value::PARENT_DESKTOP_PLATFORM_MATRIX_SPLIT_PROOF_ROWS
            .to_string(),
        release_branch_state:
            constants::value::PARENT_DESKTOP_RELEASE_BRANCH_PRODUCTION_PROMOTION_REQUIRED
                .to_string(),
        artifact_proof_state: constants::value::PARENT_DESKTOP_ARTIFACT_PROOF_CI_PREVIEW
            .to_string(),
        actions_available: false,
    }
}
