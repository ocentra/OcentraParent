import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserAiConfidenceSchema,
  BrowserAiDegradedStateSchema,
  BrowserAiRecommendedPolicyInputSchema,
  BrowserAiUncertaintyReasonSchema,
} from './browser-ai-analysis-schemas';
import { BrowserSocialAiAnalysisIdSchema } from './browser-social-ai-analysis-values';
import { BrowserSocialAiAnalysisResultSchema } from './browser-social-ai-analysis-schemas';
import {
  BrowserSocialPlatformSchema,
  BrowserSocialRouteEvidenceIdSchema,
  BrowserSocialRouteKindSchema,
} from './browser-social-platform-route-schemas';
import {
  BrowserSocialBenefitSignalIdSchema,
  BrowserSocialBenefitSignalKindSchema,
  BrowserSocialRiskBenefitSignalSchemaVersion,
  BrowserSocialRiskBenefitSignalSetIdSchema,
  BrowserSocialRiskSignalIdSchema,
  BrowserSocialRiskSignalKindSchema,
  BrowserSocialSignalSeveritySchema,
  BrowserSocialSignalSourceKindSchema,
  BrowserSocialSignalStateSchema,
} from './browser-social-riskbenefit-values';

const SignalEvidenceRefsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected social signal evidence refs')
);
const BrowserSocialRiskSignalBaseSchema = Schema.Struct({
  signalId: BrowserSocialRiskSignalIdSchema,
  kind: BrowserSocialRiskSignalKindSchema,
  severity: BrowserSocialSignalSeveritySchema,
  state: BrowserSocialSignalStateSchema,
  confidence: BrowserAiConfidenceSchema,
  evidenceRefs: SignalEvidenceRefsSchema,
  rawMessageContentUsed: Schema.Boolean,
  rawFeedContentUsed: Schema.Boolean,
  rawPageBodyUsed: Schema.Boolean,
  accountIdentityVerifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});
export const BrowserSocialRiskSignalSchema = withParser(
  BrowserSocialRiskSignalBaseSchema.pipe(
    Schema.filter((value) => riskSignalIsConsistent(value) || 'Expected bounded social risk signal')
  )
);
const BrowserSocialBenefitSignalBaseSchema = Schema.Struct({
  signalId: BrowserSocialBenefitSignalIdSchema,
  kind: BrowserSocialBenefitSignalKindSchema,
  severity: BrowserSocialSignalSeveritySchema,
  state: BrowserSocialSignalStateSchema,
  confidence: BrowserAiConfidenceSchema,
  evidenceRefs: SignalEvidenceRefsSchema,
  rawMessageContentUsed: Schema.Boolean,
  rawFeedContentUsed: Schema.Boolean,
  rawPageBodyUsed: Schema.Boolean,
  accountIdentityVerifiedClaimed: Schema.Boolean,
  policyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});
export const BrowserSocialBenefitSignalSchema = withParser(
  BrowserSocialBenefitSignalBaseSchema.pipe(
    Schema.filter((value) => benefitSignalIsConsistent(value) || 'Expected bounded social benefit signal')
  )
);
const RiskSignalsSchema = Schema.Array(BrowserSocialRiskSignalSchema);
const BenefitSignalsSchema = Schema.Array(BrowserSocialBenefitSignalSchema);

const BrowserSocialRiskBenefitSignalSetBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserSocialRiskBenefitSignalSchemaVersion),
  signalSetId: BrowserSocialRiskBenefitSignalSetIdSchema,
  modeledAt: ActivityTimestampSchema,
  sourceEvidenceIds: SignalEvidenceRefsSchema,
  socialAiAnalysisId: BrowserSocialAiAnalysisIdSchema,
  socialRouteEvidenceId: BrowserSocialRouteEvidenceIdSchema,
  platform: BrowserSocialPlatformSchema,
  routeKind: BrowserSocialRouteKindSchema,
  signalSourceKind: BrowserSocialSignalSourceKindSchema,
  riskSignals: RiskSignalsSchema,
  benefitSignals: BenefitSignalsSchema,
  recommendedPolicyInput: BrowserAiRecommendedPolicyInputSchema,
  confidence: BrowserAiConfidenceSchema,
  degradedState: BrowserAiDegradedStateSchema,
  uncertaintyReasons: Schema.Array(BrowserAiUncertaintyReasonSchema),
  finalPolicyDecisionClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
  rawModelTextUsed: Schema.Boolean,
  rawMessageContentUsed: Schema.Boolean,
  rawFeedContentUsed: Schema.Boolean,
  nativeAppControlClaimed: Schema.Boolean,
  platformConnectorClaimed: Schema.Boolean,
});
export const BrowserSocialRiskBenefitSignalSetSchema = withParser(
  BrowserSocialRiskBenefitSignalSetBaseSchema.pipe(
    Schema.filter(
      (value) =>
        socialRiskBenefitSignalSetIsConsistent(value) ||
        'Expected social risk/benefit signal set to remain candidate-only and evidence-backed'
    )
  )
);

const BrowserSocialRiskBenefitSignalSetRequestSchema = withParser(
  Schema.Struct({
    signalSetId: BrowserSocialRiskBenefitSignalSetIdSchema,
    modeledAt: ActivityTimestampSchema,
    socialAiAnalysisResult: BrowserSocialAiAnalysisResultSchema,
    signalSourceKind: BrowserSocialSignalSourceKindSchema,
    riskSignals: RiskSignalsSchema,
    benefitSignals: BenefitSignalsSchema,
  })
);

export const decodeBrowserSocialRiskBenefitSignalSet = Schema.decodeUnknownSync(
  BrowserSocialRiskBenefitSignalSetSchema
);

export type BrowserSocialRiskBenefitSignalSet = Infer<typeof BrowserSocialRiskBenefitSignalSetSchema>;
export type BrowserSocialRiskBenefitSignalSetRequest = Infer<typeof BrowserSocialRiskBenefitSignalSetRequestSchema>;

export function buildBrowserSocialRiskBenefitSignalSet(
  request: BrowserSocialRiskBenefitSignalSetRequest
): BrowserSocialRiskBenefitSignalSet {
  const parsed = BrowserSocialRiskBenefitSignalSetRequestSchema.parse(request);
  const analysis = parsed.socialAiAnalysisResult;

  return BrowserSocialRiskBenefitSignalSetSchema.parse({
    schemaVersion: BrowserSocialRiskBenefitSignalSchemaVersion,
    signalSetId: parsed.signalSetId,
    modeledAt: parsed.modeledAt,
    sourceEvidenceIds: analysis.sourceEvidenceIds,
    socialAiAnalysisId: analysis.analysisId,
    socialRouteEvidenceId: analysis.socialRouteEvidenceId,
    platform: analysis.platform,
    routeKind: analysis.routeKind,
    signalSourceKind: parsed.signalSourceKind,
    riskSignals: parsed.riskSignals,
    benefitSignals: parsed.benefitSignals,
    recommendedPolicyInput: analysis.recommendedPolicyInput,
    confidence: analysis.confidence,
    degradedState: analysis.degradedState,
    uncertaintyReasons: analysis.uncertaintyReasons,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    rawModelTextUsed: false,
    rawMessageContentUsed: false,
    rawFeedContentUsed: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
  });
}

function riskSignalIsConsistent(value: Infer<typeof BrowserSocialRiskSignalBaseSchema>) {
  if (signalClaimsAuthority(value)) {
    return false;
  }
  if (value.state === 'candidate') {
    return value.kind !== 'unknown-risk' && value.severity !== 'unknown' && value.confidence !== 'unknown';
  }
  return value.kind === 'unknown-risk' && value.severity === 'unknown' && value.confidence === 'unknown';
}

function benefitSignalIsConsistent(value: Infer<typeof BrowserSocialBenefitSignalBaseSchema>) {
  if (signalClaimsAuthority(value)) {
    return false;
  }
  if (value.state === 'candidate') {
    return value.kind !== 'unknown-benefit' && value.severity !== 'unknown' && value.confidence !== 'unknown';
  }
  return value.kind === 'unknown-benefit' && value.severity === 'unknown' && value.confidence === 'unknown';
}

function signalClaimsAuthority(value: {
  readonly rawMessageContentUsed: boolean;
  readonly rawFeedContentUsed: boolean;
  readonly rawPageBodyUsed: boolean;
  readonly accountIdentityVerifiedClaimed: boolean;
  readonly policyDecisionClaimed: boolean;
  readonly enforcementClaimed: boolean;
}) {
  return (
    value.rawMessageContentUsed ||
    value.rawFeedContentUsed ||
    value.rawPageBodyUsed ||
    value.accountIdentityVerifiedClaimed ||
    value.policyDecisionClaimed ||
    value.enforcementClaimed
  );
}

function socialRiskBenefitSignalSetIsConsistent(value: Infer<typeof BrowserSocialRiskBenefitSignalSetBaseSchema>) {
  if (signalSetClaimsAuthority(value) || value.riskSignals.length + value.benefitSignals.length === 0) {
    return false;
  }
  if (value.degradedState === 'none') {
    return value.confidence !== 'unknown' && value.uncertaintyReasons.length === 0;
  }
  return value.confidence !== 'high' && value.uncertaintyReasons.length > 0;
}

function signalSetClaimsAuthority(value: Infer<typeof BrowserSocialRiskBenefitSignalSetBaseSchema>) {
  return (
    value.finalPolicyDecisionClaimed ||
    value.enforcementClaimed ||
    value.rawModelTextUsed ||
    value.rawMessageContentUsed ||
    value.rawFeedContentUsed ||
    value.nativeAppControlClaimed ||
    value.platformConnectorClaimed
  );
}
