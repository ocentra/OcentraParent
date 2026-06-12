import {
  BrowserBoundaryState,
  BrowserCustodyLabel,
  BrowserExactUrlClaimState,
  BrowserInterventionCapabilityState,
  BrowserInterventionDeliveryState,
  BrowserInterventionReadModelSchema,
  BrowserInterventionSchemaVersion,
  BrowserQueryVisibilityLabel,
  BrowserUnmanagedDetectionState,
  BrowserUnmanagedEnforcementState,
  BrowserUnmanagedFallbackActionState,
  type BrowserInterventionReadModel,
} from '@ocentra-parent/activity-domain/browser';
import { AgentProtocolDefaults, isAgentProtocolLogText, type AgentProtocolLogFields } from './contracts';

export function parseBrowserInterventionReadModel(
  payload: AgentProtocolLogFields
): BrowserInterventionReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  const rows = returned === 0 ? [] : [browserInterventionRow(payload)];
  const parsed = BrowserInterventionReadModelSchema.safeParse({
    schemaVersion: BrowserInterventionSchemaVersion,
    generatedAt: payload[AgentProtocolDefaults.Field.GeneratedAt],
    limit: payload[AgentProtocolDefaults.Field.Limit],
    returned: payload[AgentProtocolDefaults.Field.Returned],
    latestEventId: payload[AgentProtocolDefaults.Field.LatestEventId],
    latestObservedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    managedSessionInterventionCapability:
      payload[AgentProtocolDefaults.Field.ManagedSessionInterventionCapability] ??
      BrowserInterventionCapabilityState.NeedsManagedSession,
    unmanagedBrowserEnforcement:
      payload[AgentProtocolDefaults.Field.UnmanagedBrowserEnforcement] ??
      BrowserUnmanagedEnforcementState.RequiresOsAppControl,
    unmanagedFallbackAction:
      payload[AgentProtocolDefaults.Field.UnmanagedFallbackAction] ??
      BrowserUnmanagedFallbackActionState.OsBlockManualRequired,
    rows,
  });

  if (!parsed.success) {
    return null;
  }
  return parsed.data;
}

function browserInterventionRow(payload: AgentProtocolLogFields) {
  return {
    schemaVersion: BrowserInterventionSchemaVersion,
    browserInterventionId: payload[AgentProtocolDefaults.Field.BrowserInterventionId],
    observedAt: payload[AgentProtocolDefaults.Field.LatestObservedAt],
    sourceId: payload[AgentProtocolDefaults.Field.SourceId],
    deviceId: AgentProtocolDefaults.Target.LocalhostWindowsAgent.deviceId,
    browserFamily: nullIfMissing(payload[AgentProtocolDefaults.Field.BrowserFamily]),
    browserChannel: nullIfMissing(payload[AgentProtocolDefaults.Field.BrowserChannel]),
    managedBrowserSessionId: nullIfMissing(payload[AgentProtocolDefaults.Field.ManagedBrowserSessionId]),
    profileId: nullIfMissing(payload[AgentProtocolDefaults.Field.ProfileId]),
    processId: nullIfMissing(payload[AgentProtocolDefaults.Field.ProcessId]),
    interventionActionId: nullIfMissing(payload[AgentProtocolDefaults.Field.BrowserInterventionActionId]),
    interventionAuditId: nullIfMissing(payload[AgentProtocolDefaults.Field.BrowserInterventionAuditId]),
    evidenceReferenceIds: listFromDelimited(payload[AgentProtocolDefaults.Field.EvidenceReferenceIds]),
    policyDecisionId: nullIfMissing(payload[AgentProtocolDefaults.Field.PolicyDecisionId]),
    decisionSource: payload[AgentProtocolDefaults.Field.DecisionSource],
    interventionAction: payload[AgentProtocolDefaults.Field.InterventionAction],
    interventionTargetType: payload[AgentProtocolDefaults.Field.InterventionTargetType],
    interventionTargetValue: payload[AgentProtocolDefaults.Field.InterventionTargetValue],
    requestedUrl: nullIfMissing(payload[AgentProtocolDefaults.Field.RequestedUrl]),
    observedUrl: nullIfMissing(payload[AgentProtocolDefaults.Field.ObservedUrl]),
    interventionMechanism: payload[AgentProtocolDefaults.Field.InterventionMechanism],
    interventionOutcome: payload[AgentProtocolDefaults.Field.InterventionOutcome],
    browserBoundaryState: payload[AgentProtocolDefaults.Field.BrowserBoundaryState] ?? BrowserBoundaryState.Unknown,
    exactUrlClaimState: payload[AgentProtocolDefaults.Field.ExactUrlClaimState] ?? BrowserExactUrlClaimState.NotClaimed,
    unmanagedDetectionState:
      payload[AgentProtocolDefaults.Field.UnmanagedDetectionState] ?? BrowserUnmanagedDetectionState.Unavailable,
    unmanagedFallbackAction:
      payload[AgentProtocolDefaults.Field.UnmanagedFallbackAction] ?? BrowserUnmanagedFallbackActionState.Unavailable,
    childDeliveryState:
      payload[AgentProtocolDefaults.Field.ChildDeliveryState] ?? BrowserInterventionDeliveryState.NotDelivered,
    reason: nullIfMissing(payload[AgentProtocolDefaults.Field.Reason]),
    custodyLabel: payload[AgentProtocolDefaults.Field.CustodyLabel] ?? BrowserCustodyLabel.Unavailable,
    queryVisibility: payload[AgentProtocolDefaults.Field.QueryVisibility] ?? BrowserQueryVisibilityLabel.Unavailable,
  };
}

function nullIfMissing(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined) {
  return value === undefined ? null : value;
}

function listFromDelimited(value: AgentProtocolLogFields[keyof AgentProtocolLogFields] | undefined) {
  if (!isAgentProtocolLogText(value)) {
    return [];
  }
  return value.split(AgentProtocolDefaults.Delimiter.List).filter((item) => item.length > 0);
}
