import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import { BrowserUrlAiAnalysisInputSchema, BrowserUrlAiAnalysisResultSchema } from './browser-ai-analysis-schemas';
import {
  BrowserAnalysisJobIdSchema,
  BrowserAnalysisJobStatusSchema,
  type BrowserAnalysisPriority,
  BrowserAnalysisPrioritySchema,
  BrowserAnalysisTimeoutDispositionSchema,
} from './browser-ai-analysis-queue-values';

const PositiveTimeoutMsSchema = Schema.Number.pipe(
  Schema.int(),
  Schema.filter((value) => value > 0 || 'Expected positive timeout milliseconds')
);
const QueueEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one analysis queue evidence id')
);
const OptionalAnalysisResultSchema = Schema.Union(BrowserUrlAiAnalysisResultSchema, Schema.Null);

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

function browserAnalysisTimeoutPolicyIsConsistent(value: Infer<typeof BrowserAnalysisTimeoutPolicyBaseSchema>) {
  if (value.priority === 'p0-strict-hold') {
    return value.timeoutMs <= 3000 && value.timeoutDisposition === 'parent-policy-fallback';
  }
  if (value.priority === 'p1-active-unknown-video') {
    return value.timeoutMs <= 15000 && value.timeoutDisposition === 'warn-or-ask';
  }
  if (value.priority === 'p2-active-normal-url') {
    return value.timeoutMs <= 15000 && value.timeoutDisposition === 'background-only';
  }
  return value.timeoutDisposition === 'wait-or-degrade';
}

function browserAnalysisJobIsConsistent(value: Infer<typeof BrowserAnalysisJobBaseSchema>) {
  if (analysisQueueJobClaimsAuthority(value) || value.priority !== value.timeoutPolicy.priority) {
    return false;
  }
  if (value.status === 'completed') {
    return value.result !== null && value.result.requestId === value.input.requestId;
  }
  return value.result === null;
}

function analysisQueueJobClaimsAuthority(value: Infer<typeof BrowserAnalysisJobBaseSchema>) {
  return (
    !value.timeoutPolicyOwnedByParent ||
    value.workerRuntimeClaimed ||
    value.finalPolicyActionClaimed ||
    value.enforcementActionClaimed
  );
}

function timeoutMsFor(priority: BrowserAnalysisPriority) {
  switch (priority) {
    case 'p0-strict-hold':
      return 3000;
    case 'p1-active-unknown-video':
    case 'p2-active-normal-url':
      return 15000;
    case 'p3-background-review':
      return 60000;
    case 'p4-memory-refresh':
      return 120000;
    case 'p5-report-enrichment':
      return 300000;
  }
}

function timeoutDispositionFor(priority: BrowserAnalysisPriority) {
  switch (priority) {
    case 'p0-strict-hold':
      return 'parent-policy-fallback';
    case 'p1-active-unknown-video':
      return 'warn-or-ask';
    case 'p2-active-normal-url':
      return 'background-only';
    case 'p3-background-review':
    case 'p4-memory-refresh':
    case 'p5-report-enrichment':
      return 'wait-or-degrade';
  }
}
