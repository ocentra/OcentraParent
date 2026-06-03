import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  appGamePerformanceHealthMatrixCoversRequiredSurfaces,
  appGamePerformanceHealthRowIsHonest,
} from './app-game-performance-health-rules';
import { EnforcementCapabilityStateSchema } from './enforcement';
import { ParentEvidenceReferenceSchema } from './references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const NonEmptyPerformanceHealthText = Schema.String.pipe(Schema.minLength(1));
const PositivePerformanceHealthNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value > 0) || 'Expected a positive finite number')
);

export const AppGamePerformanceHealthSchemaVersionSchema = withParser(
  Schema.Literal('app-game-performance-health-proof')
);

export const AppGamePerformanceHealthProductSliceSchema = withParser(
  Schema.Literal('native-app', 'native-game', 'shared-app-game')
);

export const AppGamePerformanceHealthSurfaceSchema = withParser(
  Schema.Literal(
    'inventory-scan-bounds',
    'runtime-polling-bounds',
    'foreground-debounce-bounds',
    'journal-write-volume',
    'session-replay-cost',
    'policy-compile-cost',
    'portal-render-bounds',
    'adapter-health-degraded-state'
  )
);

export const AppGamePerformanceHealthMeasurementModeSchema = withParser(
  Schema.Literal(
    'contract-budget-target',
    'generated-scale-smoke',
    'existing-portal-intent-smoke',
    'degraded-state-contract'
  )
);

export const AppGamePerformanceHealthServiceStateSchema = withParser(
  Schema.Literal('healthy', 'degraded', 'manual-required', 'not-claimed')
);

export const AppGamePerformanceHealthDegradedTriggerSchema = withParser(
  Schema.Literal(
    'adapter-error',
    'stale-evidence',
    'query-store-degraded',
    'journal-backlog',
    'replay-backlog',
    'policy-budget-pressure',
    'portal-row-pressure',
    'live-source-not-claimed'
  )
);

export const AppGamePerformanceHealthNoClaimBoundarySchema = withParser(
  Schema.Literal(
    'fixture-scale-not-live-load-test',
    'portal-intent-not-browser-dom-render-proof',
    'inventory-is-not-use',
    'runtime-is-not-foreground',
    'foreground-is-not-content',
    'launcher-is-not-game',
    'manual-required-cannot-execute',
    'no-platform-adapter-execution',
    'not-product-complete-performance-claim',
    'raw-private-paths-not-exposed'
  )
);

const AppGamePerformanceHealthCheckIdSchema = NonEmptyPerformanceHealthText.pipe(
  Schema.brand('AppGamePerformanceHealthCheckId')
);
const AppGamePerformanceHealthMatrixIdSchema = NonEmptyPerformanceHealthText.pipe(
  Schema.brand('AppGamePerformanceHealthMatrixId')
);
const AppGamePerformanceHealthParentVisibleStateSchema = NonEmptyPerformanceHealthText.pipe(
  Schema.brand('AppGamePerformanceHealthParentVisibleState')
);
const AppGamePerformanceHealthProofPackRefSchema = NonEmptyPerformanceHealthText.pipe(
  Schema.brand('AppGamePerformanceHealthProofPackRef')
);

const AppGamePerformanceHealthRowBaseSchema = Schema.Struct({
  schemaVersion: AppGamePerformanceHealthSchemaVersionSchema,
  healthCheckId: AppGamePerformanceHealthCheckIdSchema,
  productSlice: AppGamePerformanceHealthProductSliceSchema,
  surface: AppGamePerformanceHealthSurfaceSchema,
  measurementMode: AppGamePerformanceHealthMeasurementModeSchema,
  minimumEntityCount: PositivePerformanceHealthNumber,
  warningThresholdMs: PositivePerformanceHealthNumber,
  budgetMs: PositivePerformanceHealthNumber,
  serviceHealthState: AppGamePerformanceHealthServiceStateSchema,
  capabilityState: EnforcementCapabilityStateSchema,
  degradedTriggers: Schema.Array(AppGamePerformanceHealthDegradedTriggerSchema),
  evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  parentVisibleState: AppGamePerformanceHealthParentVisibleStateSchema,
  noClaimBoundaries: Schema.Array(AppGamePerformanceHealthNoClaimBoundarySchema),
  proofPackRefs: Schema.Array(AppGamePerformanceHealthProofPackRefSchema),
  adapterExecutionClaim: Schema.Literal('not-claimed'),
  livePlatformClaim: Schema.Literal('not-claimed'),
  recordedAt: ParentTimestampSchema,
});

type AppGamePerformanceHealthRowCandidate = Infer<typeof AppGamePerformanceHealthRowBaseSchema>;

export const AppGamePerformanceHealthRowSchema = withParser(
  AppGamePerformanceHealthRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGamePerformanceHealthRowIsHonest(row) ||
        'Expected app/game performance health rows to keep scale proof bounded, parent-visible, and free of live platform or adapter claims'
    )
  )
);

export const AppGamePerformanceHealthMatrixSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    matrixId: AppGamePerformanceHealthMatrixIdSchema,
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(AppGamePerformanceHealthRowSchema),
  }).pipe(
    Schema.filter(
      (matrix) =>
        matrix.rows.length > 0 &&
        matrixRowsAreUnique(matrix.rows) &&
        appGamePerformanceHealthMatrixCoversRequiredSurfaces(matrix.rows)
    )
  )
);

function matrixRowsAreUnique(rows: readonly AppGamePerformanceHealthRowCandidate[]): boolean {
  return new Set(rows.map((row) => row.healthCheckId)).size === rows.length;
}

export type AppGamePerformanceHealthProductSlice = Infer<typeof AppGamePerformanceHealthProductSliceSchema>;
export type AppGamePerformanceHealthSurface = Infer<typeof AppGamePerformanceHealthSurfaceSchema>;
export type AppGamePerformanceHealthMeasurementMode = Infer<typeof AppGamePerformanceHealthMeasurementModeSchema>;
export type AppGamePerformanceHealthServiceState = Infer<typeof AppGamePerformanceHealthServiceStateSchema>;
export type AppGamePerformanceHealthDegradedTrigger = Infer<typeof AppGamePerformanceHealthDegradedTriggerSchema>;
export type AppGamePerformanceHealthNoClaimBoundary = Infer<typeof AppGamePerformanceHealthNoClaimBoundarySchema>;
export type AppGamePerformanceHealthRow = Infer<typeof AppGamePerformanceHealthRowSchema>;
export type AppGamePerformanceHealthMatrix = Infer<typeof AppGamePerformanceHealthMatrixSchema>;

export const decodeAppGamePerformanceHealthRow = Schema.decodeUnknownSync(AppGamePerformanceHealthRowSchema);
export const decodeAppGamePerformanceHealthMatrix = Schema.decodeUnknownSync(AppGamePerformanceHealthMatrixSchema);
