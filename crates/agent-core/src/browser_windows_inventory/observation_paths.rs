use std::path::{Path, PathBuf};

use super::{BrowserWindowsInventoryObservation, BrowserWindowsSupportKind};
use crate::process_capture::ProcessObservation;

pub(super) fn windows_browser_inventory_observations(
    candidate_paths: &[PathBuf],
    process_observations: &[ProcessObservation],
    managed_process_id: Option<u32>,
) -> Vec<BrowserWindowsInventoryObservation> {
    let mut observations = candidate_paths
        .iter()
        .filter_map(|path| windows_browser_inventory_path_observation(path))
        .collect::<Vec<_>>();
    for process in process_observations {
        let Some(process_observation) =
            super::observation_processes::windows_browser_inventory_process_observation(
                process,
                managed_process_id,
            )
        else {
            continue;
        };
        if !merge_process_observation(&mut observations, process, &process_observation) {
            observations.push(process_observation);
        }
    }
    observations.sort_by(|left, right| {
        left.product_name
            .cmp(&right.product_name)
            .then_with(|| {
                left.browser_channel
                    .as_protocol_str()
                    .cmp(right.browser_channel.as_protocol_str())
            })
            .then_with(|| left.process_id.cmp(&right.process_id))
    });
    observations
}

fn windows_browser_inventory_path_observation(
    path: &Path,
) -> Option<BrowserWindowsInventoryObservation> {
    if !path.is_file() {
        return None;
    }
    let identity = super::windows_browser_executable_identity(path);
    match identity.support_kind {
        BrowserWindowsSupportKind::ManagedChromium => {
            Some(super::observation_processes::managed_chromium_path_observation(path, &identity))
        }
        BrowserWindowsSupportKind::ManualChromium => {
            Some(super::observation_processes::manual_chromium_path_observation(path, &identity))
        }
        BrowserWindowsSupportKind::Unsupported => Some(
            super::observation_processes::unsupported_path_observation(path, &identity),
        ),
        BrowserWindowsSupportKind::Unknown => None,
    }
}

fn merge_process_observation(
    observations: &mut [BrowserWindowsInventoryObservation],
    process: &ProcessObservation,
    process_observation: &BrowserWindowsInventoryObservation,
) -> bool {
    let Some(process_path) = process
        .executable_path
        .as_deref()
        .filter(|path| !path.as_os_str().is_empty())
    else {
        return false;
    };
    let Some(candidate_observation) = observations.iter_mut().find(|observation| {
        observation
            .executable_path
            .as_deref()
            .is_some_and(|candidate_path| {
                super::super::normalized_component_names(candidate_path)
                    == super::super::normalized_component_names(process_path)
            })
    }) else {
        return false;
    };
    if candidate_observation.process_id.is_none() {
        candidate_observation.process_id = process_observation.process_id;
    }
    candidate_observation.running_state = process_observation.running_state;
    candidate_observation.management_tier = process_observation.management_tier;
    candidate_observation.support_tier = process_observation.support_tier;
    candidate_observation.exact_url_capability = process_observation.exact_url_capability;
    candidate_observation.active_tab_capability = process_observation.active_tab_capability;
    candidate_observation.managed_profile_state = process_observation.managed_profile_state;
    candidate_observation.unmanaged_fallback_capability =
        process_observation.unmanaged_fallback_capability;
    candidate_observation.capability_status = process_observation.capability_status;
    candidate_observation.reason_code = process_observation.reason_code;
    true
}
