use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::activity_surface::{
    ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::ACTIVITY_SURFACE_SCHEMA_VERSION;

pub(crate) const TEST_TIMESTAMP: &str = "2026-07-13T00:00:00.000Z";
pub(crate) const TEST_FIRST_OBSERVED_AT: &str = "2026-07-13T00:01:00.000Z";
pub(crate) const TEST_SECOND_OBSERVED_AT: &str = "2026-07-13T00:02:00.000Z";
pub(crate) const TEST_THIRD_OBSERVED_AT: &str = "2026-07-13T00:03:00.000Z";

pub(crate) fn family_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Family,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: None,
        },
        requested_at: TEST_TIMESTAMP.to_string(),
        range_start: TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: TEST_THIRD_OBSERVED_AT.to_string(),
    }
}

pub(crate) fn remote_device_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Device,
            family_id: Some(constants::activity_surface::DEFAULT_FAMILY_ID.to_string()),
            device_id: Some("remote-device-7".to_string()),
        },
        requested_at: TEST_TIMESTAMP.to_string(),
        range_start: TEST_FIRST_OBSERVED_AT.to_string(),
        range_end: TEST_THIRD_OBSERVED_AT.to_string(),
    }
}

pub(crate) fn evidence_ref(evidence_id: &str, digest: Option<&str>) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::LocalDbRow,
        digest: digest.map(ToString::to_string),
        uri: None,
    }
}
