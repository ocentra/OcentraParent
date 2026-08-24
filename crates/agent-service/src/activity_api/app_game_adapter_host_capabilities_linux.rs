use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use ocentra_parent_screen_capture_adapter::linux_foreground_source::{
    LinuxDisplayEnvironment, LinuxForegroundSourcePreflight, LinuxSocketReadiness, LinuxToolProbe,
};

use super::app_game_adapter_host_capabilities::{EvidenceRefs, HostCapabilitySignals, ProbeRefs};

impl HostCapabilitySignals {
    pub(super) fn linux_evidence_refs_for(
        &self,
        preflight: &LinuxForegroundSourcePreflight,
    ) -> EvidenceRefs {
        if !preflight.source_ready() {
            return EvidenceRefs(Vec::new());
        }
        let mut refs = display_refs(preflight).0;
        refs.extend(socket_refs(preflight).0);
        refs.extend(source_refs(preflight).0);
        EvidenceRefs(refs)
    }

    pub(super) fn linux_probe_refs_for(
        &self,
        preflight: &LinuxForegroundSourcePreflight,
    ) -> ProbeRefs {
        if !preflight.source_ready() {
            return ProbeRefs(Vec::new());
        }
        ProbeRefs(
            [
                matches!(preflight.xprop, LinuxToolProbe::Succeeded)
                    .then_some(proof::REF_LINUX_XPROP_PROBE.to_string()),
                matches!(preflight.xdotool, LinuxToolProbe::Succeeded)
                    .then_some(proof::REF_LINUX_XDOTOOL_PROBE.to_string()),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    pub(super) fn linux_proof_refs_for(
        &self,
        preflight: &LinuxForegroundSourcePreflight,
    ) -> EvidenceRefs {
        let mut refs = self.linux_evidence_refs_for(preflight).0;
        refs.extend(self.linux_probe_refs_for(preflight).0);
        EvidenceRefs(refs)
    }
}

fn display_refs(preflight: &LinuxForegroundSourcePreflight) -> EvidenceRefs {
    match (preflight.display_environment, preflight.display_ready()) {
        (LinuxDisplayEnvironment::Wslg, true) => {
            EvidenceRefs(vec![proof::REF_LINUX_WSLG_DISPLAY.to_string()])
        }
        (LinuxDisplayEnvironment::Native, true) => {
            EvidenceRefs(vec![proof::REF_LINUX_NATIVE_DISPLAY.to_string()])
        }
        _ => EvidenceRefs(Vec::new()),
    }
}

fn socket_refs(preflight: &LinuxForegroundSourcePreflight) -> EvidenceRefs {
    match (
        preflight.display_environment,
        preflight.x11_socket,
        preflight.wayland_socket,
    ) {
        (LinuxDisplayEnvironment::Wslg, LinuxSocketReadiness::Ready, _) => {
            EvidenceRefs(vec![proof::REF_LINUX_WSLG_X11_SOCKET.to_string()])
        }
        (LinuxDisplayEnvironment::Wslg, _, LinuxSocketReadiness::Ready) => {
            EvidenceRefs(vec![proof::REF_LINUX_WSLG_WAYLAND_SOCKET.to_string()])
        }
        (LinuxDisplayEnvironment::Native, LinuxSocketReadiness::Ready, _) => {
            EvidenceRefs(vec![proof::REF_LINUX_NATIVE_X11_SOCKET.to_string()])
        }
        (LinuxDisplayEnvironment::Native, _, LinuxSocketReadiness::Ready) => {
            EvidenceRefs(vec![proof::REF_LINUX_NATIVE_WAYLAND_SOCKET.to_string()])
        }
        _ => EvidenceRefs(Vec::new()),
    }
}

fn source_refs(preflight: &LinuxForegroundSourcePreflight) -> EvidenceRefs {
    EvidenceRefs(
        [
            preflight
                .source_ready()
                .then_some(proof::REF_LINUX_FOREGROUND_SOURCE_PREFLIGHT.to_string()),
            preflight
                .active_window_observed()
                .then_some(proof::REF_LINUX_ACTIVE_WINDOW_OBSERVED.to_string()),
        ]
        .into_iter()
        .flatten()
        .collect(),
    )
}
