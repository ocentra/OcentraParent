mod assessment;
mod assessment_notes;
mod assessment_reasons;
mod builders;
mod decisions;
mod index;
mod merge;
pub mod values;

use std::collections::HashMap;

use assessment::{assess_merge_candidate, MergeAssessment, MergeDecisionState};
use assessment_notes::annotate_merge_assessment;
use builders::{device_from_discovery, device_from_registry};
use decisions::{apply_household_device_decisions, assigned_child_profiles};
use index::{candidate_indices, index_device};
use merge::merge_device;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;

pub fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    observed_at: &str,
) -> Vec<LanCanonicalHouseholdDevice> {
    let assigned_child_profiles = assigned_child_profiles(household_device_decisions);
    let mut devices: Vec<LanCanonicalHouseholdDevice> = Vec::new();
    let mut merge_index: HashMap<String, Vec<usize>> = HashMap::new();

    for discovered in discovered_devices {
        upsert_device(
            &mut devices,
            &mut merge_index,
            device_from_discovery(discovered, observed_at),
            &discovered.child_device,
            &assigned_child_profiles,
        );
    }

    for entry in trusted_registry {
        upsert_device(
            &mut devices,
            &mut merge_index,
            device_from_registry(entry, observed_at),
            &entry.child_device,
            &assigned_child_profiles,
        );
    }

    apply_household_device_decisions(&mut devices, household_device_decisions);
    devices
}

fn upsert_device(
    devices: &mut Vec<LanCanonicalHouseholdDevice>,
    merge_index: &mut HashMap<String, Vec<usize>>,
    device: LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    assigned_child_profiles: &HashMap<String, String>,
) {
    let MergeSelection {
        best_automatic,
        best_blocked,
    } = best_merge_assessments(
        devices,
        merge_index,
        &device,
        source_ref,
        assigned_child_profiles,
    );
    if let Some((index, assessment)) = best_automatic {
        let existing = &mut devices[index];
        merge_device(existing, device);
        annotate_merge_assessment(existing, &assessment);
        index_device(merge_index, existing, source_ref, index);
        return;
    }

    let mut device = device;
    if let Some(assessment) = best_blocked {
        annotate_merge_assessment(&mut device, &assessment);
    }

    let index = devices.len();
    index_device(merge_index, &device, source_ref, index);
    devices.push(device);
}

struct MergeSelection {
    best_automatic: Option<(usize, MergeAssessment)>,
    best_blocked: Option<MergeAssessment>,
}

fn best_merge_assessments(
    devices: &[LanCanonicalHouseholdDevice],
    merge_index: &HashMap<String, Vec<usize>>,
    device: &LanCanonicalHouseholdDevice,
    source_ref: &LanPairingDeviceRef,
    assigned_child_profiles: &HashMap<String, String>,
) -> MergeSelection {
    let mut selection = MergeSelection {
        best_automatic: None,
        best_blocked: None,
    };
    for index in candidate_indices(merge_index, device, source_ref) {
        let Some(candidate) = devices.get(index) else {
            continue;
        };
        let assessment =
            assess_merge_candidate(candidate, source_ref, device, assigned_child_profiles);
        select_merge_assessment(index, assessment, &mut selection);
    }
    selection
}

fn select_merge_assessment(
    index: usize,
    assessment: MergeAssessment,
    selection: &mut MergeSelection,
) {
    match assessment.state {
        MergeDecisionState::Automatic => {
            if selection
                .best_automatic
                .as_ref()
                .is_none_or(|(_, current)| assessment.score > current.score)
            {
                selection.best_automatic = Some((index, assessment));
            }
        }
        MergeDecisionState::ManualRequired | MergeDecisionState::Forbidden => {
            if selection
                .best_blocked
                .as_ref()
                .is_none_or(|current| assessment.score > current.score)
            {
                selection.best_blocked = Some(assessment);
            }
        }
        MergeDecisionState::NoMatch => {}
    }
}
