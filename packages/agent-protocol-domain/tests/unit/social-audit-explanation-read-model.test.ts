import { describe, expect, it } from 'vitest';
import { SocialAuditExplanationSnapshotSchema } from '@ocentra-parent/social-domain/social-audit-explanation-read-model';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '../../src/primitives';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const SocialAuditExplanationSnapshot = {
  schemaVersion: 'social-audit-explanation-read-model',
  snapshotId: 'social-audit-explanation-service-snapshot',
  familyId: 'family-social-audit-service',
  childProfileId: 'child-social-audit-service',
  capturedAt: '2026-06-06T06:52:00.000Z',
  entries: [
    accountApprovalEntry(),
    feedVideoGateEntry(),
    nativeAppGapEntry(),
    connectorBoundaryEntry(),
    decisionMemoryEntry(),
    manualRequiredGapEntry(),
  ],
  claimBoundaries: {
    runtimeAuditStore: 'not-claimed',
    renderedExplanationUi: 'not-claimed',
    notificationDelivery: 'not-claimed',
    rawAccountVideoMessageContent: 'not-claimed',
    connectorAuthorization: 'not-claimed',
    nativeAppControl: 'not-claimed',
    finalPolicyDecision: 'not-claimed',
    enforcement: 'not-claimed',
  },
} as const;

describe('agent social audit explanation read-model payload', () => {
  it('parses the dedicated social audit explanation read-model event payload', () => {
    const parsed = parseSocialAuditExplanationEvent(
      socialAuditExplanationEvent(JSON.stringify(SocialAuditExplanationSnapshot))
    );

    expect(parsed).toEqual({
      ok: true,
      value: SocialAuditExplanationSnapshot,
    });
  });

  it('rejects wrong events, invalid json, and hidden runtime claims', () => {
    expect(
      parseSocialAuditExplanationEvent({
        ...socialAuditExplanationEvent(JSON.stringify(SocialAuditExplanationSnapshot)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseSocialAuditExplanationEvent(socialAuditExplanationEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseSocialAuditExplanationEvent(
        socialAuditExplanationEvent(
          JSON.stringify({
            ...SocialAuditExplanationSnapshot,
            entries: SocialAuditExplanationSnapshot.entries.map((row) =>
              row.subjectKind === 'feed-video-gate' ? { ...row, finalPolicyDecisionClaimed: true } : row
            ),
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

type ParseResult =
  | {
      readonly ok: true;
      readonly value: unknown;
    }
  | {
      readonly ok: false;
      readonly reason: 'wrong-event' | 'missing-json-field' | 'invalid-json' | 'invalid-payload';
    };

function parseSocialAuditExplanationEvent(event: AgentEventEnvelope): ParseResult {
  if (event.event !== AgentEvent.BrowserSocialAuditExplanationReadModelReported) {
    return { ok: false, reason: 'wrong-event' };
  }
  const raw = event.payload[AgentProtocolDefaults.Field.BrowserSocialAuditExplanationReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-json-field' };
  }
  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return { ok: false, reason: 'invalid-json' };
  }
  const parsed = SocialAuditExplanationSnapshotSchema.safeParse(decoded);
  return parsed.success ? { ok: true, value: parsed.data } : { ok: false, reason: 'invalid-payload' };
}

function socialAuditExplanationEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'browser-social-audit-explanation-event',
    correlationId: 'browser-social-audit-explanation-command',
    sentAt: '2026-06-06T06:52:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.BrowserSocialAuditExplanationReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.BrowserSocialAuditExplanationReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}

function accountApprovalEntry() {
  return auditEntry('account-approval', {
    eventId: 'social-audit-account-approval',
    decisionState: 'parent-recorded',
    actionCandidate: 'parent-review-candidate',
    evidenceLinks: evidenceLinks('policy-candidate', 'parent-approval'),
    explanationReasons: ['evidence-linked', 'policy-candidate-linked', 'parent-decision-linked'],
    parentApprovalRequestRef: 'parent-evidence-approval-request-service',
    parentApprovalDecisionRef: 'parent-evidence-approval-decision-service',
  });
}

function feedVideoGateEntry() {
  return auditEntry('feed-video-gate', {
    eventId: 'social-audit-feed-video-gate',
    actionCandidate: 'warn-candidate',
    policyReasonCodes: ['social-risk-high', 'video-safety-risk'],
    evidenceLinks: evidenceLinks('route-evidence', 'policy-candidate'),
  });
}

function nativeAppGapEntry() {
  return manualEntry('native-app-gap', {
    eventId: 'social-audit-native-gap',
    evidenceLinks: evidenceLinks('native-capability'),
    nativeCapabilityRef: 'parent-evidence-native-capability-service',
    explanationReasons: ['native-app-manual-required', 'missing-runtime-proof'],
  });
}

function connectorBoundaryEntry() {
  return manualEntry('connector-boundary', {
    eventId: 'social-audit-connector-boundary',
    evidenceLinks: evidenceLinks('connector-boundary'),
    connectorBoundaryRef: 'parent-evidence-connector-boundary-service',
    explanationReasons: ['connector-boundary-linked', 'manual-review-required'],
  });
}

function decisionMemoryEntry() {
  return auditEntry('decision-memory', {
    eventId: 'social-audit-decision-memory',
    status: 'contract-only',
    evidenceLinks: evidenceLinks('decision-memory'),
    explanationReasons: ['memory-linked', 'evidence-linked'],
    decisionMemoryRef: 'parent-evidence-decision-memory-service',
  });
}

function manualRequiredGapEntry() {
  return manualEntry('manual-required-gap', {
    eventId: 'social-audit-manual-required-gap',
    evidenceLinks: evidenceLinks('manual-gap'),
    manualRequiredRef: 'parent-evidence-manual-gap-service',
  });
}

function auditEntry(subjectKind: string, overrides: Record<string, unknown>) {
  return {
    eventId: 'social-audit-event',
    subjectKind,
    status: 'ready-for-parent',
    decisionState: 'candidate-only',
    audience: 'parent',
    policyVersionRef: 'policy-version-social-audit-service',
    actionCandidate: 'allow-candidate',
    policyReasonCodes: ['parent-rule-match'],
    explanationReasons: ['evidence-linked', 'policy-candidate-linked'],
    evidenceLinks: evidenceLinks('policy-candidate'),
    auditRefs: ['parent-evidence-social-audit-service'],
    parentApprovalRequestRef: null,
    parentApprovalDecisionRef: null,
    decisionMemoryRef: null,
    connectorBoundaryRef: null,
    nativeCapabilityRef: null,
    manualRequiredRef: null,
    runtimeAuditStoreClaimed: false,
    renderedExplanationUiClaimed: false,
    notificationDeliveredClaimed: false,
    rawAccountDataIncluded: false,
    rawVideoContentIncluded: false,
    rawMessageContentIncluded: false,
    connectorAuthorizationClaimed: false,
    nativeAppControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function manualEntry(subjectKind: string, overrides: Record<string, unknown>) {
  return auditEntry(subjectKind, {
    status: 'manual-required',
    decisionState: 'manual-required',
    actionCandidate: 'manual-review-candidate',
    policyVersionRef: null,
    policyReasonCodes: ['manual-required'],
    explanationReasons: ['manual-review-required'],
    ...overrides,
  });
}

function evidenceLinks(...kinds: readonly string[]) {
  return kinds.map((evidenceKind) => ({
    evidenceKind,
    evidenceRef: `parent-evidence-${evidenceKind}-service`,
  }));
}
