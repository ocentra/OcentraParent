import { describe, expect, it } from 'vitest';
import {
  TrackingAndroidStatusProofReadModelSchema,
  TrackingAndroidStatusProofRowSchema,
  buildTrackingAndroidStatusProofReadModel,
  type TrackingAndroidStatusInputRow,
} from '../../src/tracking-android-status-proof';

const Timestamp = '2026-06-05T22:36:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-android-status-proof',
  familyId: 'family-tracking-android-status',
  childProfileId: 'child-profile-aarav',
  deviceId: 'device-aarav-android',
  deviceLabel: 'Aarav Android emulator',
  sourceProofRefs: [
    'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/04-device-status-proof.json',
    'test-results/tracking-plan-android-emulator-proof/proof.json',
    'test-results/tracking-android-physical-device-runtime-proof/proof.json',
    'docs/plans/tracking-plan/workpacks/10-android-battery-connectivity-and-status-adapter.md',
  ],
} as const;

describe('tracking Android status proof', () => {
  it('builds low-power, killed/restarted, pending-upload, and manual-required rows', () => {
    const readModel = buildTrackingAndroidStatusProofReadModel(ProofOptions, androidStatusRows());

    expectStatusSummary(readModel);
    expect(readModel.runtimeEvidenceRefs).toEqual(expectedRuntimeEvidenceRefs());
  });

  it('keeps parent-visible degraded status and audit refs attached to each open WP10 gap', () => {
    const readModel = buildTrackingAndroidStatusProofReadModel(ProofOptions, androidStatusRows());

    expectEmulatorStatusRows(readModel.rows);
    expectPhysicalStatusRow(readModel.rows[3]);
  });

  it('rejects Android status rows and read models that overclaim device runtime behavior', () => {
    const readModel = buildTrackingAndroidStatusProofReadModel(ProofOptions, androidStatusRows());
    const lowPower = readModel.rows[0];

    expect(
      TrackingAndroidStatusProofRowSchema.safeParse({
        ...lowPower,
        backgroundLocationRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingAndroidStatusProofRowSchema.safeParse({
        ...lowPower,
        physicalDeviceProofClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingAndroidStatusProofReadModelSchema.safeParse({
        ...readModel,
        productReadyAndroidTrackingClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingAndroidStatusProofReadModelSchema.safeParse({
        ...readModel,
        physicalStatusObservedCount: 0,
      }).success
    ).toBe(false);
  });
});

type TrackingAndroidStatusProofReadModel = ReturnType<typeof buildTrackingAndroidStatusProofReadModel>;
type TrackingAndroidStatusProofRow = TrackingAndroidStatusProofReadModel['rows'][number];

function expectStatusSummary(readModel: TrackingAndroidStatusProofReadModel) {
  expect(readModel.rows.map((row) => row.caseKind)).toEqual([
    'low-power-degraded',
    'app-killed-restarted',
    'pending-upload-auditable',
    'physical-status-observed',
    'manual-required',
  ]);
  expect(readModel.lowPowerDegradedCount).toBe(1);
  expect(readModel.appRestartObservedCount).toBe(1);
  expect(readModel.pendingUploadAuditableCount).toBe(1);
  expect(readModel.physicalStatusObservedCount).toBe(1);
  expect(readModel.physicalDeviceStatusEvidenceObserved).toBe(true);
  expect(readModel.manualRequiredCount).toBe(1);
}

function expectEmulatorStatusRows(rows: readonly TrackingAndroidStatusProofRow[]) {
  expect(rows[0].claimState).toBe('degraded');
  expect(rows[0].lowPowerMode).toBe(true);
  expect(rows[0].parentVisibleStatusToken).toBe('tracking-android-status-low-power-degraded');
  expect(rows[0].auditRefs).toEqual(['tracking-android-status-audit-low-power']);
  expect(rows[1].appProcessRunning).toBe(true);
  expect(rows[1].appRestartObserved).toBe(true);
  expect(rows[1].parentVisibleStatusToken).toBe('tracking-android-status-app-restarted-audit');
  expect(rows[2].pendingUploadCount).toBe(3);
  expect(rows[2].parentVisibleStatusToken).toBe('tracking-android-status-pending-upload-audit');
}

function expectPhysicalStatusRow(physicalStatus: TrackingAndroidStatusProofRow) {
  expect(physicalStatus.claimState).toBe('physical-status-observed');
  expect(physicalStatus.source).toBe('physical-device-battery-connectivity-dump');
  expect(physicalStatus.batteryPercent).toBe(83);
  expect(physicalStatus.parentVisibleStatusToken).toBe(
    'tracking-android-status-physical-battery-connectivity-observed'
  );
  expect(physicalStatus.evidenceRefs).toEqual([
    'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
    'test-results/tracking-android-physical-device-runtime-proof/08-connectivity.txt',
  ]);
}

function expectedRuntimeEvidenceRefs() {
  return [
    {
      evidenceReferenceId: 'android-battery-dumpsys-low-power',
      kind: 'activity-event',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'android-activity-manager-force-stop-and-restart',
      kind: 'activity-event',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'android-query-store-pending-upload-count',
      kind: 'query-store-summary',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
      kind: 'activity-event',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'test-results/tracking-android-physical-device-runtime-proof/08-connectivity.txt',
      kind: 'activity-event',
      observedAt: Timestamp,
    },
    {
      evidenceReferenceId: 'android-manual-platform-proof-plan',
      kind: 'policy-decision',
      observedAt: Timestamp,
    },
  ];
}

function androidStatusRows(): readonly TrackingAndroidStatusInputRow[] {
  return [
    {
      rowId: 'tracking-android-status-low-power',
      caseKind: 'low-power-degraded',
      source: 'emulator-battery-dump',
      observedAt: Timestamp,
      batteryPercent: 14,
      charging: false,
      lowPowerMode: true,
      appProcessRunning: true,
      appRestartObserved: false,
      pendingUploadCount: 0,
      evidenceRefs: ['android-battery-dumpsys-low-power'],
      auditRefs: ['tracking-android-status-audit-low-power'],
    },
    {
      rowId: 'tracking-android-status-restarted',
      caseKind: 'app-killed-restarted',
      source: 'emulator-activity-manager',
      observedAt: Timestamp,
      batteryPercent: 39,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: true,
      appRestartObserved: true,
      pendingUploadCount: 0,
      evidenceRefs: ['android-activity-manager-force-stop-and-restart'],
      auditRefs: ['tracking-android-status-audit-restart'],
    },
    {
      rowId: 'tracking-android-status-pending-upload',
      caseKind: 'pending-upload-auditable',
      source: 'query-store-pending-upload',
      observedAt: Timestamp,
      batteryPercent: 21,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: true,
      appRestartObserved: false,
      pendingUploadCount: 3,
      evidenceRefs: ['android-query-store-pending-upload-count'],
      auditRefs: ['tracking-android-status-audit-pending-upload'],
    },
    {
      rowId: 'tracking-android-status-physical-device-status',
      caseKind: 'physical-status-observed',
      source: 'physical-device-battery-connectivity-dump',
      observedAt: Timestamp,
      batteryPercent: 83,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: true,
      appRestartObserved: false,
      pendingUploadCount: 0,
      evidenceRefs: [
        'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
        'test-results/tracking-android-physical-device-runtime-proof/08-connectivity.txt',
      ],
      auditRefs: ['tracking-android-status-audit-physical-status-observed'],
    },
    {
      rowId: 'tracking-android-status-manual-required',
      caseKind: 'manual-required',
      source: 'manual-platform-plan',
      observedAt: Timestamp,
      batteryPercent: null,
      charging: false,
      lowPowerMode: false,
      appProcessRunning: false,
      appRestartObserved: false,
      pendingUploadCount: 0,
      evidenceRefs: ['android-manual-platform-proof-plan'],
      auditRefs: ['tracking-android-status-audit-manual-required'],
    },
  ];
}
