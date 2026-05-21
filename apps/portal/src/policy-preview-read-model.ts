import { AgentProtocolDefaults, type AgentProtocolLogFields } from '@ocentra-parent/agent-protocol-domain/contracts';
import type { LogFieldValue } from '@ocentra-parent/logging-domain/contracts';

type PolicyPreviewValue = LogFieldValue;

export interface PortalPolicyPreviewReadModel {
  readonly schemaVersion: PolicyPreviewValue;
  readonly generatedAt: PolicyPreviewValue;
  readonly custody: PolicyPreviewValue;
  readonly limit: PolicyPreviewValue;
  readonly returned: PolicyPreviewValue;
  readonly capabilityStatus: PolicyPreviewValue;
  readonly previewId: PolicyPreviewValue;
  readonly latestEventId: PolicyPreviewValue;
  readonly latestObservedAt: PolicyPreviewValue;
  readonly targetId: PolicyPreviewValue;
  readonly targetType: PolicyPreviewValue;
  readonly targetValue: PolicyPreviewValue;
  readonly evidenceReferenceCount: PolicyPreviewValue;
  readonly decisionId: PolicyPreviewValue;
  readonly decisionAction: PolicyPreviewValue;
  readonly reasonCodes: PolicyPreviewValue;
  readonly ruleIds: PolicyPreviewValue;
  readonly localAiResultId: PolicyPreviewValue;
  readonly dryRun: PolicyPreviewValue;
  readonly enforcementHandoffState: PolicyPreviewValue;
}

export function parsePolicyPreviewReadModel(payload: AgentProtocolLogFields): PortalPolicyPreviewReadModel | null {
  const returned = payload[AgentProtocolDefaults.Field.Returned];
  if (returned === undefined || returned === null) {
    return null;
  }

  return {
    schemaVersion: valueOrNull(payload[AgentProtocolDefaults.Field.SchemaVersion]),
    generatedAt: valueOrNull(payload[AgentProtocolDefaults.Field.GeneratedAt]),
    custody: valueOrNull(payload[AgentProtocolDefaults.Field.Custody]),
    limit: valueOrNull(payload[AgentProtocolDefaults.Field.Limit]),
    returned,
    capabilityStatus: valueOrNull(payload[AgentProtocolDefaults.Field.CapabilityStatus]),
    previewId: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyPreviewId]),
    latestEventId: valueOrNull(payload[AgentProtocolDefaults.Field.LatestEventId]),
    latestObservedAt: valueOrNull(payload[AgentProtocolDefaults.Field.LatestObservedAt]),
    targetId: valueOrNull(payload[AgentProtocolDefaults.Field.TargetId]),
    targetType: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyTargetType]),
    targetValue: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyTargetValue]),
    evidenceReferenceCount: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyEvidenceReferenceCount]),
    decisionId: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyDecisionId]),
    decisionAction: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyAction]),
    reasonCodes: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyReasonCodes]),
    ruleIds: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyRuleIds]),
    localAiResultId: valueOrNull(payload[AgentProtocolDefaults.Field.LocalAiResultId]),
    dryRun: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyDryRun]),
    enforcementHandoffState: valueOrNull(payload[AgentProtocolDefaults.Field.PolicyHandoffState]),
  };
}

function valueOrNull(value: LogFieldValue | undefined): LogFieldValue {
  return value === undefined ? null : value;
}
