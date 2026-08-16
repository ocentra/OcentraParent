use ocentra_parent_agent_protocol::browser::BrowserRuntimePhase;
use ocentra_parent_agent_protocol::constants;

use crate::browser_event_runtime::{
    browser_correlation_id, should_publish_phase, BrowserRuntimeInput,
};

pub(crate) fn previous_phase_ref(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
) -> Option<String> {
    previous_published_phase(phase, input).map(|previous| browser_event_ref(previous, input))
}

fn previous_published_phase(
    phase: BrowserRuntimePhase,
    input: &BrowserRuntimeInput,
) -> Option<BrowserRuntimePhase> {
    BrowserRuntimePhase::ordered_chain()
        .iter()
        .copied()
        .filter(|candidate| should_publish_phase(*candidate, input))
        .take_while(|candidate| *candidate != phase)
        .last()
}

fn browser_event_ref(phase: BrowserRuntimePhase, input: &BrowserRuntimeInput) -> String {
    let mut value = browser_correlation_id(input);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(phase.event_type());
    value
}
