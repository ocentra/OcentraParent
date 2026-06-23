import {
  BrowserBoundaryStateSchema,
  BrowserExactUrlClaimStateSchema,
  BrowserInterventionCapabilityStateSchema,
  BrowserInterventionDeliveryStateSchema,
  BrowserInterventionReadModelSchema,
  BrowserInterventionSchemaVersion,
  BrowserUnmanagedDetectionStateSchema,
  BrowserUnmanagedEnforcementStateSchema,
  BrowserUnmanagedFallbackActionStateSchema,
  type BrowserInterventionReadModel,
} from '@ocentra-parent/schema-domain/browser-intervention-schemas';
import { BrowserCustodyLabel, BrowserQueryVisibilityLabel } from '@ocentra-parent/schema-domain/browser-values';
import {
  isAgentProtocolLogText,
  type AgentProtocolLogFields,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';

const DefaultManagedSessionInterventionCapability =
  BrowserInterventionCapabilityStateSchema.parse('needs-managed-session');
const DefaultUnmanagedBrowserEnforcement = BrowserUnmanagedEnforcementStateSchema.parse('requires-os-app-control');
const DefaultUnmanagedFallbackAction = BrowserUnmanagedFallbackActionStateSchema.parse('os-block-manual-required');
const DefaultBrowserBoundaryState = BrowserBoundaryStateSchema.parse('unknown');
const DefaultExactUrlClaimState = BrowserExactUrlClaimStateSchema.parse('not-claimed');
const DefaultUnmanagedDetectionState = BrowserUnmanagedDetectionStateSchema.parse('unavailable');
const DefaultUnavailableFallbackAction = BrowserUnmanagedFallbackActionStateSchema.parse('unavailable');
const DefaultChildDeliveryState = BrowserInterventionDeliveryStateSchema.parse('not-delivered');

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
      DefaultManagedSessionInterventionCapability,
    unmanagedBrowserEnforcement:
      payload[AgentProtocolDefaults.Field.UnmanagedBrowserEnforcement] ?? DefaultUnmanagedBrowserEnforcement,
    unmanagedFallbackAction:
      payload[AgentProtocolDefaults.Field.UnmanagedFallbackAction] ?? DefaultUnmanagedFallbackAction,
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
    browserBoundaryState: payload[AgentProtocolDefaults.Field.BrowserBoundaryState] ?? DefaultBrowserBoundaryState,
    exactUrlClaimState: payload[AgentProtocolDefaults.Field.ExactUrlClaimState] ?? DefaultExactUrlClaimState,
    unmanagedDetectionState:
      payload[AgentProtocolDefaults.Field.UnmanagedDetectionState] ?? DefaultUnmanagedDetectionState,
    unmanagedFallbackAction:
      payload[AgentProtocolDefaults.Field.UnmanagedFallbackAction] ?? DefaultUnavailableFallbackAction,
    childDeliveryState: payload[AgentProtocolDefaults.Field.ChildDeliveryState] ?? DefaultChildDeliveryState,
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
