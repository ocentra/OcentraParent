import { type Infer, Schema, withParser } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityTimestampSchema } from './evidence-primitives';
import {
  ScreenDeletionStateSchema,
  ScreenEvidenceCustodyStateSchema,
  ScreenQueueStatusSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';
import { ScreenAnalysisResultSchema } from './screen-evidence-result';
import {
  ScreenEvidenceConfidenceSchema,
  ScreenEvidenceCountSchema,
  ScreenEvidenceQueueJobIdSchema,
  ScreenEvidenceResultIdSchema,
  ScreenEvidenceSchemaVersion,
  ScreenEvidenceSummaryTextSchema,
} from './screen-evidence-primitives';
import {
  ScreenPolicyEvidenceActionSchema,
  ScreenPolicyEvidenceRefListSchema,
  ScreenPolicyEvidenceRefSchema,
} from './screen-policy-evidence-chain';

export const ScreenEvidenceQueueHealthSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    custodyState: ScreenEvidenceCustodyStateSchema,
    pendingCount: ScreenEvidenceCountSchema,
    expiredCount: ScreenEvidenceCountSchema,
    deletePendingCount: ScreenEvidenceCountSchema,
    deleteFailedCount: ScreenEvidenceCountSchema,
    latestQueueJobId: Schema.Union(ScreenEvidenceQueueJobIdSchema, Schema.Null),
    latestStatus: Schema.Union(ScreenQueueStatusSchema, Schema.Null),
    lastSuccessfulAnalysisAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
  })
);

export const ScreenEvidenceRecentSummarySchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenEvidenceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    custodyState: ScreenEvidenceCustodyStateSchema,
    limit: ScreenEvidenceCountSchema,
    returned: ScreenEvidenceCountSchema,
    queueHealth: ScreenEvidenceQueueHealthSchema,
    latestResultId: Schema.Union(ScreenEvidenceResultIdSchema, Schema.Null),
    latestSummary: Schema.Union(ScreenEvidenceSummaryTextSchema, Schema.Null),
    latestPrimaryCategory: Schema.Union(ScreenVisibleCategorySchema, Schema.Null),
    latestConfidence: Schema.Union(ScreenEvidenceConfidenceSchema, Schema.Null),
    latestImageDeletionState: Schema.Union(ScreenDeletionStateSchema, Schema.Null),
    latestPolicyEligible: Schema.Union(Schema.Boolean, Schema.Null),
    latestPolicyDecisionRef: Schema.optionalWith(Schema.Union(ScreenPolicyEvidenceRefSchema, Schema.Null), {
      default: () => null,
    }),
    latestPolicyAction: Schema.optionalWith(Schema.Union(ScreenPolicyEvidenceActionSchema, Schema.Null), {
      default: () => null,
    }),
    latestPolicyReasonCodes: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    latestParentRuleRefs: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    latestLocalModelRuntimeRefs: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    latestParentExplanationRefs: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    latestExplanationReasons: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    latestDeletionReasons: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
    results: Schema.Array(ScreenAnalysisResultSchema),
  })
);

export type ScreenEvidenceQueueHealth = Infer<typeof ScreenEvidenceQueueHealthSchema>;
export type ScreenEvidenceRecentSummary = Infer<typeof ScreenEvidenceRecentSummarySchema>;
