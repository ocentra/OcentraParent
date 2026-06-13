import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityHistoricalReportListSchema,
  ActivityReadModelStateSchema,
  ActivityReportDocumentSchema,
  ActivityReportSummarySchema,
  ActivityReportSourceStateSchema,
  ActivityReportSourceStateSummarySchema,
  ActivitySavedReportStateSchema,
  ActivitySurfaceRequestSchema,
  ActivitySurfaceSchemaVersion,
  type ActivityReportSourceReachabilityState,
  type ActivityReportSourceState,
  type ActivityReportSourceStateSummary,
} from './activity-surface';
import {
  ActivityDeviceIdSchema,
  ActivityTimestampSchema,
  type ActivityDeviceId,
} from '@ocentra-parent/evidence-domain/primitives';

type ActivityFamilyAggregationReadModelState = ReturnType<typeof ActivityReadModelStateSchema.parse>;
type ActivityFamilyAggregationHistory = ReturnType<typeof ActivityHistoricalReportListSchema.parse>;
type ActivityFamilyAggregationHistoryReport = ActivityFamilyAggregationHistory['reports'][number];

export const ActivityFamilyAggregationDataOwnerSchema = withParser(Schema.Literal('rust-service-read-model'));
export const ActivityFamilyAggregationUiConsumerSchema = withParser(Schema.Literal('c-owned-activity-ui'));

const ActivityFamilyAggregationModelBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
  request: ActivitySurfaceRequestSchema,
  state: ActivityReadModelStateSchema,
  generatedAt: ActivityTimestampSchema,
  dataOwner: ActivityFamilyAggregationDataOwnerSchema,
  uiConsumer: ActivityFamilyAggregationUiConsumerSchema,
  viteDataOwner: Schema.Literal(false),
  storageState: ActivitySavedReportStateSchema,
  storageReason: Schema.Union(ActivityReportSummarySchema, Schema.Null),
  sourceStateSummary: ActivityReportSourceStateSummarySchema,
  sourceStates: Schema.Array(ActivityReportSourceStateSchema),
  readyDeviceIds: Schema.Array(ActivityDeviceIdSchema),
  offlineDeviceIds: Schema.Array(ActivityDeviceIdSchema),
  staleDeviceIds: Schema.Array(ActivityDeviceIdSchema),
  unavailableDeviceIds: Schema.Array(ActivityDeviceIdSchema),
  unreachableDeviceIds: Schema.Array(ActivityDeviceIdSchema),
  errorDeviceIds: Schema.Array(ActivityDeviceIdSchema),
});

type ActivityFamilyAggregationModelCandidate = Infer<typeof ActivityFamilyAggregationModelBaseSchema>;

export const ActivityFamilyAggregationModelSchema = withParser(
  ActivityFamilyAggregationModelBaseSchema.pipe(
    Schema.filter(
      (model) =>
        model.request.scope.scopeKind === 'family' ||
        'Expected Activity family aggregation model request to target family scope'
    ),
    Schema.filter(
      (model) =>
        model.sourceStateSummary.totalSources === model.sourceStates.length ||
        'Expected Activity family aggregation sourceStateSummary.totalSources to match sourceStates length'
    ),
    Schema.filter(
      (model) =>
        sourceStateSummaryMatches(model.sourceStateSummary, model.sourceStates) ||
        'Expected Activity family aggregation sourceStateSummary counts to match sourceStates'
    ),
    Schema.filter(
      (model) =>
        deviceIdBucketsMatch(model) || 'Expected Activity family aggregation device-id buckets to match sourceStates'
    )
  )
);

export type ActivityFamilyAggregationDataOwner = Infer<typeof ActivityFamilyAggregationDataOwnerSchema>;
export type ActivityFamilyAggregationUiConsumer = Infer<typeof ActivityFamilyAggregationUiConsumerSchema>;
export type ActivityFamilyAggregationModel = Infer<typeof ActivityFamilyAggregationModelSchema>;

export function activityFamilyAggregationModelFromHistory(input: unknown): ActivityFamilyAggregationModel {
  const history = ActivityHistoricalReportListSchema.parse(input);
  const report = latestHistoryReport(history);
  const sourceStates = sourceStatesFromHistoryReport(report);
  return ActivityFamilyAggregationModelSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: history.request,
    state: history.state,
    generatedAt: generatedAtFromHistoryReport(history, report),
    dataOwner: 'rust-service-read-model',
    uiConsumer: 'c-owned-activity-ui',
    viteDataOwner: false,
    storageState: history.storageState,
    storageReason: history.storageReason,
    sourceStateSummary: sourceStateSummaryFromHistoryReport(report),
    sourceStates,
    readyDeviceIds: deviceIdsWithState(sourceStates, 'ready'),
    offlineDeviceIds: deviceIdsWithState(sourceStates, 'offline'),
    staleDeviceIds: deviceIdsWithState(sourceStates, 'stale'),
    unavailableDeviceIds: deviceIdsWithState(sourceStates, 'unavailable'),
    unreachableDeviceIds: deviceIdsWithReachability(sourceStates, 'unreachable'),
    errorDeviceIds: deviceIdsWithReachability(sourceStates, 'error'),
  });
}

export function activityFamilyAggregationModelFromReport(input: unknown): ActivityFamilyAggregationModel {
  const report = ActivityReportDocumentSchema.parse(input);
  return ActivityFamilyAggregationModelSchema.parse({
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: {
      schemaVersion: ActivitySurfaceSchemaVersion,
      scope: report.scope,
      requestedAt: report.requestedAt,
      rangeStart: report.rangeStart,
      rangeEnd: report.rangeEnd,
    },
    state: report.sections.some((section) => section.state === 'ready') ? 'ready' : 'empty',
    generatedAt: report.generatedAt,
    dataOwner: 'rust-service-read-model',
    uiConsumer: 'c-owned-activity-ui',
    viteDataOwner: false,
    storageState: report.savedMetadata?.savedState ?? 'draft',
    storageReason: report.savedMetadata?.storageReason ?? null,
    sourceStateSummary: summarizeSourceStates(report.sourceStates),
    sourceStates: report.sourceStates,
    readyDeviceIds: deviceIdsWithState(report.sourceStates, 'ready'),
    offlineDeviceIds: deviceIdsWithState(report.sourceStates, 'offline'),
    staleDeviceIds: deviceIdsWithState(report.sourceStates, 'stale'),
    unavailableDeviceIds: deviceIdsWithState(report.sourceStates, 'unavailable'),
    unreachableDeviceIds: deviceIdsWithReachability(report.sourceStates, 'unreachable'),
    errorDeviceIds: deviceIdsWithReachability(report.sourceStates, 'error'),
  });
}

function emptySourceStateSummary(): ActivityReportSourceStateSummary {
  return {
    totalSources: 0,
    readySources: 0,
    offlineSources: 0,
    staleSources: 0,
    unavailableSources: 0,
    unreachableSources: 0,
    errorSources: 0,
  };
}

function latestHistoryReport(
  history: ActivityFamilyAggregationHistory
): ActivityFamilyAggregationHistoryReport | undefined {
  return history.reports[0];
}

function generatedAtFromHistoryReport(
  history: ActivityFamilyAggregationHistory,
  report: ActivityFamilyAggregationHistoryReport | undefined
) {
  if (report === undefined) {
    return history.request.requestedAt;
  }
  return report.parsedReport.generatedAt;
}

function sourceStateSummaryFromHistoryReport(
  report: ActivityFamilyAggregationHistoryReport | undefined
): ActivityReportSourceStateSummary {
  if (report === undefined) {
    return emptySourceStateSummary();
  }
  return report.sourceStateSummary;
}

function sourceStatesFromHistoryReport(
  report: ActivityFamilyAggregationHistoryReport | undefined
): ReadonlyArray<ActivityReportSourceState> {
  if (report === undefined) {
    return [];
  }
  return report.parsedReport.sourceStates;
}

function summarizeSourceStates(sourceStates: readonly ActivityReportSourceState[]): ActivityReportSourceStateSummary {
  return {
    totalSources: sourceStates.length,
    readySources: countSourcesWithState(sourceStates, 'ready'),
    offlineSources: countSourcesWithState(sourceStates, 'offline'),
    staleSources: countSourcesWithState(sourceStates, 'stale'),
    unavailableSources: countSourcesWithState(sourceStates, 'unavailable'),
    unreachableSources: countSourcesWithReachability(sourceStates, 'unreachable'),
    errorSources: countSourcesWithReachability(sourceStates, 'error'),
  };
}

function sourceStateSummaryMatches(
  sourceStateSummary: ActivityReportSourceStateSummary,
  sourceStates: readonly ActivityReportSourceState[]
): boolean {
  const expected = summarizeSourceStates(sourceStates);
  return (
    sourceStateSummary.totalSources === expected.totalSources &&
    sourceStateSummary.readySources === expected.readySources &&
    sourceStateSummary.offlineSources === expected.offlineSources &&
    sourceStateSummary.staleSources === expected.staleSources &&
    sourceStateSummary.unavailableSources === expected.unavailableSources &&
    sourceStateSummary.unreachableSources === expected.unreachableSources &&
    sourceStateSummary.errorSources === expected.errorSources
  );
}

function deviceIdBucketsMatch(model: ActivityFamilyAggregationModelCandidate): boolean {
  return (
    deviceIdsMatch(model.readyDeviceIds, deviceIdsWithState(model.sourceStates, 'ready')) &&
    deviceIdsMatch(model.offlineDeviceIds, deviceIdsWithState(model.sourceStates, 'offline')) &&
    deviceIdsMatch(model.staleDeviceIds, deviceIdsWithState(model.sourceStates, 'stale')) &&
    deviceIdsMatch(model.unavailableDeviceIds, deviceIdsWithState(model.sourceStates, 'unavailable')) &&
    deviceIdsMatch(model.unreachableDeviceIds, deviceIdsWithReachability(model.sourceStates, 'unreachable')) &&
    deviceIdsMatch(model.errorDeviceIds, deviceIdsWithReachability(model.sourceStates, 'error'))
  );
}

function deviceIdsMatch(actual: ReadonlyArray<ActivityDeviceId>, expected: ReadonlyArray<ActivityDeviceId>): boolean {
  return actual.length === expected.length && actual.every((deviceId, index) => deviceId === expected[index]);
}

function countSourcesWithState(
  sourceStates: readonly ActivityReportSourceState[],
  state: ActivityFamilyAggregationReadModelState
): number {
  return sourceStates.filter((source) => source.state === state).length;
}

function countSourcesWithReachability(
  sourceStates: readonly ActivityReportSourceState[],
  reachabilityState: ActivityReportSourceReachabilityState
): number {
  return sourceStates.filter((source) => source.reachabilityState === reachabilityState).length;
}

function deviceIdsWithState(
  sourceStates: readonly ActivityReportSourceState[],
  state: ActivityFamilyAggregationReadModelState
): ReadonlyArray<ActivityDeviceId> {
  return sourceStates.filter((source) => source.state === state).map((source) => source.deviceId);
}

function deviceIdsWithReachability(
  sourceStates: readonly ActivityReportSourceState[],
  reachabilityState: ActivityReportSourceReachabilityState
): ReadonlyArray<ActivityDeviceId> {
  return sourceStates
    .filter((source) => source.reachabilityState === reachabilityState)
    .map((source) => source.deviceId);
}
