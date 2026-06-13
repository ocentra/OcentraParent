import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { LocalAiContextReasonCodeSchema, LocalAiEvidenceContextRefIdSchema } from '@ocentra-parent/ai-domain/local-ai-context';
import { LocalAiRuntimeReferenceIdSchema } from './local-ai-primitives';
import {
  PolicyActionSchema,
  PolicyDecisionHandoffStateSchema,
  PolicyDecisionIdSchema,
  PolicyReasonCodeSchema,
  PolicyRuleIdSchema,
} from '@ocentra-parent/policy-domain/policy';
import { ParentEvidenceReferenceIdSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  ScreenSummaryParentExplanationClaimBoundarySchema,
  ScreenSummaryParentExplanationReasonSchema,
} from './local-ai-screen-summary-parent-explanation';

export const ScreenSummaryParentReadModelTextSchema = NonEmptyStringSchema;
export const ScreenSummaryParentReadModelNonNegativeIntegerSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.greaterThanOrEqualTo(0)
);
export const ScreenSummaryParentExplanationReadModelIdSchema = ScreenSummaryParentReadModelTextSchema.pipe(
  Schema.brand('ScreenSummaryParentExplanationReadModelId')
);
export const ScreenSummaryParentExplanationReadModelRowIdSchema = ScreenSummaryParentReadModelTextSchema.pipe(
  Schema.brand('ScreenSummaryParentExplanationReadModelRowId')
);
export const ScreenSummaryParentExplanationReadModelDisplayStateSchema = withParser(
  Schema.Literal('ready-for-parent-explanation')
);
export const ScreenSummaryParentExplanationReadModelClaimBoundarySchema = withParser(
  Schema.Struct({
    rawImageShown: Schema.Literal(false),
    rawImageRetained: Schema.Literal(false),
    remoteAiUsed: Schema.Literal(false),
    apiAiUsed: Schema.Literal(false),
    policyAuthorityClaimed: Schema.Literal(false),
    portalRuntimeClaimed: Schema.Literal(false),
    enforcementClaimed: Schema.Literal(false),
  })
);

export const ScreenSummaryParentReadModelScreenRefsSchema = Schema.Array(LocalAiEvidenceContextRefIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected parent read-model screen-summary refs')
);
export const ScreenSummaryParentReadModelEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected parent read-model evidence refs')
);
export const ScreenSummaryParentReadModelPolicyRulesSchema = Schema.Array(PolicyRuleIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected parent read-model rule refs')
);
export const ScreenSummaryParentReadModelPolicyReasonsSchema = Schema.Array(PolicyReasonCodeSchema).pipe(
  Schema.filter((reasons) => reasons.length > 0 || 'Expected parent read-model policy reasons')
);
export const ScreenSummaryParentReadModelRuntimeRefsSchema = Schema.Array(LocalAiRuntimeReferenceIdSchema).pipe(
  Schema.filter((refs) => refs.length > 0 || 'Expected parent read-model runtime refs')
);
export const ScreenSummaryParentReadModelDeletionReasonsSchema = Schema.Array(LocalAiContextReasonCodeSchema).pipe(
  Schema.filter((reasons) => reasons.includes('screen-image-deleted') || 'Expected deleted-image reason')
);
export const ScreenSummaryParentReadModelExplanationReasonsSchema = Schema.Array(
  ScreenSummaryParentExplanationReasonSchema
).pipe(Schema.filter((reasons) => reasons.length > 0 || 'Expected parent explanation reasons'));

const ScreenSummaryParentExplanationReadModelSourceRowBaseSchema = Schema.Struct({
  ocrResultId: ScreenSummaryParentReadModelTextSchema,
  sourceQueueJobId: ScreenSummaryParentReadModelTextSchema,
  primaryCategory: ScreenSummaryParentReadModelTextSchema,
  imageDigest: ScreenSummaryParentReadModelTextSchema,
  sourceImageDeletionState: Schema.Literal('deleted'),
  sourceCustodyState: Schema.Literal('child-device-query-store'),
  sourceRawImageRetained: Schema.Literal(false),
  contextState: Schema.Literal('ready'),
  readiness: Schema.Literal('ready-for-parent-audit'),
  screenSummaryRefs: ScreenSummaryParentReadModelScreenRefsSchema,
  auditEvidenceReferences: ScreenSummaryParentReadModelEvidenceRefsSchema,
  policyDecisionRef: PolicyDecisionIdSchema,
  policyAction: PolicyActionSchema,
  policyReasonCodes: ScreenSummaryParentReadModelPolicyReasonsSchema,
  policyDryRun: Schema.Literal(true),
  enforcementHandoffState: PolicyDecisionHandoffStateSchema,
  parentRuleRefs: ScreenSummaryParentReadModelPolicyRulesSchema,
  localModelRuntimeRefs: ScreenSummaryParentReadModelRuntimeRefsSchema,
  custodyLabels: Schema.Array(ScreenSummaryParentReadModelTextSchema),
  deletionReasons: ScreenSummaryParentReadModelDeletionReasonsSchema,
  explanationReasons: ScreenSummaryParentReadModelExplanationReasonsSchema,
  claimBoundaries: ScreenSummaryParentExplanationClaimBoundarySchema,
});

export type ScreenSummaryParentExplanationReadModelSourceRow = Infer<
  typeof ScreenSummaryParentExplanationReadModelSourceRowBaseSchema
>;

export const ScreenSummaryParentExplanationReadModelSourceRowSchema = withParser(
  ScreenSummaryParentExplanationReadModelSourceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenSummaryParentExplanationReadModelSourceRowIsReady(row) ||
        'Expected ready deleted local-only parent explanation proof row'
    )
  )
);

function screenSummaryParentExplanationReadModelSourceRowIsReady(
  row: ScreenSummaryParentExplanationReadModelSourceRow
): boolean {
  return (
    row.policyDryRun &&
    row.enforcementHandoffState !== 'handed-off' &&
    row.custodyLabels.includes('child-device-query-store') &&
    row.deletionReasons.includes('screen-image-deleted')
  );
}

