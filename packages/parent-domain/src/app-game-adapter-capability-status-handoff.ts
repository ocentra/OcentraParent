import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  type ParentPlatform,
  ParentPlatformSchema,
  ParentTimestampSchema,
} from './reference-primitives';
import {
  V08CrossPlatformEnforcementCapabilityProofReadModel,
  type V08CrossPlatformEnforcementCapabilitySurface,
} from './v0-8-cross-platform-enforcement-capability-proof';
import {
  V08SupportedAdapterRuntimeBoundary,
  V08SupportedAdapterRuntimeProofReadModel,
} from './v0-8-supported-adapter-runtime-proof';

const NonEmptyAdapterCapabilityStatusText = Schema.String.pipe(Schema.minLength(1));

export const AppGameAdapterCapabilityStatusReadModelIdSchema = NonEmptyAdapterCapabilityStatusText.pipe(
  Schema.brand('AppGameAdapterCapabilityStatusReadModelId')
);
export const AppGameAdapterCapabilityStatusRowIdSchema = NonEmptyAdapterCapabilityStatusText.pipe(
  Schema.brand('AppGameAdapterCapabilityStatusRowId')
);
export const AppGameAdapterCapabilityStatusReferenceSchema = NonEmptyAdapterCapabilityStatusText.pipe(
  Schema.brand('AppGameAdapterCapabilityStatusReference')
);
export const AppGameAdapterCapabilityStatusRequirementSchema = NonEmptyAdapterCapabilityStatusText.pipe(
  Schema.brand('AppGameAdapterCapabilityStatusRequirement')
);
export const AppGameAdapterCapabilityStatusBoundarySchema = NonEmptyAdapterCapabilityStatusText.pipe(
  Schema.brand('AppGameAdapterCapabilityStatusBoundary')
);

export const AppGameAdapterCapabilityProductTargetSchema = withParser(Schema.Literal('native-app', 'native-game'));
export const AppGameAdapterCapabilityStatusSchema = withParser(
  Schema.Literal('runtime-boundary-ready', 'manual-required', 'scaffold-only', 'unavailable')
);
export const AppGameAdapterCapabilityBroadBlockingStatusSchema = withParser(
  Schema.Literal('manual-required', 'unavailable', 'not-claimed')
);

const AppGameAdapterCapabilityStatusRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameAdapterCapabilityStatusRowIdSchema,
  platform: ParentPlatformSchema,
  productTarget: AppGameAdapterCapabilityProductTargetSchema,
  adapterStatus: AppGameAdapterCapabilityStatusSchema,
  broadBlockingStatus: AppGameAdapterCapabilityBroadBlockingStatusSchema,
  timeLimitProofRefs: Schema.Array(AppGameAdapterCapabilityStatusReferenceSchema),
  platformProofRefs: Schema.Array(AppGameAdapterCapabilityStatusReferenceSchema),
  manualProofRequirements: Schema.Array(AppGameAdapterCapabilityStatusRequirementSchema),
  claimBoundary: AppGameAdapterCapabilityStatusBoundarySchema,
  adapterDispatchClaimed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type AppGameAdapterCapabilityStatusRowCandidate = Infer<typeof AppGameAdapterCapabilityStatusRowBaseSchema>;

export const AppGameAdapterCapabilityStatusRowSchema = withParser(
  AppGameAdapterCapabilityStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGameAdapterCapabilityStatusRowIsHonest(row) ||
        'Expected app/game adapter capability status rows to keep platform status visible without adapter dispatch, broad blocking, platform enforcement, or child delivery claim upgrades'
    )
  )
);

export const AppGameAdapterCapabilityStatusReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: AppGameAdapterCapabilityStatusReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(AppGameAdapterCapabilityStatusReferenceSchema),
    rows: Schema.Array(AppGameAdapterCapabilityStatusRowSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
        'Expected app/game adapter capability status row ids to be unique'
    )
  )
);

function appGameAdapterCapabilityStatusRowIsHonest(row: AppGameAdapterCapabilityStatusRowCandidate): boolean {
  if (
    row.adapterDispatchClaimed ||
    row.broadBlockingClaimed ||
    row.platformEnforcementClaimed ||
    row.childDeliveryClaimed
  ) {
    return false;
  }

  if (row.adapterStatus === 'runtime-boundary-ready') {
    return (
      row.platform === 'windows' &&
      row.broadBlockingStatus === 'manual-required' &&
      row.timeLimitProofRefs.length > 0 &&
      row.platformProofRefs.length > 0 &&
      row.manualProofRequirements.length > 0
    );
  }

  return row.timeLimitProofRefs.length === 0 && row.manualProofRequirements.length > 0;
}

export type AppGameAdapterCapabilityStatusReadModelId = typeof AppGameAdapterCapabilityStatusReadModelIdSchema.Type;
export type AppGameAdapterCapabilityStatusRowId = typeof AppGameAdapterCapabilityStatusRowIdSchema.Type;
export type AppGameAdapterCapabilityStatusReference = typeof AppGameAdapterCapabilityStatusReferenceSchema.Type;
export type AppGameAdapterCapabilityStatusRequirement = typeof AppGameAdapterCapabilityStatusRequirementSchema.Type;
export type AppGameAdapterCapabilityStatusBoundary = typeof AppGameAdapterCapabilityStatusBoundarySchema.Type;
export type AppGameAdapterCapabilityProductTarget = Infer<typeof AppGameAdapterCapabilityProductTargetSchema>;
export type AppGameAdapterCapabilityStatus = Infer<typeof AppGameAdapterCapabilityStatusSchema>;
export type AppGameAdapterCapabilityBroadBlockingStatus = Infer<
  typeof AppGameAdapterCapabilityBroadBlockingStatusSchema
>;
export type AppGameAdapterCapabilityStatusRow = Infer<typeof AppGameAdapterCapabilityStatusRowSchema>;
export type AppGameAdapterCapabilityStatusReadModel = Infer<typeof AppGameAdapterCapabilityStatusReadModelSchema>;

type AppGameAdapterCapabilityStatusRowInput = {
  platform: ParentPlatform;
  productTarget: AppGameAdapterCapabilityProductTarget;
  adapterStatus: AppGameAdapterCapabilityStatus;
  broadBlockingStatus: AppGameAdapterCapabilityBroadBlockingStatus;
  timeLimitProofRefs: readonly string[];
  platformProofRefs: readonly string[];
  manualProofRequirements: readonly string[];
  claimBoundary: string;
};

const generatedAt = '2026-06-07T05:40:00.000Z';

const SourceReadModelIds = {
  SupportedRuntime: String(V08SupportedAdapterRuntimeProofReadModel.readModelId),
  CrossPlatformCapability: String(V08CrossPlatformEnforcementCapabilityProofReadModel.readModelId),
} as const;

const SupportedRuntimeProofIds = new Set(
  V08SupportedAdapterRuntimeProofReadModel.entries.map((entry) => String(entry.proofEntryId))
);
const CrossPlatformSurfaceIds = new Set(
  V08CrossPlatformEnforcementCapabilityProofReadModel.entries.map((entry) => String(entry.surface))
);

assertSourceProofExists(String(V08SupportedAdapterRuntimeBoundary.WindowsAppGameOwnedProcessTimeLimit));
assertSourceProofExists(String(V08SupportedAdapterRuntimeBoundary.WindowsBroadInstalledAppBlockingManualGate));

export const AppGameAdapterCapabilityStatusReadModel = AppGameAdapterCapabilityStatusReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'app-game-adapter-capability-status-handoff',
  generatedAt,
  sourceReadModelIds: Object.values(SourceReadModelIds),
  rows: [
    platformRows('windows', 'runtime-boundary-ready', 'manual-required', {
      timeLimitProofRefs: [
        String(V08SupportedAdapterRuntimeBoundary.WindowsAppGameOwnedProcessTimeLimit),
        crossPlatformSurface('windows-app-time-limit-lifecycle'),
      ],
      platformProofRefs: [crossPlatformSurface('windows-owned-process-terminate')],
      manualProofRequirements: [
        'broad installed-app identity proof',
        'host block apply artifact',
        'rollback artifact',
        'audit custody artifact',
      ],
      claimBoundary:
        'Windows native app/game status is ready only for owned-process time-limit runtime proof; it is not broad installed-app blocking.',
    }),
    platformRows('macos', 'scaffold-only', 'manual-required', {
      manualProofRequirements: [
        'macOS permission artifact',
        'macOS package identity artifact',
        'macOS adapter apply artifact',
        'macOS rollback artifact',
      ],
      claimBoundary:
        'macOS native app/game adapter status is scaffold/manual-required and cannot inherit Windows runtime proof.',
    }),
    platformRows('linux', 'unavailable', 'unavailable', {
      manualProofRequirements: [
        'Linux service manager artifact',
        'Linux package identity artifact',
        'Linux adapter apply artifact',
        'Linux rollback artifact',
      ],
      claimBoundary:
        'Linux native app/game adapter status is unavailable until Linux-specific host adapter proof exists.',
    }),
    platformRows('android', 'manual-required', 'manual-required', {
      manualProofRequirements: [
        'device-owner or managed-profile artifact',
        'UsageStats artifact',
        'package lifecycle artifact',
        'accessibility or VPN/DNS artifact',
      ],
      claimBoundary:
        'Android native app/game adapter status is manual-required until privileged child-device artifacts exist.',
    }),
    platformRows('ios', 'manual-required', 'manual-required', {
      manualProofRequirements: [
        'Family Controls entitlement artifact',
        'DeviceActivity artifact',
        'Network Extension artifact',
        'signing and TestFlight device artifact',
      ],
      claimBoundary:
        'iOS native app/game adapter status is manual-required until Apple entitlement and device proof exists.',
    }),
  ].flat(),
});

function platformRows(
  platform: ParentPlatform,
  adapterStatus: AppGameAdapterCapabilityStatus,
  broadBlockingStatus: AppGameAdapterCapabilityBroadBlockingStatus,
  proof: {
    timeLimitProofRefs?: readonly string[];
    platformProofRefs?: readonly string[];
    manualProofRequirements: readonly string[];
    claimBoundary: string;
  }
): readonly AppGameAdapterCapabilityStatusRow[] {
  return [
    statusRow({
      platform,
      productTarget: 'native-app',
      adapterStatus,
      broadBlockingStatus,
      timeLimitProofRefs: proof.timeLimitProofRefs ?? [],
      platformProofRefs: proof.platformProofRefs ?? [],
      manualProofRequirements: proof.manualProofRequirements,
      claimBoundary: proof.claimBoundary,
    }),
    statusRow({
      platform,
      productTarget: 'native-game',
      adapterStatus,
      broadBlockingStatus,
      timeLimitProofRefs: proof.timeLimitProofRefs ?? [],
      platformProofRefs: proof.platformProofRefs ?? [],
      manualProofRequirements: proof.manualProofRequirements,
      claimBoundary: proof.claimBoundary,
    }),
  ];
}

function statusRow(input: AppGameAdapterCapabilityStatusRowInput): AppGameAdapterCapabilityStatusRow {
  return AppGameAdapterCapabilityStatusRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    rowId: `${input.platform}-${input.productTarget}-adapter-capability-status`,
    lastCheckedAt: generatedAt,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    childDeliveryClaimed: false,
    ...input,
  });
}

function crossPlatformSurface(surface: V08CrossPlatformEnforcementCapabilitySurface): string {
  const surfaceId = String(surface);
  if (!CrossPlatformSurfaceIds.has(surfaceId)) {
    throw new Error(`Missing cross-platform capability proof surface: ${surfaceId}`);
  }
  return surfaceId;
}

function assertSourceProofExists(proofEntryId: string): void {
  if (!SupportedRuntimeProofIds.has(proofEntryId)) {
    throw new Error(`Missing supported adapter runtime proof entry: ${proofEntryId}`);
  }
}

export const decodeAppGameAdapterCapabilityStatusRow = Schema.decodeUnknownSync(
  AppGameAdapterCapabilityStatusRowSchema
);
export const decodeAppGameAdapterCapabilityStatusReadModel = Schema.decodeUnknownSync(
  AppGameAdapterCapabilityStatusReadModelSchema
);
