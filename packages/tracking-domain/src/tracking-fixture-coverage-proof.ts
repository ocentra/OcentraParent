import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  TrackingFixtureCoverageRequiredStates,
  TrackingFixtureCoverageStateExpectations,
} from './tracking-fixture-coverage-proof-values';

export const TrackingFixtureCoverageProofSchemaVersionSchema = withParser(
  Schema.Literal('tracking-fixture-coverage-proof')
);
export const TrackingFixtureStateSchema = withParser(
  Schema.Literal(
    'fresh',
    'stale',
    'offline',
    'permission-denied',
    'low-accuracy',
    'ambiguous-nearby-place',
    'exception-active',
    'parent-acknowledged',
    'child-check-in-requested',
    'temporary-live-expired',
    'missing-device',
    'retention-deleted',
    'remote-sync-disabled',
    'remote-ai-disabled'
  )
);
export const TrackingFixtureProofTierSchema = withParser(
  Schema.Literal('P0_CONTRACT', 'P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI')
);
export const TrackingFixtureCoverageStateStatusSchema = withParser(Schema.Literal('covered', 'manual-required'));

const TrackingFixtureProofRefSchema = brandedNonEmptyStringSchema('TrackingFixtureProofRef');
const TrackingFixtureRequirementSchema = brandedNonEmptyStringSchema('TrackingFixtureRequirement');
const TrackingFixtureBoundarySchema = brandedNonEmptyStringSchema('TrackingFixtureBoundary');

export const TrackingFixtureCoverageRowSchema = withParser(
  Schema.Struct({
    state: TrackingFixtureStateSchema,
    requiredProofTier: TrackingFixtureProofTierSchema,
    currentProofTier: TrackingFixtureProofTierSchema,
    status: TrackingFixtureCoverageStateStatusSchema,
    artifactRefs: Schema.Array(TrackingFixtureProofRefSchema).pipe(Schema.minItems(1)),
    proofRequirement: TrackingFixtureRequirementSchema,
    productClaimReady: Schema.Literal(false),
    liveDeviceClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    claimBoundary: TrackingFixtureBoundarySchema,
  })
);

export const TrackingFixtureCoverageSummarySchema = withParser(
  Schema.Struct({
    requiredStateCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    coveredStateCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    manualRequiredStateCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    productClaimReadyRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    liveDeviceClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    providerDeliveryClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    childDeviceRuntimeClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    physicalDeviceClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
    productionWorkerClaimedRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  })
);

const TrackingFixtureCoverageReadModelBaseSchema = Schema.Struct({
  schemaVersion: TrackingFixtureCoverageProofSchemaVersionSchema,
  updatedAt: ParentTimestampSchema,
  rows: Schema.Array(TrackingFixtureCoverageRowSchema),
  summary: TrackingFixtureCoverageSummarySchema,
});

type TrackingFixtureCoverageReadModelCandidate = Infer<typeof TrackingFixtureCoverageReadModelBaseSchema>;

export const TrackingFixtureCoverageReadModelSchema = withParser(
  TrackingFixtureCoverageReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        trackingFixtureCoverageReadModelIsHonest(readModel) ||
        'Expected tracking fixture coverage proof to cover every required fixture state while keeping product, live-device, provider-delivery, child-runtime, physical-device, and production-worker claims false'
    )
  )
);

export function buildTrackingFixtureCoverageReadModel(
  updatedAt: string = '2026-06-05T19:45:00.000Z'
): TrackingFixtureCoverageReadModel {
  const rows = TrackingFixtureCoverageRequiredStates.map((state) => fixtureCoverageRowFor(state));
  return TrackingFixtureCoverageReadModelSchema.parse({
    schemaVersion: 'tracking-fixture-coverage-proof',
    updatedAt,
    rows,
    summary: summarize(rows),
  });
}

function fixtureCoverageRowFor(state: TrackingFixtureState): TrackingFixtureCoverageRow {
  const expected = TrackingFixtureCoverageStateExpectations[state];
  return TrackingFixtureCoverageRowSchema.parse({
    state,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: expected.currentProofTier,
    status: 'covered',
    artifactRefs: expected.artifactRefs,
    proofRequirement: expected.proofRequirement,
    productClaimReady: false,
    liveDeviceClaimed: false,
    providerDeliveryClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    productionWorkerClaimed: false,
    claimBoundary:
      'Fixture coverage proves parser/read-model/UI-state evidence only; live devices, provider delivery, production workers, and product readiness remain separate gates.',
  });
}

function summarize(rows: ReadonlyArray<TrackingFixtureCoverageRow>): TrackingFixtureCoverageSummary {
  return TrackingFixtureCoverageSummarySchema.parse({
    requiredStateCount: TrackingFixtureCoverageRequiredStates.length,
    coveredStateCount: rows.filter((row) => row.status === 'covered').length,
    manualRequiredStateCount: rows.filter((row) => row.status === 'manual-required').length,
    productClaimReadyRows: rows.filter((row) => row.productClaimReady).length,
    liveDeviceClaimedRows: rows.filter((row) => row.liveDeviceClaimed).length,
    providerDeliveryClaimedRows: rows.filter((row) => row.providerDeliveryClaimed).length,
    childDeviceRuntimeClaimedRows: rows.filter((row) => row.childDeviceRuntimeClaimed).length,
    physicalDeviceClaimedRows: rows.filter((row) => row.physicalDeviceClaimed).length,
    productionWorkerClaimedRows: rows.filter((row) => row.productionWorkerClaimed).length,
  });
}

function trackingFixtureCoverageReadModelIsHonest(readModel: TrackingFixtureCoverageReadModelCandidate): boolean {
  const byState = new Map(readModel.rows.map((row) => [row.state, row] as const));
  return (
    byState.size === readModel.rows.length &&
    TrackingFixtureCoverageRequiredStates.every((state) => fixtureCoverageRowIsHonest(byState.get(state), state)) &&
    readModel.summary.requiredStateCount === TrackingFixtureCoverageRequiredStates.length &&
    readModel.summary.coveredStateCount === TrackingFixtureCoverageRequiredStates.length &&
    readModel.summary.manualRequiredStateCount === 0 &&
    readModel.summary.productClaimReadyRows === 0 &&
    readModel.summary.liveDeviceClaimedRows === 0 &&
    readModel.summary.providerDeliveryClaimedRows === 0 &&
    readModel.summary.childDeviceRuntimeClaimedRows === 0 &&
    readModel.summary.physicalDeviceClaimedRows === 0 &&
    readModel.summary.productionWorkerClaimedRows === 0
  );
}

function fixtureCoverageRowIsHonest(row: TrackingFixtureCoverageRow | undefined, state: TrackingFixtureState): boolean {
  const expected = TrackingFixtureCoverageStateExpectations[state];
  return Boolean(
    row &&
    row.state === state &&
    row.requiredProofTier === 'P1_FIXTURE_SIMULATION' &&
    row.currentProofTier === expected.currentProofTier &&
    row.status === 'covered' &&
    row.artifactRefs.length === expected.artifactRefs.length &&
    expected.artifactRefs.every((artifactRef, index) => row.artifactRefs[index] === artifactRef) &&
    fixtureCoverageRowClaimsAreFalse(row)
  );
}

function fixtureCoverageRowClaimsAreFalse(row: TrackingFixtureCoverageRow): boolean {
  return (
    row.productClaimReady === false &&
    row.liveDeviceClaimed === false &&
    row.providerDeliveryClaimed === false &&
    row.childDeviceRuntimeClaimed === false &&
    row.physicalDeviceClaimed === false &&
    row.productionWorkerClaimed === false
  );
}

export type TrackingFixtureState = Infer<typeof TrackingFixtureStateSchema>;
export type TrackingFixtureProofTier = Infer<typeof TrackingFixtureProofTierSchema>;
export type TrackingFixtureCoverageStateStatus = Infer<typeof TrackingFixtureCoverageStateStatusSchema>;
export type TrackingFixtureCoverageRow = Infer<typeof TrackingFixtureCoverageRowSchema>;
export type TrackingFixtureCoverageSummary = Infer<typeof TrackingFixtureCoverageSummarySchema>;
export type TrackingFixtureCoverageReadModel = Infer<typeof TrackingFixtureCoverageReadModelSchema>;

