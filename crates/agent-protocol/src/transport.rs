use serde::{Deserialize, Serialize};

use crate::{AgentLogSnapshot, LogFields};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPeerRole {
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "cloud-relay")]
    CloudRelay,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPeer {
    pub peer_id: String,
    pub role: AgentPeerRole,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRoute {
    #[serde(rename = "localhost")]
    Localhost,
    #[serde(rename = "local-network")]
    LocalNetwork,
    #[serde(rename = "cloud-relay")]
    CloudRelay,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPairingState {
    #[serde(rename = "unauthenticated")]
    Unauthenticated,
    #[serde(rename = "unpaired")]
    Unpaired,
    #[serde(rename = "pairing")]
    Pairing,
    #[serde(rename = "paired")]
    Paired,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentMessageTarget {
    pub device_id: String,
    pub platform: String,
    pub route: AgentRoute,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPairingProof {
    pub pairing_id: String,
    pub device_id: String,
    pub parent_peer_id: String,
    pub issued_at: String,
    pub expires_at: String,
    pub token_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRouteSecurityPolicy {
    pub route: AgentRoute,
    pub requires_pairing: bool,
    pub allows_anonymous_control: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentCommandName {
    #[serde(rename = "agent.health.check")]
    AgentHealthCheck,
    #[serde(rename = "agent.log.snapshot.get")]
    AgentLogSnapshotGet,
    #[serde(rename = "agent.dev.echo")]
    AgentDevEcho,
    #[serde(rename = "agent.watch.status.get")]
    AgentWatchStatusGet,
    #[serde(rename = "agent.activity.ingest.status.get")]
    AgentActivityIngestStatusGet,
    #[serde(rename = "agent.activity.recent.summary.get")]
    AgentActivityRecentSummaryGet,
    #[serde(rename = "agent.activity.memory-graph.get")]
    AgentActivityMemoryGraphGet,
    #[serde(rename = "agent.browser.evidence.recent.get")]
    AgentBrowserEvidenceRecentGet,
    #[serde(rename = "agent.browser.managed.bridge.poll")]
    AgentBrowserManagedBridgePoll,
    #[serde(rename = "agent.browser.intervention.read-model.get")]
    AgentBrowserInterventionReadModelGet,
    #[serde(rename = "agent.network.flow.read-model.get")]
    AgentNetworkFlowReadModelGet,
    #[serde(rename = "agent.local-ai.runtime.status.get")]
    AgentLocalAiRuntimeStatusGet,
    #[serde(rename = "agent.local-ai.chat.generate")]
    AgentLocalAiChatGenerate,
    #[serde(rename = "agent.policy.preview.read-model.get")]
    AgentPolicyPreviewReadModelGet,
    #[serde(rename = "agent.lan-pairing.proof.submit")]
    AgentLanPairingProofSubmit,
    #[serde(rename = "agent.lan-pairing.route.select")]
    AgentLanPairingRouteSelect,
    #[serde(rename = "agent.lan-pairing.route.revoke")]
    AgentLanPairingRouteRevoke,
    #[serde(rename = "agent.lan-pairing.status.get")]
    AgentLanPairingStatusGet,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentEventName {
    #[serde(rename = "agent.connection.ready")]
    AgentConnectionReady,
    #[serde(rename = "agent.command.rejected")]
    AgentCommandRejected,
    #[serde(rename = "agent.health.reported")]
    AgentHealthReported,
    #[serde(rename = "agent.log.snapshot.reported")]
    AgentLogSnapshotReported,
    #[serde(rename = "agent.dev.echoed")]
    AgentDevEchoed,
    #[serde(rename = "agent.watch.status.reported")]
    AgentWatchStatusReported,
    #[serde(rename = "agent.activity.ingest.status.reported")]
    AgentActivityIngestStatusReported,
    #[serde(rename = "agent.activity.recent.summary.reported")]
    AgentActivityRecentSummaryReported,
    #[serde(rename = "agent.activity.memory-graph.reported")]
    AgentActivityMemoryGraphReported,
    #[serde(rename = "agent.browser.evidence.recent.reported")]
    AgentBrowserEvidenceRecentReported,
    #[serde(rename = "agent.browser.managed.status.reported")]
    AgentBrowserManagedStatusReported,
    #[serde(rename = "agent.browser.intervention.read-model.reported")]
    AgentBrowserInterventionReadModelReported,
    #[serde(rename = "agent.network.flow.read-model.reported")]
    AgentNetworkFlowReadModelReported,
    #[serde(rename = "agent.local-ai.runtime.status.reported")]
    AgentLocalAiRuntimeStatusReported,
    #[serde(rename = "agent.local-ai.chat.generation.reported")]
    AgentLocalAiChatGenerationReported,
    #[serde(rename = "agent.policy.preview.read-model.reported")]
    AgentPolicyPreviewReadModelReported,
    #[serde(rename = "agent.lan-pairing.status.reported")]
    AgentLanPairingStatusReported,
    #[serde(rename = "agent.lan-pairing.audit.reported")]
    AgentLanPairingAuditReported,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCommandEnvelope {
    pub schema_version: u16,
    pub message_id: String,
    pub sent_at: String,
    pub source: AgentPeer,
    pub target: AgentMessageTarget,
    pub command: AgentCommandName,
    pub payload: LogFields,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentEventEnvelope {
    pub schema_version: u16,
    pub event_id: String,
    pub correlation_id: String,
    pub sent_at: String,
    pub source: AgentPeer,
    pub target: AgentPeer,
    pub event: AgentEventName,
    pub severity: crate::LogLevel,
    pub payload: LogFields,
    pub snapshot: Option<AgentLogSnapshot>,
}
