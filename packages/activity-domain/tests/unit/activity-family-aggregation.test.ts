import { describe, expect, it } from 'vitest';
import {
  ActivityFamilyAggregationModelSchema,
  activityFamilyAggregationModelFromHistory,
  activityFamilyAggregationModelFromReport,
} from '../../src/activity-family-aggregation';
import { ActivitySurfaceSchemaVersion } from '../../src/activity-surface';

type ActivityFamilyAggregationModel = ReturnType<typeof activityFamilyAggregationModelFromReport>;

const FamilyScope = {
  scopeKind: 'family',
  familyId: 'family-local-1',
  deviceId: null,
} as const;

const DeviceScope = {
  scopeKind: 'device',
  familyId: null,
  deviceId: 'child-device-1',
} as const;

const ActivityRequest = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: FamilyScope,
  requestedAt: '2026-06-01T00:00:00Z',
  rangeStart: '2026-05-31T00:00:00Z',
  rangeEnd: '2026-06-01T00:00:00Z',
} as const;

const FamilyReport = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  reportId: 'activity-report-daily-family-1',
  frequency: 'daily',
  scope: FamilyScope,
  requestedAt: '2026-06-01T00:00:00Z',
  rangeStart: '2026-05-31T00:00:00Z',
  rangeEnd: '2026-06-01T00:00:00Z',
  generatedAt: '2026-06-01T00:01:00Z',
  savedMetadata: {
    reportId: 'activity-report-daily-family-1',
    fileName: 'activity-report-daily-family-1.json',
    savedState: 'saved',
    savedAt: '2026-06-01T00:02:00Z',
    storageReason: null,
    custodyLabel: 'parent-device-local-report-json',
    sourceLabel: 'saved-report-json',
    rawChildEvidenceIncluded: false,
  },
  sourceStates: [
    {
      deviceId: 'child-device-1',
      reachabilityState: 'reachable',
      state: 'ready',
      reason: 'Local child source is ready',
      lastUpdatedAt: '2026-06-01T00:00:30Z',
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'activity-query-store-summary',
      rawChildEvidenceIncluded: false,
    },
    {
      deviceId: 'child-device-2',
      reachabilityState: 'offline',
      state: 'offline',
      reason: 'Child device is offline',
      lastUpdatedAt: null,
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'family-fanout-source-state',
      rawChildEvidenceIncluded: false,
    },
    {
      deviceId: 'child-device-3',
      reachabilityState: 'unreachable',
      state: 'stale',
      reason: 'Child source is stale',
      lastUpdatedAt: '2026-05-31T23:00:00Z',
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'family-fanout-source-state',
      rawChildEvidenceIncluded: false,
    },
    {
      deviceId: 'child-device-4',
      reachabilityState: 'error',
      state: 'unavailable',
      reason: 'Child source returned an error',
      lastUpdatedAt: null,
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'family-fanout-source-state',
      rawChildEvidenceIncluded: false,
    },
  ],
  sections: [
    {
      sectionKind: 'summary',
      title: 'Summary',
      state: 'ready',
      summary: 'Family report has mixed source states',
      itemCount: 4,
      evidence: [],
    },
  ],
} as const;

const FamilyHistory = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  request: ActivityRequest,
  state: 'ready',
  storageState: 'saved',
  storageReason: null,
  reports: [
    {
      schemaVersion: ActivitySurfaceSchemaVersion,
      reportId: 'activity-report-daily-family-1',
      fileName: 'activity-report-daily-family-1.json',
      reportDate: '2026-06-01T00:02:00Z',
      rangeStart: '2026-05-31T00:00:00Z',
      rangeEnd: '2026-06-01T00:00:00Z',
      summary: 'Family report has mixed source states',
      savedState: 'saved',
      savedAt: '2026-06-01T00:02:00Z',
      sourceStateSummary: {
        totalSources: 4,
        readySources: 1,
        offlineSources: 1,
        staleSources: 1,
        unavailableSources: 1,
        unreachableSources: 1,
        errorSources: 1,
      },
      parsedReport: FamilyReport,
      custodyLabel: 'parent-device-local-history',
      sourceLabel: 'saved-report-history',
      rawChildEvidenceIncluded: false,
    },
  ],
} as const;

function expectServiceOwnedFamilyBuckets(model: ActivityFamilyAggregationModel) {
  expect(model.dataOwner).toBe('rust-service-read-model');
  expect(model.uiConsumer).toBe('c-owned-activity-ui');
  expect(model.viteDataOwner).toBe(false);
  expect(model.storageState).toBe('saved');
  expect(model.sourceStates.every((source) => source.rawChildEvidenceIncluded === false)).toBe(true);
  expect(model.sourceStates.map((source) => source.sourceLabel)).toEqual([
    'activity-query-store-summary',
    'family-fanout-source-state',
    'family-fanout-source-state',
    'family-fanout-source-state',
  ]);
  expect(model.sourceStateSummary.totalSources).toBe(4);
  expect(model.readyDeviceIds).toEqual(['child-device-1']);
  expect(model.offlineDeviceIds).toEqual(['child-device-2']);
  expect(model.staleDeviceIds).toEqual(['child-device-3']);
  expect(model.unreachableDeviceIds).toEqual(['child-device-3']);
  expect(model.unavailableDeviceIds).toEqual(['child-device-4']);
  expect(model.errorDeviceIds).toEqual(['child-device-4']);
}

function expectMixedSourceSummary(model: ActivityFamilyAggregationModel) {
  expect(model.sourceStateSummary.readySources).toBe(1);
  expect(model.sourceStateSummary.offlineSources).toBe(1);
  expect(model.sourceStateSummary.staleSources).toBe(1);
  expect(model.sourceStateSummary.unavailableSources).toBe(1);
  expect(model.sourceStateSummary.unreachableSources).toBe(1);
  expect(model.sourceStateSummary.errorSources).toBe(1);
}

function expectAggregationRejected(input: unknown) {
  expect(ActivityFamilyAggregationModelSchema.safeParse(input).success).toBe(false);
}

function expectStaleReadySourceCountRejected(model: ActivityFamilyAggregationModel) {
  expectAggregationRejected({
    ...model,
    sourceStateSummary: {
      ...model.sourceStateSummary,
      readySources: 0,
    },
  });
}

function expectStaleReadyDeviceBucketRejected(model: ActivityFamilyAggregationModel) {
  expectAggregationRejected({
    ...model,
    readyDeviceIds: ['child-device-2'],
  });
}

function expectStorageUnavailableFallback(model: ActivityFamilyAggregationModel) {
  expect(model.state).toBe('unavailable');
  expect(model.storageState).toBe('storage-unavailable');
  expect(model.sourceStateSummary.totalSources).toBe(0);
  expect(model.sourceStates).toEqual([]);
}

function expectDeviceScopedRecordRejected(model: ActivityFamilyAggregationModel) {
  expectAggregationRejected({
    ...model,
    request: {
      ...ActivityRequest,
      scope: DeviceScope,
    },
  });
}

describe('activity family aggregation contracts', () => {
  it('builds a service-owned family aggregation model from saved report history', () => {
    const model = activityFamilyAggregationModelFromHistory(FamilyHistory);

    expectServiceOwnedFamilyBuckets(model);
  });

  it('builds the same aggregation counts from a generated report document', () => {
    const model = activityFamilyAggregationModelFromReport(FamilyReport);

    expectMixedSourceSummary(model);
  });

  it('preserves typed storage-unavailable fallback without promoting it to ready data', () => {
    const model = activityFamilyAggregationModelFromHistory({
      schemaVersion: ActivitySurfaceSchemaVersion,
      request: ActivityRequest,
      state: 'unavailable',
      storageState: 'storage-unavailable',
      storageReason: 'Local parent report storage is unavailable.',
      reports: [],
    });

    expectStorageUnavailableFallback(model);
  });

  it('rejects device-scoped records as family aggregation models', () => {
    expectDeviceScopedRecordRejected(activityFamilyAggregationModelFromReport(FamilyReport));
  });

  it('rejects mismatched source summaries and Vite-owned product data claims', () => {
    const model = activityFamilyAggregationModelFromHistory(FamilyHistory);

    expectAggregationRejected({
      ...model,
      sourceStateSummary: {
        ...model.sourceStateSummary,
        totalSources: 99,
      },
    });

    expectAggregationRejected({
      ...model,
      viteDataOwner: true,
    });
  });

  it('rejects stale source summary counts when totalSources still matches', () => {
    const model = activityFamilyAggregationModelFromHistory(FamilyHistory);

    expectStaleReadySourceCountRejected(model);
  });

  it('rejects stale device-id buckets when source counts still match', () => {
    const model = activityFamilyAggregationModelFromHistory(FamilyHistory);

    expectStaleReadyDeviceBucketRejected(model);
  });
});
