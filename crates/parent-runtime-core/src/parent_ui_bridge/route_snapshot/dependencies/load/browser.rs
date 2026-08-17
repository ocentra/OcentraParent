use crate::agent_service_client::types::{
    BrowserActivityReadModelAgentServiceSnapshot, BrowserEvidenceReadModelAgentServiceSnapshot,
    BrowserInterventionReadModelAgentServiceSnapshot,
    BrowserInventoryReadModelAgentServiceSnapshot, BrowserManagedStatusAgentServiceSnapshot,
};
use crate::agent_service_client::{
    load_browser_activity_read_model_snapshot, load_browser_evidence_read_model_snapshot,
    load_browser_intervention_read_model_snapshot, load_browser_inventory_read_model_snapshot,
    load_browser_managed_status_snapshot,
};
use crate::parent_ui_bridge::route_requirements::{
    route_requires_browser_activity_read_model, route_requires_browser_evidence_read_model,
    route_requires_browser_inventory_read_model, route_requires_browser_read_models,
};
use crate::parent_ui_bridge::route_snapshot::dependencies::DependencyFailures;
use crate::parent_ui_bridge::ParentRouteId;

pub(super) struct BrowserStatusDependencies {
    pub(super) managed_status_snapshot: Option<BrowserManagedStatusAgentServiceSnapshot>,
    pub(super) intervention_read_model_snapshot:
        Option<BrowserInterventionReadModelAgentServiceSnapshot>,
}

pub(super) fn load_activity(
    route: &ParentRouteId,
    failures: &mut DependencyFailures,
) -> Option<BrowserActivityReadModelAgentServiceSnapshot> {
    if route_requires_browser_activity_read_model(route) {
        failures.capture(
            "browser-activity-read-model",
            load_browser_activity_read_model_snapshot(None),
        )
    } else {
        None
    }
}

pub(super) fn load_inventory(
    route: &ParentRouteId,
    failures: &mut DependencyFailures,
) -> Option<BrowserInventoryReadModelAgentServiceSnapshot> {
    if route_requires_browser_inventory_read_model(route) {
        failures.capture(
            "browser-inventory-read-model",
            load_browser_inventory_read_model_snapshot(None),
        )
    } else {
        None
    }
}

pub(super) fn load_evidence(
    route: &ParentRouteId,
    failures: &mut DependencyFailures,
) -> Option<BrowserEvidenceReadModelAgentServiceSnapshot> {
    if route_requires_browser_evidence_read_model(route) {
        failures.capture(
            "browser-evidence-read-model",
            load_browser_evidence_read_model_snapshot(None),
        )
    } else {
        None
    }
}

pub(super) fn load_status(
    route: &ParentRouteId,
    failures: &mut DependencyFailures,
) -> BrowserStatusDependencies {
    let browser_required = route_requires_browser_read_models(route);
    let managed_status_snapshot = if browser_required {
        failures.capture(
            "browser-managed-status",
            load_browser_managed_status_snapshot(None),
        )
    } else {
        None
    };
    let intervention_read_model_snapshot = if browser_required {
        failures.capture(
            "browser-intervention-read-model",
            load_browser_intervention_read_model_snapshot(None),
        )
    } else {
        None
    };
    BrowserStatusDependencies {
        managed_status_snapshot,
        intervention_read_model_snapshot,
    }
}
