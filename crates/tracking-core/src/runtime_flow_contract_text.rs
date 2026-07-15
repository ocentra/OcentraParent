#[path = "runtime_flow_contract_text_refs.rs"]
mod runtime_flow_contract_text_refs;
#[path = "runtime_flow_contract_text_states.rs"]
mod runtime_flow_contract_text_states;

use super::{
    TrackingAcknowledgementStateValue, TrackingAiPurposeKind, TrackingCheckInStateValue,
    TrackingLocationRelationKind, TrackingNotificationChannelKind, TrackingRuntimeRef,
    TrackingTimestampKind, TrackingUncertaintyKind,
};
use ocentra_eventing::expect_value::ExpectValue;

pub(super) fn parse_contract_text<T, E>(
    value: &'static str,
    parse: impl FnOnce(&'static str) -> Result<T, E>,
) -> T
where
    E: core::fmt::Debug,
{
    parse(value).expect_value("tracking runtime contract drift")
}
