import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/schema-domain/evidence-primitives';
import { BrowserUrlAiAnalysisInputSchema, BrowserUrlAiAnalysisResultSchema } from './browser-ai-analysis-schemas';
import {
  BrowserAnalysisJobIdSchema,
  BrowserAnalysisJobStatusSchema,
  type BrowserAnalysisPriority,
  BrowserAnalysisPrioritySchema,
  BrowserAnalysisTimeoutDispositionSchema,
} from '@ocentra-parent/schema-domain/browser-ai-analysis-queue-values';
import { nonEmptyArraySchema, optionalSchema } from './browser-ai-schema-shared';
import {
  browserAnalysisJobIsConsistent,
  browserAnalysisTimeoutPolicyIsConsistent,
  timeoutDispositionFor,
  timeoutMsFor,
} from './browser-ai-analysis-queue-rules';

const PositiveTimeoutMsSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected positive timeout milliseconds')
);
const QueueEvidenceIdsSchema = nonEmptyArraySchema(
  ActivityEvidenceIdSchema,
  'Expected at least one analysis queue evidence id'
);
const OptionalAnalysisResultSchema = optionalSchema(BrowserUrlAiAnalysisResultSchema);

export const BrowserAnalysisQueueSchemaVersion = 1;

const BrowserAnalysisTimeoutPolicyBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAnalysisQueueSchemaVersion),
  priority: BrowserAnalysisPrioritySchema,
  timeoutMs: PositiveTimeoutMsSchema,
  timeoutDisposition: BrowserAnalysisTimeoutDispositionSchema,
});
export const BrowserAnalysisTimeoutPolicySchema = withParser(
  BrowserAnalysisTimeoutPolicyBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAnalysisTimeoutPolicyIsConsistent(value) ||
        'Expected analysis timeout policy to match priority-owned fallback semantics'
    )
  )
);

const BrowserAnalysisJobBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAnalysisQueueSchemaVersion),
  jobId: BrowserAnalysisJobIdSchema,
  queuedAt: ActivityTimestampSchema,
  input: BrowserUrlAiAnalysisInputSchema,
  priority: BrowserAnalysisPrioritySchema,
  status: BrowserAnalysisJobStatusSchema,
  timeoutPolicy: BrowserAnalysisTimeoutPolicySchema,
  queuedEvidenceIds: QueueEvidenceIdsSchema,
  result: OptionalAnalysisResultSchema,
  timeoutPolicyOwnedByParent: Schema.Boolean,
  workerRuntimeClaimed: Schema.Boolean,
  finalPolicyActionClaimed: Schema.Boolean,
  enforcementActionClaimed: Schema.Boolean,
});
export const BrowserAnalysisJobSchema = withParser(
  BrowserAnalysisJobBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAnalysisJobIsConsistent(value) ||
        'Expected analysis queue job to stay parent-timeout-owned and non-authoritative'
    )
  )
);

const BrowserAnalysisJobRequestSchema = withParser(
  Schema.Struct({
    jobId: BrowserAnalysisJobIdSchema,
    queuedAt: ActivityTimestampSchema,
    input: BrowserUrlAiAnalysisInputSchema,
    priority: BrowserAnalysisPrioritySchema,
    queuedEvidenceIds: QueueEvidenceIdsSchema,
  })
);

export const decodeBrowserAnalysisTimeoutPolicy = Schema.decodeUnknownSync(BrowserAnalysisTimeoutPolicySchema);
export const decodeBrowserAnalysisJob = Schema.decodeUnknownSync(BrowserAnalysisJobSchema);

export function browserAnalysisTimeoutPolicyFor(priority: BrowserAnalysisPriority): BrowserAnalysisTimeoutPolicy {
  return BrowserAnalysisTimeoutPolicySchema.parse({
    schemaVersion: BrowserAnalysisQueueSchemaVersion,
    priority,
    timeoutMs: timeoutMsFor(priority),
    timeoutDisposition: timeoutDispositionFor(priority),
  });
}

export function createBrowserAnalysisQueuedJob(
  request: Infer<typeof BrowserAnalysisJobRequestSchema>
): BrowserAnalysisJob {
  const parsed = BrowserAnalysisJobRequestSchema.parse(request);

  return BrowserAnalysisJobSchema.parse({
    schemaVersion: BrowserAnalysisQueueSchemaVersion,
    jobId: parsed.jobId,
    queuedAt: parsed.queuedAt,
    input: parsed.input,
    priority: parsed.priority,
    status: 'queued',
    timeoutPolicy: browserAnalysisTimeoutPolicyFor(parsed.priority),
    queuedEvidenceIds: parsed.queuedEvidenceIds,
    result: null,
    timeoutPolicyOwnedByParent: true,
    workerRuntimeClaimed: false,
    finalPolicyActionClaimed: false,
    enforcementActionClaimed: false,
  });
}

export type BrowserAnalysisTimeoutPolicy = Infer<typeof BrowserAnalysisTimeoutPolicySchema>;
export type BrowserAnalysisJob = Infer<typeof BrowserAnalysisJobSchema>;
