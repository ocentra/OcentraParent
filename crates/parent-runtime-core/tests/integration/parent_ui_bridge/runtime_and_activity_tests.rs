use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::{AgentEventEnvelope, AgentEventName};
use serde_json::{json, Value};

use super::tests_support;
use super::tests_support::{
    require_ok, start_local_server_with_capture_responses, with_agent_addr,
};
use super::{
    dispatch_parent_ui_action, load_parent_route_snapshot, ParentRouteId, ParentUiAction,
    ParentUiActionKind, ParentUiActionResult,
};

use super::common::events::responses::screen_settings_response_event;
use super::common::events::responses::*;
use super::common::events::samples::*;
use super::common::events::tracking::*;
use super::common::helpers::*;

#[path = "runtime_and_activity_tests/actions.rs"]
mod actions;
#[path = "runtime_and_activity_tests/routes.rs"]
mod routes;
#[path = "runtime_and_activity_tests/support.rs"]
mod support;
#[path = "runtime_and_activity_tests/timer_support.rs"]
mod timer_support;
