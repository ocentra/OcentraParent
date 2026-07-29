use ocentra_eventing::bus::{DispatchMode, EventBus};
use ocentra_eventing::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService,
};

use super::AuthenticatedDeliveryGrantIssuanceError;

mod rejection;
use rejection::issuance_rejection;

const EVENT_TYPE: &str = "authenticated-delivery-grant.issuance.milestone";
const AGGREGATE_KEY: &str = "authenticated-delivery-grant.issuance";
const SOURCE_CUSTODY: &str = "parent";
const SOURCE_ROLE: &str = "policy-control";
const SOURCE_SERVICE: &str = "policy-control-core";
const SOURCE_COMPONENT: &str = "authenticated-delivery-grant-issuer";
const SOURCE_INSTANCE: &str = "primary";
const SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AuthenticatedDeliveryGrantIssuanceOutcome {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AuthenticatedDeliveryGrantIssuanceRejection {
    ParentAuthority,
    ParentStepUp,
    Policy,
    ManualReview,
    Capability,
    Evidence,
    DryRun,
    AuthorizationBinding,
    AuthorizationSnapshot,
    Timestamp,
    Bindings,
    AuthorityProvenance,
    IssuerKey,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedDeliveryGrantIssuanceMilestone {
    pub outcome: AuthenticatedDeliveryGrantIssuanceOutcome,
    pub rejection: Option<AuthenticatedDeliveryGrantIssuanceRejection>,
    pub redaction_state: bool,
}

impl DomainEvent for AuthenticatedDeliveryGrantIssuanceMilestone {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(EVENT_TYPE)?,
            SchemaVersion::new(SCHEMA_VERSION)?,
        ))
    }
    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(AGGREGATE_KEY)
    }
    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{EVENT_TYPE}:{:?}:{:?}",
            self.outcome, self.rejection
        ))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct AuthenticatedDeliveryGrantIssuanceAttemptMilestone {
    /// Opaque, generated value used only to distinguish independent issuance attempts.
    /// It carries no household, device, child, capability, or evidence data.
    attempt_id: EventId,
    #[serde(flatten)]
    milestone: AuthenticatedDeliveryGrantIssuanceMilestone,
}

impl DomainEvent for AuthenticatedDeliveryGrantIssuanceAttemptMilestone {
    fn contract(&self) -> Result<EventContract, EventingError> {
        self.milestone.contract()
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        self.milestone.aggregate_key()
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{EVENT_TYPE}:{}:{:?}:{:?}",
            self.attempt_id.as_str(),
            self.milestone.outcome,
            self.milestone.rejection
        ))
    }
}

#[derive(Clone)]
pub struct EventBusAuthenticatedDeliveryGrantIssuancePublisher {
    event_bus: EventBus,
    source: EventSource,
}

impl EventBusAuthenticatedDeliveryGrantIssuancePublisher {
    pub fn new(event_bus: EventBus) -> Result<Self, EventingError> {
        Ok(Self {
            event_bus,
            source: EventSource::new(
                EventCustody::parse(SOURCE_CUSTODY)?,
                RuntimeRole::parse(SOURCE_ROLE)?,
                SourceService::parse(SOURCE_SERVICE)?,
                SourceComponent::parse(SOURCE_COMPONENT)?,
                RuntimeInstanceId::parse(SOURCE_INSTANCE)?,
            ),
        })
    }
    pub(crate) fn publish(&self, milestone: AuthenticatedDeliveryGrantIssuanceMilestone) {
        let Ok(correlation_id) = CorrelationId::parse(format!(
            "authenticated-delivery-grant-issuance-{}",
            ocentra_eventing::ids::EventId::generated().as_str()
        )) else {
            return;
        };
        let milestone = AuthenticatedDeliveryGrantIssuanceAttemptMilestone {
            attempt_id: EventId::generated(),
            milestone,
        };
        let _publish = self.event_bus.publish_detached(
            milestone,
            EventMetadata::new(correlation_id, self.source.clone()),
            DispatchMode::Sequential,
        );
    }
}

pub(crate) fn rejection_for(
    error: AuthenticatedDeliveryGrantIssuanceError,
) -> AuthenticatedDeliveryGrantIssuanceRejection {
    issuance_rejection(error)
}
