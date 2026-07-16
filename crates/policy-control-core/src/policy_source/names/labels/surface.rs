#![forbid(unsafe_code)]

use crate::policy_source::PolicySourceSurface;
use ocentra_parent_agent_protocol::constants::policy_control;

pub(super) fn policy_surface_name(surface: PolicySourceSurface) -> &'static str {
    match surface {
        PolicySourceSurface::ParentPortal => policy_control::source::SURFACE_PARENT_PORTAL,
        PolicySourceSurface::ParentCompanion => policy_control::source::SURFACE_PARENT_COMPANION,
        PolicySourceSurface::AiPreview => policy_control::source::SURFACE_AI_PREVIEW,
        PolicySourceSurface::DomainCache => policy_control::source::SURFACE_DOMAIN_CACHE,
    }
}
