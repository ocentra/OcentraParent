import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentProtocolDefaults,
  parseAgentSocialSourceCustodyMutationEvent,
  SocialSourceCustodyMutationSnapshotSchema,
  type AgentEventEnvelope,
} from '../../src/contracts';

const Timestamp = '2026-06-07T03:56:00Z';

describe('social source custody mutation adapter', () => {
  it('parses a service-applied ref-only custody mutation snapshot', () => {
    const result = parseAgentSocialSourceCustodyMutationEvent(eventWithSnapshot(snapshot()));

    expect(result.ok).toBe(true);
    if (!result.ok) {
      return;
    }
    expect(result.value.serviceMutationExecuted).toBe(true);
    expect(result.value.runtimeCustodyMutationApplied).toBe(true);
    expect(result.value.settings.runtimeCustodyMutationClaimed).toBe(false);
    expect(result.value.finalPolicyDecisionClaimed).toBe(false);
    expect(result.value.enforcementClaimed).toBe(false);
  });

  it('rejects wrong event missing json invalid json and policy or enforcement overclaims', () => {
    expect(
      parseAgentSocialSourceCustodyMutationEvent({ ...eventWithSnapshot(snapshot()), event: AgentEvent.HealthReported })
        .ok
    ).toBe(false);
    expect(parseAgentSocialSourceCustodyMutationEvent(eventWithPayload({})).ok).toBe(false);
    expect(
      parseAgentSocialSourceCustodyMutationEvent(
        eventWithPayload({ [AgentProtocolDefaults.Field.BrowserSocialSourceCustodyMutation]: '{' })
      ).ok
    ).toBe(false);
    expect(
      SocialSourceCustodyMutationSnapshotSchema.safeParse({
        ...snapshot(),
        finalPolicyDecisionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      SocialSourceCustodyMutationSnapshotSchema.safeParse({
        ...snapshot(),
        enforcementClaimed: true,
      }).success
    ).toBe(false);
  });
});

function eventWithSnapshot(value: unknown): AgentEventEnvelope {
  return eventWithPayload({
    [AgentProtocolDefaults.Field.BrowserSocialSourceCustodyMutation]: JSON.stringify(value),
  });
}

function eventWithPayload(payload: AgentEventEnvelope['payload']): AgentEventEnvelope {
  return {
    schemaVersion: 1,
    eventId: 'event-social-source-custody-mutation',
    correlationId: 'command-social-source-custody-mutation',
    sentAt: Timestamp,
    source: {
      peerId: 'agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal',
      role: 'portal',
    },
    event: AgentEvent.BrowserSocialSourceCustodyMutationApplied,
    severity: 'info',
    payload,
    snapshot: null,
  };
}

function snapshot() {
  return {
    schemaVersion: 'social-source-custody-mutation-proof',
    mutationId: 'social-source-custody-mutation-service',
    requestedAt: Timestamp,
    appliedAt: Timestamp,
    mutationState: 'applied',
    settings: {
      schemaVersion: 1,
      settingsId: 'social-source-custody-settings-service',
      generatedAt: Timestamp,
      childProfileRef: 'child-social-source-custody',
      deviceId: 'device-social-source-custody',
      sourcePrivacyEvidenceIds: ['social-video-source-privacy-service'],
      evidenceRefs: ['evidence-social-source-custody-service'],
      settingScope: 'managed-browser-social-route',
      permissionState: 'enabled',
      custodyMode: 'local-redacted-refs-only',
      retentionMode: 'redacted-ref-journal-only',
      permittedDownstreamUses: ['ai-candidate-input', 'parent-explanation'],
      disabledUseReasons: [],
      parentReviewRefs: [],
      connectorAuthorizationRefs: [],
      manualProofRequirements: [],
      noClaimLabels: [
        'raw-message-content-not-allowed',
        'raw-video-content-not-allowed',
        'screenshot-custody-not-allowed',
        'connector-token-not-stored',
        'connector-api-not-called',
        'runtime-settings-ui-not-claimed',
        'runtime-custody-mutation-not-claimed',
        'final-policy-decision-not-claimed',
        'enforcement-not-claimed',
      ],
      rawMessageContentAllowed: false,
      rawVideoContentAllowed: false,
      screenshotCustodyAllowed: false,
      connectorTokenStored: false,
      connectorApiCalled: false,
      runtimeSettingsUiClaimed: false,
      runtimeCustodyMutationClaimed: false,
      finalPolicyDecisionClaimed: false,
      enforcementClaimed: false,
    },
    evidenceRefs: ['evidence-social-source-custody-service'],
    auditRefs: ['audit-social-source-custody-service'],
    serviceMutationExecuted: true,
    runtimeCustodyMutationApplied: true,
    rawContentCustodyClaimed: false,
    connectorApiCalled: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    productClaimReady: false,
  };
}
