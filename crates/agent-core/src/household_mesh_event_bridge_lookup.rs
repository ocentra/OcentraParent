use crate::household_mesh_event_bridge::HouseholdMeshLocalEventKind;

#[path = "household_mesh_event_bridge_lookup_tables.rs"]
mod household_mesh_event_bridge_lookup_tables;

pub(crate) fn bridge_local_event_kind_for_local_event(
    event_type: &str,
) -> Option<HouseholdMeshLocalEventKind> {
    household_mesh_event_bridge_lookup_tables::bridge_local_event_kind_for_local_event(event_type)
}

pub(crate) fn local_event_ref(event_kind: HouseholdMeshLocalEventKind) -> Option<&'static str> {
    household_mesh_event_bridge_lookup_tables::local_event_ref(event_kind)
}

pub(crate) fn lan_message_type_for_ref(local_event_ref: &str) -> Option<&'static str> {
    household_mesh_event_bridge_lookup_tables::lan_message_type_for_ref(local_event_ref)
}
