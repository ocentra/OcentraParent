import { describe, expect, it } from 'vitest';
import {
  TrackingAndroidPermissionBackgroundProofReadModelSchema,
  TrackingAndroidPermissionBackgroundProofRowSchema,
  buildTrackingAndroidPermissionBackgroundProofReadModel,
  type TrackingAndroidPermissionBackgroundInputRow,
} from '../src/tracking-android-permission-background-proof';

const Timestamp = '2026-06-05T23:46:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-android-permission-background-proof',
  familyId: 'family-tracking-android-permission-background',
  childProfileId: 'child-profile-aarav',
  deviceId: 'device-aarav-android',
  deviceLabel: 'Aarav Android emulator',
  sourceProofRefs: [
    'output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json',
    'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json',
    'docs/plans/tracking-plan/workpacks/08-android-foreground-location-adapter.md',
    'docs/plans/tracking-plan/workpacks/09-android-background-location-and-geofence-adapter.md',
  ],
} as const;

describe('tracking Android permission and background proof', () => {
  it('builds manual-required foreground and background proof rows from emulator scaffold evidence', () => {
    const readModel = buildTrackingAndroidPermissionBackgroundProofReadModel(ProofOptions, permissionRows());

    expect(readModel.rows.map((row) => row.caseKind)).toEqual([
      'foreground-permission-manual-required',
      'foreground-sample-manual-required',
      'background-permission-manual-required',
      'geofence-transition-manual-required',
    ]);
    expect(readModel.foregroundPermissionManualRequiredCount).toBe(1);
    expect(readModel.foregroundSampleManualRequiredCount).toBe(1);
    expect(readModel.backgroundPermissionManualRequiredCount).toBe(1);
    expect(readModel.geofenceTransitionManualRequiredCount).toBe(1);
    expect(readModel.runtimeEvidenceRefs).toEqual(expectedRuntimeEvidenceRefs());
  });

  it('keeps parent-visible status tokens and manual proof refs attached to each open WP08/WP09 gap', () => {
    const readModel = buildTrackingAndroidPermissionBackgroundProofReadModel(ProofOptions, permissionRows());

    expect(readModel.rows.map((row) => row.parentVisibleStatusToken)).toEqual([
      'tracking-android-foreground-permission-manual-required',
      'tracking-android-foreground-sample-manual-required',
      'tracking-android-background-permission-manual-required',
      'tracking-android-geofence-transition-manual-required',
    ]);
    expect(readModel.rows.flatMap((row) => row.manualProofRefs)).toEqual([
      'android-studio-foreground-permission-proof-plan',
      'physical-device-foreground-location-proof-plan',
      'android-settings-background-permission-proof-plan',
      'physical-device-geofence-transition-proof-plan',
    ]);
  });

  it('rejects rows and read models that overclaim Android runtime behavior', () => {
    const readModel = buildTrackingAndroidPermissionBackgroundProofReadModel(ProofOptions, permissionRows());
    const foregroundPermission = readModel.rows[0];

    expect(
      TrackingAndroidPermissionBackgroundProofRowSchema.safeParse({
        ...foregroundPermission,
        foregroundPermissionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingAndroidPermissionBackgroundProofRowSchema.safeParse({
        ...foregroundPermission,
        physicalDeviceProofClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingAndroidPermissionBackgroundProofReadModelSchema.safeParse({
        ...readModel,
        productReadyAndroidTrackingClaimed: true,
      }).success
    ).toBe(false);
  });
});

function expectedRuntimeEvidenceRefs() {
  return [
    {
      evidenceReferenceId: 'wp08-runtime-location-evidence-manual-required',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp08-foreground-location-sample-absent',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp09-background-permission-manual-required',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp09-geofence-transition-count-zero',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
  ];
}

function permissionRows(): readonly TrackingAndroidPermissionBackgroundInputRow[] {
  return [
    {
      rowId: 'tracking-android-foreground-permission',
      caseKind: 'foreground-permission-manual-required',
      source: 'android-emulator-foreground-proof',
      observedAt: Timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp08-runtime-location-evidence-manual-required'],
      manualProofRefs: ['android-studio-foreground-permission-proof-plan'],
    },
    {
      rowId: 'tracking-android-foreground-sample',
      caseKind: 'foreground-sample-manual-required',
      source: 'physical-device-manual-plan',
      observedAt: Timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp08-foreground-location-sample-absent'],
      manualProofRefs: ['physical-device-foreground-location-proof-plan'],
    },
    {
      rowId: 'tracking-android-background-permission',
      caseKind: 'background-permission-manual-required',
      source: 'android-emulator-background-proof',
      observedAt: Timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp09-background-permission-manual-required'],
      manualProofRefs: ['android-settings-background-permission-proof-plan'],
    },
    {
      rowId: 'tracking-android-geofence-transition',
      caseKind: 'geofence-transition-manual-required',
      source: 'physical-device-manual-plan',
      observedAt: Timestamp,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      foregroundPermissionRequested: false,
      foregroundLocationSampleCaptured: false,
      backgroundPermissionRequested: false,
      geofenceTransitionCount: 0,
      evidenceRefs: ['wp09-geofence-transition-count-zero'],
      manualProofRefs: ['physical-device-geofence-transition-proof-plan'],
    },
  ];
}
