import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingIosLocationRuntimeArtifactRefs,
  TrackingIosLocationManualRequiredProofReadModelSchema,
  TrackingIosLocationManualRequiredProofRowSchema,
  buildTrackingIosLocationManualRequiredProofReadModel,
  type TrackingIosLocationManualRequiredInputRow,
} from '../src/tracking-ios-location-manual-required-proof';

const Timestamp = '2026-06-06T02:30:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-ios-location-manual-required-proof',
  familyId: 'family-tracking-ios-location-manual-required',
  childProfileId: 'child-profile-maya',
  deviceId: 'device-maya-ios',
  deviceLabel: 'Maya iOS simulator',
  sourceProofRefs: [
    'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/18-ios-simulator-proof.json',
    'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/18-ios-simulator-proof.json',
    'docs/plans/tracking-plan/workpacks/11-ios-core-location-foreground-adapter.md',
    'docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md',
  ],
} as const;

describe('tracking iOS location manual-required proof', () => {
  it('builds manual-required foreground and background proof rows from simulator package evidence', () => {
    const readModel = buildTrackingIosLocationManualRequiredProofReadModel(ProofOptions, iosLocationRows());

    expect(readModel.rows.map((row) => row.caseKind)).toEqual([
      'when-in-use-authorization-manual-required',
      'foreground-sample-manual-required',
      'denied-restricted-services-disabled-manual-required',
      'always-authorization-manual-required',
      'region-transition-manual-required',
      'significant-change-visit-manual-required',
      'background-terminated-relaunch-manual-required',
    ]);
    expect(readModel.whenInUseAuthorizationManualRequiredCount).toBe(1);
    expect(readModel.foregroundSampleManualRequiredCount).toBe(1);
    expect(readModel.degradedStateManualRequiredCount).toBe(1);
    expect(readModel.alwaysAuthorizationManualRequiredCount).toBe(1);
    expect(readModel.regionTransitionManualRequiredCount).toBe(1);
    expect(readModel.significantChangeVisitManualRequiredCount).toBe(1);
    expect(readModel.backgroundTerminatedRelaunchManualRequiredCount).toBe(1);
    expect(readModel.runtimeEvidenceRefs).toEqual(expectedRuntimeEvidenceRefs());
    expect(readModel.localEvidenceArtifactRefs).toEqual([
      'output/tracking-plan-proof/11-ios-core-location-foreground-adapter/18-ios-simulator-proof.json',
      'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/18-ios-simulator-proof.json',
    ]);
    expect(readModel.requiredRuntimeArtifactRefs).toEqual([...RequiredTrackingIosLocationRuntimeArtifactRefs]);
    expect(readModel.presentRuntimeArtifactRefs).toEqual([]);
    expect(readModel.missingRuntimeArtifactRefs).toEqual([...RequiredTrackingIosLocationRuntimeArtifactRefs]);
    expect(readModel.runtimeArtifactSetComplete).toBe(false);
  });

  it('keeps parent-visible status tokens and manual proof refs attached to each WP11/WP12 gap', () => {
    const readModel = buildTrackingIosLocationManualRequiredProofReadModel(ProofOptions, iosLocationRows());

    expect(readModel.rows.map((row) => row.parentVisibleStatusToken)).toEqual([
      'tracking-ios-when-in-use-authorization-manual-required',
      'tracking-ios-foreground-sample-manual-required',
      'tracking-ios-degraded-location-state-manual-required',
      'tracking-ios-always-authorization-manual-required',
      'tracking-ios-region-transition-manual-required',
      'tracking-ios-significant-change-visit-manual-required',
      'tracking-ios-background-terminated-relaunch-manual-required',
    ]);
    expect(readModel.rows.flatMap((row) => row.manualProofRefs)).toEqual([
      'xcode-ios-when-in-use-authorization-proof-plan',
      'physical-device-ios-foreground-location-proof-plan',
      'ios-simulator-denied-restricted-services-disabled-proof-plan',
      'apple-always-authorization-entitlement-proof-plan',
      'physical-device-ios-region-transition-proof-plan',
      'physical-device-ios-significant-change-visit-proof-plan',
      'physical-device-ios-background-terminated-relaunch-proof-plan',
    ]);
  });

  it('rejects rows and read models that overclaim iOS Core Location runtime behavior', () => {
    const readModel = buildTrackingIosLocationManualRequiredProofReadModel(ProofOptions, iosLocationRows());
    const whenInUse = readModel.rows[0];

    expect(
      TrackingIosLocationManualRequiredProofRowSchema.safeParse({
        ...whenInUse,
        whenInUseAuthorizationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingIosLocationManualRequiredProofRowSchema.safeParse({
        ...whenInUse,
        physicalDeviceProofClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingIosLocationManualRequiredProofReadModelSchema.safeParse({
        ...readModel,
        productReadyIosTrackingClaimed: true,
      }).success
    ).toBe(false);
  });
});

function expectedRuntimeEvidenceRefs() {
  return [
    {
      evidenceReferenceId: 'wp11-when-in-use-authorization-manual-required',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp11-foreground-location-sample-absent',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp11-denied-restricted-services-disabled-state-absent',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp12-always-authorization-manual-required',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp12-region-transition-count-zero',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp12-significant-change-visit-count-zero',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'wp12-background-terminated-relaunch-absent',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
  ];
}

function iosLocationRows(): readonly TrackingIosLocationManualRequiredInputRow[] {
  return [
    iosLocationRow({
      rowId: 'tracking-ios-when-in-use-authorization',
      caseKind: 'when-in-use-authorization-manual-required',
      source: 'ios-simulator-package-proof',
      evidenceRefs: ['wp11-when-in-use-authorization-manual-required'],
      manualProofRefs: ['xcode-ios-when-in-use-authorization-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-foreground-sample',
      caseKind: 'foreground-sample-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp11-foreground-location-sample-absent'],
      manualProofRefs: ['physical-device-ios-foreground-location-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-degraded-location-state',
      caseKind: 'denied-restricted-services-disabled-manual-required',
      source: 'ios-simulator-manual-plan',
      evidenceRefs: ['wp11-denied-restricted-services-disabled-state-absent'],
      manualProofRefs: ['ios-simulator-denied-restricted-services-disabled-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-always-authorization',
      caseKind: 'always-authorization-manual-required',
      source: 'apple-entitlement-manual-plan',
      evidenceRefs: ['wp12-always-authorization-manual-required'],
      manualProofRefs: ['apple-always-authorization-entitlement-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-region-transition',
      caseKind: 'region-transition-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp12-region-transition-count-zero'],
      manualProofRefs: ['physical-device-ios-region-transition-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-significant-change-visit',
      caseKind: 'significant-change-visit-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp12-significant-change-visit-count-zero'],
      manualProofRefs: ['physical-device-ios-significant-change-visit-proof-plan'],
    }),
    iosLocationRow({
      rowId: 'tracking-ios-background-terminated-relaunch',
      caseKind: 'background-terminated-relaunch-manual-required',
      source: 'physical-device-manual-plan',
      evidenceRefs: ['wp12-background-terminated-relaunch-absent'],
      manualProofRefs: ['physical-device-ios-background-terminated-relaunch-proof-plan'],
    }),
  ];
}

function iosLocationRow(
  input: Pick<
    TrackingIosLocationManualRequiredInputRow,
    'rowId' | 'caseKind' | 'source' | 'evidenceRefs' | 'manualProofRefs'
  >
): TrackingIosLocationManualRequiredInputRow {
  return {
    ...input,
    observedAt: Timestamp,
    simulatorPackageBuilt: true,
    simulatorLaunchObserved: true,
    whenInUseAuthorizationObserved: false,
    foregroundLocationSampleCaptured: false,
    deniedRestrictedStateCaptured: false,
    locationServicesDisabledStateCaptured: false,
    alwaysAuthorizationObserved: false,
    regionTransitionCount: 0,
    significantChangeEventCount: 0,
    visitEventCount: 0,
    backgroundDeliveryObserved: false,
    terminatedRelaunchObserved: false,
    entitlementProofObserved: false,
  };
}
