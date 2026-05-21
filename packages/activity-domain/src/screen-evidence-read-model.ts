import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityTimestampSchema } from './primitives';
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
    evidence: Schema.Array(ActivityEvidenceRefSchema),
    results: Schema.Array(ScreenAnalysisResultSchema),
  })
);

export type ScreenEvidenceQueueHealth = Infer<typeof ScreenEvidenceQueueHealthSchema>;
export type ScreenEvidenceRecentSummary = Infer<typeof ScreenEvidenceRecentSummarySchema>;
