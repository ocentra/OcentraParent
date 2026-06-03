import { describe, expect, it } from 'vitest';
import {
  TrackingAcknowledgementSchema,
  TrackingAiProviderRouteSchema,
  TrackingAlertIntentSchema,
  TrackingChildCheckInRequestSchema,
  TrackingChildCheckInResponseSchema,
  TrackingEscalationChainSchema,
  TrackingLocationAiAnalysisResultSchema,
  TrackingLocationPolicyReadModelSchema,
  TrackingMissingDeviceCaseSchema,
  TrackingPlatformProofRouteSchema,
  TrackingPolicyDecisionSchema,
  TrackingPolicyRuleSchema,
  TrackingPolicySchemaVersion,
  TrackingTemporaryLiveTrackingGrantSchema,
  evaluateTrackingAcknowledgementImpact,
  resolveTrackingChildCheckIn,
} from '../src/tracking-location-policy';

const EvidenceTrace = {
  evidenceReferenceId: 'location-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-03T02:00:00.000Z',
} as const;

const Rule = {
  schemaVersion: TrackingPolicySchemaVersion,
  ruleId: 'home-arrival-rule',
  familyId: 'family-1',
  childProfileId: 'child-1',
  deviceId: 'parent-device-1',
  policyVersion: 'tracking-policy-v1',
  targetKind: 'geofence-transition',
  action: 'notify-parent',
  enabled: true,
  requiresFreshEvidence: true,
  requiresParentConfirmation: false,
  reasonCodes: ['home-arrival-notification'],
  auditRefs: ['tracking-rule-created'],
} as const;

const Decision = {
  schemaVersion: TrackingPolicySchemaVersion,
  decisionId: 'tracking-decision-1',
  decidedAt: '2026-06-03T02:01:00.000Z',
  ruleId: 'home-arrival-rule',
  action: 'notify-parent',
  dryRun: false,
  evidenceReferences: [EvidenceTrace],
  aiAnalysisId: 'location-ai-result-1',
  alertIntentId: 'tracking-alert-1',
  reasonCodes: ['policy-rule-matched'],
  auditRefs: ['tracking-decision-recorded'],
} as const;

const ProviderRoute = {
  schemaVersion: TrackingPolicySchemaVersion,
  providerRouteId: 'tracking-ai-route-1',
  mode: 'metadata-only',
  capabilityState: 'available',
  remoteDataAllowed: false,
  unavailableReason: null,
  auditRefs: ['remote-ai-disabled-by-default'],
} as const;

const AiResult = {
  schemaVersion: TrackingPolicySchemaVersion,
  analysisId: 'location-ai-result-1',
  completedAt: '2026-06-03T02:01:00.000Z',
  riskLevel: 'low',
  confidence: 0.74,
  providerRouteId: 'tracking-ai-route-1',
  evidenceReferences: [EvidenceTrace],
  reasonCodes: ['where-expected'],
  canTriggerAlertDirectly: false,
  isFinalAuthority: false,
} as const;

const Alert = {
  schemaVersion: TrackingPolicySchemaVersion,
  alertId: 'tracking-alert-1',
  createdAt: '2026-06-03T02:01:10.000Z',
  severity: 'info',
  policyDecisionId: 'tracking-decision-1',
  evidenceReferences: [EvidenceTrace],
  sensitiveDetailMode: 'minimal-provider-body',
  notificationStatusRefs: ['notification-intent-queued'],
  acknowledgementId: 'tracking-ack-1',
  reasonCodes: ['parent-notification-intent-created'],
} as const;

const Acknowledgement = {
  schemaVersion: TrackingPolicySchemaVersion,
  acknowledgementId: 'tracking-ack-1',
  alertId: 'tracking-alert-1',
  state: 'acknowledged-safe',
  acknowledgedAt: '2026-06-03T02:03:00.000Z',
  expiresAt: null,
  stillAlertForCritical: true,
  reasonCodes: ['parent-confirmed-safe'],
  auditRefs: ['ack-recorded'],
} as const;

const CheckInRequest = {
  schemaVersion: TrackingPolicySchemaVersion,
  checkInId: 'tracking-checkin-1',
  requestedAt: '2026-06-03T02:02:00.000Z',
  state: 'sent',
  relatedAlertId: 'tracking-alert-1',
  includeLocationIfPermitted: true,
  expiresAt: '2026-06-03T02:12:00.000Z',
  evidenceReferences: [EvidenceTrace],
  auditRefs: ['checkin-request-sent'],
} as const;

const CheckInResponse = {
  schemaVersion: TrackingPolicySchemaVersion,
  checkInId: 'tracking-checkin-1',
  respondedAt: '2026-06-03T02:05:00.000Z',
  response: 'safe',
  locationEvidenceReference: EvidenceTrace,
  auditRefs: ['checkin-safe-response'],
} as const;

const Escalation = {
  schemaVersion: TrackingPolicySchemaVersion,
  escalationId: 'tracking-escalation-1',
  alertId: 'tracking-alert-1',
  state: 'waiting-for-parent',
  startedAt: '2026-06-03T02:01:10.000Z',
  nextActionAt: '2026-06-03T02:11:10.000Z',
  steps: ['wait-parent-ack', 'ask-child-check-in'],
  auditRefs: ['escalation-created'],
} as const;

const LiveGrant = {
  schemaVersion: TrackingPolicySchemaVersion,
  grantId: 'tracking-live-grant-1',
  state: 'requested',
  requestedAt: '2026-06-03T02:01:10.000Z',
  expiresAt: '2026-06-03T02:31:10.000Z',
  durationSeconds: 1800,
  parentApproved: true,
  childDisclosureRequired: true,
  auditRefs: ['temporary-live-requested'],
} as const;

const MissingDevice = {
  schemaVersion: TrackingPolicySchemaVersion,
  caseId: 'tracking-missing-device-1',
  openedAt: '2026-06-03T02:01:10.000Z',
  state: 'last-known-only',
  lastKnownEvidence: EvidenceTrace,
  deviceStatusEvidence: EvidenceTrace,
  contactActionRefs: ['contact-child'],
  reasonCodes: ['offline-last-known-only'],
} as const;

const PlatformRoute = {
  schemaVersion: TrackingPolicySchemaVersion,
  platform: 'android',
  foregroundLocation: 'manual-required',
  backgroundLocation: 'real-device-required',
  geofence: 'real-device-required',
  deviceStatus: 'manual-required',
  proofArtifactRefs: ['output/tracking-plan-proof/15-manual-platform-proof.md'],
  manualRequiredReason: 'real-device-permission-proof-required',
} as const;

describe('tracking location policy contracts', () => {
  it('parses policy, AI evidence, and alert contracts', () => {
    const rule = TrackingPolicyRuleSchema.parse(Rule);
    const decision = TrackingPolicyDecisionSchema.parse(Decision);
    const route = TrackingAiProviderRouteSchema.parse(ProviderRoute);
    const aiResult = TrackingLocationAiAnalysisResultSchema.parse(AiResult);
    const alert = TrackingAlertIntentSchema.parse(Alert);

    expect(rule.action).toBe('notify-parent');
    expect(decision.evidenceReferences[0]?.evidenceReferenceId).toBe('location-evidence-1');
    expect(route.remoteDataAllowed).toBe(false);
    expect(aiResult.isFinalAuthority).toBe(false);
    expect(alert.sensitiveDetailMode).toBe('minimal-provider-body');
  });

  it('parses acknowledgement, child check-in, and escalation state', () => {
    const acknowledgement = TrackingAcknowledgementSchema.parse(Acknowledgement);
    const checkInRequest = TrackingChildCheckInRequestSchema.parse(CheckInRequest);
    const checkInResponse = TrackingChildCheckInResponseSchema.parse(CheckInResponse);
    const escalation = TrackingEscalationChainSchema.parse(Escalation);

    expect(acknowledgement.stillAlertForCritical).toBe(true);
    expect(checkInRequest.includeLocationIfPermitted).toBe(true);
    expect(checkInResponse.response).toBe('safe');
    expect(escalation.state).toBe('waiting-for-parent');
  });

  it('parses temporary live, missing device, platform proof, and aggregate read-model state', () => {
    const liveGrant = TrackingTemporaryLiveTrackingGrantSchema.parse(LiveGrant);
    const missingDevice = TrackingMissingDeviceCaseSchema.parse(MissingDevice);
    const platformRoute = TrackingPlatformProofRouteSchema.parse(PlatformRoute);
    const readModel = TrackingLocationPolicyReadModelSchema.parse(policyReadModelSample());

    expect(liveGrant.childDisclosureRequired).toBe(true);
    expect(missingDevice.state).toBe('last-known-only');
    expect(platformRoute.backgroundLocation).toBe('real-device-required');
    expect(readModel.platformProofRoutes[0]?.foregroundLocation).toBe('manual-required');
  });

  it('parses background-permission and unsupported platform route states', () => {
    const route = TrackingPlatformProofRouteSchema.parse({
      ...PlatformRoute,
      backgroundLocation: 'background-permission-required',
      geofence: 'platform-unsupported',
      manualRequiredReason: 'background-permission-required',
    });

    expect(route.backgroundLocation).toBe('background-permission-required');
    expect(route.geofence).toBe('platform-unsupported');
  });
});

describe('tracking location policy negative contracts', () => {
  it('rejects policy actions without evidence and remote AI routes without parent approval', () => {
    const noEvidenceDecision = TrackingPolicyDecisionSchema.safeParse({
      ...Decision,
      evidenceReferences: [],
    });
    const unsafeRemoteRoute = TrackingAiProviderRouteSchema.safeParse({
      ...ProviderRoute,
      mode: 'metadata-only',
      remoteDataAllowed: true,
    });
    const aiAuthority = TrackingLocationAiAnalysisResultSchema.safeParse({
      ...AiResult,
      canTriggerAlertDirectly: true,
    });

    expect(noEvidenceDecision.success).toBe(false);
    expect(unsafeRemoteRoute.success).toBe(false);
    expect(aiAuthority.success).toBe(false);
  });
});

describe('tracking platform proof route guards', () => {
  it('rejects contract-proved platform routes without artifact references', () => {
    const result = TrackingPlatformProofRouteSchema.safeParse({
      ...PlatformRoute,
      foregroundLocation: 'contract-proved',
      proofArtifactRefs: [],
      manualRequiredReason: null,
    });

    expect(result.success).toBe(false);
  });
});

describe('tracking critical acknowledgement guards', () => {
  it('rejects tracking exceptions that would suppress critical alerts', () => {
    const holidayMode = TrackingAcknowledgementSchema.safeParse({
      ...Acknowledgement,
      acknowledgementId: 'tracking-ack-holiday-critical-off',
      state: 'holiday-mode',
      stillAlertForCritical: false,
      reasonCodes: ['holiday-mode-exception'],
    });
    const tripException = TrackingAcknowledgementSchema.safeParse({
      ...Acknowledgement,
      acknowledgementId: 'tracking-ack-trip-critical-off',
      state: 'trip-exception',
      stillAlertForCritical: false,
      reasonCodes: ['trip-exception'],
    });

    expect(holidayMode.success).toBe(false);
    expect(tripException.success).toBe(false);
  });
});

describe('tracking location policy runtime helpers', () => {
  it('evaluates parent acknowledgement and critical alert non-suppression', () => {
    const alert = TrackingAlertIntentSchema.parse(Alert);
    const acknowledgement = TrackingAcknowledgementSchema.parse(Acknowledgement);
    const infoImpact = evaluateTrackingAcknowledgementImpact({
      alert,
      acknowledgement,
      evaluatedAt: '2026-06-03T02:04:00.000Z',
    });
    const criticalImpact = evaluateTrackingAcknowledgementImpact({
      alert: TrackingAlertIntentSchema.parse({
        ...Alert,
        alertId: 'tracking-alert-critical-1',
        severity: 'critical',
      }),
      acknowledgement,
      evaluatedAt: '2026-06-03T02:04:00.000Z',
    });

    expect(infoImpact.suppressesParentAlert).toBe(true);
    expect(infoImpact.state).toBe('suppressed-by-acknowledgement');
    expect(criticalImpact.suppressesParentAlert).toBe(false);
    expect(criticalImpact.state).toBe('critical-still-alert');
  });

  it('keeps critical alerts non-suppressed even if an invalid exception bypasses schema parsing', () => {
    const parsedAcknowledgement = TrackingAcknowledgementSchema.parse({
      ...Acknowledgement,
      state: 'open',
      stillAlertForCritical: false,
      reasonCodes: ['acknowledgement-open'],
    });
    const criticalImpact = evaluateTrackingAcknowledgementImpact({
      alert: TrackingAlertIntentSchema.parse({
        ...Alert,
        alertId: 'tracking-alert-critical-trip-exception-1',
        severity: 'critical',
      }),
      acknowledgement: {
        ...parsedAcknowledgement,
        state: 'trip-exception',
      },
      evaluatedAt: '2026-06-03T02:04:00.000Z',
    });

    expect(criticalImpact.suppressesParentAlert).toBe(false);
    expect(criticalImpact.state).toBe('critical-still-alert');
  });

  it('resolves child check-in response and expiry state without direct enforcement', () => {
    const request = TrackingChildCheckInRequestSchema.parse(CheckInRequest);
    const response = TrackingChildCheckInResponseSchema.parse(CheckInResponse);
    const answered = resolveTrackingChildCheckIn({
      request,
      response,
      evaluatedAt: '2026-06-03T02:06:00.000Z',
    });
    const expired = resolveTrackingChildCheckIn({
      request,
      response: null,
      evaluatedAt: '2026-06-03T02:13:00.000Z',
    });

    expect(answered.state).toBe('answered');
    expect(answered.escalates).toBe(false);
    expect(expired.state).toBe('escalated');
    expect(expired.escalates).toBe(true);
  });
});

function policyReadModelSample() {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt: '2026-06-03T02:06:00.000Z',
    rules: [Rule],
    decisions: [Decision],
    acknowledgements: [Acknowledgement],
    checkInRequests: [CheckInRequest],
    checkInResponses: [CheckInResponse],
    aiRoutes: [ProviderRoute],
    aiResults: [AiResult],
    alerts: [Alert],
    escalations: [Escalation],
    temporaryLiveGrants: [LiveGrant],
    missingDeviceCases: [MissingDevice],
    platformProofRoutes: [PlatformRoute],
  };
}
