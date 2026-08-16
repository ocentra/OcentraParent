use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use ocentra_eventing::{
    error::EventingError,
    ids::CorrelationId,
    journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions},
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainObservedEvent, ChildRuntimeDomain,
};
use ocentra_storage_custody_core::retention_delete_tombstone_store::RetentionDeleteTombstoneStore;
use tokio::sync::{mpsc, oneshot};

use crate::{
    child_domain_runtime::ChildDomainRuntimeEventFlow,
    child_runtime_tombstone_event_flow::ChildRuntimeTombstoneEventFlow,
};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentServicePaths {
    root: PathBuf,
    journal: PathBuf,
    tombstones: PathBuf,
}

impl ChildAgentServicePaths {
    pub fn from_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        Self {
            journal: root.join("child-runtime.ndjson"),
            tombstones: root.join("tombstones"),
            root,
        }
    }

    pub fn from_environment() -> Result<Self, ChildAgentServiceError> {
        let root = std::env::var_os(CHILD_AGENT_DATA_DIR_ENV).ok_or_else(|| {
            ChildAgentServiceError::Configuration(format!(
                "{CHILD_AGENT_DATA_DIR_ENV} must identify the child service data directory"
            ))
        })?;
        Ok(Self::from_root(root))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn journal(&self) -> &Path {
        &self.journal
    }

    pub fn tombstones(&self) -> &Path {
        &self.tombstones
    }

    fn prepare(&self) -> Result<(), ChildAgentServiceError> {
        fs::create_dir_all(&self.root).map_err(ChildAgentServiceError::Storage)?;
        if fs::symlink_metadata(&self.root)
            .map_err(ChildAgentServiceError::Storage)?
            .file_type()
            .is_symlink()
        {
            return Err(ChildAgentServiceError::Storage(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "child service data directory must not be a symlink",
            )));
        }
        fs::create_dir_all(&self.tombstones).map_err(ChildAgentServiceError::Storage)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChildAgentReadiness {
    Ready,
    RecoveryPending { correlation_ids: Vec<CorrelationId> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentHealth {
    pub readiness: ChildAgentReadiness,
    pub domain_flow_count: usize,
    pub durable_root: PathBuf,
}

#[derive(Debug)]
pub enum ChildAgentServiceError {
    Configuration(String),
    Runtime(EventingError),
    Storage(std::io::Error),
    Shutdown(std::io::Error),
    RecoveryPending(Box<ChildAgentReadiness>),
    UnknownDomain(ChildRuntimeDomain),
}

impl fmt::Display for ChildAgentServiceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Configuration(error) => {
                write!(formatter, "child service configuration failed: {error}")
            }
            Self::Runtime(error) => write!(formatter, "child runtime operation failed: {error}"),
            Self::Storage(error) => {
                write!(formatter, "child service durable storage failed: {error}")
            }
            Self::Shutdown(error) => {
                write!(formatter, "child service shutdown signal failed: {error}")
            }
            Self::RecoveryPending(readiness) => {
                write!(formatter, "child service is not ready: {readiness:?}")
            }
            Self::UnknownDomain(domain) => {
                write!(
                    formatter,
                    "child service has no runtime flow for {domain:?}"
                )
            }
        }
    }
}

impl std::error::Error for ChildAgentServiceError {}

impl From<EventingError> for ChildAgentServiceError {
    fn from(error: EventingError) -> Self {
        Self::Runtime(error)
    }
}

#[derive(Debug)]
pub enum ChildAgentIngressError {
    QueueFull,
    ServiceClosed,
    Service(Box<ChildAgentServiceError>),
}

impl fmt::Display for ChildAgentIngressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueueFull => formatter.write_str("child service command queue is full"),
            Self::ServiceClosed => formatter.write_str("child service command queue is closed"),
            Self::Service(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for ChildAgentIngressError {}

pub enum ChildAgentCommand {
    Observe(ChildDomainObservedEvent),
}

type CommandResponse = oneshot::Sender<
    Result<crate::child_domain_runtime::ChildDomainRuntimeFlowReport, ChildAgentServiceError>,
>;

struct QueuedCommand {
    command: ChildAgentCommand,
    response: CommandResponse,
}

#[derive(Clone)]
pub struct ChildAgentIngress {
    sender: mpsc::Sender<QueuedCommand>,
}

impl ChildAgentIngress {
    pub async fn submit(
        &self,
        command: ChildAgentCommand,
    ) -> Result<crate::child_domain_runtime::ChildDomainRuntimeFlowReport, ChildAgentIngressError>
    {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .try_send(QueuedCommand {
                command,
                response: response_sender,
            })
            .map_err(|error| match error {
                mpsc::error::TrySendError::Full(_) => ChildAgentIngressError::QueueFull,
                mpsc::error::TrySendError::Closed(_) => ChildAgentIngressError::ServiceClosed,
            })?;
        response_receiver
            .await
            .map_err(|_| ChildAgentIngressError::ServiceClosed)?
            .map_err(|error| ChildAgentIngressError::Service(Box::new(error)))
    }

    pub async fn submit_observed_event(
        &self,
        event: ChildDomainObservedEvent,
    ) -> Result<crate::child_domain_runtime::ChildDomainRuntimeFlowReport, ChildAgentIngressError>
    {
        self.submit(ChildAgentCommand::Observe(event)).await
    }
}

pub struct ChildAgentService {
    paths: ChildAgentServicePaths,
    domain_flows: Vec<ChildDomainRuntimeEventFlow>,
    tombstone_flow: ChildRuntimeTombstoneEventFlow,
    readiness: ChildAgentReadiness,
    ingress: ChildAgentIngress,
    commands: mpsc::Receiver<QueuedCommand>,
}

impl ChildAgentService {
    pub async fn initialize() -> Result<Self, ChildAgentServiceError> {
        Self::initialize_with_paths(ChildAgentServicePaths::from_environment()?).await
    }

    pub async fn initialize_with_paths(
        paths: ChildAgentServicePaths,
    ) -> Result<Self, ChildAgentServiceError> {
        paths.prepare()?;
        let journal =
            NdjsonEventJournal::with_options(paths.journal(), NdjsonJournalOptions::hash_chain());
        let store = RetentionDeleteTombstoneStore::open(paths.tombstones())
            .map_err(ChildAgentServiceError::Storage)?;
        let tombstone_flow = ChildRuntimeTombstoneEventFlow::new(journal.clone(), store);
        journal.recover().await?;
        let recovery = tombstone_flow
            .recover_pending()
            .await
            .map_err(ChildAgentServiceError::Storage)?;
        let readiness = if recovery.pending_journal_retry.is_empty() {
            ChildAgentReadiness::Ready
        } else {
            ChildAgentReadiness::RecoveryPending {
                correlation_ids: recovery.pending_journal_retry,
            }
        };

        let mut domain_flows = Vec::with_capacity(CHILD_RUNTIME_DOMAINS.len());
        for domain in CHILD_RUNTIME_DOMAINS {
            domain_flows.push(ChildDomainRuntimeEventFlow::for_domain(domain).await?);
        }
        let (sender, commands) = mpsc::channel(CHILD_AGENT_COMMAND_CAPACITY);

        Ok(Self {
            paths,
            domain_flows,
            tombstone_flow,
            readiness,
            ingress: ChildAgentIngress { sender },
            commands,
        })
    }

    pub fn health(&self) -> ChildAgentHealth {
        ChildAgentHealth {
            readiness: self.readiness.clone(),
            domain_flow_count: self.domain_flows.len(),
            durable_root: self.paths.root().to_owned(),
        }
    }

    pub fn readiness(&self) -> &ChildAgentReadiness {
        &self.readiness
    }

    pub fn ingress(&self) -> ChildAgentIngress {
        self.ingress.clone()
    }

    pub fn domain_flow_count(&self) -> usize {
        self.domain_flows.len()
    }

    pub async fn run_until_shutdown(mut self) -> Result<(), ChildAgentServiceError> {
        let shutdown = tokio::signal::ctrl_c();
        tokio::pin!(shutdown);
        loop {
            tokio::select! {
                signal = &mut shutdown => {
                    signal.map_err(ChildAgentServiceError::Shutdown)?;
                    return Ok(());
                }
                queued = self.commands.recv() => {
                    let Some(QueuedCommand { command, response }) = queued else {
                        return Ok(());
                    };
                    let result = self.dispatch(command).await;
                    let _ = response.send(result);
                }
            }
        }
    }

    async fn dispatch(
        &self,
        command: ChildAgentCommand,
    ) -> Result<crate::child_domain_runtime::ChildDomainRuntimeFlowReport, ChildAgentServiceError>
    {
        if self.readiness != ChildAgentReadiness::Ready {
            return Err(ChildAgentServiceError::RecoveryPending(Box::new(
                self.readiness.clone(),
            )));
        }
        match command {
            ChildAgentCommand::Observe(event) => {
                let flow = self
                    .domain_flows
                    .iter()
                    .find(|flow| flow.domain() == event.domain)
                    .ok_or(ChildAgentServiceError::UnknownDomain(event.domain))?;
                flow.publish_observed(event).await.map_err(Into::into)
            }
        }
    }

    pub fn tombstone_flow(&self) -> &ChildRuntimeTombstoneEventFlow {
        &self.tombstone_flow
    }
}
