use std::path::PathBuf;
use std::sync::Arc;

use ocentra_eventing::{error::EventingError, ids::CorrelationId};
use ocentra_family_identity_core::device_trust_current_binding::CurrentChildDeviceTrustBinding;
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainObservedEvent, ChildRuntimeDomain,
};
use tokio::sync::{mpsc, oneshot};

use crate::{
    child_domain_runtime_flow::ChildDomainRuntimeEventFlow,
    removal::{
        ChildAgentRemovalBoundary, ChildAgentRemovalStatus, ChildAgentServiceIdentity,
        ChildAgentTamperSignalKind, ChildAgentTrustState, VerifiedParentRemovalAuthorization,
    },
};
use ocentra_eventing::envelope::EventMetadata;
use ocentra_storage_custody_core::storage_custody::StorageCustodyExecutionRequest;

#[path = "service_dispatch.rs"]
mod service_dispatch;
#[path = "service_error_display.rs"]
mod service_error_display;
#[path = "service_errors.rs"]
mod service_errors;
#[path = "service_ingress.rs"]
mod service_ingress;
#[path = "service_ingress_errors.rs"]
mod service_ingress_errors;
#[path = "service_lifecycle.rs"]
mod service_lifecycle;
#[path = "service_paths.rs"]
mod service_paths;
#[path = "service_readiness.rs"]
mod service_readiness;
#[path = "service_recovery.rs"]
mod service_recovery;
#[path = "service_supervision.rs"]
mod service_supervision;
#[path = "storage_custody_runtime.rs"]
pub mod storage_custody_runtime;
#[path = "trust_binding.rs"]
pub mod trust_binding;

use self::trust_binding::ChildAgentTrustBindingSource;

pub const CHILD_AGENT_DATA_DIR_ENV: &str = "OCENTRA_CHILD_AGENT_DATA_DIR";
const CHILD_AGENT_COMMAND_CAPACITY: usize = 64;
const CHILD_RUNTIME_DOMAINS: [ChildRuntimeDomain; 7] = [
    ChildRuntimeDomain::App,
    ChildRuntimeDomain::AppGame,
    ChildRuntimeDomain::Browser,
    ChildRuntimeDomain::Lan,
    ChildRuntimeDomain::Network,
    ChildRuntimeDomain::Screen,
    ChildRuntimeDomain::ScreenLiveView,
];

#[derive(Clone)]
pub struct ChildAgentServicePaths {
    root: PathBuf,
    journal: PathBuf,
    tombstones: PathBuf,
    removal: PathBuf,
    trust_binding_source: Option<Arc<dyn ChildAgentTrustBindingSource>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildAgentReadiness {
    Ready,
    RecoveryPending { correlation_ids: Vec<CorrelationId> },
    TrustBindingManualRequired,
    TamperManualRequired { signal_ref: Option<String> },
    Revoked { audit_ref: Option<String> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentHealth {
    pub readiness: ChildAgentReadiness,
    pub storage_custody: storage_custody_runtime::ChildStorageCustodyReadiness,
    pub domain_flow_count: usize,
    pub durable_root: PathBuf,
    pub removal: ChildAgentRemovalStatus,
}

#[derive(Debug)]
pub enum ChildAgentServiceError {
    Configuration(String),
    Runtime(EventingError),
    Storage(std::io::Error),
    Shutdown(std::io::Error),
    RecoveryPending(Box<ChildAgentReadiness>),
    TrustBindingManualRequired,
    TamperManualRequired { signal_ref: Option<String> },
    TrustRevoked { audit_ref: Option<String> },
    UnknownDomain(ChildRuntimeDomain),
}

#[derive(Debug)]
pub enum ChildAgentIngressError {
    QueueFull,
    ServiceClosed,
    Service(Box<ChildAgentServiceError>),
}

pub(crate) enum ChildAgentCommand {
    Observe(ChildDomainObservedEvent),
    PublishStorageCustody {
        request: StorageCustodyExecutionRequest,
        metadata: EventMetadata,
    },
}

pub(crate) enum ChildAgentCommandResult {
    Domain(Box<crate::child_domain_runtime_flow::ChildDomainRuntimeFlowReport>),
    StorageCustody(storage_custody_runtime::ChildStorageCustodyOutcome),
}

type CommandResponse = oneshot::Sender<Result<ChildAgentCommandResult, ChildAgentServiceError>>;

struct QueuedCommand {
    command: ChildAgentCommand,
    response: CommandResponse,
}

#[derive(Clone)]
pub struct ChildAgentIngress {
    sender: mpsc::Sender<QueuedCommand>,
}

pub struct ChildAgentService {
    paths: ChildAgentServicePaths,
    domain_flows: Vec<ChildDomainRuntimeEventFlow>,
    storage_custody: storage_custody_runtime::ChildStorageCustodyRuntime,
    removal: ChildAgentRemovalBoundary,
    trust_binding: Option<CurrentChildDeviceTrustBinding>,
    recovery_pending: Option<Vec<CorrelationId>>,
    ingress: ChildAgentIngress,
    commands: mpsc::Receiver<QueuedCommand>,
}

pub async fn run_child_agent_service() -> Result<(), ChildAgentServiceError> {
    Box::pin(service_lifecycle::run_child_agent_service()).await
}
