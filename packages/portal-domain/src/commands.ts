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
    label: resolvePortalDevText(PortalDevTextToken.PollManagedBrowserBridge),
    command: AgentCommand.BrowserManagedBridgePoll,
    resultEvent: AgentEvent.BrowserManagedStatusReported,
    payload: {},
  },
] as const;
