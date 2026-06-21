import { describe, expect, it } from 'vitest';
import {
  PolicyCompilerCapabilityState,
  PolicyCompilerDomain,
  PolicyCompilerNoClaimLabel,
  PolicyCompilerRuleStatus,
  parsePolicyCompiledArtifact,
} from '@ocentra-parent/schema-domain/policy-compiler';
import {
  TrackingLocationAiAnalysisResultSchema,
  TrackingPolicySchemaVersion,
} from '@ocentra-parent/schema-domain/tracking-location-policy';
import {
  TrackingPolicyCompilerRuntimeProofRequestSchema,
  type TrackingPolicyCompilerRuntimeProofRequest,
  type TrackingPolicyCompilerRuntimeProofResult,
} from '@ocentra-parent/schema-domain/tracking-policy-compiler-runtime-proof';
import type { TrackingPolicyRule } from '@ocentra-parent/schema-domain/tracking-location-policy-types';
import { compileTrackingPolicyRuntimeProofDecision } from '../../src/tracking-policy-compiler-runtime-proof';

const EvidenceTrace = {
  evidenceReferenceId: 'tracking-policy-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-05T17:00:00.000Z',
} as const;

const RuleBase = {
  schemaVersion: TrackingPolicySchemaVersion,
  ruleId: 'tracking-policy-rule-1',
  familyId: 'family-1',
  childProfileId: 'child-1',
  deviceId: 'parent-device-1',
  policyVersion: 'tracking-policy-v3',
  targetKind: 'geofence-transition',
  action: 'observe',
  enabled: true,
  requiresFreshEvidence: true,
  requiresParentConfirmation: false,
  reasonCodes: ['tracking-rule-matched'],
  auditRefs: ['tracking-rule-audit'],
} as const;

const AiCritical = TrackingLocationAiAnalysisResultSchema.parse({
  schemaVersion: TrackingPolicySchemaVersion,
  analysisId: 'tracking-ai-critical-1',
  completedAt: '2026-06-05T17:01:00.000Z',
  riskLevel: 'critical',
  confidence: 0.91,
  providerRouteId: 'tracking-ai-route-1',
  evidenceReferences: [EvidenceTrace],
  reasonCodes: ['ai-critical-location-risk'],
  canTriggerAlertDirectly: false,
  isFinalAuthority: false,
});

describe('tracking policy compiler runtime proof', () => {
  registerActionCompilationCases();
  registerAiAuthorityCases();
  registerDegradedStateCases();
  registerRequestValidationCases();
});

function registerActionCompilationCases() {
  it('compiles observe, notify, child check-in, and parent acknowledgement actions', () => {
    const observe = compile('observe');
    const notify = compile('notify-parent', { alertId: 'tracking-alert-notify' });
    const checkIn = compile('ask-child-check-in', { checkInId: 'tracking-checkin-1' });
    const parentAck = compile('request-parent-acknowledgement', { alertId: 'tracking-alert-ack' });

    expectAction(observe, 'observe', null);
    expectAction(notify, 'notify-parent', 'watch');
    expectAction(checkIn, 'ask-child-check-in', null);
    expect(checkIn.childCheckInRequest?.state).toBe('sent');
    expectAction(parentAck, 'request-parent-acknowledgement', 'watch');
  });

  it('compiles live tracking and escalation follow-up rows without runtime claims', () => {
    const live = compile('start-temporary-live-tracking', {
      liveTrackingGrantId: 'tracking-live-grant-1',
      liveTrackingDurationSeconds: 900,
      parentConfirmationReceived: true,
    });
    const escalation = compile('escalate', {
      alertId: 'tracking-alert-escalate',
      escalationId: 'tracking-escalation-1',
    });

    expectAction(live, 'start-temporary-live-tracking', null);
    expect(live.temporaryLiveGrant?.durationSeconds).toBe(900);
    expectAction(escalation, 'escalate', 'urgent');
    expect(escalation.escalationChain?.state).toBe('waiting-for-parent');
  });

  it('compiles suppress, manual-required, and critical-alert candidates', () => {
    const suppress = compile('no-action', { requestedAction: 'suppress' });
    const manual = compile('notify-parent', {
      requestedAction: 'manual-required',
      platformManualRequired: true,
      alertId: 'tracking-alert-manual',
    });
    const critical = compile('notify-parent', {
      requestedAction: 'critical-alert',
      alertId: 'tracking-alert-critical',
      aiAnalysis: AiCritical,
    });

    expectAction(suppress, 'no-action', null);
    expectAction(manual, 'manual-required', null);
    expectAction(critical, 'notify-parent', 'critical');
    expect(critical.decision.reasonCodes).toContain('ai-evidence-not-final-authority');
  });
}

function registerAiAuthorityCases() {
  it('keeps AI critical candidates from becoming final alert authority', () => {
    const result = compile('observe', {
      requestedAction: 'critical-alert',
      aiAnalysis: AiCritical,
      alertId: 'tracking-alert-ai-not-authority',
    });

    expect(result.decision.action).toBe('observe');
    expect(result.alertIntent).toBeNull();
    expect(result.parentPolicyFinalAuthority).toBe(true);
    expect(result.aiFinalAuthority).toBe(false);
    expect(result.decision.reasonCodes).toContain('parent-policy-overrode-candidate');
    expect(result.decision.reasonCodes).toContain('ai-evidence-not-final-authority');
  });
}

function registerDegradedStateCases() {
  it('routes stale evidence, missing confirmation, and disabled rules to deterministic non-enforcing states', () => {
    const stale = compile('notify-parent', {
      freshEvidenceAvailable: false,
      alertId: 'tracking-alert-stale',
    });
    const unconfirmed = compile('notify-parent', {
      ruleOverrides: { requiresParentConfirmation: true },
      parentConfirmationReceived: false,
      alertId: 'tracking-alert-confirmation',
    });
    const disabled = compile('notify-parent', {
      ruleOverrides: { enabled: false },
      alertId: 'tracking-alert-disabled',
    });

    expect(stale.finalActionSource).toBe('manual-required');
    expect(stale.decision.action).toBe('manual-required');
    expect(stale.providerDeliveryClaimed).toBe(false);
    expect(unconfirmed.finalActionSource).toBe('manual-required');
    expect(unconfirmed.decision.action).toBe('manual-required');
    expect(disabled.finalActionSource).toBe('disabled-rule');
    expect(disabled.decision.action).toBe('no-action');
  });
}

function registerRequestValidationCases() {
  it('accepts matching tracking compiled-artifact provenance for runtime-proof requests', () => {
    const request = TrackingPolicyCompilerRuntimeProofRequestSchema.parse(requestShapeFor('observe'));

    expect(request.compiledArtifact.domain).toBe(PolicyCompilerDomain.Tracking);
    expect(request.compiledArtifact.sourcePolicyVersion).toBe(request.rule.policyVersion);
    expect(request.compiledArtifact.rules.some((rule) => rule.ruleId === request.rule.ruleId)).toBe(true);
  });

  it('rejects runtime-proof requests whose compiled artifact targets another domain', () => {
    const baseRequest = requestShapeFor('observe');
    const result = TrackingPolicyCompilerRuntimeProofRequestSchema.safeParse({
      ...baseRequest,
      compiledArtifact: {
        ...baseRequest.compiledArtifact,
        domain: PolicyCompilerDomain.Browser,
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects runtime-proof requests whose compiled artifact source version differs from the request rule', () => {
    const baseRequest = requestShapeFor('observe');
    const result = TrackingPolicyCompilerRuntimeProofRequestSchema.safeParse({
      ...baseRequest,
      compiledArtifact: {
        ...baseRequest.compiledArtifact,
        sourcePolicyVersion: 'tracking-policy-v999',
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects runtime-proof requests whose compiled artifact omits the request rule', () => {
    const baseRequest = requestShapeFor('observe');
    const result = TrackingPolicyCompilerRuntimeProofRequestSchema.safeParse({
      ...baseRequest,
      compiledArtifact: {
        ...baseRequest.compiledArtifact,
        rules: [],
      },
    });

    expect(result.success).toBe(false);
  });

  it('rejects non-manual compiler requests without evidence and live grants without runtime grant data', () => {
    const missingEvidence = TrackingPolicyCompilerRuntimeProofRequestSchema.safeParse({
      ...requestFor('notify-parent', { alertId: 'tracking-alert-no-evidence' }),
      evidenceReferences: [],
    });
    const missingGrantData = TrackingPolicyCompilerRuntimeProofRequestSchema.safeParse(
      requestShapeFor('start-temporary-live-tracking', {
        liveTrackingGrantId: null,
        liveTrackingDurationSeconds: null,
      })
    );

    expect(missingEvidence.success).toBe(false);
    expect(missingGrantData.success).toBe(false);
  });
}

function compile(
  ruleAction: TrackingPolicyRule['action'],
  options: Partial<TrackingPolicyCompilerRuntimeProofRequest> & {
    readonly ruleOverrides?: Partial<TrackingPolicyRule>;
  } = {}
): TrackingPolicyCompilerRuntimeProofResult {
  return compileTrackingPolicyRuntimeProofDecision(requestFor(ruleAction, options));
}

function requestFor(
  ruleAction: TrackingPolicyRule['action'],
  options: Partial<TrackingPolicyCompilerRuntimeProofRequest> & {
    readonly ruleOverrides?: Partial<TrackingPolicyRule>;
  } = {}
): TrackingPolicyCompilerRuntimeProofRequest {
  return TrackingPolicyCompilerRuntimeProofRequestSchema.parse(requestShapeFor(ruleAction, options));
}

function requestShapeFor(
  ruleAction: TrackingPolicyRule['action'],
  options: Partial<TrackingPolicyCompilerRuntimeProofRequest> & {
    readonly ruleOverrides?: Partial<TrackingPolicyRule>;
  } = {}
) {
  const { ruleOverrides, ...requestOptions } = options;
  const rule = {
    ...RuleBase,
    ...ruleOverrides,
    action: ruleAction,
  };
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    requestId: `tracking-policy-compile-${ruleAction}`,
    compiledArtifact: compiledArtifactFor(rule),
    rule,
    requestedAt: '2026-06-05T17:00:00.000Z',
    decidedAt: '2026-06-05T17:01:00.000Z',
    followUpExpiresAt: '2026-06-05T17:11:00.000Z',
    decisionId: `tracking-policy-decision-${ruleAction}`,
    requestedAction: ruleAction,
    compilerMode: 'dry-run',
    evidenceReferences: [EvidenceTrace],
    aiAnalysis: null,
    alertId: null,
    alertSeverity: null,
    checkInId: null,
    escalationId: null,
    liveTrackingGrantId: null,
    liveTrackingDurationSeconds: null,
    parentConfirmationReceived: true,
    freshEvidenceAvailable: true,
    platformManualRequired: false,
    reasonCodes: ['tracking-compiler-requested'],
    auditRefs: ['tracking-compiler-audit'],
    ...requestOptions,
  };
}

function compiledArtifactFor(rule: TrackingPolicyRule) {
  return parsePolicyCompiledArtifact({
    compiledArtifactId: `policy-compiler:tracking:${rule.ruleId}`,
    compilerSchemaVersion: 1,
    householdId: 'tracking-household-1',
    sourcePolicyVersion: rule.policyVersion,
    consumerPolicyVersion: rule.policyVersion,
    sourceDocumentId: 'tracking-policy-source-1',
    sourceStatus: 'confirmed',
    domain: PolicyCompilerDomain.Tracking,
    deliveryTarget: {
      childProfileIds: [rule.childProfileId],
      deviceIds: [rule.deviceId],
      domain: PolicyCompilerDomain.Tracking,
    },
    supportMatrix: {
      domain: PolicyCompilerDomain.Tracking,
      rows: [
        { targetKind: 'child-profile', capabilityState: PolicyCompilerCapabilityState.Supported },
        { targetKind: 'device', capabilityState: PolicyCompilerCapabilityState.Supported },
        { targetKind: 'app', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'site', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'category', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'resource', capabilityState: PolicyCompilerCapabilityState.Supported },
      ],
    },
    evidenceCustodyRequirements: {
      exportAllowed: true,
      deleteAllowed: true,
      syncAllowed: true,
    },
    noClaimLabels: [
      PolicyCompilerNoClaimLabel.CompiledArtifactNotSourceTruth,
      PolicyCompilerNoClaimLabel.RuntimeMutationNotClaimed,
      PolicyCompilerNoClaimLabel.EnforcementNotClaimed,
      PolicyCompilerNoClaimLabel.UiDeliveryNotClaimed,
      PolicyCompilerNoClaimLabel.PlatformSupportNotClaimed,
    ],
    auditReferenceIds: ['tracking-policy-compiler-audit'],
    supersededByPolicyVersion: null,
    rollbackRef: null,
    schedules: [],
    rules: [
      {
        ruleId: rule.ruleId,
        target: {
          kind: 'resource',
          referenceId: `tracking:${rule.targetKind}:${rule.deviceId}`,
        },
        action: 'warn',
        scheduleId: null,
        capabilityState: PolicyCompilerCapabilityState.Supported,
        status: PolicyCompilerRuleStatus.Ready,
        reasonCode: null,
      },
    ],
  });
}

function expectAction(
  result: TrackingPolicyCompilerRuntimeProofResult,
  action: TrackingPolicyCompilerRuntimeProofResult['decision']['action'],
  alertSeverity: AlertSeverityExpectation
) {
  expect(result.decision.action).toBe(action);
  expect(result.parentPolicyFinalAuthority).toBe(true);
  expect(result.runtimeEnforcementClaimed).toBe(false);
  expect(result.providerDeliveryClaimed).toBe(false);
  expect(result.platformAdapterClaimed).toBe(false);
  expect(result.physicalDeviceClaimed).toBe(false);
  if (alertSeverity === null) {
    expect(result.alertIntent).toBeNull();
    return;
  }
  expect(result.alertIntent?.severity).toBe(alertSeverity);
}

type AlertSeverityExpectation = 'watch' | 'urgent' | 'critical' | null;
