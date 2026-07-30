use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::bus::{DispatchMode, EventBus};
use ocentra_eventing::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService,
};
use ocentra_eventing::journal::policy::JournalMode;
use ocentra_schema::authenticated_delivery_grant::{
    authenticated_delivery_grant_audit_fingerprint, AuthenticatedDeliveryGrant,
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
const SCHEMA_VERSION: u16 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AuthenticatedDeliveryGrantIssuanceOutcome {
    Prepared,
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
    CorrelationId,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AuthenticatedDeliveryGrantIssuanceMilestone {
    pub outcome: AuthenticatedDeliveryGrantIssuanceOutcome,
    pub rejection: Option<AuthenticatedDeliveryGrantIssuanceRejection>,
    /// SHA-256 over the signed grant wire plus its signature. This is a
    /// non-secret, stable audit join key: an auditor can derive it from the
    /// returned signed grant without placing any grant binding in the journal.
    #[serde(default)]
    pub grant_fingerprint: Option<String>,
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
            "{EVENT_TYPE}:{:?}:{:?}:{}",
            self.outcome,
            self.rejection,
            self.grant_fingerprint.as_deref().unwrap_or("rejected")
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
        if event_bus.journal_mode() != JournalMode::BeforeDispatch {
            return Err(EventingError::InvalidHandlerPolicy {
                reason: "authenticated delivery grant issuance requires a before-dispatch-only journal policy so an accepted milestone cannot survive a failed after-dispatch phase"
                    .to_owned(),
            });
        }
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
    pub(crate) fn publish(
        &self,
        correlation_id: CorrelationId,
        attempt_id: EventId,
        milestone: AuthenticatedDeliveryGrantIssuanceMilestone,
    ) -> Result<(), EventingError> {
        let milestone = AuthenticatedDeliveryGrantIssuanceAttemptMilestone {
            attempt_id,
            milestone,
        };
        let metadata = EventMetadata::new(correlation_id, self.source.clone());
        if tokio::runtime::Handle::try_current().is_ok() {
            return Err(EventingError::InvalidHandlerPolicy {
                reason: "authenticated delivery grant issuance must await durable publication inside a Tokio runtime"
                    .to_owned(),
            });
        }
        publish_on_current_thread_runtime(self.event_bus.clone(), milestone, metadata)
    }

    pub(crate) async fn publish_async(
        &self,
        correlation_id: CorrelationId,
        attempt_id: EventId,
        milestone: AuthenticatedDeliveryGrantIssuanceMilestone,
    ) -> Result<(), EventingError> {
        let milestone = AuthenticatedDeliveryGrantIssuanceAttemptMilestone {
            attempt_id,
            milestone,
        };
        let metadata = EventMetadata::new(correlation_id, self.source.clone());
        self.event_bus
            .publish_with_mode(milestone, metadata, DispatchMode::Sequential)
            .await
            .and_then(|report| require_durable_milestone(&report))
    }
}

fn publish_on_current_thread_runtime(
    event_bus: EventBus,
    milestone: AuthenticatedDeliveryGrantIssuanceAttemptMilestone,
    metadata: EventMetadata,
) -> Result<(), EventingError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| EventingError::InvalidHandlerPolicy {
            reason: error.to_string(),
        })?;
    runtime
        .block_on(async move {
            event_bus
                .publish_with_mode(milestone, metadata, DispatchMode::Sequential)
                .await
        })
        .and_then(|report| require_durable_milestone(&report))
}

fn require_durable_milestone(report: &PublishReport) -> Result<(), EventingError> {
    if !report
        .journal_appends
        .iter()
        .any(ocentra_eventing::journal::JournalAppend::is_synchronized)
    {
        return Err(EventingError::InvalidHandlerPolicy {
            reason:
                "authenticated delivery grant issuance milestone requires a synchronized durable journal append"
                    .to_owned(),
        });
    }
    Ok(())
}

pub(crate) fn rejection_for(
    error: AuthenticatedDeliveryGrantIssuanceError,
) -> AuthenticatedDeliveryGrantIssuanceRejection {
    issuance_rejection(error)
}

pub(crate) fn prepared_issuance_milestone_for(
    grant: &AuthenticatedDeliveryGrant,
) -> AuthenticatedDeliveryGrantIssuanceMilestone {
    AuthenticatedDeliveryGrantIssuanceMilestone {
        outcome: AuthenticatedDeliveryGrantIssuanceOutcome::Prepared,
        rejection: None,
        grant_fingerprint: Some(authenticated_delivery_grant_audit_fingerprint(grant)),
        redaction_state: true,
    }
}

pub(crate) fn accepted_issuance_milestone_for(
    grant: &AuthenticatedDeliveryGrant,
) -> AuthenticatedDeliveryGrantIssuanceMilestone {
    AuthenticatedDeliveryGrantIssuanceMilestone {
        outcome: AuthenticatedDeliveryGrantIssuanceOutcome::Accepted,
        rejection: None,
        grant_fingerprint: Some(authenticated_delivery_grant_audit_fingerprint(grant)),
        redaction_state: true,
    }
}

pub(crate) fn rejected_issuance_milestone_for(
    error: AuthenticatedDeliveryGrantIssuanceError,
) -> AuthenticatedDeliveryGrantIssuanceMilestone {
    AuthenticatedDeliveryGrantIssuanceMilestone {
        outcome: AuthenticatedDeliveryGrantIssuanceOutcome::Rejected,
        rejection: Some(rejection_for(error)),
        grant_fingerprint: None,
        redaction_state: true,
    }
}
