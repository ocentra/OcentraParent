import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  V08OsAdapterManualArtifactGateReadModel,
  type V08OsAdapterManualArtifactGateEntry,
} from './v0-8-os-adapter-manual-artifact-gates';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const AppleCiPreflightText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAppleCiPlatformProofPreflightSchemaVersionSchema = withParser(
  Schema.Literal('app-game-apple-ci-platform-proof-preflight')
);

export const AppGameAppleCiPlatformProofPreflightPlatformSchema = withParser(Schema.Literal('macos', 'ios'));

export const AppGameAppleCiPlatformProofPreflightStateSchema = withParser(Schema.Literal('ci-artifacts-required'));

export const AppGameAppleCiPlatformProofPreflightProofRefSchema = withParser(
  Schema.Literal(
    'macos-ci-runner-ref',
    'macos-xcodebuild-ref',
    'macos-permission-profile-proof',
    'macos-mdm-endpoint-proof',
    'macos-rollback-audit-proof',
    'ios-ci-runner-ref',
    'ios-family-controls-entitlement-proof',
    'ios-device-activity-proof',
    'ios-managed-settings-proof',
    'ios-testflight-device-proof'
  )
);

export const AppGameAppleCiPlatformProofPreflightBlockerSchema = withParser(
  Schema.Literal(
    'macos-ci-runner-not-proved',
    'macos-permission-profile-not-proved',
    'macos-mdm-endpoint-not-proved',
    'macos-rollback-audit-not-proved',
    'ios-ci-runner-not-proved',
    'ios-family-controls-not-proved',
    'ios-device-activity-not-proved',
    'ios-managed-settings-not-proved',
    'ios-testflight-device-not-proved',
    'apple-platform-adapter-dispatch-blocked-before-ci-proof'
  )
);

const AppleCiPreflightLabelSchema = AppleCiPreflightText.pipe(
  Schema.brand('AppGameAppleCiPlatformProofPreflightLabel')
);

const AppleCiPreflightCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppleCiPlatformProofPreflightRowBaseSchema = Schema.Struct({
  platform: AppGameAppleCiPlatformProofPreflightPlatformSchema,
  preflightState: AppGameAppleCiPlatformProofPreflightStateSchema,
  sourceGateIds: Schema.Array(AppleCiPreflightLabelSchema),
  requiredProofRefs: Schema.Array(AppGameAppleCiPlatformProofPreflightProofRefSchema),
  blockerRefs: Schema.Array(AppGameAppleCiPlatformProofPreflightBlockerSchema),
  canRunOnWindowsHost: Schema.Literal(false),
  canDispatchAdapter: Schema.Literal(false),
  ciRunnerClaimed: Schema.Literal(false),
  entitlementClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
});

const AppleCiPlatformProofPreflightReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGameAppleCiPlatformProofPreflightSchemaVersionSchema,
  preflightId: AppleCiPreflightLabelSchema,
  generatedAt: ParentTimestampSchema,
  rows: Schema.Array(AppleCiPlatformProofPreflightRowBaseSchema),
  macosGateCount: AppleCiPreflightCountSchema,
  iosGateCount: AppleCiPreflightCountSchema,
  dispatchableRowCount: AppleCiPreflightCountSchema,
  blockedRowCount: AppleCiPreflightCountSchema,
  windowsLocalProofClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  childDeviceDeliveryClaimed: Schema.Literal(false),
  proofRefs: Schema.Array(AppGameAppleCiPlatformProofPreflightProofRefSchema),
  openBlockers: Schema.Array(AppGameAppleCiPlatformProofPreflightBlockerSchema),
  parentVisibleSummary: AppleCiPreflightLabelSchema,
});

type AppleCiPlatformProofPreflightRowCandidate = Infer<typeof AppleCiPlatformProofPreflightRowBaseSchema>;
type AppleCiPlatformProofPreflightReadModelCandidate = Infer<typeof AppleCiPlatformProofPreflightReadModelBaseSchema>;

export const AppGameAppleCiPlatformProofPreflightRowSchema = withParser(
  AppleCiPlatformProofPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appleCiPreflightRowIsHonest(row) ||
        'Expected Apple CI platform preflight rows to stay blocked before CI/device artifacts exist'
    )
  )
);

export const AppGameAppleCiPlatformProofPreflightReadModelSchema = withParser(
  AppleCiPlatformProofPreflightReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appleCiPreflightReadModelIsHonest(readModel) ||
        'Expected Apple CI platform preflight to keep macOS/iOS local Windows proof and adapter dispatch unclaimed'
    )
  )
);

export type AppGameAppleCiPlatformProofPreflightRow = Infer<typeof AppGameAppleCiPlatformProofPreflightRowSchema>;
export type AppGameAppleCiPlatformProofPreflightReadModel = Infer<
  typeof AppGameAppleCiPlatformProofPreflightReadModelSchema
>;

export const decodeAppGameAppleCiPlatformProofPreflightReadModel = Schema.decodeUnknownSync(
  AppGameAppleCiPlatformProofPreflightReadModelSchema
);

export function createAppGameAppleCiPlatformProofPreflightReadModel(input: {
  readonly generatedAt: AppGameAppleCiPlatformProofPreflightReadModel['generatedAt'];
  readonly manualArtifactGates?: readonly V08OsAdapterManualArtifactGateEntry[];
}): AppGameAppleCiPlatformProofPreflightReadModel {
  const manualArtifactGates = input.manualArtifactGates ?? V08OsAdapterManualArtifactGateReadModel.entries;
  const macosGates = platformGateIds(manualArtifactGates, 'macos');
  const iosGates = platformGateIds(manualArtifactGates, 'ios');
  const rows = [
    appleCiPreflightRow('macos', macosGates, macosProofRefs(), macosBlockers()),
    appleCiPreflightRow('ios', iosGates, iosProofRefs(), iosBlockers()),
  ] as const;

  return decodeAppGameAppleCiPlatformProofPreflightReadModel({
    schemaVersion: 'app-game-apple-ci-platform-proof-preflight',
    preflightId: 'app-game-apple-ci-platform-proof-preflight-ref',
    generatedAt: input.generatedAt,
    rows,
    macosGateCount: macosGates.length,
    iosGateCount: iosGates.length,
    dispatchableRowCount: rows.filter((row) => row.canDispatchAdapter).length,
    blockedRowCount: rows.filter((row) => !row.canDispatchAdapter).length,
    windowsLocalProofClaimed: false,
    adapterDispatchClaimed: false,
    platformEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    proofRefs: [...macosProofRefs(), ...iosProofRefs()],
    openBlockers: [...macosBlockers(), ...iosBlockers(), 'apple-platform-adapter-dispatch-blocked-before-ci-proof'],
    parentVisibleSummary:
      'macOS and iOS app/game control require Apple-platform CI runner, entitlement, device, rollback, and audit artifacts; Windows-local proof remains unclaimed.',
  });
}

export function summarizeAppGameAppleCiPlatformProofPreflightReadModel(
  readModel: AppGameAppleCiPlatformProofPreflightReadModel
) {
  return {
    macosGateCount: readModel.macosGateCount,
    iosGateCount: readModel.iosGateCount,
    dispatchableRowCount: readModel.dispatchableRowCount,
    blockedRowCount: readModel.blockedRowCount,
    openBlockerCount: readModel.openBlockers.length,
  } as const;
}

function appleCiPreflightRow(
  platform: 'macos' | 'ios',
  sourceGateIds: readonly string[],
  requiredProofRefs: readonly AppGameAppleCiPlatformProofPreflightReadModel['proofRefs'][number][],
  blockerRefs: readonly AppGameAppleCiPlatformProofPreflightReadModel['openBlockers'][number][]
): AppGameAppleCiPlatformProofPreflightRow {
  return AppGameAppleCiPlatformProofPreflightRowSchema.parse({
    platform,
    preflightState: 'ci-artifacts-required',
    sourceGateIds,
    requiredProofRefs,
    blockerRefs: [...blockerRefs, 'apple-platform-adapter-dispatch-blocked-before-ci-proof'],
    canRunOnWindowsHost: false,
    canDispatchAdapter: false,
    ciRunnerClaimed: false,
    entitlementClaimed: false,
    platformEnforcementClaimed: false,
  });
}

function platformGateIds(
  manualArtifactGates: readonly V08OsAdapterManualArtifactGateEntry[],
  platform: 'macos' | 'ios'
) {
  return manualArtifactGates.filter((entry) => entry.platform === platform).map((entry) => entry.gateEntryId);
}

function macosProofRefs() {
  return [
    'macos-ci-runner-ref',
    'macos-xcodebuild-ref',
    'macos-permission-profile-proof',
    'macos-mdm-endpoint-proof',
    'macos-rollback-audit-proof',
  ] as const;
}

function iosProofRefs() {
  return [
    'ios-ci-runner-ref',
    'ios-family-controls-entitlement-proof',
    'ios-device-activity-proof',
    'ios-managed-settings-proof',
    'ios-testflight-device-proof',
  ] as const;
}

function macosBlockers() {
  return [
    'macos-ci-runner-not-proved',
    'macos-permission-profile-not-proved',
    'macos-mdm-endpoint-not-proved',
    'macos-rollback-audit-not-proved',
  ] as const;
}

function iosBlockers() {
  return [
    'ios-ci-runner-not-proved',
    'ios-family-controls-not-proved',
    'ios-device-activity-not-proved',
    'ios-managed-settings-not-proved',
    'ios-testflight-device-not-proved',
  ] as const;
}

function appleCiPreflightRowIsHonest(row: AppleCiPlatformProofPreflightRowCandidate): boolean {
  return (
    row.sourceGateIds.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    row.blockerRefs.includes('apple-platform-adapter-dispatch-blocked-before-ci-proof') &&
    !row.canRunOnWindowsHost &&
    !row.canDispatchAdapter &&
    !row.ciRunnerClaimed &&
    !row.entitlementClaimed &&
    !row.platformEnforcementClaimed
  );
}

function appleCiPreflightReadModelIsHonest(readModel: AppleCiPlatformProofPreflightReadModelCandidate): boolean {
  return (
    readModel.rows.length === 2 &&
    readModel.macosGateCount > 0 &&
    readModel.iosGateCount > 0 &&
    readModel.dispatchableRowCount === 0 &&
    readModel.blockedRowCount === readModel.rows.length &&
    readModel.openBlockers.includes('apple-platform-adapter-dispatch-blocked-before-ci-proof') &&
    !readModel.windowsLocalProofClaimed &&
    !readModel.adapterDispatchClaimed &&
    !readModel.platformEnforcementClaimed &&
    !readModel.childDeviceDeliveryClaimed
  );
}
