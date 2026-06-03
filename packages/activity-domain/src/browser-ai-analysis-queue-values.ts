import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyAnalysisQueueValueText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAnalysisJobIdSchema = withParser(
  NonEmptyAnalysisQueueValueText.pipe(Schema.brand('BrowserAnalysisJobId'))
);

export const BrowserAnalysisPrioritySchema = withParser(
  Schema.Literal(
    'p0-strict-hold',
    'p1-active-unknown-video',
    'p2-active-normal-url',
    'p3-background-review',
    'p4-memory-refresh',
    'p5-report-enrichment'
  )
);
export const BrowserAnalysisJobStatusSchema = withParser(
  Schema.Literal('queued', 'running', 'completed', 'timed-out', 'degraded', 'manual-required', 'cancelled')
);
export const BrowserAnalysisTimeoutDispositionSchema = withParser(
  Schema.Literal('parent-policy-fallback', 'warn-or-ask', 'background-only', 'wait-or-degrade', 'manual-required')
);

export type BrowserAnalysisPriority = Infer<typeof BrowserAnalysisPrioritySchema>;
