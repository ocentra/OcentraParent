use ocentra_eventing::error::EventingError;
use ocentra_lan_core::lan_pairing;
use ocentra_network_core::network_runtime;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainEvidenceRecordedEvent, ChildDomainObservedEvent, ChildRuntimeDomain,
};

pub(super) fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
    match event.domain {
        ChildRuntimeDomain::App => Ok(ocentra_app_core::app_evidence_recorded_event(event)),
        ChildRuntimeDomain::AppGame => Ok(ocentra_app_game_core::app_game_evidence_recorded_event(
            event,
        )),
        ChildRuntimeDomain::Browser => {
            Ok(ocentra_browser_core::browser_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::Lan => Ok(lan_pairing::lan_evidence_recorded_event(event)),
        ChildRuntimeDomain::Network => Ok(network_runtime::network_evidence_recorded_event(event)),
        ChildRuntimeDomain::Screen => {
            Ok(ocentra_screen_core::screen_evidence_recorded_event(event))
        }
        ChildRuntimeDomain::ScreenLiveView => {
            Ok(ocentra_screen_live_view_core::screen_live_view_evidence_recorded_event(event))
        }
    }
}
