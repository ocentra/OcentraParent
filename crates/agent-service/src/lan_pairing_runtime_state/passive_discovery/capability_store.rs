use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use chrono::{DateTime, Utc};
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryUdpListenerIssue,
};
use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};
use crate::time::timestamp_now;

use super::pipeline_health::{
    LanPassiveDiscoveryPipelineHealthSnapshot, LanPassiveDiscoveryPipelineState,
};

#[path = "capability_store/availability.rs"]
mod availability;
#[path = "capability_store/persistence.rs"]
mod persistence;
#[path = "capability_store/validation.rs"]
mod validation;

const CAPABILITY_SCHEMA_VERSION: u16 = 2;
const CAPABILITY_FILE_SUFFIX: &str = "-lan-passive-runtime.json";
const CAPABILITY_MAX_AGE: Duration = Duration::from_secs(420);

#[derive(Clone, Debug)]
pub(super) struct LanPassiveDiscoveryCapabilityStore {
    path: Option<LanPassiveDiscoveryCapabilityPath>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LanPassiveDiscoveryCapabilityPath(PathBuf);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum LanPassiveDiscoveryRuntimeAvailability {
    Starting,
    Available,
    Degraded,
    Unavailable,
    ManualRequired,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum LanPassiveDiscoverySourceAvailability {
    PendingBind,
    Listening,
    RetryScheduled,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LanPassiveDiscoverySourceCapability {
    pub(super) source: LanPassiveDiscoverySource,
    pub(super) availability: LanPassiveDiscoverySourceAvailability,
    pub(super) consecutive_failures: u32,
    pub(super) retry_delay_millis: Option<u64>,
    pub(super) issue: Option<LanPassiveDiscoveryUdpListenerIssue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LanPassiveDiscoveryRuntimeCapability {
    schema_version: u16,
    process_id: u32,
    observed_at: String,
    availability: LanPassiveDiscoveryRuntimeAvailability,
    expected_listener_count: usize,
    active_listener_count: usize,
    sources: Vec<LanPassiveDiscoverySourceCapability>,
    pipeline_health: LanPassiveDiscoveryPipelineHealthSnapshot,
}

impl LanPassiveDiscoveryCapabilityStore {
    pub(super) fn for_runtime(runtime: &LanPairingRuntime) -> Self {
        Self {
            path: capability_path(runtime),
        }
    }

    pub(super) fn save(&self, capability: &LanPassiveDiscoveryRuntimeCapability) -> bool {
        let Some(path) = self.path.as_ref() else {
            return false;
        };
        persistence::save(path, capability)
    }

    fn load(&self) -> Option<LanPassiveDiscoveryRuntimeCapability> {
        let path = self.path.as_ref()?;
        if persistence::failed(path) {
            return None;
        }
        let json = fs::read_to_string(&path.0).ok()?;
        let capability = serde_json::from_str(&json).ok()?;
        validation::validate_and_rederive(capability)
    }

    pub(super) fn save_pipeline_health(
        &self,
        pipeline_health: &LanPassiveDiscoveryPipelineHealthSnapshot,
    ) -> bool {
        let sources = self
            .load()
            .map(|capability| capability.sources)
            .unwrap_or_else(pending_source_capabilities);
        self.save(&LanPassiveDiscoveryRuntimeCapability::from_sources(
            sources,
            pipeline_health.clone(),
        ))
    }
}

impl LanPassiveDiscoveryRuntimeCapability {
    pub(super) fn from_sources(
        sources: Vec<LanPassiveDiscoverySourceCapability>,
        pipeline_health: LanPassiveDiscoveryPipelineHealthSnapshot,
    ) -> Self {
        let active_listener_count = sources
            .iter()
            .filter(|source| {
                source.availability == LanPassiveDiscoverySourceAvailability::Listening
            })
            .count();
        let availability =
            availability::for_sources(&sources, active_listener_count, &pipeline_health);
        Self {
            schema_version: CAPABILITY_SCHEMA_VERSION,
            process_id: std::process::id(),
            observed_at: timestamp_now(),
            availability,
            expected_listener_count: sources.len(),
            active_listener_count,
            sources,
            pipeline_health,
        }
    }

    pub(super) fn stopped(
        sources: Vec<LanPassiveDiscoverySourceCapability>,
        pipeline_health: LanPassiveDiscoveryPipelineHealthSnapshot,
    ) -> Self {
        let mut capability = Self::from_sources(sources, pipeline_health);
        for source in &mut capability.sources {
            source.availability = LanPassiveDiscoverySourceAvailability::Stopped;
            source.retry_delay_millis = None;
        }
        capability.availability = LanPassiveDiscoveryRuntimeAvailability::Stopped;
        capability.active_listener_count = 0;
        capability
    }

    pub(crate) fn service_data_available(&self) -> bool {
        matches!(
            self.availability,
            LanPassiveDiscoveryRuntimeAvailability::Available
                | LanPassiveDiscoveryRuntimeAvailability::Degraded
        ) && self.active_listener_count > 0
            && self.pipeline_health.state == LanPassiveDiscoveryPipelineState::Healthy
    }
}

pub(crate) fn current_runtime_capability(
    runtime: &LanPairingRuntime,
) -> LanPassiveDiscoveryRuntimeCapability {
    let store = LanPassiveDiscoveryCapabilityStore::for_runtime(runtime);
    store
        .load()
        .filter(capability_is_current)
        .unwrap_or_else(unavailable_capability)
}

pub(super) fn record_starting(
    runtime: &LanPairingRuntime,
    pipeline_health: &LanPassiveDiscoveryPipelineHealthSnapshot,
) {
    let capability = LanPassiveDiscoveryRuntimeCapability::from_sources(
        pending_source_capabilities(),
        pipeline_health.clone(),
    );
    let store = LanPassiveDiscoveryCapabilityStore::for_runtime(runtime);
    let _persisted = store.save(&capability);
}

fn capability_is_current(capability: &LanPassiveDiscoveryRuntimeCapability) -> bool {
    if capability.schema_version != CAPABILITY_SCHEMA_VERSION
        || capability.process_id != std::process::id()
    {
        return false;
    }
    if capability.availability == LanPassiveDiscoveryRuntimeAvailability::Stopped {
        return true;
    }
    let Ok(observed_at) = DateTime::parse_from_rfc3339(&capability.observed_at) else {
        return false;
    };
    let observed_at = observed_at.with_timezone(&Utc);
    let now = Utc::now();
    observed_at <= now
        && now
            .signed_duration_since(observed_at)
            .to_std()
            .is_ok_and(|age| age <= CAPABILITY_MAX_AGE)
}

fn unavailable_capability() -> LanPassiveDiscoveryRuntimeCapability {
    let sources = pending_source_capabilities();
    LanPassiveDiscoveryRuntimeCapability {
        schema_version: CAPABILITY_SCHEMA_VERSION,
        process_id: std::process::id(),
        observed_at: timestamp_now(),
        availability: LanPassiveDiscoveryRuntimeAvailability::Unavailable,
        expected_listener_count: sources.len(),
        active_listener_count: 0,
        sources,
        pipeline_health: LanPassiveDiscoveryPipelineHealthSnapshot::unavailable(),
    }
}

fn pending_source_capabilities() -> Vec<LanPassiveDiscoverySourceCapability> {
    super::passive_discovery_udp_sources()
        .iter()
        .copied()
        .map(pending_source_capability)
        .collect()
}

fn pending_source_capability(
    source: LanPassiveDiscoverySource,
) -> LanPassiveDiscoverySourceCapability {
    LanPassiveDiscoverySourceCapability {
        source,
        availability: LanPassiveDiscoverySourceAvailability::PendingBind,
        consecutive_failures: 0,
        retry_delay_millis: None,
        issue: None,
    }
}

fn capability_path(runtime: &LanPairingRuntime) -> Option<LanPassiveDiscoveryCapabilityPath> {
    let registry_path = match &runtime.persistence {
        LanPairingRegistryPersistence::LocalJsonRegistry(path) => path,
        LanPairingRegistryPersistence::InMemory
        | LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => return None,
    };
    let file_stem = registry_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(constants::lan_pairing::REGISTRY_FILE_STEM_FALLBACK);
    Some(LanPassiveDiscoveryCapabilityPath(
        registry_path.with_file_name(format!("{file_stem}{CAPABILITY_FILE_SUFFIX}")),
    ))
}
