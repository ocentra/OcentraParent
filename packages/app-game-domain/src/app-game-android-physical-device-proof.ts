import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameAndroidPhysicalDeviceProofSchemaVersionSchema = withParser(
  Schema.Literal('app-game-android-physical-device-proof')
);

export const AppGameAndroidPhysicalDeviceConnectionStateSchema = withParser(
  Schema.Literal('physical-device-connected')
);

export const AppGameAndroidPhysicalDeviceKindSchema = withParser(Schema.Literal('physical-device'));

export const AppGameAndroidPhysicalDevicePrivilegeStateSchema = withParser(
  Schema.Literal('not-device-owner', 'not-profile-owner', 'not-proved')
);

export const AppGameAndroidPhysicalDeviceServiceStateSchema = withParser(
  Schema.Literal('service-visible', 'service-not-visible')
);

export const AppGameAndroidPhysicalDeviceUsageEventsDumpStateSchema = withParser(
  Schema.Literal('usage-events-dump-observed', 'usage-events-dump-unavailable')
);

export const AppGameAndroidPhysicalDeviceProofRefSchema = withParser(
  Schema.Literal(
    'android-physical-adb-device-ref',
    'android-physical-build-prop-ref',
    'android-physical-package-manager-ref',
    'android-physical-usage-stats-service-ref',
    'android-physical-usage-events-dump-ref',
    'android-physical-device-policy-ref'
  )
);

const AndroidPhysicalDeviceLabelSchema = brandedNonEmptyStringSchema('AppGameAndroidPhysicalDeviceLabel');

const AndroidPhysicalDeviceApiLevelSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(1));
const AndroidPhysicalDevicePackageCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(1));
const AndroidPhysicalDeviceEventCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const AppGameAndroidPhysicalDeviceProofBaseSchema = Schema.Struct({
  schemaVersion: AppGameAndroidPhysicalDeviceProofSchemaVersionSchema,
  proofId: AndroidPhysicalDeviceLabelSchema,
  targetKind: AppGameAndroidPhysicalDeviceKindSchema,
  connectionState: AppGameAndroidPhysicalDeviceConnectionStateSchema,
  adbTargetRef: AndroidPhysicalDeviceLabelSchema,
  product: AndroidPhysicalDeviceLabelSchema,
  model: AndroidPhysicalDeviceLabelSchema,
  device: AndroidPhysicalDeviceLabelSchema,
  androidRelease: AndroidPhysicalDeviceLabelSchema,
  sdkInt: AndroidPhysicalDeviceApiLevelSchema,
  supportedAbiCount: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(1)),
  packageManagerVisibleCount: AndroidPhysicalDevicePackageCountSchema,
  usageStatsServiceState: AppGameAndroidPhysicalDeviceServiceStateSchema,
  usageEventsDumpState: AppGameAndroidPhysicalDeviceUsageEventsDumpStateSchema,
  usageEventsSampleCount: AndroidPhysicalDeviceEventCountSchema,
  foregroundActivityEventCount: AndroidPhysicalDeviceEventCountSchema,
  deviceOwnerState: AppGameAndroidPhysicalDevicePrivilegeStateSchema,
  profileOwnerState: AppGameAndroidPhysicalDevicePrivilegeStateSchema,
  proofRefs: Schema.Array(AppGameAndroidPhysicalDeviceProofRefSchema),
  packageNamesRedacted: Schema.Boolean,
  usageEventsPackageNamesRedacted: Schema.Boolean,
  rawDeviceSerialRedacted: Schema.Boolean,
  foregroundEvidenceObserved: Schema.Boolean,
  hideSuspendClaimed: Schema.Boolean,
  adapterDispatchClaimed: Schema.Boolean,
  broadBlockingClaimed: Schema.Boolean,
  platformEnforcementClaimed: Schema.Boolean,
  parentVisibleSummary: AndroidPhysicalDeviceLabelSchema,
  checkedAt: ParentTimestampSchema,
});

type AppGameAndroidPhysicalDeviceProofCandidate = Infer<typeof AppGameAndroidPhysicalDeviceProofBaseSchema>;

export const AppGameAndroidPhysicalDeviceProofSchema = withParser(
  AppGameAndroidPhysicalDeviceProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        androidPhysicalDeviceProofIsHonest(proof) ||
        'Expected Android physical device proof to use a physical ADB target, redact package names/serials, and keep app hide/suspend/enforcement claims false without owner proof'
    )
  )
);

export type AppGameAndroidPhysicalDeviceProof = Infer<typeof AppGameAndroidPhysicalDeviceProofSchema>;
export type AppGameAndroidPhysicalDeviceProofRef = Infer<typeof AppGameAndroidPhysicalDeviceProofRefSchema>;

export const decodeAppGameAndroidPhysicalDeviceProof = Schema.decodeUnknownSync(
  AppGameAndroidPhysicalDeviceProofSchema
);

export function summarizeAppGameAndroidPhysicalDeviceProof(proof: AppGameAndroidPhysicalDeviceProof) {
  return {
    targetKind: proof.targetKind,
    model: proof.model,
    androidRelease: proof.androidRelease,
    sdkInt: proof.sdkInt,
    packageManagerVisibleCount: proof.packageManagerVisibleCount,
    usageStatsServiceState: proof.usageStatsServiceState,
    usageEventsDumpState: proof.usageEventsDumpState,
    usageEventsSampleCount: proof.usageEventsSampleCount,
    foregroundActivityEventCount: proof.foregroundActivityEventCount,
    foregroundEvidenceObserved: proof.foregroundEvidenceObserved,
    ownerProofAttached:
      proof.deviceOwnerState !== 'not-device-owner' || proof.profileOwnerState !== 'not-profile-owner',
    adapterDispatchClaimed: proof.adapterDispatchClaimed,
    platformEnforcementClaimed: proof.platformEnforcementClaimed,
  } as const;
}

function androidPhysicalDeviceProofIsHonest(proof: AppGameAndroidPhysicalDeviceProofCandidate): boolean {
  return (
    proof.targetKind === 'physical-device' &&
    proof.connectionState === 'physical-device-connected' &&
    proof.adbTargetRef === 'android-physical-adb-device-ref' &&
    proof.proofRefs.includes('android-physical-adb-device-ref') &&
    proof.proofRefs.includes('android-physical-build-prop-ref') &&
    proof.proofRefs.includes('android-physical-package-manager-ref') &&
    proof.proofRefs.includes('android-physical-device-policy-ref') &&
    proof.packageNamesRedacted &&
    proof.usageEventsPackageNamesRedacted &&
    proof.rawDeviceSerialRedacted &&
    androidUsageEventsProofIsHonest(proof) &&
    !proof.hideSuspendClaimed &&
    !proof.adapterDispatchClaimed &&
    !proof.broadBlockingClaimed &&
    !proof.platformEnforcementClaimed
  );
}

function androidUsageEventsProofIsHonest(proof: AppGameAndroidPhysicalDeviceProofCandidate): boolean {
  if (proof.usageEventsDumpState === 'usage-events-dump-unavailable') {
    return (
      proof.usageEventsSampleCount === 0 &&
      proof.foregroundActivityEventCount === 0 &&
      !proof.foregroundEvidenceObserved
    );
  }

  return (
    proof.proofRefs.includes('android-physical-usage-events-dump-ref') &&
    proof.usageEventsSampleCount > 0 &&
    proof.foregroundActivityEventCount > 0 &&
    proof.foregroundEvidenceObserved
  );
}

