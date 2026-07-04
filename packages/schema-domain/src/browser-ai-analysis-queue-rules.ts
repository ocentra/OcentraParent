import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  type BrowserAnalysisPriority,
  BrowserAnalysisTimeoutDispositionSchema,
} from '@ocentra-parent/schema-domain/browser-ai-analysis-queue-values';

type TimeoutDisposition = Infer<typeof BrowserAnalysisTimeoutDispositionSchema>;

type TimeoutPolicyShape = {
  readonly priority: BrowserAnalysisPriority;
  readonly timeoutMs: number;
  readonly timeoutDisposition: TimeoutDisposition;
};

type AnalysisJobShape = {
  readonly priority: BrowserAnalysisPriority;
  readonly status: string;
  readonly timeoutPolicy: TimeoutPolicyShape;
  readonly input: { readonly requestId: string };
  readonly result: { readonly requestId: string } | null;
  readonly timeoutPolicyOwnedByParent: boolean;
  readonly workerRuntimeClaimed: boolean;
  readonly finalPolicyActionClaimed: boolean;
  readonly enforcementActionClaimed: boolean;
};

const TimeoutPolicyByPriority = {
  'p0-strict-hold': { timeoutMs: 3000, timeoutDisposition: 'parent-policy-fallback' },
  'p1-active-unknown-video': { timeoutMs: 15000, timeoutDisposition: 'warn-or-ask' },
  'p2-active-normal-url': { timeoutMs: 15000, timeoutDisposition: 'background-only' },
  'p3-background-review': { timeoutMs: 60000, timeoutDisposition: 'wait-or-degrade' },
  'p4-memory-refresh': { timeoutMs: 120000, timeoutDisposition: 'wait-or-degrade' },
  'p5-report-enrichment': { timeoutMs: 300000, timeoutDisposition: 'wait-or-degrade' },
} as const satisfies Record<BrowserAnalysisPriority, { timeoutMs: number; timeoutDisposition: TimeoutDisposition }>;

export function browserAnalysisTimeoutPolicyIsConsistent(value: TimeoutPolicyShape) {
  const expected = TimeoutPolicyByPriority[value.priority];
  return value.timeoutMs <= expected.timeoutMs && value.timeoutDisposition === expected.timeoutDisposition;
}

export function browserAnalysisJobIsConsistent(value: AnalysisJobShape) {
  return (
    !analysisQueueJobClaimsAuthority(value) &&
    value.priority === value.timeoutPolicy.priority &&
    (value.status === 'completed'
      ? value.result !== null && value.result.requestId === value.input.requestId
      : value.result === null)
  );
}

function analysisQueueJobClaimsAuthority(value: AnalysisJobShape) {
  return (
    !value.timeoutPolicyOwnedByParent ||
    value.workerRuntimeClaimed ||
    value.finalPolicyActionClaimed ||
    value.enforcementActionClaimed
  );
}

export function timeoutMsFor(priority: BrowserAnalysisPriority) {
  return TimeoutPolicyByPriority[priority].timeoutMs;
}

export function timeoutDispositionFor(priority: BrowserAnalysisPriority): TimeoutDisposition {
  return TimeoutPolicyByPriority[priority].timeoutDisposition;
}
