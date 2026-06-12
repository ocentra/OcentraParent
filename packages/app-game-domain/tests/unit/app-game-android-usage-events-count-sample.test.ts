import { describe, expect, it } from 'vitest';
import {
  AppGameAndroidUsageEventsCountSampleReadModelSchema,
  createAppGameAndroidUsageEventsCountSampleReadModel,
  summarizeAppGameAndroidUsageEventsCountSampleReadModel,
} from '../../src/app-game-android-usage-events-count-sample';

describe('app-game Android UsageEvents count sample', () => {
  it('accepts observed count-only samples without raw row or dispatch claims', () => {
    const readModel = createAppGameAndroidUsageEventsCountSampleReadModel({
      sampleState: 'sample-observed',
      sampleLookbackMillis: 900000,
      sampleEventCount: 9,
      foregroundEventCount: 3,
      checkedAt: '2026-06-08T19:50:00.000Z',
    });
    const summary = summarizeAppGameAndroidUsageEventsCountSampleReadModel(readModel);

    expect(summary.sampleState).toBe('sample-observed');
    expect(summary.sampleEventCount).toBe(9);
    expect(summary.foregroundEventCount).toBe(3);
    expect(summary.runtimeCollectionClaimed).toBe(false);
    expect(summary.adapterDispatchClaimed).toBe(false);
    expect(summary.platformEnforcementClaimed).toBe(false);
    expect(summary.childDeviceDeliveryClaimed).toBe(false);
    expect(readModel.rawUsageEventsStored).toBe(false);
    expect(readModel.packageNamesStored).toBe(false);
    expect(readModel.rawActivityRowsStored).toBe(false);
  });

  it('keeps missing settings grant as permission-required with open sample gap', () => {
    const readModel = createAppGameAndroidUsageEventsCountSampleReadModel({
      sampleState: 'sample-permission-required',
      sampleLookbackMillis: 900000,
      sampleEventCount: 0,
      foregroundEventCount: 0,
      checkedAt: '2026-06-08T19:50:00.000Z',
    });

    expect(readModel.openGaps).toEqual(
      expect.arrayContaining([
        'android-usage-stats-settings-grant-not-proved',
        'android-usage-events-runtime-sample-not-observed',
      ])
    );
  });

  it('rejects raw storage, dispatch, enforcement, delivery, and mismatched counts', () => {
    const readModel = createAppGameAndroidUsageEventsCountSampleReadModel({
      sampleState: 'sample-observed',
      sampleLookbackMillis: 900000,
      sampleEventCount: 9,
      foregroundEventCount: 3,
      checkedAt: '2026-06-08T19:50:00.000Z',
    });

    expect(
      AppGameAndroidUsageEventsCountSampleReadModelSchema.safeParse({
        ...readModel,
        rawUsageEventsStored: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCountSampleReadModelSchema.safeParse({
        ...readModel,
        adapterDispatchClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCountSampleReadModelSchema.safeParse({
        ...readModel,
        childDeviceDeliveryClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameAndroidUsageEventsCountSampleReadModelSchema.safeParse({
        ...readModel,
        foregroundEventCount: 10,
      }).success
    ).toBe(false);
  });
});
