import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  LocalAiContextReasonCodeSchema,
  LocalAiEvidenceContextBuildResultSchema,
  LocalAiEvidenceContextRefIdSchema,
  type LocalAiEvidenceContextBuildResult,
} from './local-ai-context';
import { LocalAiRuntimeReferenceIdSchema } from './local-ai-primitives';
import {
  PolicyActionSchema,
  PolicyDecisionHandoffStateSchema,
  PolicyDecisionIdSchema,
  PolicyDecisionSchema,
  PolicyReasonCodeSchema,
  PolicyRuleIdSchema,
} from './policy';
import { ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyExplanationText = Schema.String.pipe(Schema.minLength(1));
const NonEmptyEvidenceRefIds = Schema.Array(LocalAiEvidenceContextRefIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected screen-summary explanation refs')
);
const NonEmptyRuntimeRefs = Schema.Array(LocalAiRuntimeReferenceIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected local runtime refs for screen-summary explanation')
);
const NonEmptyPolicyRules = Schema.Array(PolicyRuleIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected parent rule refs for screen-summary explanation')
);
const NonEmptyPolicyReasons = Schema.Array(PolicyReasonCodeSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected policy reason refs for screen-summary explanation')
);
const NonEmptyParentEvidenceRefs = Schema.Array(ParentEvidenceReferenceSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected audit evidence refs for screen-summary explanation')
);
const ScreenSummaryDeletionReasons = Schema.Array(LocalAiContextReasonCodeSchema).pipe(
  Schema.filter((reasons) => reasons.includes('screen-image-deleted') || 'Expected deleted screen-image custody reason')
);

export const ScreenSummaryParentExplanationIdSchema = NonEmptyExplanationText.pipe(
  Schema.brand('ScreenSummaryParentExplanationId')
);

export const ScreenSummaryParentExplanationReadinessSchema = withParser(Schema.Literal('ready-for-parent-audit'));

export const ScreenSummaryParentExplanationReasonSchema = withParser(
  Schema.Literal(
    'screen-summary-evidence-cited',
    'parent-rule-cited',
    'dry-run-policy-cited',
    'image-deleted',
    'local-only-custody',
    'remote-ai-not-used',
    'enforcement-not-claimed'
  )
);

export type ScreenSummaryParentExplanationReason = Infer<typeof ScreenSummaryParentExplanationReasonSchema>;

const requiredExplanationReasons = [
  'screen-summary-evidence-cited',
  'parent-rule-cited',
  'dry-run-policy-cited',
  'image-deleted',
  'local-only-custody',
  'remote-ai-not-used',
  'enforcement-not-claimed',
] as const satisfies ReadonlyArray<ScreenSummaryParentExplanationReason>;

const ScreenSummaryParentExplanationReasonsSchema = Schema.Array(ScreenSummaryParentExplanationReasonSchema).pipe(
  Schema.filter((reasons) => requiredExplanationReasons.every((reason) => reasons.includes(reason)))
);

export const ScreenSummaryParentExplanationClaimBoundarySchema = withParser(
  Schema.Struct({
    rawImageRetained: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
    portalRuntimeClaimed: Schema.Literal(false),
  })
);

const ScreenSummaryParentExplanationInputBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  explanationId: ScreenSummaryParentExplanationIdSchema,
  generatedAt: ParentTimestampSchema,
  contextResult: LocalAiEvidenceContextBuildResultSchema,
  policyDecision: PolicyDecisionSchema,
  claimBoundaries: ScreenSummaryParentExplanationClaimBoundarySchema,
});

type ScreenSummaryParentExplanationInputCandidate = Infer<typeof ScreenSummaryParentExplanationInputBaseSchema>;
type ReadyScreenSummaryContext = NonNullable<ScreenSummaryParentExplanationInputCandidate['contextResult']['context']>;
type ScreenSummaryParentExplanationPolicyDecision = ScreenSummaryParentExplanationInputCandidate['policyDecision'];

export const ScreenSummaryParentExplanationInputSchema = withParser(
  ScreenSummaryParentExplanationInputBaseSchema.pipe(
    Schema.filter(
      (input) =>
        screenSummaryParentExplanationInputIsReady(input) ||
        'Expected ready screen-summary context, dry-run policy decision, deleted image custody, and no remote/enforcement claims'
    )
  )
);

const ScreenSummaryParentExplanationBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  explanationId: ScreenSummaryParentExplanationIdSchema,
  generatedAt: ParentTimestampSchema,
  readiness: ScreenSummaryParentExplanationReadinessSchema,
  sourceContextId: NonEmptyExplanationText,
  sourceRequestId: NonEmptyExplanationText,
  screenSummaryRefs: NonEmptyEvidenceRefIds,
  auditEvidenceReferences: NonEmptyParentEvidenceRefs,
  policyDecisionEvidenceReferences: NonEmptyParentEvidenceRefs,
  parentRuleRefs: NonEmptyPolicyRules,
  policyDecisionRef: PolicyDecisionIdSchema,
  policyAction: PolicyActionSchema,
  policyReasonCodes: NonEmptyPolicyReasons,
  policyDryRun: Schema.Literal(true),
  enforcementHandoffState: PolicyDecisionHandoffStateSchema,
  localModelRuntimeRefs: NonEmptyRuntimeRefs,
  custodyLabels: Schema.Array(NonEmptyExplanationText),
  deletionReasons: ScreenSummaryDeletionReasons,
  explanationReasons: ScreenSummaryParentExplanationReasonsSchema,
  claimBoundaries: ScreenSummaryParentExplanationClaimBoundarySchema,
});

type ScreenSummaryParentExplanationCandidate = Infer<typeof ScreenSummaryParentExplanationBaseSchema>;

export const ScreenSummaryParentExplanationSchema = withParser(
  ScreenSummaryParentExplanationBaseSchema.pipe(
    Schema.filter(
      (explanation) =>
        screenSummaryParentExplanationIsHonest(explanation) ||
        'Expected screen-summary parent explanation to stay local-only, evidence-cited, deleted-image, dry-run, and non-enforcing'
    )
  )
);

export type ScreenSummaryParentExplanationId = typeof ScreenSummaryParentExplanationIdSchema.Type;
export type ScreenSummaryParentExplanationReadiness = Infer<typeof ScreenSummaryParentExplanationReadinessSchema>;
export type ScreenSummaryParentExplanationClaimBoundary = Infer<
  typeof ScreenSummaryParentExplanationClaimBoundarySchema
>;
export type ScreenSummaryParentExplanationInput = Infer<typeof ScreenSummaryParentExplanationInputSchema>;
export type ScreenSummaryParentExplanation = Infer<typeof ScreenSummaryParentExplanationSchema>;

export function buildScreenSummaryParentExplanation(input: unknown): ScreenSummaryParentExplanation {
  const parsed = ScreenSummaryParentExplanationInputSchema.parse(input);
  const context = contextForReadyInput(parsed.contextResult);
  return ScreenSummaryParentExplanationSchema.parse({
    schemaVersion: parsed.schemaVersion,
    explanationId: parsed.explanationId,
    generatedAt: parsed.generatedAt,
    readiness: 'ready-for-parent-audit',
    sourceContextId: context.contextId,
    sourceRequestId: context.requestId,
    screenSummaryRefs: context.screenSummaryRefs,
    auditEvidenceReferences: parsed.contextResult.auditEvidenceReferences,
    policyDecisionEvidenceReferences: parsed.policyDecision.evidenceReferences,
    parentRuleRefs: context.parentRuleReferences,
    policyDecisionRef: parsed.policyDecision.decisionId,
    policyAction: parsed.policyDecision.action,
    policyReasonCodes: parsed.policyDecision.reasonCodes,
    policyDryRun: parsed.policyDecision.dryRun,
    enforcementHandoffState: parsed.policyDecision.enforcementHandoffState,
    localModelRuntimeRefs: context.localModelRuntimeRefs,
    custodyLabels: context.custodyLabels,
    deletionReasons: context.degradedReasons.filter((reason) => reason === 'screen-image-deleted'),
    explanationReasons: requiredExplanationReasons,
    claimBoundaries: parsed.claimBoundaries,
  });
}

function contextForReadyInput(result: LocalAiEvidenceContextBuildResult) {
  if (result.context === null) {
    throw new Error('Expected parsed ready screen-summary context');
  }
  return result.context;
}

function screenSummaryParentExplanationInputIsReady(input: ScreenSummaryParentExplanationInputCandidate): boolean {
  const context = input.contextResult.context;
  if (context === null || input.contextResult.state !== 'ready') {
    return false;
  }
  return (
    screenSummaryContextIsReadyForParentExplanation(context) &&
    input.contextResult.auditEvidenceReferences.length > 0 &&
    policyDecisionIsReadyForParentExplanation(input.policyDecision)
  );
}

function screenSummaryContextIsReadyForParentExplanation(context: ReadyScreenSummaryContext): boolean {
  return (
    context.screenSummaryRefs.length > 0 &&
    context.custodyLabels.includes('child-device-query-store') &&
    !context.custodyLabels.includes('ocentra-hosted-non-activity') &&
    context.degradedReasons.includes('screen-image-deleted') &&
    context.localModelRuntimeRefs.length > 0
  );
}

function policyDecisionIsReadyForParentExplanation(
  policyDecision: ScreenSummaryParentExplanationPolicyDecision
): boolean {
  return (
    policyDecision.dryRun &&
    policyDecision.evidenceReferences.length > 0 &&
    policyDecision.ruleIds.length > 0 &&
    policyDecision.reasonCodes.length > 0 &&
    policyDecisionHandoffIsNonEnforcing(policyDecision.enforcementHandoffState)
  );
}

function policyDecisionHandoffIsNonEnforcing(
  state: ScreenSummaryParentExplanationPolicyDecision['enforcementHandoffState']
): boolean {
  return state === 'disabled' || state === 'not-requested';
}

function screenSummaryParentExplanationIsHonest(explanation: ScreenSummaryParentExplanationCandidate): boolean {
  return (
    explanation.custodyLabels.includes('child-device-query-store') &&
    !explanation.custodyLabels.includes('ocentra-hosted-non-activity') &&
    explanation.deletionReasons.includes('screen-image-deleted') &&
    explanation.policyDryRun &&
    explanation.enforcementHandoffState !== 'handed-off' &&
    explanation.claimBoundaries.rawImageRetained === false &&
    explanation.claimBoundaries.remoteAiUsed === false &&
    explanation.claimBoundaries.apiAiUsed === false &&
    explanation.claimBoundaries.policyAuthorityClaimed === false &&
    explanation.claimBoundaries.enforcementClaimed === false &&
    explanation.claimBoundaries.portalRuntimeClaimed === false
  );
}
