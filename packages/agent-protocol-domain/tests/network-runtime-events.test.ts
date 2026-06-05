import { describe, expect, it } from 'vitest';
import {
  AgentNetworkRuntimeEventSchemaVersion,
  AgentNetworkRuntimeEventType,
  parseAgentNetworkRuntimeEvent,
  type AgentNetworkRuntimeEventResult,
} from '../src/network-runtime-events';

const NoClaimBoundary = {
  exactUrlAvailable: false,
  decryptedHttpsPayloadAvailable: false,
  messageContentAvailable: false,
  searchQueryAvailable: false,
  adapterActionExecuted: false,
} as const;

const FlowObserved = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  flowEventRef: 'event.network.flow.observed.1',
  observedAt: '2026-06-05T06:35:00Z',
  deviceRef: 'device.child.windows-1',
  flowEvidenceRef: 'evidence.network.flow.1',
  custody: 'child-device-query-store',
  evidenceGrade: 'A',
  claimBoundary: NoClaimBoundary,
} as const;

const DomainObserved = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  domainEventRef: 'event.network.domain.observed.1',
  previousEventRef: FlowObserved.flowEventRef,
  flowEvidenceRef: FlowObserved.flowEvidenceRef,
  domainEvidenceRef: 'evidence.network.domain.1',
  attribution: 'dns-answer',
  evidenceGrade: 'A',
  uncertaintyCodes: ['network-only-no-exact-url'],
  claimBoundary: NoClaimBoundary,
} as const;

const ActivityClassified = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  classificationEventRef: 'event.network.activity.classified.1',
  previousEventRef: DomainObserved.domainEventRef,
  evidenceRefs: [FlowObserved.flowEvidenceRef, DomainObserved.domainEvidenceRef],
  activityKind: 'game-candidate',
  confidence: 0.92,
  evidenceGrade: 'A',
  uncertaintyCodes: ['network-only-no-page-content'],
} as const;

const AiAnalysisRequested = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  aiRequestRef: 'event.ai.analysis.requested.1',
  previousEventRef: ActivityClassified.classificationEventRef,
  evidenceRefs: ActivityClassified.evidenceRefs,
  promptTemplateRef: 'prompt.network-ai-audit.v1',
  custody: 'child-device-query-store',
  rawPacketPayloadIncluded: false,
} as const;

const AiAnalysisCompleted = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  aiAnalysisRef: 'event.ai.analysis.completed.1',
  aiRequestRef: AiAnalysisRequested.aiRequestRef,
  previousEventRef: AiAnalysisRequested.aiRequestRef,
  advisoryState: 'completed',
  evidenceRefs: ActivityClassified.evidenceRefs,
  unsupportedClaims: ['decrypted-https-payload'],
} as const;

const PolicyEvaluationRequested = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  policyEvaluationRef: 'event.policy.evaluation.requested.1',
  previousEventRef: AiAnalysisCompleted.aiAnalysisRef,
  evidenceRefs: ActivityClassified.evidenceRefs,
  aiAnalysisRef: AiAnalysisCompleted.aiAnalysisRef,
  parentRuleRefs: ['policy.rule.network-domain.1'],
  dryRun: true,
} as const;

const PolicyDecisionCompleted = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  policyDecisionRef: 'event.policy.decision.completed.1',
  policyEvaluationRef: PolicyEvaluationRequested.policyEvaluationRef,
  previousEventRef: PolicyEvaluationRequested.policyEvaluationRef,
  decisionAction: 'block',
  evidenceRefs: ActivityClassified.evidenceRefs,
  parentRuleRefs: PolicyEvaluationRequested.parentRuleRefs,
  adapterCapabilityRequired: true,
} as const;

const EnforcementCommandIssued = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  enforcementCommandRef: 'event.enforcement.command.issued.1',
  previousEventRef: PolicyDecisionCompleted.policyDecisionRef,
  policyDecisionRef: PolicyDecisionCompleted.policyDecisionRef,
  adapterCapabilityRef: 'adapter.capability.network.dry-run.1',
  enforcementMode: 'dry-run',
  evidenceRefs: ActivityClassified.evidenceRefs,
  rollbackRef: 'rollback.network.command.1',
} as const;

const EnforcementResultObserved = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  enforcementResultRef: 'event.enforcement.result.observed.1',
  enforcementCommandRef: EnforcementCommandIssued.enforcementCommandRef,
  previousEventRef: EnforcementCommandIssued.enforcementCommandRef,
  resultStatus: 'dry-run',
  adapterActionExecuted: false,
  rollbackRef: EnforcementCommandIssued.rollbackRef,
  unavailableReasonCode: null,
} as const;

const AuditEntryCommitted = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  auditEntryRef: 'event.audit.entry.committed.1',
  previousEventRef: EnforcementResultObserved.enforcementResultRef,
  policyDecisionRef: PolicyDecisionCompleted.policyDecisionRef,
  enforcementCommandRef: EnforcementCommandIssued.enforcementCommandRef,
  enforcementResultRef: EnforcementResultObserved.enforcementResultRef,
  evidenceRefs: ActivityClassified.evidenceRefs,
  auditOutcome: 'committed',
} as const;

const PortalReadModelUpdated = {
  schemaVersion: AgentNetworkRuntimeEventSchemaVersion,
  readModelRef: 'event.portal.read-model.updated.1',
  previousEventRef: AuditEntryCommitted.auditEntryRef,
  auditEntryRef: AuditEntryCommitted.auditEntryRef,
  updateKind: 'network-read-model',
  visibleManualRequired: false,
  visibleUnavailable: false,
} as const;

const RuntimeEventSamples = [
  [AgentNetworkRuntimeEventType.NetworkFlowObserved, FlowObserved],
  [AgentNetworkRuntimeEventType.NetworkDomainObserved, DomainObserved],
  [AgentNetworkRuntimeEventType.NetworkActivityClassified, ActivityClassified],
  [AgentNetworkRuntimeEventType.AiAnalysisRequested, AiAnalysisRequested],
  [AgentNetworkRuntimeEventType.AiAnalysisCompleted, AiAnalysisCompleted],
  [AgentNetworkRuntimeEventType.PolicyEvaluationRequested, PolicyEvaluationRequested],
  [AgentNetworkRuntimeEventType.PolicyDecisionCompleted, PolicyDecisionCompleted],
  [AgentNetworkRuntimeEventType.EnforcementCommandIssued, EnforcementCommandIssued],
  [AgentNetworkRuntimeEventType.EnforcementResultObserved, EnforcementResultObserved],
  [AgentNetworkRuntimeEventType.AuditEntryCommitted, AuditEntryCommitted],
  [AgentNetworkRuntimeEventType.PortalReadModelUpdated, PortalReadModelUpdated],
] as const;

describe('agent network runtime event contracts', () => {
  it('parses the protocol-facing network runtime event chain with exact event types', specifyRuntimeChainParsing);
  it('rejects unsupported content, raw packet, and adapter-action claims', specifyUnsupportedClaimRejection);
  it('rejects wrong event types, mismatched payloads, and missing required refs', specifyInvalidEventRejection);
});

function specifyRuntimeChainParsing() {
  const parsed = RuntimeEventSamples.map(([eventType, payload]) =>
    expectRuntimeEventOk(parseAgentNetworkRuntimeEvent({ eventType, payload }))
  );

  expect(parsed.map((event) => event.eventType)).toEqual([
    'network.flow.observed',
    'network.domain.observed',
    'network.activity.classified',
    'ai.analysis.requested',
    'ai.analysis.completed',
    'policy.evaluation.requested',
    'policy.decision.completed',
    'enforcement.command.issued',
    'enforcement.result.observed',
    'audit.entry.committed',
    'portal.read_model.updated',
  ]);
  expect(parsed[0]?.value).toEqual(FlowObserved);
  expect(parsed[7]?.value).toEqual(EnforcementCommandIssued);
  expect(parsed[10]?.value).toEqual(PortalReadModelUpdated);
}

function specifyUnsupportedClaimRejection() {
  expect(
    parseAgentNetworkRuntimeEvent({
      eventType: AgentNetworkRuntimeEventType.NetworkFlowObserved,
      payload: {
        ...FlowObserved,
        claimBoundary: {
          ...NoClaimBoundary,
          exactUrlAvailable: true,
        },
      },
    })
  ).toEqual({ ok: false, reason: 'invalid-payload' });

  expect(
    parseAgentNetworkRuntimeEvent({
      eventType: AgentNetworkRuntimeEventType.AiAnalysisRequested,
      payload: {
        ...AiAnalysisRequested,
        rawPacketPayloadIncluded: true,
      },
    })
  ).toEqual({ ok: false, reason: 'invalid-payload' });

  expect(
    parseAgentNetworkRuntimeEvent({
      eventType: AgentNetworkRuntimeEventType.EnforcementResultObserved,
      payload: {
        ...EnforcementResultObserved,
        adapterActionExecuted: true,
      },
    })
  ).toEqual({ ok: false, reason: 'invalid-payload' });
}

function specifyInvalidEventRejection() {
  expect(
    parseAgentNetworkRuntimeEvent({
      eventType: 'network.unowned.event',
      payload: FlowObserved,
    })
  ).toEqual({ ok: false, reason: 'invalid-event-type' });

  expect(
    parseAgentNetworkRuntimeEvent({
      eventType: AgentNetworkRuntimeEventType.NetworkFlowObserved,
      payload: DomainObserved,
    })
  ).toEqual({ ok: false, reason: 'invalid-payload' });

  expect(
    parseAgentNetworkRuntimeEvent({
      eventType: AgentNetworkRuntimeEventType.EnforcementCommandIssued,
      payload: {
        ...EnforcementCommandIssued,
        policyDecisionRef: '',
      },
    })
  ).toEqual({ ok: false, reason: 'invalid-payload' });
}

function expectRuntimeEventOk(result: AgentNetworkRuntimeEventResult) {
  if (!result.ok) {
    throw new Error(`expected network runtime event to parse: ${result.reason}`);
  }
  return result;
}
