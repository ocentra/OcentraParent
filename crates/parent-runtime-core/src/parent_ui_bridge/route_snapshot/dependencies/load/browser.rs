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
use crate::parent_ui_bridge::ParentRouteId;

pub(super) struct BrowserStatusDependencies {
    pub(super) managed_status_snapshot: Option<BrowserManagedStatusAgentServiceSnapshot>,
    pub(super) intervention_read_model_snapshot:
        Option<BrowserInterventionReadModelAgentServiceSnapshot>,
}

pub(super) fn load_activity(
    route: &ParentRouteId,
) -> Option<BrowserActivityReadModelAgentServiceSnapshot> {
    if route_requires_browser_activity_read_model(route) {
        load_browser_activity_read_model_snapshot(None).ok()
    } else {
        None
    }
}

pub(super) fn load_inventory(
    route: &ParentRouteId,
) -> Option<BrowserInventoryReadModelAgentServiceSnapshot> {
    if route_requires_browser_inventory_read_model(route) {
        load_browser_inventory_read_model_snapshot(None).ok()
    } else {
        None
    }
}

pub(super) fn load_evidence(
    route: &ParentRouteId,
) -> Option<BrowserEvidenceReadModelAgentServiceSnapshot> {
    if route_requires_browser_evidence_read_model(route) {
        load_browser_evidence_read_model_snapshot(None).ok()
    } else {
        None
    }
}

pub(super) fn load_status(route: &ParentRouteId) -> BrowserStatusDependencies {
    let browser_required = route_requires_browser_read_models(route);
    let managed_status_snapshot = if browser_required {
        load_browser_managed_status_snapshot(None).ok()
    } else {
        None
    };
    let intervention_read_model_snapshot = if browser_required {
        load_browser_intervention_read_model_snapshot(None).ok()
    } else {
        None
    };
    BrowserStatusDependencies {
        managed_status_snapshot,
        intervention_read_model_snapshot,
    }
}
