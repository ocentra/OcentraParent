import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { type ActivityEvidenceRef, ActivityEvidenceRefSchema } from './contracts';
import { ActivityTimestampSchema } from './primitives';
import {
  type ActivityAppUseReadModel,
  ActivityReadModelStateSchema,
  type ActivityGamesReadModel,
  ActivitySurfaceSchemaVersion,
} from './activity-surface';
import {
  type AppGameCapabilityStatus,
  AppGameCapabilityStatusSchema,
  AppGameObservationModeSchema,
} from './app-game-primitives';
import { AppGameInventorySourceKindSchema } from './app-game-inventory-primitives';

const NonNegativeInteger = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const PositiveInteger = Schema.Number.pipe(Schema.positive(), Schema.int());
const ParsedNonNegativeInteger = withParser(NonNegativeInteger);
const ParsedActivityTimestamp = withParser(ActivityTimestampSchema);
const ParsedActivityEvidenceRefs = withParser(Schema.Array(ActivityEvidenceRefSchema));
const AppGameSourceQualityText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceFreshnessQualityStateSchema = withParser(
  Schema.Literal('fresh', 'stale', 'missing', 'manual-required', 'unavailable', 'empty')
);
export const AppGameSourceFreshnessQualityReasonSchema = withParser(
  Schema.Literal(
    'observed-recently',
    'older-than-threshold',
    'no-row-for-required-source',
    'ready-row-without-evidence',
    'permission-limited',
    'source-unavailable',
    'source-empty',
    'source-error',
    'source-not-claimed'
  )
);
export const AppGameSourceFreshnessRequiredSourceSchema = withParser(
  Schema.Union(AppGameInventorySourceKindSchema, AppGameObservationModeSchema)
);

export const AppGameSourceFreshnessQualityRowSchema = withParser(
  Schema.Struct({
    sourceKind: AppGameSourceFreshnessRequiredSourceSchema,
    qualityState: AppGameSourceFreshnessQualityStateSchema,
    reason: AppGameSourceFreshnessQualityReasonSchema,
    rowCount: NonNegativeInteger,
    lastObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    staleAfterMs: PositiveInteger,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
    sourceRowsChecked: NonNegativeInteger,
    policyEligible: Schema.Boolean,
    adapterDispatchClaimed: Schema.Literal(false),
  })
);

export const AppGameSourceFreshnessQualityReportSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
    generatedAt: ActivityTimestampSchema,
    staleAfterMs: PositiveInteger,
    summary: Schema.Struct({
      requiredSourceCount: NonNegativeInteger,
      freshSources: NonNegativeInteger,
      staleSources: NonNegativeInteger,
      missingSources: NonNegativeInteger,
      manualRequiredSources: NonNegativeInteger,
      unavailableSources: NonNegativeInteger,
      emptySources: NonNegativeInteger,
      policyEligibleFreshSources: NonNegativeInteger,
      adapterDispatchClaimed: Schema.Literal(false),
    }),
    rows: Schema.Array(AppGameSourceFreshnessQualityRowSchema),
    noClaimBoundary: AppGameSourceQualityText,
  })
);

export type AppGameSourceFreshnessQualityState = Infer<typeof AppGameSourceFreshnessQualityStateSchema>;
export type AppGameSourceFreshnessQualityReason = Infer<typeof AppGameSourceFreshnessQualityReasonSchema>;
export type AppGameSourceFreshnessRequiredSource = Infer<typeof AppGameSourceFreshnessRequiredSourceSchema>;
export type AppGameSourceFreshnessQualityRow = Infer<typeof AppGameSourceFreshnessQualityRowSchema>;
export type AppGameSourceFreshnessQualityReport = Infer<typeof AppGameSourceFreshnessQualityReportSchema>;
export type ActivityAppGameSourceStatusRow =
  | ActivityAppUseReadModel['rows'][number]['sourceStatusRows'][number]
  | ActivityGamesReadModel['rows'][number]['sourceStatusRows'][number];

export type AppGameSourceFreshnessQualityInput = {
  readonly generatedAt: string;
  readonly staleAfterMs: number;
  readonly requiredSources: readonly AppGameSourceFreshnessRequiredSource[];
  readonly sourceStatusRows: readonly ActivityAppGameSourceStatusRow[];
};

export function buildAppGameSourceFreshnessQualityReport(
  input: AppGameSourceFreshnessQualityInput
): AppGameSourceFreshnessQualityReport {
  const generatedAtMs = parseTimestamp(input.generatedAt, 'generatedAt');
  const rows = input.requiredSources.map((sourceKind) =>
    buildQualityRow({
      sourceKind,
      generatedAtMs,
      staleAfterMs: input.staleAfterMs,
      sourceRows: input.sourceStatusRows.filter((row) => row.sourceKind === sourceKind),
    })
  );
  return AppGameSourceFreshnessQualityReportSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    generatedAt: input.generatedAt,
    staleAfterMs: input.staleAfterMs,
    summary: {
      requiredSourceCount: input.requiredSources.length,
      freshSources: countRows(rows, 'fresh'),
      staleSources: countRows(rows, 'stale'),
      missingSources: countRows(rows, 'missing'),
      manualRequiredSources: countRows(rows, 'manual-required'),
      unavailableSources: countRows(rows, 'unavailable'),
      emptySources: countRows(rows, 'empty'),
      policyEligibleFreshSources: rows.filter((row) => row.policyEligible).length,
      adapterDispatchClaimed: false,
    },
    rows,
    noClaimBoundary:
      'Source freshness quality summarizes stored app/game source rows only; it does not execute policy, adapters, timers, blocking, or platform support.',
  });
}

export function parseAppGameSourceFreshnessQualityRows(
  rows: readonly unknown[]
): ReadonlyArray<ActivityAppGameSourceStatusRow> {
  return rows.map(parseSourceStatusRow);
}

function parseSourceStatusRow(row: unknown): ActivityAppGameSourceStatusRow {
  const candidate = row as Partial<ActivityAppGameSourceStatusRow>;
  return {
    sourceKind: AppGameSourceFreshnessRequiredSourceSchema.parse(candidate.sourceKind),
    state: ActivityReadModelStateSchema.parse(candidate.state),
    rowCount: ParsedNonNegativeInteger.parse(candidate.rowCount),
    lastObservedAt: candidate.lastObservedAt === null ? null : ParsedActivityTimestamp.parse(candidate.lastObservedAt),
    capabilityStatus: AppGameCapabilityStatusSchema.parse(candidate.capabilityStatus),
    evidence: ParsedActivityEvidenceRefs.parse(candidate.evidence),
  };
}

function buildQualityRow(input: {
  readonly sourceKind: AppGameSourceFreshnessRequiredSource;
  readonly generatedAtMs: number;
  readonly staleAfterMs: number;
  readonly sourceRows: readonly ActivityAppGameSourceStatusRow[];
}): AppGameSourceFreshnessQualityRow {
  if (input.sourceRows.length === 0) {
    return qualityRow(input, 'missing', 'no-row-for-required-source', 0, null, []);
  }
  const rowCount = input.sourceRows.reduce((total, row) => total + row.rowCount, 0);
  const evidence = input.sourceRows.flatMap((row) => row.evidence);
  const lastObservedAt = latestObservedAt(input.sourceRows);
  const capabilityStatuses = input.sourceRows.map((row) => row.capabilityStatus);
  const states = input.sourceRows.map((row) => row.state);
  if (capabilityStatuses.some(isManualCapability) || states.includes('permission-required')) {
    return qualityRow(input, 'manual-required', 'permission-limited', rowCount, lastObservedAt, evidence);
  }
  if (capabilityStatuses.some(isUnavailableCapability) || states.some(isUnavailableState)) {
    return qualityRow(input, 'unavailable', unavailableReason(capabilityStatuses), rowCount, lastObservedAt, evidence);
  }
  if (rowCount === 0) {
    return qualityRow(input, 'empty', 'source-empty', rowCount, lastObservedAt, evidence);
  }
  if (states.includes('stale') || capabilityStatuses.includes('stale')) {
    return qualityRow(input, 'stale', 'older-than-threshold', rowCount, lastObservedAt, evidence);
  }
  if (
    lastObservedAt === null ||
    input.generatedAtMs - parseTimestamp(lastObservedAt, 'lastObservedAt') > input.staleAfterMs
  ) {
    return qualityRow(input, 'stale', 'older-than-threshold', rowCount, lastObservedAt, evidence);
  }
  if (evidence.length === 0) {
    return qualityRow(input, 'stale', 'ready-row-without-evidence', rowCount, lastObservedAt, evidence);
  }
  return qualityRow(input, 'fresh', 'observed-recently', rowCount, lastObservedAt, evidence);
}

function qualityRow(
  input: {
    readonly sourceKind: AppGameSourceFreshnessRequiredSource;
    readonly staleAfterMs: number;
    readonly sourceRows: readonly ActivityAppGameSourceStatusRow[];
  },
  qualityState: AppGameSourceFreshnessQualityState,
  reason: AppGameSourceFreshnessQualityReason,
  rowCount: number,
  lastObservedAt: string | null,
  evidence: ReadonlyArray<ActivityEvidenceRef>
): AppGameSourceFreshnessQualityRow {
  return AppGameSourceFreshnessQualityRowSchema.parse({
    sourceKind: input.sourceKind,
    qualityState,
    reason,
    rowCount,
    lastObservedAt,
    staleAfterMs: input.staleAfterMs,
    evidence,
    sourceRowsChecked: input.sourceRows.length,
    policyEligible: qualityState === 'fresh',
    adapterDispatchClaimed: false,
  });
}

function latestObservedAt(rows: readonly ActivityAppGameSourceStatusRow[]): string | null {
  const timestamps = rows
    .reduce<string[]>((values, row) => {
      if (row.lastObservedAt !== null) {
        values.push(row.lastObservedAt);
      }
      return values;
    }, [])
    .sort((left, right) => parseTimestamp(right, 'lastObservedAt') - parseTimestamp(left, 'lastObservedAt'));
  return timestamps[0] ?? null;
}

function countRows(
  rows: readonly AppGameSourceFreshnessQualityRow[],
  state: AppGameSourceFreshnessQualityState
): number {
  return rows.filter((row) => row.qualityState === state).length;
}

function parseTimestamp(value: string, label: string): number {
  const parsed = Date.parse(value);
  if (Number.isNaN(parsed)) {
    throw new Error(`Invalid ${label} timestamp`);
  }
  return parsed;
}

function isManualCapability(status: AppGameCapabilityStatus): boolean {
  return status === 'manualRequired' || status === 'permissionLimited' || status === 'notClaimed';
}

function isUnavailableCapability(status: AppGameCapabilityStatus): boolean {
  return status === 'unavailable' || status === 'unsupportedPlatform' || status === 'adapterError';
}

function isUnavailableState(state: ActivityAppGameSourceStatusRow['state']): boolean {
  return state === 'unavailable' || state === 'offline' || state === 'scaffold-only';
}

function unavailableReason(statuses: readonly AppGameCapabilityStatus[]): AppGameSourceFreshnessQualityReason {
  return statuses.includes('adapterError') ? 'source-error' : 'source-unavailable';
}
