import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';

export const AppGamePlatformProofStatusSchemaVersionSchema = withParser(
  Schema.Literal('app-game-platform-proof-status')
);

export const AppGamePlatformProofStatusPlatformSchema = withParser(
  Schema.Literal('windows', 'android', 'linux', 'macos', 'ios')
);

export const AppGamePlatformProofStatusStateSchema = withParser(
  Schema.Literal(
    'windows-policy-preflight-observed',
    'physical-device-observed',
    'wsl-runtime-observed',
    'apple-ci-artifacts-required'
  )
);

export const AppGamePlatformProofStatusAuthoritySchema = withParser(
  Schema.Literal('visibility-only', 'enforcement-not-proved')
);

export const AppGamePlatformProofStatusGapSchema = withParser(
  Schema.Literal(
    'android-device-owner-not-proved',
    'android-profile-owner-not-proved',
    'android-usage-events-not-proved',
    'android-durable-usage-events-replay-not-proved',
    'android-child-runtime-replay-consumer-not-attached',
    'android-authority-preflight-not-attached',
    'android-accessibility-overlay-not-proved',
    'android-hide-suspend-not-proved',
    'windows-applocker-enforce-not-proved',
    'windows-app-control-not-proved',
    'windows-system-app-allowlist-not-proved',
    'windows-rollback-not-proved',
    'windows-audit-custody-not-proved',
    'windows-broad-blocking-not-proved',
    'macos-ci-runner-not-proved',
    'macos-permission-profile-not-proved',
    'macos-mdm-endpoint-not-proved',
    'macos-rollback-audit-not-proved',
    'ios-ci-runner-not-proved',
    'ios-family-controls-not-proved',
    'ios-device-activity-not-proved',
    'ios-managed-settings-not-proved',
    'ios-testflight-device-not-proved',
    'apple-platform-adapter-dispatch-blocked-before-ci-proof',
    'linux-foreground-capture-not-proved',
    'linux-container-policy-not-proved',
    'linux-native-session-not-proved',
    'linux-policy-mechanism-not-proved',
    'linux-rollback-not-proved',
    'linux-audit-not-proved',
    'cross-platform-child-delivery-not-proved'
  )
);

export const AppGamePlatformProofStatusRefSchema = withParser(
  Schema.Literal(
    'android-physical-device-proof-ref',
    'android-authority-preflight-ref',
    'android-accessibility-overlay-preflight-ref',
    'android-accessibility-runtime-proof-ref',
    'android-usage-events-replay-ref',
    'windows-broad-blocking-authority-preflight-ref',
    'windows-local-policy-evidence-proof-ref',
    'apple-ci-platform-proof-preflight-ref',
    'linux-foreground-capture-readiness-ref',
    'linux-active-window-tool-proof-ref',
    'linux-docker-host-preflight-ref',
    'linux-wsl-runtime-proof-ref',
    'app-game-platform-proof-status-ref'
  )
);

const PlatformProofStatusLabelSchema = brandedNonEmptyStringSchema('AppGamePlatformProofStatusLabel');
const PlatformProofStatusCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGamePlatformProofStatusRowBaseSchema = Schema.Struct({
  platform: AppGamePlatformProofStatusPlatformSchema,
  proofState: AppGamePlatformProofStatusStateSchema,
  authorityState: AppGamePlatformProofStatusAuthoritySchema,
  parentVisibleSummary: PlatformProofStatusLabelSchema,
  packageVisibilityCount: PlatformProofStatusCountSchema,
  runtimeVisibilityCount: PlatformProofStatusCountSchema,
  ownerProofAttached: Schema.Boolean,
  mechanismProofAttached: Schema.Boolean,
  rollbackProofAttached: Schema.Boolean,
  auditProofAttached: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  childDeliveryClaimed: Schema.Boolean,
  proofRefs: Schema.Array(AppGamePlatformProofStatusRefSchema),
  openGaps: Schema.Array(AppGamePlatformProofStatusGapSchema),
});

const AppGamePlatformProofStatusReadModelBaseSchema = Schema.Struct({
  schemaVersion: AppGamePlatformProofStatusSchemaVersionSchema,
  readModelId: PlatformProofStatusLabelSchema,
  generatedAt: ParentTimestampSchema,
  rows: Schema.Array(AppGamePlatformProofStatusRowBaseSchema),
  platformProofObservedCount: PlatformProofStatusCountSchema,
  visibilityOnlyCount: PlatformProofStatusCountSchema,
  enforcementReadyCount: PlatformProofStatusCountSchema,
  openGapCount: PlatformProofStatusCountSchema,
  productClaim: PlatformProofStatusLabelSchema,
});

type AppGamePlatformProofStatusRowCandidate = Infer<typeof AppGamePlatformProofStatusRowBaseSchema>;
type AppGamePlatformProofStatusReadModelCandidate = Infer<typeof AppGamePlatformProofStatusReadModelBaseSchema>;

export const AppGamePlatformProofStatusRowSchema = withParser(
  AppGamePlatformProofStatusRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        appGamePlatformProofStatusRowIsHonest(row) ||
        'Expected app/game platform proof status rows to expose visibility-only platform proof and keep enforcement, adapter dispatch, broad blocking, and child delivery unclaimed'
    )
  )
);

export const AppGamePlatformProofStatusReadModelSchema = withParser(
  AppGamePlatformProofStatusReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGamePlatformProofStatusReadModelCountsMatch(readModel) ||
        'Expected app/game platform proof status summary counts to match the platform proof rows'
    )
  )
);

export type AppGamePlatformProofStatusRow = Infer<typeof AppGamePlatformProofStatusRowSchema>;
export type AppGamePlatformProofStatusReadModel = Infer<typeof AppGamePlatformProofStatusReadModelSchema>;

function appGamePlatformProofStatusRowIsHonest(row: AppGamePlatformProofStatusRowCandidate): boolean {
  return (
    row.authorityState === 'visibility-only' &&
    !row.adapterDispatchClaimed &&
    !row.broadBlockingClaimed &&
    !row.platformEnforcementClaimed &&
    !row.childDeliveryClaimed &&
    row.openGaps.includes('cross-platform-child-delivery-not-proved') &&
    row.proofRefs.length > 0
  );
}

function appGamePlatformProofStatusReadModelCountsMatch(
  readModel: AppGamePlatformProofStatusReadModelCandidate
): boolean {
  return (
    readModel.platformProofObservedCount === readModel.rows.length &&
    readModel.visibilityOnlyCount === readModel.rows.filter((row) => row.authorityState === 'visibility-only').length &&
    readModel.enforcementReadyCount === readModel.rows.filter((row) => row.platformEnforcementClaimed).length &&
    readModel.openGapCount === readModel.rows.reduce((total, row) => total + row.openGaps.length, 0)
  );
}
