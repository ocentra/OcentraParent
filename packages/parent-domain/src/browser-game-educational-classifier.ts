import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  BrowserGameEducationalCategorySchema,
  BrowserGameEducationalClassificationOutcomeSchema,
  BrowserGameEducationalClassifierConfidenceSchema,
  BrowserGameEducationalClassifierDegradedStateSchema,
  BrowserGameEducationalClassifierResultIdSchema,
  BrowserGameEducationalClassifierSchemaVersionSchema,
  BrowserGameEducationalEvidenceKindSchema,
  BrowserGameEducationalEvidenceRefsSchema,
  BrowserGameEducationalEvidenceRowIdSchema,
  BrowserGameEducationalRecommendedGateSchema,
  BrowserGameEducationalUncertaintyReasonSchema,
} from './browser-game-educational-classifier-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const BrowserGameEducationalUncertaintyReasonsSchema = Schema.Array(BrowserGameEducationalUncertaintyReasonSchema);

const BrowserGameEducationalEvidenceRowBaseSchema = Schema.Struct({
  evidenceRowId: BrowserGameEducationalEvidenceRowIdSchema,
  evidenceKind: BrowserGameEducationalEvidenceKindSchema,
  evidenceRefs: BrowserGameEducationalEvidenceRefsSchema,
  confidence: BrowserGameEducationalClassifierConfidenceSchema,
  schoolOrParentVerified: Schema.Boolean,
  platformSelfLabelOnly: Schema.Boolean,
  rawPageBodyUsed: Schema.Boolean,
  rawGamePayloadUsed: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameEducationalEvidenceRowCandidate = Infer<typeof BrowserGameEducationalEvidenceRowBaseSchema>;

export const BrowserGameEducationalEvidenceRowSchema = withParser(
  BrowserGameEducationalEvidenceRowBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGameEducationalEvidenceRowIsHonest(value) ||
        'Expected browser-game educational evidence row to stay reference-only'
    )
  )
);

const BrowserGameEducationalEvidenceRowsSchema = Schema.Array(BrowserGameEducationalEvidenceRowSchema);

const BrowserGameEducationalClassifierResultBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameEducationalClassifierSchemaVersionSchema,
  classifierResultId: BrowserGameEducationalClassifierResultIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  classifiedAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameEducationalEvidenceRefsSchema,
  evidenceRows: BrowserGameEducationalEvidenceRowsSchema,
  category: BrowserGameEducationalCategorySchema,
  outcome: BrowserGameEducationalClassificationOutcomeSchema,
  confidence: BrowserGameEducationalClassifierConfidenceSchema,
  recommendedGate: BrowserGameEducationalRecommendedGateSchema,
  degradedState: BrowserGameEducationalClassifierDegradedStateSchema,
  uncertaintyReasons: BrowserGameEducationalUncertaintyReasonsSchema,
  homeworkContextRef: OptionalParentEvidenceRefSchema,
  parentAllowlistRef: OptionalParentEvidenceRefSchema,
  schoolSourceRef: OptionalParentEvidenceRefSchema,
  aiAnalysisRef: OptionalParentEvidenceRefSchema,
  metadataRef: OptionalParentEvidenceRefSchema,
  rawPageBodyUsed: Schema.Boolean,
  rawGamePayloadUsed: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
  platformLabelTreatedAsAuthority: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameEducationalClassifierResultCandidate = Infer<typeof BrowserGameEducationalClassifierResultBaseSchema>;

export const BrowserGameEducationalClassifierResultSchema = withParser(
  BrowserGameEducationalClassifierResultBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGameEducationalClassifierResultIsHonest(value) ||
        'Expected browser-game educational classifier result to be evidence-backed and candidate-only'
    )
  )
);

export const decodeBrowserGameEducationalClassifierResult = Schema.decodeUnknownSync(
  BrowserGameEducationalClassifierResultSchema
);

export type BrowserGameEducationalClassifierResult = Infer<typeof BrowserGameEducationalClassifierResultSchema>;
export type BrowserGameEducationalEvidenceRow = Infer<typeof BrowserGameEducationalEvidenceRowSchema>;

function browserGameEducationalEvidenceRowIsHonest(value: BrowserGameEducationalEvidenceRowCandidate): boolean {
  if (browserGameEducationalEvidenceRowClaimsAuthority(value)) {
    return false;
  }
  if (value.evidenceKind === 'platform-self-label') {
    return value.platformSelfLabelOnly && !value.schoolOrParentVerified && value.confidence !== 'high';
  }
  if (value.evidenceKind === 'manual-required') {
    return value.confidence === 'unknown' && !value.schoolOrParentVerified;
  }
  return !value.platformSelfLabelOnly && value.confidence !== 'unknown';
}

function browserGameEducationalClassifierResultIsHonest(
  value: BrowserGameEducationalClassifierResultCandidate
): boolean {
  if (browserGameEducationalClassifierResultClaimsAuthority(value) || value.evidenceRows.length === 0) {
    return false;
  }
  if (value.degradedState === 'none') {
    return (
      value.confidence !== 'unknown' &&
      value.uncertaintyReasons.length === 0 &&
      value.outcome !== 'manual-required' &&
      value.outcome !== 'unavailable' &&
      recommendedGateIsSupported(value)
    );
  }
  return (
    value.confidence !== 'high' &&
    value.uncertaintyReasons.length > 0 &&
    value.outcome !== 'educational-candidate' &&
    value.recommendedGate !== 'allow-during-homework-candidate'
  );
}

function recommendedGateIsSupported(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return BrowserGameEducationalOutcomeValidators[value.outcome](value);
}

type BrowserGameEducationalOutcomeValidator = (value: BrowserGameEducationalClassifierResultCandidate) => boolean;
type BrowserGameEducationalRecommendedGate = BrowserGameEducationalClassifierResultCandidate['recommendedGate'];

const BrowserGameEducationalOutcomeValidators = {
  'educational-candidate': educationalCandidateGateIsSupported,
  'misleading-educational-claim': misleadingEducationalClaimGateIsSupported,
  'entertainment-candidate': entertainmentCandidateGateIsSupported,
  'unknown-candidate': unknownCandidateGateIsSupported,
  'manual-required': degradedEducationalGateIsSupported,
  unavailable: degradedEducationalGateIsSupported,
} satisfies Record<BrowserGameEducationalClassifierResultCandidate['outcome'], BrowserGameEducationalOutcomeValidator>;

const BrowserGameEducationalAllowedGates: ReadonlyArray<BrowserGameEducationalRecommendedGate> = [
  'allow-during-homework-candidate',
  'allow-with-time-limit-candidate',
  'parent-review-candidate',
] as const;

const BrowserGameEducationalReviewGates: ReadonlyArray<BrowserGameEducationalRecommendedGate> = [
  'block-portal-candidate',
  'parent-review-candidate',
] as const;

const BrowserGameEducationalDegradedGates: ReadonlyArray<BrowserGameEducationalRecommendedGate> = [
  'manual-review-candidate',
  'unknown-candidate',
] as const;

function educationalCandidateGateIsSupported(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return (
    value.category !== 'unknown-educational-category' &&
    hasTrustedEducationalEvidence(value) &&
    BrowserGameEducationalAllowedGates.includes(value.recommendedGate)
  );
}

function misleadingEducationalClaimGateIsSupported(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return hasPlatformSelfLabelEvidence(value) && BrowserGameEducationalReviewGates.includes(value.recommendedGate);
}

function entertainmentCandidateGateIsSupported(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return BrowserGameEducationalReviewGates.includes(value.recommendedGate);
}

function unknownCandidateGateIsSupported(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return value.category === 'unknown-educational-category' && value.recommendedGate === 'parent-review-candidate';
}

function degradedEducationalGateIsSupported(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return BrowserGameEducationalDegradedGates.includes(value.recommendedGate);
}

function hasTrustedEducationalEvidence(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return value.evidenceRows.some((row) => {
    if (row.platformSelfLabelOnly || row.evidenceKind === 'platform-self-label') {
      return false;
    }
    if (row.schoolOrParentVerified) {
      return true;
    }
    return (
      row.evidenceKind === 'school-provided-url' ||
      row.evidenceKind === 'teacher-allowlist' ||
      row.evidenceKind === 'parent-allowlist' ||
      row.evidenceKind === 'school-platform' ||
      row.evidenceKind === 'past-parent-approval'
    );
  });
}

function hasPlatformSelfLabelEvidence(value: BrowserGameEducationalClassifierResultCandidate): boolean {
  return value.evidenceRows.some((row) => row.evidenceKind === 'platform-self-label' || row.platformSelfLabelOnly);
}

function browserGameEducationalEvidenceRowClaimsAuthority(value: BrowserGameEducationalEvidenceRowCandidate): boolean {
  return (
    value.rawPageBodyUsed ||
    value.rawGamePayloadUsed ||
    value.rawModelTextUsed ||
    value.accountOrPurchaseExecutionClaimed ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed
  );
}

function browserGameEducationalClassifierResultClaimsAuthority(
  value: BrowserGameEducationalClassifierResultCandidate
): boolean {
  return (
    value.rawPageBodyUsed ||
    value.rawGamePayloadUsed ||
    value.rawModelTextUsed ||
    value.platformLabelTreatedAsAuthority ||
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.uiRenderedClaimed ||
    value.accountOrPurchaseExecutionClaimed ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed ||
    value.enforcementClaimed
  );
}
