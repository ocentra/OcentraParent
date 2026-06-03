import { describe, expect, it } from 'vitest';
import {
  type SocialAuditExplanationSnapshot,
  SocialAuditExplanationSnapshotSchema,
} from '../src/social-audit-explanation-read-model';

describe('social audit explanation read model contracts', () => {
  it('accepts account, feed, native, connector, memory, and manual audit rows', acceptsHonestSnapshot);
  it('rejects snapshots missing required audit/explanation subjects', rejectsMissingSubjects);
  it('rejects ready parent rows without policy or evidence refs', rejectsUnbackedReadyRows);
  it('rejects audit store, UI, raw content, connector, native, policy, and enforcement claims', rejectsRuntimeClaims);
});

function acceptsHonestSnapshot() {
  const parsed = SocialAuditExplanationSnapshotSchema.parse(validSnapshot());

  expect(parsed.schemaVersion).toBe('social-audit-explanation-read-model');
  expect(parsed.entries).toHaveLength(6);
  expect(entryState(parsed, 'feed-video-gate')).toEqual({
    status: 'ready-for-parent',
    decisionState: 'candidate-only',
    actionCandidate: 'warn-candidate',
  });
}

function rejectsMissingSubjects() {
  const snapshot = validSnapshot();

  expect(
    SocialAuditExplanationSnapshotSchema.safeParse({
      ...snapshot,
      entries: snapshot.entries.filter((entry) => entry.subjectKind !== 'manual-required-gap'),
    }).success
  ).toBe(false);
}

function rejectsUnbackedReadyRows() {
  const snapshot = validSnapshot();
  const withoutPolicy = replaceEntry(snapshot, 'feed-video-gate', { policyVersionRef: null });
  const withoutRouteEvidence = replaceEntry(snapshot, 'feed-video-gate', {
    evidenceLinks: evidenceLinks('policy-candidate'),
  });
  const unknownCandidate = replaceEntry(snapshot, 'feed-video-gate', { actionCandidate: 'unknown-candidate' });

  expect(SocialAuditExplanationSnapshotSchema.safeParse({ ...snapshot, entries: withoutPolicy }).success).toBe(false);
  expect(SocialAuditExplanationSnapshotSchema.safeParse({ ...snapshot, entries: withoutRouteEvidence }).success).toBe(
    false
  );
  expect(SocialAuditExplanationSnapshotSchema.safeParse({ ...snapshot, entries: unknownCandidate }).success).toBe(
    false
  );
}

function rejectsRuntimeClaims() {
  const snapshot = validSnapshot();
  const invalidRows = [
    { runtimeAuditStoreClaimed: true },
    { renderedExplanationUiClaimed: true },
    { notificationDeliveredClaimed: true },
    { rawAccountDataIncluded: true },
    { rawVideoContentIncluded: true },
    { rawMessageContentIncluded: true },
    { connectorAuthorizationClaimed: true },
    { nativeAppControlClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(
      SocialAuditExplanationSnapshotSchema.safeParse({
        ...snapshot,
        entries: replaceEntry(snapshot, 'account-approval', invalid),
      }).success
    ).toBe(false);
  }
}

function validSnapshot(): SocialAuditExplanationSnapshot {
  return {
    schemaVersion: 'social-audit-explanation-read-model',
    snapshotId: 'social-audit-explanation-snapshot-1',
    familyId: 'family-social-audit',
    childProfileId: 'child-social-audit',
    capturedAt: '2026-06-03T08:30:00.000Z',
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
    eventId: 'social-audit-account-approval',
    decisionState: 'parent-recorded',
    actionCandidate: 'ask-parent-candidate',
    evidenceLinks: evidenceLinks('policy-candidate', 'parent-approval'),
    explanationReasons: ['evidence-linked', 'policy-candidate-linked', 'parent-decision-linked'],
    parentApprovalRequestRef: 'parent-evidence-approval-request',
    parentApprovalDecisionRef: 'parent-evidence-approval-decision',
  });
}

function feedVideoGateEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return auditEntry('feed-video-gate', {
    eventId: 'social-audit-feed-video-gate',
    actionCandidate: 'warn-candidate',
    policyReasonCodes: ['social-risk-high', 'video-safety-risk'],
    evidenceLinks: evidenceLinks('route-evidence', 'policy-candidate'),
  });
}

function nativeAppGapEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return manualEntry('native-app-gap', {
    eventId: 'social-audit-native-gap',
    evidenceLinks: evidenceLinks('native-capability'),
    nativeCapabilityRef: 'parent-evidence-native-capability',
    explanationReasons: ['native-app-manual-required', 'missing-runtime-proof'],
  });
}

function connectorBoundaryEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return manualEntry('connector-boundary', {
    eventId: 'social-audit-connector-boundary',
    evidenceLinks: evidenceLinks('connector-boundary'),
    connectorBoundaryRef: 'parent-evidence-connector-boundary',
    explanationReasons: ['connector-boundary-linked', 'manual-review-required'],
  });
}

function decisionMemoryEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return auditEntry('decision-memory', {
    eventId: 'social-audit-decision-memory',
    status: 'contract-only',
    evidenceLinks: evidenceLinks('decision-memory'),
    explanationReasons: ['memory-linked', 'evidence-linked'],
    decisionMemoryRef: 'parent-evidence-decision-memory',
  });
}

function manualRequiredGapEntry(): SocialAuditExplanationSnapshot['entries'][number] {
  return manualEntry('manual-required-gap', {
    eventId: 'social-audit-manual-required-gap',
    evidenceLinks: evidenceLinks('manual-gap'),
    manualRequiredRef: 'parent-evidence-manual-gap',
  });
}

function auditEntry(
  subjectKind: SocialAuditExplanationSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<SocialAuditExplanationSnapshot['entries'][number]>
): SocialAuditExplanationSnapshot['entries'][number] {
  return {
    eventId: 'social-audit-event',
    subjectKind,
    status: 'ready-for-parent',
    decisionState: 'candidate-only',
    audience: 'parent',
    policyVersionRef: 'policy-version-social-audit',
    actionCandidate: 'allow-candidate',
    policyReasonCodes: ['parent-rule-match'],
    explanationReasons: ['evidence-linked', 'policy-candidate-linked'],
    evidenceLinks: evidenceLinks('policy-candidate'),
    auditRefs: ['parent-evidence-audit-ref'],
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

function entryState(
  snapshot: SocialAuditExplanationSnapshot,
  subjectKind: SocialAuditExplanationSnapshot['entries'][number]['subjectKind']
) {
  const entry = snapshot.entries.find((candidate) => candidate.subjectKind === subjectKind);
  return {
    status: entry?.status,
    decisionState: entry?.decisionState,
    actionCandidate: entry?.actionCandidate,
  };
}

function replaceEntry(
  snapshot: SocialAuditExplanationSnapshot,
  subjectKind: SocialAuditExplanationSnapshot['entries'][number]['subjectKind'],
  overrides: Partial<SocialAuditExplanationSnapshot['entries'][number]>
) {
  return snapshot.entries.map((entry) => (entry.subjectKind === subjectKind ? { ...entry, ...overrides } : entry));
}
