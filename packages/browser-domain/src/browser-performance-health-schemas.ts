import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivitySourceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';

export const BrowserPerformanceHealthSchemaVersion = 1;

export const BrowserPerformanceBudgetIdSchema = withParser(
  Schema.Literal(
    'inventory-scan',
    'support-matrix-derivation',
    'cdp-target-mapping-100-tabs',
    'journal-write-per-event',
    'sqlite-replay-10000-events',
    'portal-render-100-tabs',
    'unmanaged-process-scan',
    'rapid-bridge-reconnect',
    'url-shape-metadata-extraction',
    'local-ai-queue-timeout',
    'memory-cache-lookup-invalidation',
    'browser-game-runtime-signal-collection',
    'cloud-gaming-heuristic-timeout'
  )
);

export const BrowserPerformanceBudgetStateSchema = withParser(
  Schema.Literal('within-budget', 'degraded', 'manual-required', 'unavailable', 'not-started')
);

export const BrowserPerformanceBudgetProofSourceSchema = withParser(
  Schema.Literal('fixture', 'service-health', 'manual-platform', 'not-implemented', 'future-work')
);

export const BrowserPerformanceHealthStateSchema = withParser(
  Schema.Literal('healthy', 'degraded', 'manual-required', 'unavailable')
);

const BrowserPerformanceMillisecondsSchema = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value >= 0) || 'Expected non-negative milliseconds')
);

const BrowserPerformanceSampleSizeSchema = Schema.Number.pipe(
  Schema.filter((value) => (Number.isInteger(value) && value > 0) || 'Expected positive integer sample size')
);

const BrowserPerformanceReasonSchema = Schema.String.pipe(Schema.minLength(1));

const BrowserPerformanceHealthRowBaseSchema = Schema.Struct({
  budgetId: BrowserPerformanceBudgetIdSchema,
  state: BrowserPerformanceBudgetStateSchema,
  proofSource: BrowserPerformanceBudgetProofSourceSchema,
  observedMs: Schema.Union(BrowserPerformanceMillisecondsSchema, Schema.Null),
  budgetMs: BrowserPerformanceMillisecondsSchema,
  sampleSize: BrowserPerformanceSampleSizeSchema,
  degradedReason: Schema.Union(BrowserPerformanceReasonSchema, Schema.Null),
  manualRequiredReason: Schema.Union(BrowserPerformanceReasonSchema, Schema.Null),
  runtimeClaimed: Schema.Boolean,
});

export const BrowserPerformanceHealthRowSchema = withParser(
  BrowserPerformanceHealthRowBaseSchema.pipe(
    Schema.filter(
      (value) => browserPerformanceHealthRowIsConsistent(value) || 'Expected performance row to match proof state'
    )
  )
);

const BrowserPerformanceHealthReadModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserPerformanceHealthSchemaVersion),
  generatedAt: ActivityTimestampSchema,
  sourceId: ActivitySourceIdSchema,
  healthState: BrowserPerformanceHealthStateSchema,
  rows: Schema.Array(BrowserPerformanceHealthRowSchema),
});

export const BrowserPerformanceHealthReadModelSchema = withParser(
  BrowserPerformanceHealthReadModelBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserPerformanceReadModelHealthIsConsistent(value) || 'Expected performance health state to match budget rows'
    )
  )
);

export type BrowserPerformanceBudgetId = Infer<typeof BrowserPerformanceBudgetIdSchema>;
export type BrowserPerformanceBudgetState = Infer<typeof BrowserPerformanceBudgetStateSchema>;
export type BrowserPerformanceBudgetProofSource = Infer<typeof BrowserPerformanceBudgetProofSourceSchema>;
export type BrowserPerformanceHealthState = Infer<typeof BrowserPerformanceHealthStateSchema>;
export type BrowserPerformanceHealthRow = Infer<typeof BrowserPerformanceHealthRowSchema>;
export type BrowserPerformanceHealthReadModel = Infer<typeof BrowserPerformanceHealthReadModelSchema>;

function browserPerformanceHealthRowIsConsistent(value: Infer<typeof BrowserPerformanceHealthRowBaseSchema>) {
  if (value.runtimeClaimed) {
    return false;
  }
  return BrowserPerformanceHealthRowStateValidators[value.state](value);
}

type BrowserPerformanceHealthRowCandidate = Infer<typeof BrowserPerformanceHealthRowBaseSchema>;
type BrowserPerformanceHealthRowValidator = (value: BrowserPerformanceHealthRowCandidate) => boolean;

const BrowserPerformanceHealthRowStateValidators = {
  'within-budget': withinBudgetPerformanceRowIsConsistent,
  degraded: degradedPerformanceRowIsConsistent,
  'manual-required': manualRequiredPerformanceRowIsConsistent,
  unavailable: manualRequiredPerformanceRowIsConsistent,
  'not-started': manualRequiredPerformanceRowIsConsistent,
} satisfies Record<BrowserPerformanceHealthRowCandidate['state'], BrowserPerformanceHealthRowValidator>;

function withinBudgetPerformanceRowIsConsistent(value: BrowserPerformanceHealthRowCandidate): boolean {
  return (
    value.observedMs !== null &&
    value.observedMs <= value.budgetMs &&
    value.degradedReason === null &&
    value.manualRequiredReason === null &&
    MeasuredPerformanceProofSources.includes(value.proofSource)
  );
}

function degradedPerformanceRowIsConsistent(value: BrowserPerformanceHealthRowCandidate): boolean {
  return (
    value.observedMs !== null &&
    value.observedMs > value.budgetMs &&
    value.degradedReason !== null &&
    value.manualRequiredReason === null
  );
}

function manualRequiredPerformanceRowIsConsistent(value: BrowserPerformanceHealthRowCandidate): boolean {
  return (
    value.observedMs === null &&
    value.manualRequiredReason !== null &&
    ManualPerformanceProofSources.includes(value.proofSource)
  );
}

const MeasuredPerformanceProofSources: ReadonlyArray<BrowserPerformanceHealthRowCandidate['proofSource']> = [
  'fixture',
  'service-health',
] as const;

const ManualPerformanceProofSources: ReadonlyArray<BrowserPerformanceHealthRowCandidate['proofSource']> = [
  'manual-platform',
  'not-implemented',
  'future-work',
] as const;

function browserPerformanceReadModelHealthIsConsistent(
  value: Infer<typeof BrowserPerformanceHealthReadModelBaseSchema>
) {
  const degradedRows = value.rows.filter((row) => row.state === 'degraded');
  const unavailableRows = value.rows.filter((row) => row.state === 'unavailable');
  const manualRows = value.rows.filter((row) => row.state === 'manual-required' || row.state === 'not-started');
  const measuredRows = value.rows.filter((row) => row.state === 'within-budget');

  if (value.healthState === 'degraded') {
    return degradedRows.length > 0;
  }
  if (value.healthState === 'unavailable') {
    return degradedRows.length === 0 && unavailableRows.length > 0;
  }
  if (value.healthState === 'manual-required') {
    return degradedRows.length === 0 && unavailableRows.length === 0 && manualRows.length > 0;
  }
  return degradedRows.length === 0 && measuredRows.length > 0;
}
