import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  BrowserGameBenefitSignalIdSchema,
  BrowserGameBenefitSignalKindSchema,
  BrowserGameRecommendedPolicyInputSchema,
  BrowserGameRiskBenefitSignalSchemaVersionSchema,
  BrowserGameRiskBenefitSignalSetIdSchema,
  BrowserGameRiskSignalIdSchema,
  BrowserGameRiskSignalKindSchema,
  BrowserGameSignalConfidenceSchema,
  BrowserGameSignalEvidenceRefsSchema,
  BrowserGameSignalSetDegradedStateSchema,
  BrowserGameSignalSeveritySchema,
  BrowserGameSignalSourceKindSchema,
  BrowserGameSignalStateSchema,
  BrowserGameSignalUncertaintyReasonSchema,
} from './browser-game-riskbenefit-signal-values';

const OptionalParentEvidenceRefSchema = Schema.Union(ParentEvidenceReferenceIdSchema, Schema.Null);
const BrowserGameSignalUncertaintyReasonsSchema = Schema.Array(BrowserGameSignalUncertaintyReasonSchema);

const BrowserGameRiskSignalBaseSchema = Schema.Struct({
  signalId: BrowserGameRiskSignalIdSchema,
  kind: BrowserGameRiskSignalKindSchema,
  severity: BrowserGameSignalSeveritySchema,
  state: BrowserGameSignalStateSchema,
  confidence: BrowserGameSignalConfidenceSchema,
  evidenceRefs: BrowserGameSignalEvidenceRefsSchema,
  analysisRef: OptionalParentEvidenceRefSchema,
  rawGamePayloadUsed: Schema.Boolean,
  rawChatContentUsed: Schema.Boolean,
  rawPageBodyUsed: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameRiskSignalCandidate = Infer<typeof BrowserGameRiskSignalBaseSchema>;

const BrowserGameBenefitSignalBaseSchema = Schema.Struct({
  signalId: BrowserGameBenefitSignalIdSchema,
  kind: BrowserGameBenefitSignalKindSchema,
  severity: BrowserGameSignalSeveritySchema,
  state: BrowserGameSignalStateSchema,
  confidence: BrowserGameSignalConfidenceSchema,
  evidenceRefs: BrowserGameSignalEvidenceRefsSchema,
  analysisRef: OptionalParentEvidenceRefSchema,
  rawGamePayloadUsed: Schema.Boolean,
  rawChatContentUsed: Schema.Boolean,
  rawPageBodyUsed: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameBenefitSignalCandidate = Infer<typeof BrowserGameBenefitSignalBaseSchema>;

export const BrowserGameRiskSignalSchema = withParser(
  BrowserGameRiskSignalBaseSchema.pipe(
    Schema.filter((signal) => browserGameRiskSignalIsHonest(signal) || 'Expected bounded browser-game risk signal')
  )
);

export const BrowserGameBenefitSignalSchema = withParser(
  BrowserGameBenefitSignalBaseSchema.pipe(
    Schema.filter(
      (signal) => browserGameBenefitSignalIsHonest(signal) || 'Expected bounded browser-game benefit signal'
    )
  )
);

const BrowserGameRiskSignalsSchema = Schema.Array(BrowserGameRiskSignalSchema);
const BrowserGameBenefitSignalsSchema = Schema.Array(BrowserGameBenefitSignalSchema);

const BrowserGameRiskBenefitSignalSetBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameRiskBenefitSignalSchemaVersionSchema,
  signalSetId: BrowserGameRiskBenefitSignalSetIdSchema,
  familyId: FamilyIdSchema,
  childProfileId: ChildProfileIdSchema,
  deviceId: ParentDeviceIdSchema,
  modeledAt: ParentTimestampSchema,
  sourceEvidenceRefs: BrowserGameSignalEvidenceRefsSchema,
  signalSourceKind: BrowserGameSignalSourceKindSchema,
  analysisRef: OptionalParentEvidenceRefSchema,
  metadataRef: OptionalParentEvidenceRefSchema,
  parentRuleRef: OptionalParentEvidenceRefSchema,
  riskSignals: BrowserGameRiskSignalsSchema,
  benefitSignals: BrowserGameBenefitSignalsSchema,
  recommendedPolicyInput: BrowserGameRecommendedPolicyInputSchema,
  confidence: BrowserGameSignalConfidenceSchema,
  degradedState: BrowserGameSignalSetDegradedStateSchema,
  uncertaintyReasons: BrowserGameSignalUncertaintyReasonsSchema,
  finalPolicyDecisionClaimed: Schema.Boolean,
  runtimeGateExecutedClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  rawGamePayloadUsed: Schema.Boolean,
  rawChatContentUsed: Schema.Boolean,
  rawPageBodyUsed: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
  accountOrPurchaseExecutionClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  cloudFrameAnalysisClaimed: Schema.Boolean,
});

type BrowserGameRiskBenefitSignalSetCandidate = Infer<typeof BrowserGameRiskBenefitSignalSetBaseSchema>;

export const BrowserGameRiskBenefitSignalSetSchema = withParser(
  BrowserGameRiskBenefitSignalSetBaseSchema.pipe(
    Schema.filter(
      (signalSet) =>
        browserGameRiskBenefitSignalSetIsHonest(signalSet) ||
        'Expected browser-game risk/benefit signal set to remain evidence-backed and candidate-only'
    )
  )
);

export const decodeBrowserGameRiskBenefitSignalSet = Schema.decodeUnknownSync(BrowserGameRiskBenefitSignalSetSchema);

export type BrowserGameBenefitSignal = Infer<typeof BrowserGameBenefitSignalSchema>;
export type BrowserGameRiskBenefitSignalSet = Infer<typeof BrowserGameRiskBenefitSignalSetSchema>;
export type BrowserGameRiskSignal = Infer<typeof BrowserGameRiskSignalSchema>;

function browserGameRiskSignalIsHonest(signal: BrowserGameRiskSignalCandidate): boolean {
  if (browserGameSignalClaimsAuthority(signal)) {
    return false;
  }
  if (signal.state === 'candidate') {
    return signal.kind !== 'unknown-risk' && signal.severity !== 'unknown' && signal.confidence !== 'unknown';
  }
  return signal.kind === 'unknown-risk' && signal.severity === 'unknown' && signal.confidence === 'unknown';
}

function browserGameBenefitSignalIsHonest(signal: BrowserGameBenefitSignalCandidate): boolean {
  if (browserGameSignalClaimsAuthority(signal)) {
    return false;
  }
  if (signal.state === 'candidate') {
    return signal.kind !== 'unknown-benefit' && signal.severity !== 'unknown' && signal.confidence !== 'unknown';
  }
  return signal.kind === 'unknown-benefit' && signal.severity === 'unknown' && signal.confidence === 'unknown';
}

function browserGameRiskBenefitSignalSetIsHonest(signalSet: BrowserGameRiskBenefitSignalSetCandidate): boolean {
  if (browserGameSignalSetClaimsAuthority(signalSet) || noSignals(signalSet)) {
    return false;
  }
  if (signalSet.degradedState === 'none') {
    return (
      signalSet.confidence !== 'unknown' &&
      signalSet.uncertaintyReasons.length === 0 &&
      signalSet.signalSourceKind !== 'manual-required' &&
      recommendedPolicyInputIsSupported(signalSet)
    );
  }
  return signalSet.confidence !== 'high' && signalSet.uncertaintyReasons.length > 0;
}

function recommendedPolicyInputIsSupported(signalSet: BrowserGameRiskBenefitSignalSetCandidate): boolean {
  if (signalSet.recommendedPolicyInput === 'allow-candidate') {
    return hasCandidateBenefit(signalSet) && !hasHighRisk(signalSet);
  }
  if (
    signalSet.recommendedPolicyInput === 'block-candidate' ||
    signalSet.recommendedPolicyInput === 'time-limit-candidate' ||
    signalSet.recommendedPolicyInput === 'parent-review-candidate'
  ) {
    return signalSet.riskSignals.length > 0;
  }
  return true;
}

function hasCandidateBenefit(signalSet: BrowserGameRiskBenefitSignalSetCandidate): boolean {
  return signalSet.benefitSignals.some((signal) => signal.state === 'candidate' && signal.kind !== 'unknown-benefit');
}

function hasHighRisk(signalSet: BrowserGameRiskBenefitSignalSetCandidate): boolean {
  return signalSet.riskSignals.some((signal) => signal.state === 'candidate' && signal.severity === 'high');
}

function noSignals(signalSet: BrowserGameRiskBenefitSignalSetCandidate): boolean {
  return signalSet.riskSignals.length + signalSet.benefitSignals.length === 0;
}

function browserGameSignalClaimsAuthority(
  signal: BrowserGameRiskSignalCandidate | BrowserGameBenefitSignalCandidate
): boolean {
  return (
    signal.rawGamePayloadUsed ||
    signal.rawChatContentUsed ||
    signal.rawPageBodyUsed ||
    signal.rawModelTextUsed ||
    signal.accountOrPurchaseExecutionClaimed ||
    signal.cloudFrameAnalysisClaimed ||
    signal.nativeGameControlClaimed ||
    signal.policyDecisionClaimed ||
    signal.enforcementClaimed
  );
}

function browserGameSignalSetClaimsAuthority(signalSet: BrowserGameRiskBenefitSignalSetCandidate): boolean {
  return (
    signalSet.finalPolicyDecisionClaimed ||
    signalSet.runtimeGateExecutedClaimed ||
    signalSet.enforcementClaimed ||
    signalSet.rawGamePayloadUsed ||
    signalSet.rawChatContentUsed ||
    signalSet.rawPageBodyUsed ||
    signalSet.rawModelTextUsed ||
    signalSet.accountOrPurchaseExecutionClaimed ||
    signalSet.nativeGameControlClaimed ||
    signalSet.cloudFrameAnalysisClaimed
  );
}
