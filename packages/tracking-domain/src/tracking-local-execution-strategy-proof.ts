import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
const TrackingLocalExecutionStrategyCount = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingLocalExecutionStrategyAreaSchema = withParser(
  Schema.Literal(
    'windows-host-local-validation',
    'wsl-local-replay',
    'docker-host-availability',
    'android-emulator-runtime',
    'android-physical-status-runtime',
    'macos-ios-ci-route',
    'physical-manual-runtime-route',
    'final-sync-validation-gate'
  )
);

export const TrackingLocalExecutionStrategyRouteSchema = withParser(
  Schema.Literal('local-runnable', 'ci-runnable', 'manual-required', 'unavailable-here', 'final-checkpoint')
);

export const TrackingLocalExecutionStrategyStatusSchema = withParser(
  Schema.Literal('ready', 'observed', 'manual-required', 'unavailable-here')
);

export const TrackingLocalExecutionStrategyRefSchema = brandedNonEmptyStringSchema('TrackingLocalExecutionStrategyRef');

export const TrackingLocalExecutionStrategyCommandSchema = brandedNonEmptyStringSchema('TrackingLocalExecutionStrategyCommand');

export const TrackingLocalExecutionStrategyRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    area: TrackingLocalExecutionStrategyAreaSchema,
    generatedAt: ParentTimestampSchema,
    route: TrackingLocalExecutionStrategyRouteSchema,
    status: TrackingLocalExecutionStrategyStatusSchema,
    proofRef: TrackingLocalExecutionStrategyRefSchema,
    sourceRefs: Schema.Array(TrackingLocalExecutionStrategyRefSchema),
    commandsToRunAfterCodeBatch: Schema.Array(TrackingLocalExecutionStrategyCommandSchema),
    evidenceRefsExpected: Schema.Array(TrackingLocalExecutionStrategyRefSchema),
    passedEvidenceRefs: Schema.Array(TrackingLocalExecutionStrategyRefSchema),
    blockers: Schema.Array(NonEmptyStringSchema),
    localRunnable: Schema.Boolean,
    ciRunnable: Schema.Boolean,
    requiresPhysicalDevice: Schema.Boolean,
    requiresMacHost: Schema.Boolean,
    requiresDockerHost: Schema.Boolean,
    productClaimReady: Schema.Literal(false),
    physicalBehaviorClaimed: Schema.Literal(false),
    iosRuntimeClaimed: Schema.Literal(false),
    childRuntimeClaimed: Schema.Literal(false),
    productionRuntimeClaimed: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.commandsToRunAfterCodeBatch.length > 0 ||
          row.evidenceRefsExpected.length > 0 ||
          'Tracking execution strategy rows need commands or expected evidence'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'manual-required' || row.status === 'unavailable-here'
            ? row.blockers.length > 0
            : row.passedEvidenceRefs.length > 0 || row.commandsToRunAfterCodeBatch.length > 0) ||
          'Tracking execution strategy status must match proof or blocker state'
      )
    )
);

export const TrackingLocalExecutionStrategyProofSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofMode: Schema.Literal('tracking-local-execution-strategy-proof'),
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingLocalExecutionStrategyRefSchema),
    rows: Schema.Array(TrackingLocalExecutionStrategyRowSchema),
    summary: Schema.Struct({
      rowCount: TrackingLocalExecutionStrategyCount,
      localRunnableRows: TrackingLocalExecutionStrategyCount,
      ciRunnableRows: TrackingLocalExecutionStrategyCount,
      manualRequiredRows: TrackingLocalExecutionStrategyCount,
      unavailableHereRows: TrackingLocalExecutionStrategyCount,
      physicalDeviceRequiredRows: TrackingLocalExecutionStrategyCount,
      macHostRequiredRows: TrackingLocalExecutionStrategyCount,
      dockerHostRequiredRows: TrackingLocalExecutionStrategyCount,
      productReadyRows: Schema.Literal(0),
    }),
    productClaims: Schema.Struct({
      localBatchStrategyReady: Schema.Literal(true),
      finalSyncRequiredBeforePr: Schema.Literal(true),
      androidPhysicalStatusOnly: Schema.Literal(true),
      dockerUnavailableOnCurrentHost: Schema.Boolean,
      macosIosRoutedToCiOrManual: Schema.Literal(true),
      physicalBehaviorClaimed: Schema.Literal(false),
      iosRuntimeClaimed: Schema.Literal(false),
      childRuntimeClaimed: Schema.Literal(false),
      productionRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.rows.length === RequiredTrackingLocalExecutionStrategyAreas.length ||
          'Tracking execution strategy proof must include every required area'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          (proof.summary.productReadyRows === 0 &&
            proof.productClaims.productClaimReady === false &&
            proof.productClaims.physicalBehaviorClaimed === false) ||
          'Tracking execution strategy cannot claim product-ready physical/runtime behavior'
      )
    )
);

export type TrackingLocalExecutionStrategyArea = Infer<typeof TrackingLocalExecutionStrategyAreaSchema>;
export type TrackingLocalExecutionStrategyProof = Infer<typeof TrackingLocalExecutionStrategyProofSchema>;
export type TrackingLocalExecutionStrategyRow = Infer<typeof TrackingLocalExecutionStrategyRowSchema>;

export type TrackingLocalExecutionStrategyRowInput = {
  readonly area: TrackingLocalExecutionStrategyArea;
  readonly route: Infer<typeof TrackingLocalExecutionStrategyRouteSchema>;
  readonly status: Infer<typeof TrackingLocalExecutionStrategyStatusSchema>;
  readonly proofRef: string;
  readonly sourceRefs: readonly string[];
  readonly commandsToRunAfterCodeBatch: readonly string[];
  readonly evidenceRefsExpected: readonly string[];
  readonly passedEvidenceRefs: readonly string[];
  readonly blockers: readonly string[];
  readonly localRunnable: boolean;
  readonly ciRunnable: boolean;
  readonly requiresPhysicalDevice: boolean;
  readonly requiresMacHost: boolean;
  readonly requiresDockerHost: boolean;
};

export const RequiredTrackingLocalExecutionStrategyAreas = [
  'windows-host-local-validation',
  'wsl-local-replay',
  'docker-host-availability',
  'android-emulator-runtime',
  'android-physical-status-runtime',
  'macos-ios-ci-route',
  'physical-manual-runtime-route',
  'final-sync-validation-gate',
] as const;

export function buildTrackingLocalExecutionStrategyProof(
  generatedAt: string,
  rowsInput: readonly TrackingLocalExecutionStrategyRowInput[]
): TrackingLocalExecutionStrategyProof {
  const rows = RequiredTrackingLocalExecutionStrategyAreas.map((area) => {
    const input = rowsInput.find((candidate) => candidate.area === area);
    if (!input) throw new Error(`Missing tracking execution strategy area: ${area}`);
    return row(generatedAt, input);
  });
  const summary = summaryFrom(rows);

  return TrackingLocalExecutionStrategyProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofMode: 'tracking-local-execution-strategy-proof',
    generatedAt,
    sourceProofRefs: uniqueRefs(rows.flatMap((candidate) => [candidate.proofRef, ...candidate.sourceRefs])),
    rows,
    summary,
    productClaims: {
      localBatchStrategyReady: true,
      finalSyncRequiredBeforePr: true,
      androidPhysicalStatusOnly: true,
      dockerUnavailableOnCurrentHost: rows.some(
        (candidate) => candidate.area === 'docker-host-availability' && candidate.status === 'unavailable-here'
      ),
      macosIosRoutedToCiOrManual: true,
      physicalBehaviorClaimed: false,
      iosRuntimeClaimed: false,
      childRuntimeClaimed: false,
      productionRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function row(generatedAt: string, input: TrackingLocalExecutionStrategyRowInput): TrackingLocalExecutionStrategyRow {
  return TrackingLocalExecutionStrategyRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    area: input.area,
    generatedAt,
    route: input.route,
    status: input.status,
    proofRef: input.proofRef,
    sourceRefs: uniqueRefs(input.sourceRefs),
    commandsToRunAfterCodeBatch: [...input.commandsToRunAfterCodeBatch],
    evidenceRefsExpected: uniqueRefs(input.evidenceRefsExpected),
    passedEvidenceRefs: uniqueRefs(input.passedEvidenceRefs),
    blockers: [...input.blockers],
    localRunnable: input.localRunnable,
    ciRunnable: input.ciRunnable,
    requiresPhysicalDevice: input.requiresPhysicalDevice,
    requiresMacHost: input.requiresMacHost,
    requiresDockerHost: input.requiresDockerHost,
    productClaimReady: false,
    physicalBehaviorClaimed: false,
    iosRuntimeClaimed: false,
    childRuntimeClaimed: false,
    productionRuntimeClaimed: false,
  });
}

function summaryFrom(rows: readonly TrackingLocalExecutionStrategyRow[]) {
  return {
    rowCount: rows.length,
    localRunnableRows: rows.filter((candidate) => candidate.localRunnable).length,
    ciRunnableRows: rows.filter((candidate) => candidate.ciRunnable).length,
    manualRequiredRows: rows.filter((candidate) => candidate.status === 'manual-required').length,
    unavailableHereRows: rows.filter((candidate) => candidate.status === 'unavailable-here').length,
    physicalDeviceRequiredRows: rows.filter((candidate) => candidate.requiresPhysicalDevice).length,
    macHostRequiredRows: rows.filter((candidate) => candidate.requiresMacHost).length,
    dockerHostRequiredRows: rows.filter((candidate) => candidate.requiresDockerHost).length,
    productReadyRows: 0,
  } as const;
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs.filter((ref) => ref.length > 0))];
}

