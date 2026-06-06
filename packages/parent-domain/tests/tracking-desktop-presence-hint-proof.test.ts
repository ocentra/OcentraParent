import { describe, expect, it } from 'vitest';
import {
  TrackingDesktopPresenceHintProofReadModelSchema,
  TrackingDesktopPresenceHintProofRowSchema,
  buildTrackingDesktopPresenceHintProofReadModel,
} from '../src/tracking-desktop-presence-hint-proof';

const Timestamp = '2026-06-06T09:45:00.000Z';

const ProofOptions = {
  generatedAt: Timestamp,
  proofId: 'tracking-desktop-presence-hint-proof',
  familyId: 'family-desktop-presence',
  deviceId: 'device-avery-laptop',
  childProfileId: 'child-profile-avery',
  deviceLabel: 'Avery laptop',
  platform: 'windows',
  sourceProofRefs: [
    'location-geofence-device-status',
    'workpack-13-desktop-location-and-presence-hint-model',
    'v0-5-location-platform-deep-dive',
    'location-geofence-expectation',
    'platforms-expectation',
  ],
} as const;

describe('tracking desktop presence hint proof', () => {
  it('builds desktop hint rows without GPS, precise-location, or physical-presence claims', () => {
    const readModel = buildTrackingDesktopPresenceHintProofReadModel(ProofOptions, inputRows());

    expect(readModel.rows.map((row) => row.claimState)).toEqual([
      'hint-only',
      'hint-only',
      'hint-only',
      'manual-check-in',
      'stale-offline',
      'missing-device',
      'manual-required',
    ]);
    expect(readModel.hintOnlyCount).toBe(3);
    expect(readModel.manualCheckInCount).toBe(1);
    expect(readModel.staleOfflineCount).toBe(1);
    expect(readModel.missingDeviceCount).toBe(1);
    expect(readModel.manualRequiredCount).toBe(1);
    expect(readModel.runtimeEvidenceRefs).toHaveLength(7);
    expect(Object.values(nonClaimFlags(readModel)).every((claim) => claim === false)).toBe(true);
  });

  it('keeps manual check-in and manual-required desktop OS location states separate', () => {
    const readModel = buildTrackingDesktopPresenceHintProofReadModel(ProofOptions, inputRows());
    const manualCheckIn = readModel.rows[3];
    const manualRequired = readModel.rows[6];

    expect(manualCheckIn.caseKind).toBe('manual-check-in');
    expect(manualCheckIn.manualCheckInRef).toBe('manual-check-in-avery-laptop-20260606');
    expect(manualCheckIn.parentVisibleStatusToken).toBe('tracking-desktop-presence-manual-check-in');
    expect(manualCheckIn.manualRequiredReasonRefs).toEqual([]);
    expect(manualRequired.caseKind).toBe('desktop-os-location-manual-required');
    expect(manualRequired.parentVisibleStatusToken).toBe('tracking-desktop-presence-os-location-manual-required');
    expect(manualRequired.manualRequiredReasonRefs).toEqual([
      'tracking-desktop-os-location-sample-not-proved',
      'tracking-desktop-physical-device-not-proved',
      'tracking-desktop-product-runtime-not-proved',
    ]);
  });

  it('rejects LAN, Wi-Fi, IP, stale, and missing-device rows that overclaim location authority', () => {
    const readModel = buildTrackingDesktopPresenceHintProofReadModel(ProofOptions, inputRows());
    const lanHint = readModel.rows[0];

    expect(
      TrackingDesktopPresenceHintProofRowSchema.safeParse({
        ...lanHint,
        gpsClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingDesktopPresenceHintProofRowSchema.safeParse({
        ...lanHint,
        preciseLocationClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingDesktopPresenceHintProofRowSchema.safeParse({
        ...lanHint,
        lanPairingPhysicalProofClaimed: true,
      }).success
    ).toBe(false);
    expect(
      TrackingDesktopPresenceHintProofReadModelSchema.safeParse({
        ...readModel,
        productReadyDesktopTrackingClaimed: true,
      }).success
    ).toBe(false);
  });
});

function inputRows() {
  return [
    row({
      rowId: 'desktop-presence-lan-hint',
      caseKind: 'lan-presence-hint',
      source: 'lan-pairing-presence',
      freshnessState: 'fresh-hint',
      evidenceRefs: ['lan-pairing-seen-on-home-network'],
      auditRefs: ['audit-desktop-lan-hint-no-gps'],
    }),
    row({
      rowId: 'desktop-presence-wifi-hint',
      caseKind: 'wifi-presence-hint',
      source: 'wifi-network-hint',
      freshnessState: 'fresh-hint',
      evidenceRefs: ['wifi-ssid-seen-home-network'],
      auditRefs: ['audit-desktop-wifi-hint-no-gps'],
    }),
    row({
      rowId: 'desktop-presence-ip-coarse-hint',
      caseKind: 'ip-coarse-hint',
      source: 'ip-coarse-hint',
      freshnessState: 'fresh-hint',
      evidenceRefs: ['ip-coarse-region-home-city'],
      auditRefs: ['audit-desktop-ip-hint-no-precise-location'],
    }),
    row({
      rowId: 'desktop-presence-manual-check-in',
      caseKind: 'manual-check-in',
      source: 'child-manual-check-in',
      freshnessState: 'manual-reported',
      evidenceRefs: ['manual-check-in-avery-laptop-20260606'],
      auditRefs: ['audit-desktop-manual-check-in'],
      manualCheckInRef: 'manual-check-in-avery-laptop-20260606',
    }),
    row({
      rowId: 'desktop-presence-stale-offline-last-known',
      caseKind: 'stale-offline-last-known',
      source: 'query-store-last-known',
      freshnessState: 'stale',
      evidenceRefs: ['last-known-desktop-presence-stale'],
      auditRefs: ['audit-desktop-stale-offline-no-current-location'],
      lastKnownEvidenceRef: 'last-known-desktop-presence-stale',
      stale: true,
      offline: true,
    }),
    row({
      rowId: 'desktop-presence-missing-device',
      caseKind: 'missing-device',
      source: 'query-store-missing-device',
      freshnessState: 'missing',
      evidenceRefs: ['missing-device-desktop-query-store'],
      auditRefs: ['audit-desktop-missing-device-no-current-location'],
      missingDevice: true,
      offline: true,
    }),
    row({
      rowId: 'desktop-presence-os-location-manual-required',
      caseKind: 'desktop-os-location-manual-required',
      source: 'manual-platform-plan',
      freshnessState: 'manual-required',
      evidenceRefs: ['desktop-os-location-runtime-not-proved'],
      auditRefs: ['audit-desktop-os-location-manual-required'],
    }),
  ] as const;
}

function row(input: {
  readonly rowId: string;
  readonly caseKind:
    | 'lan-presence-hint'
    | 'wifi-presence-hint'
    | 'ip-coarse-hint'
    | 'manual-check-in'
    | 'stale-offline-last-known'
    | 'missing-device'
    | 'desktop-os-location-manual-required';
  readonly source:
    | 'lan-pairing-presence'
    | 'wifi-network-hint'
    | 'ip-coarse-hint'
    | 'child-manual-check-in'
    | 'query-store-last-known'
    | 'query-store-missing-device'
    | 'manual-platform-plan';
  readonly freshnessState: 'fresh-hint' | 'manual-reported' | 'stale' | 'offline' | 'missing' | 'manual-required';
  readonly evidenceRefs: readonly string[];
  readonly auditRefs: readonly string[];
  readonly lastKnownEvidenceRef?: string;
  readonly manualCheckInRef?: string;
  readonly stale?: boolean;
  readonly offline?: boolean;
  readonly missingDevice?: boolean;
}) {
  return {
    observedAt: Timestamp,
    platform: 'windows',
    lastKnownEvidenceRef: null,
    manualCheckInRef: null,
    stale: false,
    offline: false,
    missingDevice: false,
    ...input,
  };
}

function nonClaimFlags(readModel: ReturnType<typeof buildTrackingDesktopPresenceHintProofReadModel>) {
  return {
    preciseLocationClaimed: readModel.preciseLocationClaimed,
    gpsClaimed: readModel.gpsClaimed,
    physicalPresenceClaimed: readModel.physicalPresenceClaimed,
    lanPairingPhysicalProofClaimed: readModel.lanPairingPhysicalProofClaimed,
    wifiPhysicalPresenceClaimed: readModel.wifiPhysicalPresenceClaimed,
    ipPhysicalPresenceClaimed: readModel.ipPhysicalPresenceClaimed,
    osLocationRuntimeClaimed: readModel.osLocationRuntimeClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    productReadyDesktopTrackingClaimed: readModel.productReadyDesktopTrackingClaimed,
  };
}
