import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidUsageEventsGrantedSampleProofSchema,
  createAppGameAndroidUsageEventsGrantedSampleProof,
  summarizeAppGameAndroidUsageEventsGrantedSampleProof,
} from '../../src/app-game-android-usage-events-granted-sample-proof';

const expectRejectedGrantedSample = (candidate: unknown): void => {
  expect(AppGameAndroidUsageEventsGrantedSampleProofSchema.safeParse(candidate).success).toBe(false);
};

describe('app-game Android UsageEvents granted sample proof', () => {
  it('accepts a granted count-only UsageEvents sample without raw event custody', () => {
    const readModel = createAppGameAndroidUsageEventsGrantedSampleProof({
      sampleEventCount: 4,
      foregroundEventCount: 2,
      checkedAt: '2026-06-08T21:40:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsGrantedSampleProof(readModel);

    expect(summary.permissionCheckState).toBe('usage-stats-granted');
    expect(summary.sampleState).toBe('sample-observed');
    expect(summary.sampleEventCount).toBe(4);
    expect(summary.foregroundEventCount).toBe(2);
    expect(readModel.rawUsageEventsStored).toBe(false);
    expect(readModel.rawPackageNamesStored).toBe(false);
    expect(readModel.adapterDispatchClaimed).toBe(false);
    expect(readModel.platformEnforcementClaimed).toBe(false);
    expect(readModel.childDeviceDeliveryClaimed).toBe(false);
  });

  it('keeps authority, delivery, and enforcement gaps explicit after sample proof', () => {
    const readModel = createAppGameAndroidUsageEventsGrantedSampleProof({
      sampleEventCount: 1,
      foregroundEventCount: 0,
      checkedAt: '2026-06-08T21:40:00.000Z',
    });

    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'android-device-owner-authority-not-proved',
        'android-play-policy-not-proved',
        'android-child-runtime-delivery-not-proved',
        'android-platform-enforcement-not-proved',
      ])
    );
  });

  it('rejects empty samples, raw rows, dispatch, delivery, and enforcement claims', () => {
    const readModel = createAppGameAndroidUsageEventsGrantedSampleProof({
      sampleEventCount: 1,
      foregroundEventCount: 0,
      checkedAt: '2026-06-08T21:40:00.000Z',
    });

    expectRejectedGrantedSample({ ...readModel, sampleEventCount: 0 });
    expectRejectedGrantedSample({ ...readModel, rawUsageEventsStored: true });
    expectRejectedGrantedSample({ ...readModel, rawActivityRowsStored: true });
    expectRejectedGrantedSample({ ...readModel, adapterDispatchClaimed: true });
    expectRejectedGrantedSample({ ...readModel, platformEnforcementClaimed: true });
    expectRejectedGrantedSample({ ...readModel, childDeviceDeliveryClaimed: true });
  });
});
