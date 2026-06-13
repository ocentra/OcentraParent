use ocentra_eventing::{
    AggregateKey, DomainEvent, EventContract, EventType, EventingError, IdempotencyKey,
    SchemaVersion,
};
use ocentra_evidence::PrivatePayloadState;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{constants, AGENT_PROTOCOL_SCHEMA_VERSION};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ChildDomainEventType(String);

impl ChildDomainEventType {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
            });
        }
        if !is_known_child_domain_event_type(&value) {
            return Err(EventingError::InvalidValue {
                field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
                value,
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn ai_analysis_completed() -> Self {
        Self::parse(constants::child_domain_runtime::AI_ANALYSIS_COMPLETED_EVENT_TYPE)
            .expect(constants::child_domain_runtime::AI_ANALYSIS_COMPLETED_EVENT_TYPE)
    }

    pub fn policy_violation_detected() -> Self {
        Self::parse(constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE)
            .expect(constants::child_domain_runtime::POLICY_VIOLATION_DETECTED_EVENT_TYPE)
    }

    pub fn notification_requested() -> Self {
        Self::parse(constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE)
            .expect(constants::child_domain_runtime::NOTIFICATION_REQUESTED_EVENT_TYPE)
    }
}

fn is_known_child_domain_event_type(value: &str) -> bool {
    constants::child_domain_runtime::CHILD_DOMAIN_EVENT_TYPES.contains(&value)
}

impl TryFrom<String> for ChildDomainEventType {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ChildDomainEventType> for String {
    fn from(value: ChildDomainEventType) -> Self {
        value.0
    }
}

impl std::fmt::Display for ChildDomainEventType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

macro_rules! child_domain_text_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue {
                        field: constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED,
                    });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

child_domain_text_type!(ChildDomainChildDeviceId);
child_domain_text_type!(ChildDomainChildProfileId);
child_domain_text_type!(ChildDomainObservationId);
child_domain_text_type!(ChildDomainSubjectRef);
child_domain_text_type!(ChildDomainObservedState);
child_domain_text_type!(ChildDomainObservedAt);
child_domain_text_type!(ChildDomainEvidenceRef);
child_domain_text_type!(ChildDomainAiRequestId);
child_domain_text_type!(ChildDomainAnalysisPurpose);
child_domain_text_type!(ChildDomainPolicyRequestId);
child_domain_text_type!(ChildDomainFactRef);
child_domain_text_type!(ChildDomainPolicyViolationId);
child_domain_text_type!(ChildDomainPolicyRuleRef);
child_domain_text_type!(ChildDomainPolicySeverity);
child_domain_text_type!(ChildDomainNotificationId);
child_domain_text_type!(ChildDomainNotificationChannel);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChildRuntimeDomain {
    App,
    AppGame,
    Browser,
    Lan,
    Network,
    Screen,
    ScreenLiveView,
}

impl ChildRuntimeDomain {
    pub fn as_contract_text(self) -> &'static str {
        match self {
            Self::App => constants::child_domain_runtime::DOMAIN_APP,
            Self::AppGame => constants::child_domain_runtime::DOMAIN_APP_GAME,
            Self::Browser => constants::child_domain_runtime::DOMAIN_BROWSER,
            Self::Lan => constants::child_domain_runtime::DOMAIN_LAN,
            Self::Network => constants::child_domain_runtime::DOMAIN_NETWORK,
            Self::Screen => constants::child_domain_runtime::DOMAIN_SCREEN,
            Self::ScreenLiveView => constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW,
        }
    }

    pub fn observed_event_type(self) -> ChildDomainEventType {
        self.event_type(match self {
            Self::App => constants::child_domain_runtime::APP_OBSERVED_EVENT_TYPE,
            Self::AppGame => constants::child_domain_runtime::APP_GAME_OBSERVED_EVENT_TYPE,
            Self::Browser => constants::child_domain_runtime::BROWSER_OBSERVED_EVENT_TYPE,
            Self::Lan => constants::child_domain_runtime::LAN_OBSERVED_EVENT_TYPE,
            Self::Network => constants::child_domain_runtime::NETWORK_OBSERVED_EVENT_TYPE,
            Self::Screen => constants::child_domain_runtime::SCREEN_OBSERVED_EVENT_TYPE,
            Self::ScreenLiveView => {
                constants::child_domain_runtime::SCREEN_LIVE_VIEW_OBSERVED_EVENT_TYPE
            }
        })
    }

    pub fn evidence_recorded_event_type(self) -> ChildDomainEventType {
        self.event_type(match self {
            Self::App => constants::child_domain_runtime::APP_EVIDENCE_RECORDED_EVENT_TYPE,
            Self::AppGame => constants::child_domain_runtime::APP_GAME_EVIDENCE_RECORDED_EVENT_TYPE,
            Self::Browser => constants::child_domain_runtime::BROWSER_EVIDENCE_RECORDED_EVENT_TYPE,
            Self::Lan => constants::child_domain_runtime::LAN_EVIDENCE_RECORDED_EVENT_TYPE,
            Self::Network => constants::child_domain_runtime::NETWORK_EVIDENCE_RECORDED_EVENT_TYPE,
            Self::Screen => constants::child_domain_runtime::SCREEN_EVIDENCE_RECORDED_EVENT_TYPE,
            Self::ScreenLiveView => {
                constants::child_domain_runtime::SCREEN_LIVE_VIEW_EVIDENCE_RECORDED_EVENT_TYPE
            }
        })
    }

    pub fn ai_analysis_requested_event_type(self) -> ChildDomainEventType {
        self.event_type(match self {
            Self::App => constants::child_domain_runtime::APP_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            Self::AppGame => {
                constants::child_domain_runtime::APP_GAME_AI_ANALYSIS_REQUESTED_EVENT_TYPE
            }
            Self::Browser => {
                constants::child_domain_runtime::BROWSER_AI_ANALYSIS_REQUESTED_EVENT_TYPE
            }
            Self::Lan => constants::child_domain_runtime::LAN_AI_ANALYSIS_REQUESTED_EVENT_TYPE,
            Self::Network => {
                constants::child_domain_runtime::NETWORK_AI_ANALYSIS_REQUESTED_EVENT_TYPE
            }
            Self::Screen => {
                constants::child_domain_runtime::SCREEN_AI_ANALYSIS_REQUESTED_EVENT_TYPE
            }
            Self::ScreenLiveView => {
                constants::child_domain_runtime::SCREEN_LIVE_VIEW_AI_ANALYSIS_REQUESTED_EVENT_TYPE
            }
        })
    }

    pub fn policy_evaluation_requested_event_type(self) -> ChildDomainEventType {
        self.event_type(match self {
            Self::App => {
                constants::child_domain_runtime::APP_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            }
            Self::AppGame => {
                constants::child_domain_runtime::APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            }
            Self::Browser => {
                constants::child_domain_runtime::BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            }
            Self::Lan => constants::child_domain_runtime::LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE,
            Self::Network => {
                constants::child_domain_runtime::NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            }
            Self::Screen => {
                constants::child_domain_runtime::SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            }
            Self::ScreenLiveView => {
                constants::child_domain_runtime::SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
            }
        })
    }

    pub fn observer_subscriber_id(self) -> &'static str {
        match self {
            Self::App => constants::child_domain_runtime::SUBSCRIBER_APP_OBSERVER,
            Self::AppGame => constants::child_domain_runtime::SUBSCRIBER_APP_GAME_OBSERVER,
            Self::Browser => constants::child_domain_runtime::SUBSCRIBER_BROWSER_OBSERVER,
            Self::Lan => constants::child_domain_runtime::SUBSCRIBER_LAN_OBSERVER,
            Self::Network => constants::child_domain_runtime::SUBSCRIBER_NETWORK_OBSERVER,
            Self::Screen => constants::child_domain_runtime::SUBSCRIBER_SCREEN_OBSERVER,
            Self::ScreenLiveView => {
                constants::child_domain_runtime::SUBSCRIBER_SCREEN_LIVE_VIEW_OBSERVER
            }
        }
    }

    pub fn default_observed_profile(self) -> ChildDomainObservedEventProfile {
        match self {
            Self::App => self.observed_profile(
                ChildDomainRefSuffix::AppSubject,
                ChildDomainObservedSignal::RequiresPolicy,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            Self::AppGame => self.observed_profile(
                ChildDomainRefSuffix::AppGameSubject,
                ChildDomainObservedSignal::RequiresPolicy,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            Self::Browser => self.observed_profile(
                ChildDomainRefSuffix::BrowserSubject,
                ChildDomainObservedSignal::RequiresAi,
                ChildDomainAiAnalysisRequirement::Required,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            Self::Lan => self.observed_profile(
                ChildDomainRefSuffix::LanSubject,
                ChildDomainObservedSignal::RequiresPolicy,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            Self::Network => self.observed_profile(
                ChildDomainRefSuffix::NetworkSubject,
                ChildDomainObservedSignal::RequiresPolicy,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            Self::Screen => self.observed_profile(
                ChildDomainRefSuffix::ScreenSubject,
                ChildDomainObservedSignal::RequiresAi,
                ChildDomainAiAnalysisRequirement::Required,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
            Self::ScreenLiveView => self.observed_profile(
                ChildDomainRefSuffix::ScreenLiveViewSubject,
                ChildDomainObservedSignal::RequiresPolicy,
                ChildDomainAiAnalysisRequirement::NotRequired,
                ChildDomainPolicyEvaluationRequirement::Required,
            ),
        }
    }

    pub fn observed_profile(
        self,
        subject_ref_suffix: ChildDomainRefSuffix,
        observed_state: ChildDomainObservedSignal,
        ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
        policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
    ) -> ChildDomainObservedEventProfile {
        ChildDomainObservedEventProfile {
            domain: self,
            subject_ref_suffix,
            observed_state,
            ai_analysis_requirement,
            policy_evaluation_requirement,
        }
    }

    fn event_type(self, value: &'static str) -> ChildDomainEventType {
        ChildDomainEventType::parse(value).expect(value)
    }
}

impl Serialize for ChildRuntimeDomain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_contract_text())
    }
}

impl<'de> Deserialize<'de> for ChildRuntimeDomain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            constants::child_domain_runtime::DOMAIN_APP => Ok(Self::App),
            constants::child_domain_runtime::DOMAIN_APP_GAME => Ok(Self::AppGame),
            constants::child_domain_runtime::DOMAIN_BROWSER => Ok(Self::Browser),
            constants::child_domain_runtime::DOMAIN_LAN => Ok(Self::Lan),
            constants::child_domain_runtime::DOMAIN_NETWORK => Ok(Self::Network),
            constants::child_domain_runtime::DOMAIN_SCREEN => Ok(Self::Screen),
            constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => Ok(Self::ScreenLiveView),
            _ => Err(serde::de::Error::unknown_variant(
                value.as_str(),
                &[
                    constants::child_domain_runtime::DOMAIN_APP,
                    constants::child_domain_runtime::DOMAIN_APP_GAME,
                    constants::child_domain_runtime::DOMAIN_BROWSER,
                    constants::child_domain_runtime::DOMAIN_LAN,
                    constants::child_domain_runtime::DOMAIN_NETWORK,
                    constants::child_domain_runtime::DOMAIN_SCREEN,
                    constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW,
                ],
            )),
        }
    }
}

impl std::fmt::Display for ChildRuntimeDomain {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_contract_text())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildDomainAiAnalysisRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildDomainPolicyEvaluationRequirement {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChildDomainObservedEventProfile {
    pub domain: ChildRuntimeDomain,
    pub subject_ref_suffix: ChildDomainRefSuffix,
    pub observed_state: ChildDomainObservedSignal,
    pub ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildDomainObservedSignal {
    RequiresAi,
    RequiresPolicy,
    ObserveOnly,
}

impl ChildDomainObservedSignal {
    pub fn into_observed_state(self) -> ChildDomainObservedState {
        let value = self.as_contract_text();
        ChildDomainObservedState::parse(value).expect(value)
    }

    fn as_contract_text(self) -> &'static str {
        match self {
            Self::RequiresAi => constants::child_domain_runtime::SIGNAL_REQUIRES_AI,
            Self::RequiresPolicy => constants::child_domain_runtime::SIGNAL_REQUIRES_POLICY,
            Self::ObserveOnly => constants::child_domain_runtime::SIGNAL_OBSERVE_ONLY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildDomainAnalysisPurposeKind {
    Classification,
}

impl ChildDomainAnalysisPurposeKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::Classification => constants::child_domain_runtime::AI_PURPOSE_CLASSIFICATION,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildDomainPolicyRuleKind {
    Default,
}

impl ChildDomainPolicyRuleKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::Default => constants::child_domain_runtime::POLICY_RULE_DEFAULT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildDomainPolicySeverityKind {
    Review,
}

impl ChildDomainPolicySeverityKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::Review => constants::child_domain_runtime::POLICY_SEVERITY_REVIEW,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildDomainNotificationChannelKind {
    ParentPortal,
}

impl ChildDomainNotificationChannelKind {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::ParentPortal => {
                constants::child_domain_runtime::NOTIFICATION_CHANNEL_PARENT_PORTAL
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildDomainRefSuffix {
    DefaultObservation,
    AppSubject,
    AppGameSubject,
    BrowserSubject,
    LanSubject,
    NetworkSubject,
    ScreenSubject,
    ScreenLiveViewSubject,
    DefaultEvidence,
    DefaultAiRequest,
    DefaultPolicyRequest,
    DefaultPolicyViolation,
    DefaultNotification,
}

impl ChildDomainRefSuffix {
    fn as_contract_text(self) -> &'static str {
        match self {
            Self::DefaultObservation => {
                constants::child_domain_runtime::DEFAULT_OBSERVATION_ID_SUFFIX
            }
            Self::AppSubject => constants::child_domain_runtime::APP_SUBJECT_REF_SUFFIX,
            Self::AppGameSubject => constants::child_domain_runtime::APP_GAME_SUBJECT_REF_SUFFIX,
            Self::BrowserSubject => constants::child_domain_runtime::BROWSER_SUBJECT_REF_SUFFIX,
            Self::LanSubject => constants::child_domain_runtime::LAN_SUBJECT_REF_SUFFIX,
            Self::NetworkSubject => constants::child_domain_runtime::NETWORK_SUBJECT_REF_SUFFIX,
            Self::ScreenSubject => constants::child_domain_runtime::SCREEN_SUBJECT_REF_SUFFIX,
            Self::ScreenLiveViewSubject => {
                constants::child_domain_runtime::SCREEN_LIVE_VIEW_SUBJECT_REF_SUFFIX
            }
            Self::DefaultEvidence => constants::child_domain_runtime::DEFAULT_EVIDENCE_REF_SUFFIX,
            Self::DefaultAiRequest => constants::child_domain_runtime::DEFAULT_AI_REQUEST_ID_SUFFIX,
            Self::DefaultPolicyRequest => {
                constants::child_domain_runtime::DEFAULT_POLICY_REQUEST_ID_SUFFIX
            }
            Self::DefaultPolicyViolation => {
                constants::child_domain_runtime::DEFAULT_POLICY_VIOLATION_ID_SUFFIX
            }
            Self::DefaultNotification => {
                constants::child_domain_runtime::DEFAULT_NOTIFICATION_ID_SUFFIX
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainObservedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub observation_id: ChildDomainObservationId,
    pub subject_ref: ChildDomainSubjectRef,
    pub observed_state: ChildDomainObservedState,
    pub observed_at: ChildDomainObservedAt,
    pub ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainEvidenceRecordedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub evidence_ref: ChildDomainEvidenceRef,
    pub source_observation_id: ChildDomainObservationId,
    pub signal: ChildDomainObservedState,
    pub ai_analysis_requirement: ChildDomainAiAnalysisRequirement,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainAiAnalysisRequestedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub ai_request_id: ChildDomainAiRequestId,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
    pub allowed_analysis_purpose: ChildDomainAnalysisPurpose,
    pub private_payload_state: PrivatePayloadState,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainAiAnalysisCompletedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub source_ai_request_id: ChildDomainAiRequestId,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
    pub result_fact_ref: ChildDomainFactRef,
    pub private_payload_state: PrivatePayloadState,
    pub policy_evaluation_requirement: ChildDomainPolicyEvaluationRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainPolicyEvaluationRequestedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub policy_request_id: ChildDomainPolicyRequestId,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
    pub source_fact_ref: ChildDomainFactRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainPolicyViolationDetectedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub violation_id: ChildDomainPolicyViolationId,
    pub policy_rule_ref: ChildDomainPolicyRuleRef,
    pub severity: ChildDomainPolicySeverity,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildDomainNotificationRequestedEvent {
    pub event_type: ChildDomainEventType,
    pub domain: ChildRuntimeDomain,
    pub child_device_id: ChildDomainChildDeviceId,
    pub child_profile_id: ChildDomainChildProfileId,
    pub notification_id: ChildDomainNotificationId,
    pub source_policy_violation_id: ChildDomainPolicyViolationId,
    pub channel: ChildDomainNotificationChannel,
    pub evidence_refs: Vec<ChildDomainEvidenceRef>,
}

impl DomainEvent for ChildDomainObservedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.observation_id.as_str())
    }
}

impl DomainEvent for ChildDomainEvidenceRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.evidence_ref.as_str())
    }
}

impl DomainEvent for ChildDomainAiAnalysisRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.ai_request_id.as_str())
    }
}

impl DomainEvent for ChildDomainAiAnalysisCompletedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.source_ai_request_id.as_str())
    }
}

impl DomainEvent for ChildDomainPolicyEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.policy_request_id.as_str())
    }
}

impl DomainEvent for ChildDomainPolicyViolationDetectedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.violation_id.as_str())
    }
}

impl DomainEvent for ChildDomainNotificationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        child_domain_contract(&self.event_type)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        child_domain_aggregate_key(
            &self.domain,
            self.child_device_id.as_str(),
            self.child_profile_id.as_str(),
        )
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        child_domain_idempotency_key(&self.event_type, self.notification_id.as_str())
    }
}

pub fn child_domain_child_device_id() -> ChildDomainChildDeviceId {
    ChildDomainChildDeviceId::parse(constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID)
        .expect(constants::child_domain_runtime::DEFAULT_CHILD_DEVICE_ID)
}

pub fn child_domain_child_profile_id() -> ChildDomainChildProfileId {
    ChildDomainChildProfileId::parse(constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID)
        .expect(constants::child_domain_runtime::DEFAULT_CHILD_PROFILE_ID)
}

pub fn child_domain_observed_at() -> ChildDomainObservedAt {
    ChildDomainObservedAt::parse(constants::child_domain_runtime::DEFAULT_OBSERVED_AT)
        .expect(constants::child_domain_runtime::DEFAULT_OBSERVED_AT)
}

pub fn child_domain_observed_state(value: ChildDomainObservedSignal) -> ChildDomainObservedState {
    value.into_observed_state()
}

pub fn child_domain_analysis_purpose(
    value: ChildDomainAnalysisPurposeKind,
) -> ChildDomainAnalysisPurpose {
    let value = value.as_contract_text();
    ChildDomainAnalysisPurpose::parse(value).expect(value)
}

pub fn child_domain_policy_rule_ref(value: ChildDomainPolicyRuleKind) -> ChildDomainPolicyRuleRef {
    let value = value.as_contract_text();
    ChildDomainPolicyRuleRef::parse(value).expect(value)
}

pub fn child_domain_policy_severity(
    value: ChildDomainPolicySeverityKind,
) -> ChildDomainPolicySeverity {
    let value = value.as_contract_text();
    ChildDomainPolicySeverity::parse(value).expect(value)
}

pub fn child_domain_notification_channel(
    value: ChildDomainNotificationChannelKind,
) -> ChildDomainNotificationChannel {
    let value = value.as_contract_text();
    ChildDomainNotificationChannel::parse(value).expect(value)
}

pub fn child_domain_observation_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainObservationId {
    let suffix_text = suffix.as_contract_text();
    ChildDomainObservationId::parse(child_domain_ref_text(domain, suffix_text)).expect(suffix_text)
}

pub fn child_domain_subject_ref(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainSubjectRef {
    let suffix_text = suffix.as_contract_text();
    ChildDomainSubjectRef::parse(child_domain_ref_text(domain, suffix_text)).expect(suffix_text)
}

pub fn child_domain_evidence_ref(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainEvidenceRef {
    let suffix_text = suffix.as_contract_text();
    ChildDomainEvidenceRef::parse(child_domain_ref_text(domain, suffix_text)).expect(suffix_text)
}

pub fn child_domain_ai_request_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainAiRequestId {
    let suffix_text = suffix.as_contract_text();
    ChildDomainAiRequestId::parse(child_domain_ref_text(domain, suffix_text)).expect(suffix_text)
}

pub fn child_domain_policy_request_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainPolicyRequestId {
    let suffix_text = suffix.as_contract_text();
    ChildDomainPolicyRequestId::parse(child_domain_ref_text(domain, suffix_text))
        .expect(suffix_text)
}

pub fn child_domain_fact_ref_from_observation_id(
    value: &ChildDomainObservationId,
) -> ChildDomainFactRef {
    child_domain_fact_ref_text(value.as_str())
}

pub fn child_domain_fact_ref_from_ai_request_id(
    value: &ChildDomainAiRequestId,
) -> ChildDomainFactRef {
    child_domain_fact_ref_text(value.as_str())
}

fn child_domain_fact_ref_text(value: &str) -> ChildDomainFactRef {
    ChildDomainFactRef::parse(value.to_owned())
        .expect(constants::child_domain_runtime::ERROR_CHILD_DOMAIN_FLOW_RECORDED)
}

pub fn child_domain_policy_violation_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainPolicyViolationId {
    let suffix_text = suffix.as_contract_text();
    ChildDomainPolicyViolationId::parse(child_domain_ref_text(domain, suffix_text))
        .expect(suffix_text)
}

pub fn child_domain_notification_id(
    domain: ChildRuntimeDomain,
    suffix: ChildDomainRefSuffix,
) -> ChildDomainNotificationId {
    let suffix_text = suffix.as_contract_text();
    ChildDomainNotificationId::parse(child_domain_ref_text(domain, suffix_text)).expect(suffix_text)
}

pub fn child_domain_observed_event(
    profile: ChildDomainObservedEventProfile,
) -> ChildDomainObservedEvent {
    ChildDomainObservedEvent {
        event_type: profile.domain.observed_event_type(),
        domain: profile.domain,
        child_device_id: child_domain_child_device_id(),
        child_profile_id: child_domain_child_profile_id(),
        observation_id: child_domain_observation_id(
            profile.domain,
            ChildDomainRefSuffix::DefaultObservation,
        ),
        subject_ref: child_domain_subject_ref(profile.domain, profile.subject_ref_suffix),
        observed_state: child_domain_observed_state(profile.observed_state),
        observed_at: child_domain_observed_at(),
        ai_analysis_requirement: profile.ai_analysis_requirement,
        policy_evaluation_requirement: profile.policy_evaluation_requirement,
    }
}

pub fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> ChildDomainEvidenceRecordedEvent {
    ChildDomainEvidenceRecordedEvent {
        event_type: event.domain.evidence_recorded_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        evidence_ref: child_domain_evidence_ref(
            event.domain,
            ChildDomainRefSuffix::DefaultEvidence,
        ),
        source_observation_id: event.observation_id.clone(),
        signal: event.observed_state.clone(),
        ai_analysis_requirement: event.ai_analysis_requirement,
        policy_evaluation_requirement: event.policy_evaluation_requirement,
    }
}

pub fn child_domain_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> ChildDomainAiAnalysisRequestedEvent {
    ChildDomainAiAnalysisRequestedEvent {
        event_type: event.domain.ai_analysis_requested_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        ai_request_id: child_domain_ai_request_id(
            event.domain,
            ChildDomainRefSuffix::DefaultAiRequest,
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        allowed_analysis_purpose: child_domain_analysis_purpose(
            ChildDomainAnalysisPurposeKind::Classification,
        ),
        private_payload_state: PrivatePayloadState::Excluded,
        policy_evaluation_requirement: event.policy_evaluation_requirement,
    }
}

pub fn child_domain_ai_analysis_requested_event_if_required(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainAiAnalysisRequestedEvent> {
    if event.ai_analysis_requirement == ChildDomainAiAnalysisRequirement::Required {
        Some(child_domain_ai_analysis_requested_event(event))
    } else {
        None
    }
}

pub fn child_domain_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
    source_fact_ref: ChildDomainFactRef,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    ChildDomainPolicyEvaluationRequestedEvent {
        event_type: event.domain.policy_evaluation_requested_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        policy_request_id: child_domain_policy_request_id(
            event.domain,
            ChildDomainRefSuffix::DefaultPolicyRequest,
        ),
        evidence_refs: vec![event.evidence_ref.clone()],
        source_fact_ref,
    }
}

pub fn child_domain_direct_policy_evaluation_requested_event_if_required(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    if event.policy_evaluation_requirement == ChildDomainPolicyEvaluationRequirement::Required
        && event.ai_analysis_requirement == ChildDomainAiAnalysisRequirement::NotRequired
    {
        Some(child_domain_policy_evaluation_requested_event(
            event,
            child_domain_fact_ref_from_observation_id(&event.source_observation_id),
        ))
    } else {
        None
    }
}

pub fn child_domain_policy_evaluation_requested_from_ai_event(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    child_domain_policy_evaluation_requested_from_ai_result_event(
        &child_domain_ai_analysis_completed_event(event),
    )
}

pub fn child_domain_ai_analysis_completed_event(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainAiAnalysisCompletedEvent {
    ChildDomainAiAnalysisCompletedEvent {
        event_type: ChildDomainEventType::ai_analysis_completed(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        source_ai_request_id: event.ai_request_id.clone(),
        evidence_refs: event.evidence_refs.clone(),
        result_fact_ref: child_domain_fact_ref_from_ai_request_id(&event.ai_request_id),
        private_payload_state: PrivatePayloadState::Excluded,
        policy_evaluation_requirement: event.policy_evaluation_requirement,
    }
}

pub fn child_domain_policy_evaluation_requested_from_ai_result_event(
    event: &ChildDomainAiAnalysisCompletedEvent,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    ChildDomainPolicyEvaluationRequestedEvent {
        event_type: event.domain.policy_evaluation_requested_event_type(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        policy_request_id: child_domain_policy_request_id(
            event.domain,
            ChildDomainRefSuffix::DefaultPolicyRequest,
        ),
        evidence_refs: event.evidence_refs.clone(),
        source_fact_ref: event.result_fact_ref.clone(),
    }
}

pub fn child_domain_policy_evaluation_requested_from_ai_event_if_required(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    child_domain_policy_evaluation_requested_from_ai_result_event_if_required(
        &child_domain_ai_analysis_completed_event(event),
    )
}

pub fn child_domain_policy_evaluation_requested_from_ai_result_event_if_required(
    event: &ChildDomainAiAnalysisCompletedEvent,
) -> Option<ChildDomainPolicyEvaluationRequestedEvent> {
    if event.policy_evaluation_requirement == ChildDomainPolicyEvaluationRequirement::Required {
        Some(child_domain_policy_evaluation_requested_from_ai_result_event(event))
    } else {
        None
    }
}

pub fn child_domain_policy_violation_detected_event(
    event: &ChildDomainPolicyEvaluationRequestedEvent,
) -> ChildDomainPolicyViolationDetectedEvent {
    ChildDomainPolicyViolationDetectedEvent {
        event_type: ChildDomainEventType::policy_violation_detected(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        violation_id: child_domain_policy_violation_id(
            event.domain,
            ChildDomainRefSuffix::DefaultPolicyViolation,
        ),
        policy_rule_ref: child_domain_policy_rule_ref(ChildDomainPolicyRuleKind::Default),
        severity: child_domain_policy_severity(ChildDomainPolicySeverityKind::Review),
        evidence_refs: event.evidence_refs.clone(),
    }
}

pub fn child_domain_notification_requested_event(
    event: &ChildDomainPolicyViolationDetectedEvent,
) -> ChildDomainNotificationRequestedEvent {
    ChildDomainNotificationRequestedEvent {
        event_type: ChildDomainEventType::notification_requested(),
        domain: event.domain,
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        notification_id: child_domain_notification_id(
            event.domain,
            ChildDomainRefSuffix::DefaultNotification,
        ),
        source_policy_violation_id: event.violation_id.clone(),
        channel: child_domain_notification_channel(
            ChildDomainNotificationChannelKind::ParentPortal,
        ),
        evidence_refs: event.evidence_refs.clone(),
    }
}

fn child_domain_ref_text(domain: ChildRuntimeDomain, suffix: &str) -> String {
    format!(
        "{}{}{}",
        domain.as_contract_text(),
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        suffix
    )
}

fn child_domain_contract(
    event_type: &ChildDomainEventType,
) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type.as_str())?,
        SchemaVersion::new(AGENT_PROTOCOL_SCHEMA_VERSION)?,
    ))
}

fn child_domain_aggregate_key(
    domain: &ChildRuntimeDomain,
    child_device_id: &str,
    child_profile_id: &str,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(format!(
        "{}{}{}{}{}",
        domain.as_contract_text(),
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        child_device_id,
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        child_profile_id
    ))
}

fn child_domain_idempotency_key(
    event_type: &ChildDomainEventType,
    unique_ref: &str,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type.as_str(),
        constants::child_domain_runtime::IDEMPOTENCY_SEPARATOR,
        unique_ref
    ))
}
