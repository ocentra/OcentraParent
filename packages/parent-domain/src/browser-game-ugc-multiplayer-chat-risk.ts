import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  BrowserGameUgcPlatformSurfaceKindSchema,
  BrowserGameUgcRecommendedControlSchema,
  BrowserGameUgcRiskAssessmentIdSchema,
  BrowserGameUgcRiskConfidenceSchema,
  BrowserGameUgcRiskDegradedStateSchema,
  BrowserGameUgcRiskEvidenceKindSchema,
  BrowserGameUgcRiskEvidenceRefsSchema,
  BrowserGameUgcRiskKindSchema,
  BrowserGameUgcRiskRowIdSchema,
  BrowserGameUgcRiskSchemaVersionSchema,
  BrowserGameUgcRiskSeveritySchema,
  BrowserGameUgcRiskStateSchema,
  BrowserGameUgcRiskUncertaintyReasonSchema,
} from './browser-game-ugc-multiplayer-chat-risk-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const BrowserGameUgcRiskUncertaintyReasonsSchema = Schema.Array(BrowserGameUgcRiskUncertaintyReasonSchema);

const BrowserGameUgcRiskRowBaseSchema = Schema.Struct({
  riskRowId: BrowserGameUgcRiskRowIdSchema,
  evidenceKind: BrowserGameUgcRiskEvidenceKindSchema,
  riskKind: BrowserGameUgcRiskKindSchema,
  state: BrowserGameUgcRiskStateSchema,
  severity: BrowserGameUgcRiskSeveritySchema,
  confidence: BrowserGameUgcRiskConfidenceSchema,
  evidenceRefs: BrowserGameUgcRiskEvidenceRefsSchema,
  rawChatContentRead: Schema.Boolean,
  rawProfileContentStored: Schema.Boolean,
  rawExperienceIdentifierStored: Schema.Boolean,
  rawAccountIdentifierStored: Schema.Boolean,
  rawGamePayloadUsed: Schema.Boolean,
  webToAppLaunchExecuted: Schema.Boolean,
  purchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameUgcRiskRowCandidate = Infer<typeof BrowserGameUgcRiskRowBaseSchema>;

export const BrowserGameUgcRiskRowSchema = withParser(
  BrowserGameUgcRiskRowBaseSchema.pipe(
    Schema.filter(
      (value) => browserGameUgcRiskRowIsHonest(value) || 'Expected browser-game UGC risk row to stay evidence-backed'
    )
  )
);

const BrowserGameUgcRiskRowsSchema = Schema.Array(BrowserGameUgcRiskRowSchema);

const BrowserGameUgcRiskAssessmentBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameUgcRiskSchemaVersionSchema,
  assessmentId: BrowserGameUgcRiskAssessmentIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  assessedAt: ParentTimestampSchema,
  platformSurfaceKind: BrowserGameUgcPlatformSurfaceKindSchema,
  sourceEvidenceRefs: BrowserGameUgcRiskEvidenceRefsSchema,
  riskRows: BrowserGameUgcRiskRowsSchema,
  recommendedControl: BrowserGameUgcRecommendedControlSchema,
  confidence: BrowserGameUgcRiskConfidenceSchema,
  degradedState: BrowserGameUgcRiskDegradedStateSchema,
  uncertaintyReasons: BrowserGameUgcRiskUncertaintyReasonsSchema,
  parentRuleRef: OptionalParentEvidenceRefSchema,
  approvedExperienceRef: OptionalParentEvidenceRefSchema,
  chatControlCapabilityRef: OptionalParentEvidenceRefSchema,
  purchaseApprovalCapabilityRef: OptionalParentEvidenceRefSchema,
  mobileCapabilityRef: OptionalParentEvidenceRefSchema,
  rawChatContentRead: Schema.Boolean,
  rawProfileContentStored: Schema.Boolean,
  rawExperienceIdentifierStored: Schema.Boolean,
  rawAccountIdentifierStored: Schema.Boolean,
  rawGamePayloadUsed: Schema.Boolean,
  webToAppLaunchExecuted: Schema.Boolean,
  purchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  uiRenderedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameUgcRiskAssessmentCandidate = Infer<typeof BrowserGameUgcRiskAssessmentBaseSchema>;

export const BrowserGameUgcRiskAssessmentSchema = withParser(
  BrowserGameUgcRiskAssessmentBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserGameUgcRiskAssessmentIsHonest(value) ||
        'Expected browser-game UGC risk assessment to remain candidate-only'
    )
  )
);

export const decodeBrowserGameUgcRiskAssessment = Schema.decodeUnknownSync(BrowserGameUgcRiskAssessmentSchema);

export type BrowserGameUgcRiskAssessment = Infer<typeof BrowserGameUgcRiskAssessmentSchema>;
export type BrowserGameUgcRiskRow = Infer<typeof BrowserGameUgcRiskRowSchema>;

function browserGameUgcRiskRowIsHonest(value: BrowserGameUgcRiskRowCandidate): boolean {
  if (browserGameUgcRiskRowClaimsAuthority(value)) {
    return false;
  }
  if (value.state === 'candidate') {
    return value.riskKind !== 'unknown-risk' && value.riskKind !== 'manual-required' && value.confidence !== 'unknown';
  }
  return (
    value.riskKind === 'unknown-risk' ||
    (value.riskKind === 'manual-required' && value.severity === 'unknown' && value.confidence === 'unknown')
  );
}

function browserGameUgcRiskAssessmentIsHonest(value: BrowserGameUgcRiskAssessmentCandidate): boolean {
  if (browserGameUgcRiskAssessmentClaimsAuthority(value) || value.riskRows.length === 0) {
    return false;
  }
  if (value.degradedState === 'none') {
    return (
      value.confidence !== 'unknown' &&
      value.uncertaintyReasons.length === 0 &&
      value.platformSurfaceKind !== 'manual-required' &&
      value.platformSurfaceKind !== 'unavailable' &&
      recommendedControlIsSupported(value)
    );
  }
  return (
    value.confidence !== 'high' &&
    value.uncertaintyReasons.length > 0 &&
    (value.recommendedControl === 'manual-review-candidate' || value.recommendedControl === 'unknown-candidate')
  );
}

function recommendedControlIsSupported(value: BrowserGameUgcRiskAssessmentCandidate): boolean {
  if (value.recommendedControl === 'approved-experience-only-candidate') {
    return value.approvedExperienceRef !== null || value.parentRuleRef !== null;
  }
  if (value.recommendedControl === 'block-chat-candidate') {
    return value.chatControlCapabilityRef !== null && hasChatRisk(value);
  }
  if (value.recommendedControl === 'purchase-approval-candidate') {
    return value.purchaseApprovalCapabilityRef !== null && hasPurchaseRisk(value);
  }
  if (value.recommendedControl === 'block-unknown-ugc-candidate') {
    return hasUgcRisk(value);
  }
  if (value.recommendedControl === 'time-limit-candidate') {
    return value.parentRuleRef !== null;
  }
  if (value.recommendedControl === 'parent-review-candidate') {
    return value.riskRows.some((row) => row.state === 'candidate');
  }
  return false;
}

function hasChatRisk(value: BrowserGameUgcRiskAssessmentCandidate): boolean {
  return value.riskRows.some((row) => row.riskKind === 'chat-contact' || row.riskKind === 'voice-contact');
}

function hasPurchaseRisk(value: BrowserGameUgcRiskAssessmentCandidate): boolean {
  return value.riskRows.some((row) => row.riskKind === 'virtual-currency' || row.riskKind === 'in-game-purchase');
}

function hasUgcRisk(value: BrowserGameUgcRiskAssessmentCandidate): boolean {
  return value.riskRows.some(
    (row) =>
      row.riskKind === 'ugc-world' ||
      row.riskKind === 'unsafe-user-created-experience' ||
      row.riskKind === 'unknown-player-contact'
  );
}

function browserGameUgcRiskRowClaimsAuthority(value: BrowserGameUgcRiskRowCandidate): boolean {
  return (
    value.rawChatContentRead ||
    value.rawProfileContentStored ||
    value.rawExperienceIdentifierStored ||
    value.rawAccountIdentifierStored ||
    value.rawGamePayloadUsed ||
    value.webToAppLaunchExecuted ||
    value.purchaseExecutionClaimed ||
    value.nativeGameControlClaimed ||
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.enforcementClaimed
  );
}

function browserGameUgcRiskAssessmentClaimsAuthority(value: BrowserGameUgcRiskAssessmentCandidate): boolean {
  return (
    value.rawChatContentRead ||
    value.rawProfileContentStored ||
    value.rawExperienceIdentifierStored ||
    value.rawAccountIdentifierStored ||
    value.rawGamePayloadUsed ||
    value.webToAppLaunchExecuted ||
    value.purchaseExecutionClaimed ||
    value.nativeGameControlClaimed ||
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.uiRenderedClaimed ||
    value.enforcementClaimed
  );
}
