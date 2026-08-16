use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserUnmanagedDetectionConfidence, BrowserUnmanagedDetectionReason,
    BrowserUnmanagedProcessKind,
};
use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_windows_inventory::windows_browser_executable_identity,
    process_capture::ProcessObservation,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedExecutableIdentity {
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub supports_managed_cdp: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserUnmanagedProcessObservation {
    pub process_id: u32,
    pub process_name: String,
    pub executable_path_ref: Option<String>,
    pub signature_ref: Option<String>,
    pub process_hash_ref: Option<String>,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub process_kind: BrowserUnmanagedProcessKind,
    pub detection_confidence: BrowserUnmanagedDetectionConfidence,
    pub detection_reason: BrowserUnmanagedDetectionReason,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserManagedInstallCandidate {
    pub executable_path: PathBuf,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
}

pub fn managed_browser_executable_identity(path: &Path) -> BrowserManagedExecutableIdentity {
    let identity = windows_browser_executable_identity(path);
    if identity.supports_managed_cdp {
        return BrowserManagedExecutableIdentity {
            browser_family: identity.browser_family,
            browser_channel: identity.browser_channel,
            supports_managed_cdp: true,
        };
    }

    BrowserManagedExecutableIdentity {
        browser_family: BrowserFamily::Unknown,
        browser_channel: BrowserChannel::Unknown,
        supports_managed_cdp: false,
    }
}

pub fn installed_managed_browser_candidates(
    candidate_paths: &[PathBuf],
) -> Vec<BrowserManagedInstallCandidate> {
    candidate_paths
        .iter()
        .filter_map(|path| installed_managed_browser_candidate(path))
        .collect()
}

pub fn unmanaged_browser_processes(
    observations: &[ProcessObservation],
    managed_process_id: Option<u32>,
) -> Vec<BrowserUnmanagedProcessObservation> {
    observations
        .iter()
        .filter_map(|observation| unmanaged_browser_process(observation, managed_process_id))
        .collect()
}

fn installed_managed_browser_candidate(path: &Path) -> Option<BrowserManagedInstallCandidate> {
    if !path.is_file() {
        return None;
    }
    let identity = managed_browser_executable_identity(path);
    if !identity.supports_managed_cdp {
        return None;
    }

    Some(BrowserManagedInstallCandidate {
        executable_path: path.to_path_buf(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
    })
}

fn unmanaged_browser_process(
    observation: &ProcessObservation,
    managed_process_id: Option<u32>,
) -> Option<BrowserUnmanagedProcessObservation> {
    if managed_process_id == Some(observation.pid) {
        return None;
    }
    let identity_path = observation
        .executable_path
        .as_deref()
        .unwrap_or_else(|| Path::new(&observation.name));
    let identity = windows_browser_executable_identity(identity_path);
    let classification = unmanaged_process_classification(&identity, identity_path)?;
    let executable_path_ref = observation
        .executable_path
        .as_ref()
        .map(|_| constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string());

    Some(BrowserUnmanagedProcessObservation {
        process_id: observation.pid,
        process_name: observation.name.clone(),
        executable_path_ref,
        signature_ref: None,
        process_hash_ref: None,
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        process_kind: classification.0,
        detection_confidence: classification.1,
        detection_reason: classification.2,
    })
}

fn unmanaged_process_classification(
    identity: &crate::browser_windows_inventory::BrowserWindowsExecutableIdentity,
    identity_path: &Path,
) -> Option<(
    BrowserUnmanagedProcessKind,
    BrowserUnmanagedDetectionConfidence,
    BrowserUnmanagedDetectionReason,
)> {
    if identity.supports_managed_cdp {
        return Some((
            BrowserUnmanagedProcessKind::SupportedBrowser,
            BrowserUnmanagedDetectionConfidence::High,
            BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
        ));
    }
    let normalized_path = identity_path.to_string_lossy().to_ascii_lowercase();
    if normalized_path.contains(constants::browser::PATH_SEGMENT_TOR_BROWSER_NORMALIZED) {
        return Some((
            BrowserUnmanagedProcessKind::TorPrivacyBrowser,
            BrowserUnmanagedDetectionConfidence::High,
            BrowserUnmanagedDetectionReason::TorPrivacyBrowserProcess,
        ));
    }
    if normalized_path.contains(constants::browser::PATH_SEGMENT_PORTABLE_NORMALIZED) {
        return Some((
            BrowserUnmanagedProcessKind::PortableBrowser,
            BrowserUnmanagedDetectionConfidence::Medium,
            BrowserUnmanagedDetectionReason::PortableBrowserProcess,
        ));
    }
    if normalized_path.contains(constants::browser::PATH_SEGMENT_WINDOWS_APPS_NORMALIZED) {
        return Some((
            BrowserUnmanagedProcessKind::PackagedBrowser,
            BrowserUnmanagedDetectionConfidence::Medium,
            BrowserUnmanagedDetectionReason::PackagedBrowserProcess,
        ));
    }
    if identity.product_name != constants::browser::FAMILY_UNKNOWN {
        return Some((
            BrowserUnmanagedProcessKind::UnsupportedBrowser,
            BrowserUnmanagedDetectionConfidence::Medium,
            BrowserUnmanagedDetectionReason::UnsupportedBrowserProcess,
        ));
    }
    if !browser_like_process_name(&normalized_path) {
        return None;
    }

    Some((
        BrowserUnmanagedProcessKind::UnknownBrowserLike,
        BrowserUnmanagedDetectionConfidence::Low,
        BrowserUnmanagedDetectionReason::BrowserLikeProcess,
    ))
}

fn browser_like_process_name(normalized_path: &str) -> bool {
    normalized_path.contains(
        constants::browser::PATH_SEGMENT_BROWSER
            .to_ascii_lowercase()
            .as_str(),
    ) || normalized_path.contains(constants::browser::PATH_SEGMENT_CHROMIUM_NORMALIZED)
        || normalized_path.contains(constants::browser::PATH_SEGMENT_WEBVIEW_NORMALIZED)
}
