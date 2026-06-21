import { describe, expect, it } from 'vitest';
import { createSocialAuditExplanationPanelIntent } from '../../src/social-audit-explanation-panel';
import type { SocialAuditExplanationSnapshot } from '@ocentra-parent/schema-domain/social-audit-explanation-read-model';

describe('social audit explanation panel intent', () => {
  it('renders six schema-backed explanation rows from an honest snapshot', () => {
    const intent = createSocialAuditExplanationPanelIntent(validSnapshot());

    expect(intent.title).toBe('Social explanations');
    expect(intent.summary).toBe('6 social explanation rows');
    expect(intent.rows.map((row) => row.title)).toEqual([
      'Account approval explanation',
      'Feed and video gate explanation',
      'Native app gap explanation',
      'Connected account boundary explanation',
      'Remembered decision explanation',
      'Manual proof gap explanation',
    ]);
    expect(intent.rows[1]?.details.map((detail) => [detail.label, detail.value])).toContainEqual([
      'Decision action',
      'warn-candidate',
    ]);
    expect(intent.rows[1]?.details.map((detail) => [detail.label, String(detail.value)])).toContainEqual([
      'Evidence references',
      'route-evidence:parent-evidence-route-evidence, policy-candidate:parent-evidence-policy-candidate',
    ]);
    expect(intent.productClaim).toContain('runtime audit-store delivery');
    expect(intent.productClaim).toContain('enforcement remain unclaimed');
  });

  it('keeps the panel unavailable for invalid or overclaiming snapshots', () => {
    const snapshot = validSnapshot();
    const overclaiming = {
      ...snapshot,
      entries: snapshot.entries.map((entry) =>
        entry.subjectKind === 'account-approval' ? { ...entry, notificationDeliveredClaimed: true } : entry
      ),
    };

    expect(createSocialAuditExplanationPanelIntent(null).rows).toEqual([]);
    expect(createSocialAuditExplanationPanelIntent(overclaiming).rows).toEqual([]);
    expect(createSocialAuditExplanationPanelIntent(overclaiming).summary).toBe('0 social explanation rows');
  });
});

function validSnapshot(): SocialAuditExplanationSnapshot {
  return {
    schemaVersion: 'social-audit-explanation-read-model',
    snapshotId: 'social-audit-explanation-snapshot-rendered',
    familyId: 'family-social-audit-rendered',
    childProfileId: 'child-social-audit-rendered',
    capturedAt: '2026-06-06T05:20:00.000Z',
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
  };
}

function accountApprovalEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return auditEntry('account-approval', {
    eventId: 'social-audit-account-approval-rendered',
    decisionState: 'parent-recorded',
    actionCandidate: 'parent-review-candidate',
    evidenceLinks: evidenceLinks('policy-candidate', 'parent-approval'),
    explanationReasons: ['evidence-linked', 'policy-candidate-linked', 'parent-decision-linked'],
    parentApprovalRequestRef: 'parent-evidence-approval-request-rendered',
    parentApprovalDecisionRef: 'parent-evidence-approval-decision-rendered',
  });
}

function feedVideoGateEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return auditEntry('feed-video-gate', {
    eventId: 'social-audit-feed-video-gate-rendered',
    actionCandidate: 'warn-candidate',
    policyReasonCodes: ['social-risk-high', 'video-safety-risk'],
    evidenceLinks: evidenceLinks('route-evidence', 'policy-candidate'),
  });
}

function nativeAppGapEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return manualEntry('native-app-gap', {
    eventId: 'social-audit-native-gap-rendered',
    evidenceLinks: evidenceLinks('native-capability'),
    nativeCapabilityRef: 'parent-evidence-native-capability-rendered',
    explanationReasons: ['native-app-manual-required', 'missing-runtime-proof'],
  });
}

function connectorBoundaryEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return manualEntry('connector-boundary', {
    eventId: 'social-audit-connector-boundary-rendered',
    evidenceLinks: evidenceLinks('connector-boundary'),
    connectorBoundaryRef: 'parent-evidence-connector-boundary-rendered',
    explanationReasons: ['connector-boundary-linked', 'manual-review-required'],
  });
}

function decisionMemoryEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return auditEntry('decision-memory', {
    eventId: 'social-audit-decision-memory-rendered',
    status: 'contract-only',
    evidenceLinks: evidenceLinks('decision-memory'),
    explanationReasons: ['memory-linked', 'evidence-linked'],
    decisionMemoryRef: 'parent-evidence-decision-memory-rendered',
  });
}

function manualRequiredGapEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return manualEntry('manual-required-gap', {
    eventId: 'social-audit-manual-required-gap-rendered',
    evidenceLinks: evidenceLinks('manual-gap'),
    manualRequiredRef: 'parent-evidence-manual-gap-rendered',
  });
}

function auditEntry(
  subjectKind: SocialAuditExplanationSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<SocialAuditExplanationSnapshot['entries'][number]>
): SocialAuditExplanationSnapshot['entries'][number] {
  return {
    eventId: 'social-audit-event-rendered',
    subjectKind,
    status: 'ready-for-parent',
    decisionState: 'candidate-only',
    audience: 'parent',
    policyVersionRef: 'policy-version-social-audit-rendered',
    actionCandidate: 'allow-candidate',
    policyReasonCodes: ['parent-rule-match'],
    explanationReasons: ['evidence-linked', 'policy-candidate-linked'],
    evidenceLinks: evidenceLinks('policy-candidate'),
    auditRefs: ['parent-evidence-audit-ref-rendered'],
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

function manualEntry(
  subjectKind: SocialAuditExplanationSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<SocialAuditExplanationSnapshot['entries'][number]>
): SocialAuditExplanationSnapshot['entries'][number] {
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

function evidenceLinks(
  ...kinds: SocialAuditExplanationSnapshot['entries'][number]['evidenceLinks'][number]['evidenceKind'][]
) {
  return kinds.map((evidenceKind) => ({
    evidenceKind,
    evidenceRef: `parent-evidence-${evidenceKind}`,
  }));
}
