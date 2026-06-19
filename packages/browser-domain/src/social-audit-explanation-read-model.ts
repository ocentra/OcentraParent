import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SocialParentPolicyActionCandidateSchema,
  SocialParentPolicyReasonCodesSchema,
} from './social-policy-compiler-values';
import {
  SocialAuditExplanationAudienceSchema,
  SocialAuditExplanationDecisionStateSchema,
  SocialAuditExplanationEventIdSchema,
  SocialAuditExplanationEvidenceKindSchema,
  type SocialAuditExplanationEvidenceKind,
  SocialAuditExplanationReadModelSchemaVersionSchema,
  SocialAuditExplanationReasonSchema,
  SocialAuditExplanationSnapshotIdSchema,
  type SocialAuditExplanationSubjectKind,
  SocialAuditExplanationSubjectKindSchema,
  SocialAuditExplanationStatusSchema,
} from './social-audit-explanation-read-model-values';

const OptionalAuditEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const SocialAuditExplanationRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social audit/explanation refs')
);
const SocialAuditExplanationReasonsSchema = Schema.Array(SocialAuditExplanationReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social audit/explanation reasons')
);
const SocialAuditExplanationEvidenceLinksSchema = Schema.Array(
  Schema.Struct({
    evidenceKind: SocialAuditExplanationEvidenceKindSchema,
    evidenceRef: ParentEvidenceReferenceIdSchema,
  })
).pipe(Schema.filter((value) => value.length > 0 || 'Expected social audit/explanation evidence links'));

const SocialAuditExplanationEntryBaseSchema = Schema.Struct({
  eventId: SocialAuditExplanationEventIdSchema,
  subjectKind: SocialAuditExplanationSubjectKindSchema,
  status: SocialAuditExplanationStatusSchema,
  decisionState: SocialAuditExplanationDecisionStateSchema,
  audience: SocialAuditExplanationAudienceSchema,
  policyVersionRef: Schema.Union(ParentPolicyVersionSchema, Schema.Null),
  actionCandidate: SocialParentPolicyActionCandidateSchema,
  policyReasonCodes: SocialParentPolicyReasonCodesSchema,
  explanationReasons: SocialAuditExplanationReasonsSchema,
  evidenceLinks: SocialAuditExplanationEvidenceLinksSchema,
  auditRefs: SocialAuditExplanationRefsSchema,
  parentApprovalRequestRef: OptionalAuditEvidenceRefSchema,
  parentApprovalDecisionRef: OptionalAuditEvidenceRefSchema,
  decisionMemoryRef: OptionalAuditEvidenceRefSchema,
  connectorBoundaryRef: OptionalAuditEvidenceRefSchema,
  nativeCapabilityRef: OptionalAuditEvidenceRefSchema,
  manualRequiredRef: OptionalAuditEvidenceRefSchema,
  runtimeAuditStoreClaimed: Schema.Boolean,
  renderedExplanationUiClaimed: Schema.Boolean,
  notificationDeliveredClaimed: Schema.Boolean,
  rawAccountDataIncluded: Schema.Boolean,
  rawVideoContentIncluded: Schema.Boolean,
  rawMessageContentIncluded: Schema.Boolean,
  connectorAuthorizationClaimed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type SocialAuditExplanationEntryCandidate = Infer<typeof SocialAuditExplanationEntryBaseSchema>;

export const SocialAuditExplanationEntrySchema = withParser(
  SocialAuditExplanationEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        socialAuditExplanationEntryIsHonest(entry) ||
        'Expected social audit/explanation row to stay ref-only, readable, and non-enforcing'
    )
  )
);

export const SocialAuditExplanationClaimBoundariesSchema = withParser(
  Schema.Struct({
    runtimeAuditStore: Schema.Literal('not-claimed'),
    renderedExplanationUi: Schema.Literal('not-claimed'),
    notificationDelivery: Schema.Literal('not-claimed'),
    rawAccountVideoMessageContent: Schema.Literal('not-claimed'),
    connectorAuthorization: Schema.Literal('not-claimed'),
    nativeAppControl: Schema.Literal('not-claimed'),
    finalPolicyDecision: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
  })
);

const SocialAuditExplanationSnapshotBaseSchema = Schema.Struct({
  schemaVersion: SocialAuditExplanationReadModelSchemaVersionSchema,
  snapshotId: SocialAuditExplanationSnapshotIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  capturedAt: ParentTimestampSchema,
  entries: Schema.Array(SocialAuditExplanationEntrySchema),
  claimBoundaries: SocialAuditExplanationClaimBoundariesSchema,
});

type SocialAuditExplanationSnapshotCandidate = Infer<typeof SocialAuditExplanationSnapshotBaseSchema>;

export const SocialAuditExplanationSnapshotSchema = withParser(
  SocialAuditExplanationSnapshotBaseSchema.pipe(
    Schema.filter(
      (snapshot) =>
        socialAuditExplanationSnapshotIsHonest(snapshot) ||
        'Expected social audit/explanation snapshot to include all required readable audit rows'
    )
  )
);

export const decodeSocialAuditExplanationSnapshot = Schema.decodeUnknownSync(SocialAuditExplanationSnapshotSchema);

export type SocialAuditExplanationEntry = Infer<typeof SocialAuditExplanationEntrySchema>;
export type SocialAuditExplanationSnapshot = Infer<typeof SocialAuditExplanationSnapshotSchema>;

const RequiredSocialAuditExplanationSubjects = [
  'account-approval',
  'feed-video-gate',
  'native-app-gap',
  'connector-boundary',
  'decision-memory',
  'manual-required-gap',
] as const satisfies ReadonlyArray<SocialAuditExplanationSubjectKind>;

function socialAuditExplanationSnapshotIsHonest(snapshot: SocialAuditExplanationSnapshotCandidate): boolean {
  const subjectKinds = new Set(snapshot.entries.map((entry) => entry.subjectKind));
  return (
    subjectKinds.size === snapshot.entries.length &&
    RequiredSocialAuditExplanationSubjects.every((subjectKind) => subjectKinds.has(subjectKind))
  );
}

function socialAuditExplanationEntryIsHonest(entry: SocialAuditExplanationEntryCandidate): boolean {
  if (socialAuditExplanationEntryClaimsRuntime(entry)) {
    return false;
  }
  if (entry.status === 'ready-for-parent' && !socialAuditExplanationReadyRowIsComplete(entry)) {
    return false;
  }
  if (entry.subjectKind === 'account-approval') {
    return socialAuditExplanationAccountRowIsComplete(entry);
  }
  if (entry.subjectKind === 'feed-video-gate') {
    return socialAuditExplanationFeedVideoRowIsComplete(entry);
  }
  if (entry.subjectKind === 'decision-memory') {
    return socialAuditExplanationDecisionMemoryRowIsComplete(entry);
  }
  return socialAuditExplanationManualBoundaryRowIsComplete(entry);
}

function socialAuditExplanationReadyRowIsComplete(entry: SocialAuditExplanationEntryCandidate): boolean {
  return (
    entry.policyVersionRef !== null &&
    entry.decisionState !== 'manual-required' &&
    entry.decisionState !== 'unavailable' &&
    socialAuditExplanationHasEvidence(entry, 'policy-candidate')
  );
}

function socialAuditExplanationAccountRowIsComplete(entry: SocialAuditExplanationEntryCandidate): boolean {
  return (
    entry.parentApprovalRequestRef !== null &&
    entry.parentApprovalDecisionRef !== null &&
    socialAuditExplanationHasEvidence(entry, 'parent-approval')
  );
}

function socialAuditExplanationFeedVideoRowIsComplete(entry: SocialAuditExplanationEntryCandidate): boolean {
  return (
    entry.status === 'ready-for-parent' &&
    entry.actionCandidate !== 'unknown-candidate' &&
    socialAuditExplanationHasEvidence(entry, 'route-evidence')
  );
}

function socialAuditExplanationDecisionMemoryRowIsComplete(entry: SocialAuditExplanationEntryCandidate): boolean {
  return (
    entry.status === 'contract-only' &&
    entry.decisionState === 'candidate-only' &&
    entry.decisionMemoryRef !== null &&
    socialAuditExplanationHasEvidence(entry, 'decision-memory')
  );
}

function socialAuditExplanationManualBoundaryRowIsComplete(entry: SocialAuditExplanationEntryCandidate): boolean {
  if (entry.subjectKind === 'native-app-gap') {
    return entry.status === 'manual-required' && entry.nativeCapabilityRef !== null;
  }
  if (entry.subjectKind === 'connector-boundary') {
    return entry.status === 'manual-required' && entry.connectorBoundaryRef !== null;
  }
  return entry.status === 'manual-required' && entry.manualRequiredRef !== null;
}

function socialAuditExplanationHasEvidence(
  entry: SocialAuditExplanationEntryCandidate,
  evidenceKind: SocialAuditExplanationEvidenceKind
): boolean {
  return entry.evidenceLinks.some((evidence) => evidence.evidenceKind === evidenceKind);
}

function socialAuditExplanationEntryClaimsRuntime(entry: SocialAuditExplanationEntryCandidate): boolean {
  return (
    entry.runtimeAuditStoreClaimed ||
    entry.renderedExplanationUiClaimed ||
    entry.notificationDeliveredClaimed ||
    entry.rawAccountDataIncluded ||
    entry.rawVideoContentIncluded ||
    entry.rawMessageContentIncluded ||
    entry.connectorAuthorizationClaimed ||
    entry.nativeAppControlClaimed ||
    entry.finalPolicyDecisionClaimed ||
    entry.enforcementClaimed
  );
}
