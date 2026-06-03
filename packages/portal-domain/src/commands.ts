import { AgentCommand, AgentEvent, AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/text-domain/portal-dev';

export const PortalOverviewCommands = [
  {
    command: AgentCommand.HealthCheck,
    payload: {},
  },
  {
    command: AgentCommand.LogSnapshotGet,
    payload: {},
  },
  {
    command: AgentCommand.LanPairingStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityIngestStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityRecentSummaryGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserEvidenceRecentGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserManagedBridgePoll,
    payload: {},
  },
  {
    command: AgentCommand.ActivityMemoryGraphGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityReportHistoryList,
    payload: {},
  },
  {
    command: AgentCommand.ActivityScreenReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityAppUseReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityBrowserReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityGamesReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityNetworkReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.BrowserInterventionReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.NetworkFlowReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.ActivityTrackingReadModelGet,
    payload: {},
  },
  {
    command: AgentCommand.LocalAiRuntimeStatusGet,
    payload: {},
  },
  {
    command: AgentCommand.PolicyPreviewReadModelGet,
    payload: {},
  },
] as const;

export const PortalCommandButtons = [
  {
    label: resolvePortalDevText(PortalDevTextToken.CheckHealth),
    command: AgentCommand.HealthCheck,
    resultEvent: AgentEvent.HealthReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetLogSnapshot),
    command: AgentCommand.LogSnapshotGet,
    resultEvent: AgentEvent.LogSnapshotReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
    command: AgentCommand.DevEcho,
    resultEvent: AgentEvent.DevEchoed,
    payload: {
      [AgentProtocolDefaults.Field.Message]: resolvePortalDevText(PortalDevTextToken.EchoPortalPing),
    },
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetWatcherStatus),
    command: AgentCommand.WatchStatusGet,
    resultEvent: AgentEvent.WatchStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityIngestStatus),
    command: AgentCommand.ActivityIngestStatusGet,
    resultEvent: AgentEvent.ActivityIngestStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetRecentActivitySummary),
    command: AgentCommand.ActivityRecentSummaryGet,
    resultEvent: AgentEvent.ActivityRecentSummaryReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetBrowserEvidenceRecent),
    command: AgentCommand.BrowserEvidenceRecentGet,
    resultEvent: AgentEvent.BrowserEvidenceRecentReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityMemoryGraph),
    command: AgentCommand.ActivityMemoryGraphGet,
    resultEvent: AgentEvent.ActivityMemoryGraphReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityReportDaily),
    command: AgentCommand.ActivityReportDailyGenerate,
    resultEvent: AgentEvent.ActivityReportGenerated,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityReportHistory),
    command: AgentCommand.ActivityReportHistoryList,
    resultEvent: AgentEvent.ActivityReportHistoryReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityScreenReadModel),
    command: AgentCommand.ActivityScreenReadModelGet,
    resultEvent: AgentEvent.ActivityScreenReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityAppUseReadModel),
    command: AgentCommand.ActivityAppUseReadModelGet,
    resultEvent: AgentEvent.ActivityAppUseReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityBrowserReadModel),
    command: AgentCommand.ActivityBrowserReadModelGet,
    resultEvent: AgentEvent.ActivityBrowserReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityGamesReadModel),
    command: AgentCommand.ActivityGamesReadModelGet,
    resultEvent: AgentEvent.ActivityGamesReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityNetworkReadModel),
    command: AgentCommand.ActivityNetworkReadModelGet,
    resultEvent: AgentEvent.ActivityNetworkReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetBrowserInterventionReadModel),
    command: AgentCommand.BrowserInterventionReadModelGet,
    resultEvent: AgentEvent.BrowserInterventionReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.PollManagedBrowserBridge),
    command: AgentCommand.BrowserManagedBridgePoll,
    resultEvent: AgentEvent.BrowserManagedStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetNetworkFlowReadModel),
    command: AgentCommand.NetworkFlowReadModelGet,
    resultEvent: AgentEvent.NetworkFlowReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetActivityTrackingReadModel),
    command: AgentCommand.ActivityTrackingReadModelGet,
    resultEvent: AgentEvent.ActivityTrackingReadModelReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetLocalAiRuntimeStatus),
    command: AgentCommand.LocalAiRuntimeStatusGet,
    resultEvent: AgentEvent.LocalAiRuntimeStatusReported,
    payload: {},
  },
  {
    label: resolvePortalDevText(PortalDevTextToken.GetPolicyPreviewReadModel),
    command: AgentCommand.PolicyPreviewReadModelGet,
    resultEvent: AgentEvent.PolicyPreviewReadModelReported,
    payload: {},
  },
] as const;
