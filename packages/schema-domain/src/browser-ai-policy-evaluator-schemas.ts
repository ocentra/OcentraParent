import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceIdSchema,
  ActivitySubjectIdSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/schema-domain/evidence-primitives';
import {
  BrowserAiConfidenceSchema,
  BrowserParentRuleRefSchema,
  BrowserPolicyVersionRefSchema,
  BrowserScheduleContextRefSchema,
  BrowserUrlAiAnalysisIdSchema,
} from './browser-ai-analysis-values';
import { BrowserUrlAiAnalysisResultSchema } from './browser-ai-analysis-schemas';
import { BrowserKnowledgeGraphRefSchema } from './browser-ai-knowledge-graph-values';
import { BrowserAiMemoryCacheEntryIdSchema } from './browser-ai-memory-cache-store-values';
import { BrowserUrlMetadataEvidenceIdSchema } from './browser-url-metadata-schemas';
import {
  BrowserUrlIntelligenceMemoryHitIdSchema,
  BrowserUrlShapeClassificationIdSchema,
} from './browser-url-intelligence-schemas';
import {
  BrowserAiPolicyEvaluatorRequestIdSchema,
  BrowserPolicyAdapterProofRefSchema,
  BrowserPolicyDecisionAuditRefSchema,
  BrowserPolicyDecisionIdSchema,
  BrowserPolicyDecisionOutcomeSchema,
  type BrowserPolicyDecisionReasonCode,
  BrowserPolicyDecisionReasonCodeSchema,
  BrowserPolicyEvaluatorHandoffStateSchema,
  BrowserPolicyEvaluatorModeSchema,
} from './browser-ai-policy-evaluator-values';

export {
  BrowserPolicyDecisionOutcomeSchema,
  BrowserPolicyDecisionReasonCodeSchema,
  BrowserPolicyEvaluatorHandoffStateSchema,
  BrowserPolicyEvaluatorModeSchema,
};

const EvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected policy evaluator source evidence ids')
);
const ParentRuleRefsSchema = Schema.Array(BrowserParentRuleRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected policy evaluator parent rule refs')
);
const ReasonCodesSchema = Schema.Array(BrowserPolicyDecisionReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected policy decision reason codes')
);
const AuditRefsSchema = Schema.Array(BrowserPolicyDecisionAuditRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected policy decision audit refs')
);
const OptionalAnalysisIdSchema = Schema.Union(BrowserUrlAiAnalysisIdSchema, Schema.Null);
const OptionalAdapterProofRefSchema = Schema.Union(BrowserPolicyAdapterProofRefSchema, Schema.Null);

export const BrowserAiPolicyEvaluatorSchemaVersion = 1;

const BrowserAiPolicyEvaluatorInputBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiPolicyEvaluatorSchemaVersion),
  requestId: BrowserAiPolicyEvaluatorRequestIdSchema,
  requestedAt: ActivityTimestampSchema,
  childProfileRef: ActivitySubjectIdSchema,
  deviceId: ActivityDeviceIdSchema,
  policyVersionRef: BrowserPolicyVersionRefSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  urlShapeClassificationId: BrowserUrlShapeClassificationIdSchema,
  metadataEvidenceIds: Schema.Array(BrowserUrlMetadataEvidenceIdSchema),
  aiResult: BrowserUrlAiAnalysisResultSchema,
  memoryHitIds: Schema.Array(BrowserUrlIntelligenceMemoryHitIdSchema),
  memoryCacheEntryIds: Schema.Array(BrowserAiMemoryCacheEntryIdSchema),
  graphRefs: Schema.Array(BrowserKnowledgeGraphRefSchema),
  parentRuleRefs: ParentRuleRefsSchema,
  scheduleContextRefs: Schema.Array(BrowserScheduleContextRefSchema),
  evaluatorMode: BrowserPolicyEvaluatorModeSchema,
  handoffState: BrowserPolicyEvaluatorHandoffStateSchema,
  rawModelTextIncluded: Schema.Boolean,
  unvalidatedAiOutputIncluded: Schema.Boolean,
  portalUiStateIncluded: Schema.Boolean,
  finalDecisionClaimedByInput: Schema.Boolean,
  directEnforcementClaimedByInput: Schema.Boolean,
});
export const BrowserAiPolicyEvaluatorInputSchema = withParser(
  BrowserAiPolicyEvaluatorInputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiPolicyEvaluatorInputIsConsistent(value) ||
        'Expected policy evaluator input to use validated refs without final authority'
    )
  )
);

const BrowserPolicyDecisionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiPolicyEvaluatorSchemaVersion),
  decisionId: BrowserPolicyDecisionIdSchema,
  requestId: BrowserAiPolicyEvaluatorRequestIdSchema,
  decidedAt: ActivityTimestampSchema,
  policyVersionRef: BrowserPolicyVersionRefSchema,
  sourceEvidenceIds: EvidenceIdsSchema,
  aiAnalysisId: OptionalAnalysisIdSchema,
  memoryHitIds: Schema.Array(BrowserUrlIntelligenceMemoryHitIdSchema),
  graphRefs: Schema.Array(BrowserKnowledgeGraphRefSchema),
  parentRuleRefs: ParentRuleRefsSchema,
  scheduleContextRefs: Schema.Array(BrowserScheduleContextRefSchema),
  outcome: BrowserPolicyDecisionOutcomeSchema,
  evaluatorMode: BrowserPolicyEvaluatorModeSchema,
  confidence: BrowserAiConfidenceSchema,
  reasonCodes: ReasonCodesSchema,
  auditRefs: AuditRefsSchema,
  adapterProofRef: OptionalAdapterProofRefSchema,
  fallbackUsed: Schema.Boolean,
  aiClaimedAsAuthority: Schema.Boolean,
  portalEvaluatedClaimed: Schema.Boolean,
  directEnforcementClaimed: Schema.Boolean,
});
export const BrowserPolicyDecisionSchema = withParser(
  BrowserPolicyDecisionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserPolicyDecisionIsConsistent(value) ||
        'Expected browser policy decision to be deterministic, auditable, and non-enforcing'
    )
  )
);

export const decodeBrowserAiPolicyEvaluatorInput = Schema.decodeUnknownSync(BrowserAiPolicyEvaluatorInputSchema);
export const decodeBrowserPolicyDecision = Schema.decodeUnknownSync(BrowserPolicyDecisionSchema);

export type BrowserAiPolicyEvaluatorInput = Infer<typeof BrowserAiPolicyEvaluatorInputSchema>;
export type BrowserPolicyDecision = Infer<typeof BrowserPolicyDecisionSchema>;

function browserAiPolicyEvaluatorInputIsConsistent(value: Infer<typeof BrowserAiPolicyEvaluatorInputBaseSchema>) {
  if (policyInputAuthorityCreepClaimed(value)) {
    return false;
  }
  if (value.handoffState === 'ready') {
    return value.evaluatorMode !== 'manual_required' && value.evaluatorMode !== 'unavailable';
  }
  return value.evaluatorMode !== 'active';
}

function browserPolicyDecisionIsConsistent(value: Infer<typeof BrowserPolicyDecisionBaseSchema>) {
  if (policyDecisionAuthorityCreepClaimed(value)) {
    return false;
  }
  if (!decisionRefsHaveReasons(value)) {
    return false;
  }
  if (value.outcome === 'block' && value.evaluatorMode === 'active') {
    return value.adapterProofRef !== null;
  }
  if (value.outcome === 'unknown') {
    return unknownDecisionUsesFallback(value);
  }
  return true;
}

function policyInputAuthorityCreepClaimed(value: Infer<typeof BrowserAiPolicyEvaluatorInputBaseSchema>) {
  return (
    value.rawModelTextIncluded ||
    value.unvalidatedAiOutputIncluded ||
    value.portalUiStateIncluded ||
    value.finalDecisionClaimedByInput ||
    value.directEnforcementClaimedByInput ||
    value.aiResult.finalPolicyActionClaimed ||
    value.aiResult.enforcementActionClaimed
  );
}

function policyDecisionAuthorityCreepClaimed(value: Infer<typeof BrowserPolicyDecisionBaseSchema>) {
  return value.aiClaimedAsAuthority || value.portalEvaluatedClaimed || value.directEnforcementClaimed;
}

function decisionRefsHaveReasons(value: Infer<typeof BrowserPolicyDecisionBaseSchema>) {
  if (value.aiAnalysisId !== null && !hasAnyReason(value.reasonCodes, 'ai_high_confidence', 'ai_low_confidence')) {
    return false;
  }
  if (value.memoryHitIds.length > 0 && !value.reasonCodes.includes('memory_hit')) {
    return false;
  }
  if (value.graphRefs.length > 0 && !value.reasonCodes.includes('graph_ref')) {
    return false;
  }
  return true;
}

function unknownDecisionUsesFallback(value: Infer<typeof BrowserPolicyDecisionBaseSchema>) {
  return (
    value.fallbackUsed &&
    hasAnyReason(value.reasonCodes, 'parent_fallback', 'ai_low_confidence', 'unknown_evidence', 'degraded_provider')
  );
}

function hasAnyReason(
  reasonCodes: ReadonlyArray<BrowserPolicyDecisionReasonCode>,
  first: BrowserPolicyDecisionReasonCode,
  second: BrowserPolicyDecisionReasonCode,
  third?: BrowserPolicyDecisionReasonCode,
  fourth?: BrowserPolicyDecisionReasonCode
) {
  return [first, second, third, fourth].some((reason) => reason !== undefined && reasonCodes.includes(reason));
}
