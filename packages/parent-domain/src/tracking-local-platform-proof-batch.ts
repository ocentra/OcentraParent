import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './reference-primitives';

const TrackingLocalPlatformProofBatchText = Schema.String.pipe(Schema.minLength(1));

export const TrackingLocalPlatformProofBatchAreaSchema = withParser(
  Schema.Literal(
    'android-emulator-runtime',
    'wsl-local-replay',
    'hosted-parent-ui-accessibility',
    'product-parent-child-ui-local-artifacts',
    'real-runtime-handoff-accounting'
  )
);

export const TrackingLocalPlatformProofBatchStatusSchema = withParser(
  Schema.Literal('local-proof-passed', 'manual-required')
);

export const TrackingLocalPlatformProofBatchRefSchema = TrackingLocalPlatformProofBatchText.pipe(
  Schema.brand('TrackingLocalPlatformProofBatchRef')
);

export const TrackingLocalPlatformProofBatchMetricSchema = withParser(
  Schema.Struct({
    name: TrackingLocalPlatformProofBatchText,
    value: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  })
);

export const TrackingLocalPlatformProofBatchRowSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    area: TrackingLocalPlatformProofBatchAreaSchema,
    status: TrackingLocalPlatformProofBatchStatusSchema,
    generatedAt: ParentTimestampSchema,
    proofRef: TrackingLocalPlatformProofBatchRefSchema,
    sourceRefs: Schema.Array(TrackingLocalPlatformProofBatchRefSchema),
    currentProofTier: TrackingLocalPlatformProofBatchText,
    requiredProofTier: TrackingLocalPlatformProofBatchText,
    passedLocalAssertions: Schema.Array(TrackingLocalPlatformProofBatchText),
    remainingBlockers: Schema.Array(TrackingLocalPlatformProofBatchText),
    metrics: Schema.Array(TrackingLocalPlatformProofBatchMetricSchema),
    ciRunnable: Schema.Boolean,
    physicalDeviceClaimed: Schema.Literal(false),
    iosRuntimeClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    authorityRuntimeClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    productionRuntimeClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.passedLocalAssertions.length > 0 ||
          row.remainingBlockers.length > 0 ||
          'Tracking local platform proof rows need assertions or blockers'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'local-proof-passed'
            ? row.passedLocalAssertions.length > 0
            : row.remainingBlockers.length > 0) ||
          'Tracking local platform proof status must match assertions or blockers'
      )
    )
);

export const TrackingLocalPlatformProofBatchSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    proofMode: Schema.Literal('tracking-local-platform-proof-batch'),
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingLocalPlatformProofBatchRefSchema),
    rows: Schema.Array(TrackingLocalPlatformProofBatchRowSchema),
    summary: Schema.Struct({
      rowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
      localProofPassedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
      manualRequiredRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
      ciRunnableRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
      physicalDeviceClaimedRows: Schema.Literal(0),
      iosRuntimeClaimedRows: Schema.Literal(0),
      childDeviceRuntimeClaimedRows: Schema.Literal(0),
      authorityRuntimeClaimedRows: Schema.Literal(0),
      providerDeliveryRuntimeClaimedRows: Schema.Literal(0),
      productionRuntimeClaimedRows: Schema.Literal(0),
      productReadyRows: Schema.Literal(0),
    }),
    productClaims: Schema.Struct({
      androidEmulatorLocalProofPassed: Schema.Literal(true),
      wslLocalReplayPassed: Schema.Literal(true),
      hostedParentUiAccessibilityPassed: Schema.Literal(true),
      productUiLocalArtifactsCaptured: Schema.Literal(true),
      realRuntimeHandoffAccountingPresent: Schema.Literal(true),
      androidPhysicalDeviceClaimed: Schema.Literal(false),
      iosRuntimeClaimed: Schema.Literal(false),
      childDeviceRuntimeClaimed: Schema.Literal(false),
      authorityRuntimeClaimed: Schema.Literal(false),
      providerDeliveryRuntimeClaimed: Schema.Literal(false),
      productionRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          (proof.rows.length === RequiredTrackingLocalPlatformProofBatchAreas.length &&
            proof.summary.rowCount === proof.rows.length &&
            proof.sourceProofRefs.length >= proof.rows.length) ||
          'Tracking local platform proof batch must include every required area and source proof'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          (proof.summary.localProofPassedRows >= 4 &&
            proof.summary.manualRequiredRows >= 1 &&
            proof.summary.productReadyRows === 0) ||
          'Tracking local platform proof batch must pass local layers while keeping manual runtime gaps visible'
      )
    )
);

export type TrackingLocalPlatformProofBatchArea = Infer<typeof TrackingLocalPlatformProofBatchAreaSchema>;
export type TrackingLocalPlatformProofBatch = Infer<typeof TrackingLocalPlatformProofBatchSchema>;
export type TrackingLocalPlatformProofBatchRow = Infer<typeof TrackingLocalPlatformProofBatchRowSchema>;

export type TrackingLocalPlatformProofBatchRowInput = {
  readonly area: TrackingLocalPlatformProofBatchArea;
  readonly status: 'local-proof-passed' | 'manual-required';
  readonly proofRef: string;
  readonly sourceRefs: readonly string[];
  readonly currentProofTier: string;
  readonly requiredProofTier: string;
  readonly passedLocalAssertions: readonly string[];
  readonly remainingBlockers: readonly string[];
  readonly metrics: readonly { readonly name: string; readonly value: number }[];
  readonly ciRunnable: boolean;
};

export const RequiredTrackingLocalPlatformProofBatchAreas = [
  'android-emulator-runtime',
  'wsl-local-replay',
  'hosted-parent-ui-accessibility',
  'product-parent-child-ui-local-artifacts',
  'real-runtime-handoff-accounting',
] as const;

export function buildTrackingLocalPlatformProofBatch(
  generatedAt: string,
  rowsInput: readonly TrackingLocalPlatformProofBatchRowInput[]
): TrackingLocalPlatformProofBatch {
  const rows = RequiredTrackingLocalPlatformProofBatchAreas.map((area) => {
    const input = rowsInput.find((candidate) => candidate.area === area);
    if (!input) throw new Error(`Missing tracking local platform proof batch area: ${area}`);
    return row(generatedAt, input);
  });

  const summary = {
    rowCount: rows.length,
    localProofPassedRows: rows.filter((candidate) => candidate.status === 'local-proof-passed').length,
    manualRequiredRows: rows.filter((candidate) => candidate.status === 'manual-required').length,
    ciRunnableRows: rows.filter((candidate) => candidate.ciRunnable).length,
    physicalDeviceClaimedRows: 0,
    iosRuntimeClaimedRows: 0,
    childDeviceRuntimeClaimedRows: 0,
    authorityRuntimeClaimedRows: 0,
    providerDeliveryRuntimeClaimedRows: 0,
    productionRuntimeClaimedRows: 0,
    productReadyRows: 0,
  } as const;

  return TrackingLocalPlatformProofBatchSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofMode: 'tracking-local-platform-proof-batch',
    generatedAt,
    sourceProofRefs: uniqueRefs(rows.flatMap((candidate) => [candidate.proofRef, ...candidate.sourceRefs])),
    rows,
    summary,
    productClaims: {
      androidEmulatorLocalProofPassed: true,
      wslLocalReplayPassed: true,
      hostedParentUiAccessibilityPassed: true,
      productUiLocalArtifactsCaptured: true,
      realRuntimeHandoffAccountingPresent: true,
      androidPhysicalDeviceClaimed: false,
      iosRuntimeClaimed: false,
      childDeviceRuntimeClaimed: false,
      authorityRuntimeClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      productionRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function row(generatedAt: string, input: TrackingLocalPlatformProofBatchRowInput): TrackingLocalPlatformProofBatchRow {
  return TrackingLocalPlatformProofBatchRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    area: input.area,
    status: input.status,
    generatedAt,
    proofRef: input.proofRef,
    sourceRefs: uniqueRefs(input.sourceRefs),
    currentProofTier: input.currentProofTier,
    requiredProofTier: input.requiredProofTier,
    passedLocalAssertions: [...input.passedLocalAssertions],
    remainingBlockers: [...input.remainingBlockers],
    metrics: input.metrics.map((metric) => ({ name: metric.name, value: metric.value })),
    ciRunnable: input.ciRunnable,
    physicalDeviceClaimed: false,
    iosRuntimeClaimed: false,
    childDeviceRuntimeClaimed: false,
    authorityRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    productionRuntimeClaimed: false,
    productClaimReady: false,
  });
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs.filter((ref) => ref.length > 0))];
}
