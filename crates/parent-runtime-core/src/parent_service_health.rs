use chrono::{DateTime, FixedOffset, Utc};
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_schema::parent_ui_bridge::{
    ParentServiceHealthAuthenticationState, ParentServiceHealthReason, ParentServiceHealthRoute,
    ParentServiceHealthSnapshot, ParentServiceHealthState, ParentServiceHealthTraceSnapshot,
    ParentServiceHealthTransport,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const HEALTH_RESPONSE_FRESHNESS_WINDOW_MS: i64 = 10_000;
const HEALTH_RESPONSE_CLOCK_SKEW_MS: i64 = 1_000;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentAgentServiceHealthState {
    Ready,
    Degraded,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentAgentServiceAuthenticationState {
    Unauthenticated,
    Unavailable,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParentAgentServiceHealthReason {
    Ready,
    TransportUnavailable,
    RouteDependencyUnavailable,
    ResponseSchemaMismatch,
    ResponseIdentityMismatch,
    ResponsePayloadMismatch,
    ResponseNonceMismatch,
    ResponseEventIdMismatch,
    ResponseTimestampMissing,
    ResponseTimestampStale,
    ServiceVersionMissing,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParentAgentServiceHealthTrace {
    pub request_id: Option<String>,
    pub correlation_id: Option<String>,
    pub response_event_id: Option<String>,
    pub request_sent_at: Option<String>,
    pub response_sent_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParentAgentServiceHealth {
    pub state: ParentAgentServiceHealthState,
    pub route: Option<AgentRoute>,
    pub protocol_schema_version: Option<u16>,
    pub service_version: Option<String>,
    pub transport: Option<String>,
    pub authentication_state: ParentAgentServiceAuthenticationState,
    pub reason: ParentAgentServiceHealthReason,
    pub trace: ParentAgentServiceHealthTrace,
}

impl ParentAgentServiceHealth {
    pub fn unavailable() -> Self {
        Self::unavailable_with_reason(ParentAgentServiceHealthReason::TransportUnavailable)
    }

    pub fn unavailable_with_reason(reason: ParentAgentServiceHealthReason) -> Self {
        Self {
            state: ParentAgentServiceHealthState::Unavailable,
            route: None,
            protocol_schema_version: None,
            service_version: None,
            transport: None,
            authentication_state: ParentAgentServiceAuthenticationState::Unavailable,
            reason,
            trace: ParentAgentServiceHealthTrace::default(),
        }
    }

    pub fn degraded(
        reason: ParentAgentServiceHealthReason,
        trace: ParentAgentServiceHealthTrace,
    ) -> Self {
        Self {
            state: ParentAgentServiceHealthState::Degraded,
            route: None,
            protocol_schema_version: None,
            service_version: None,
            transport: None,
            authentication_state: ParentAgentServiceAuthenticationState::Unavailable,
            reason,
            trace,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state == ParentAgentServiceHealthState::Ready
    }

    pub fn redacted_detail(&self) -> String {
        format!(
            "agent-service health {} ({}){}",
            enum_label(&self.state),
            enum_label(&self.reason),
            self.trace.redacted_suffix(),
        )
    }

    pub(crate) fn to_route_snapshot(&self) -> ParentServiceHealthSnapshot {
        ParentServiceHealthSnapshot {
            state: schema_enum(&self.state, ParentServiceHealthState::Unavailable),
            route: self
                .route
                .as_ref()
                .map(|route| schema_enum(route, ParentServiceHealthRoute::Localhost)),
            protocol_schema_version: self.protocol_schema_version,
            service_version: self.service_version.clone(),
            transport: self
                .transport
                .as_ref()
                .map(|transport| schema_enum(transport, ParentServiceHealthTransport::WebSocket)),
            authentication_state: schema_enum(
                &self.authentication_state,
                ParentServiceHealthAuthenticationState::Unavailable,
            ),
            reason: schema_enum(
                &self.reason,
                ParentServiceHealthReason::TransportUnavailable,
            ),
            trace: ParentServiceHealthTraceSnapshot {
                request_id: self.trace.request_id.clone(),
                correlation_id: self.trace.correlation_id.clone(),
                response_event_id: self.trace.response_event_id.clone(),
                request_sent_at: self.trace.request_sent_at.clone(),
                response_sent_at: self.trace.response_sent_at.clone(),
            },
        }
    }
}

impl ParentAgentServiceHealthTrace {
    pub(crate) fn redacted_suffix(&self) -> String {
        let mut fields = Vec::new();
        if let Some(value) = self.correlation_id.as_deref() {
            fields.push(format!("correlation={value}"));
        }
        if let Some(value) = self.response_event_id.as_deref() {
            fields.push(format!("responseEvent={value}"));
        }
        if let Some(value) = self.request_sent_at.as_deref() {
            fields.push(format!("requestSentAt={value}"));
        }
        if let Some(value) = self.response_sent_at.as_deref() {
            fields.push(format!("responseSentAt={value}"));
        }
        if fields.is_empty() {
            String::new()
        } else {
            format!("; {}", fields.join(", "))
        }
    }
}

pub(crate) fn response_timestamp_is_fresh(
    request_sent_at: &str,
    response_sent_at: &str,
) -> Result<(), ParentAgentServiceHealthReason> {
    let request = parse_timestamp(request_sent_at)?;
    let response = parse_timestamp(response_sent_at)?;
    if response < request {
        return Err(ParentAgentServiceHealthReason::ResponseTimestampStale);
    }
    let response_age = response.signed_duration_since(request).num_milliseconds();
    if response_age > HEALTH_RESPONSE_FRESHNESS_WINDOW_MS {
        return Err(ParentAgentServiceHealthReason::ResponseTimestampStale);
    }
    let latest_allowed = Utc::now() + chrono::Duration::milliseconds(HEALTH_RESPONSE_CLOCK_SKEW_MS);
    if response > latest_allowed.fixed_offset() {
        return Err(ParentAgentServiceHealthReason::ResponseTimestampStale);
    }
    Ok(())
}

pub(crate) fn response_timestamp_is_current(
    response_sent_at: &str,
) -> Result<(), ParentAgentServiceHealthReason> {
    let response = parse_timestamp(response_sent_at)?;
    let now = Utc::now();
    let earliest_allowed =
        now - chrono::Duration::milliseconds(HEALTH_RESPONSE_FRESHNESS_WINDOW_MS);
    let latest_allowed = now + chrono::Duration::milliseconds(HEALTH_RESPONSE_CLOCK_SKEW_MS);
    if response < earliest_allowed.fixed_offset() || response > latest_allowed.fixed_offset() {
        return Err(ParentAgentServiceHealthReason::ResponseTimestampStale);
    }
    Ok(())
}

fn parse_timestamp(value: &str) -> Result<DateTime<FixedOffset>, ParentAgentServiceHealthReason> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|_| ParentAgentServiceHealthReason::ResponseTimestampMissing)
}

fn enum_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

fn schema_enum<T, S>(value: &S, fallback: T) -> T
where
    T: DeserializeOwned,
    S: Serialize,
{
    serde_json::to_value(value)
        .ok()
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or(fallback)
}
