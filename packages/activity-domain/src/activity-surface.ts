import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceDigestSchema,
  ActivitySubjectNameSchema,
  ActivityTimestampSchema,
} from './primitives';
import {
  AppGameCapabilityStatusSchema,
  AppGameClassificationStateSchema,
  AppGameForegroundStateSchema,
  AppGameRuntimeStateSchema,
} from './app-game-primitives';
import { AppGameProductKindSchema } from './app-game-identity-primitives';
import { AppGameInventoryDetectionStateSchema } from './app-game-inventory-primitives';

const NonEmptyActivitySurfaceText = Schema.String.pipe(Schema.minLength(1));
const NonNegativeActivityCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NonNegativeActivityDuration = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const ActivitySurfaceSchemaVersion = 1;

export const ActivityFamilyIdSchema = NonEmptyActivitySurfaceText.pipe(Schema.brand('ActivityFamilyId'));
export const ActivityReportIdSchema = NonEmptyActivitySurfaceText.pipe(Schema.brand('ActivityReportId'));
export const ActivityReportFileNameSchema = NonEmptyActivitySurfaceText.pipe(Schema.brand('ActivityReportFileName'));
export const ActivityReportSummarySchema = NonEmptyActivitySurfaceText.pipe(Schema.brand('ActivityReportSummary'));
export const ActivitySurfaceRowIdSchema = NonEmptyActivitySurfaceText.pipe(Schema.brand('ActivitySurfaceRowId'));
export const ActivitySurfaceLabelSchema = NonEmptyActivitySurfaceText.pipe(Schema.brand('ActivitySurfaceLabel'));

export const ActivitySurfaceScopeKindSchema = withParser(Schema.Literal('family', 'device'));
export const ActivityReportFrequencySchema = withParser(Schema.Literal('daily', 'weekly', 'monthly'));
export const ActivityReportSectionKindSchema = withParser(
  Schema.Literal('summary', 'screen', 'app-use', 'browser', 'games', 'network')
);
export const ActivityReadModelStateSchema = withParser(
  Schema.Literal('ready', 'empty', 'unavailable', 'offline', 'stale', 'permission-required', 'scaffold-only')
);
export const ActivityReportSourceReachabilityStateSchema = withParser(
  Schema.Literal('reachable', 'unreachable', 'offline', 'error')
);
export const ActivitySavedReportStateSchema = withParser(
  Schema.Literal('draft', 'saved', 'storage-unavailable', 'degraded', 'scaffold-only')
);
const ActivityReportCustodyLabelSchema = withParser(
  Schema.Literal('child-device-local-summary', 'parent-device-local-report-json', 'parent-device-local-history')
);
const ActivityReportSourceLabelSchema = withParser(
  Schema.Literal(
    'activity-query-store-summary',
    'family-fanout-source-state',
    'saved-report-json',
    'saved-report-history'
  )
);

const ActivitySurfaceScopeBaseSchema = Schema.Struct({
  scopeKind: ActivitySurfaceScopeKindSchema,
  familyId: Schema.Union(ActivityFamilyIdSchema, Schema.Null),
  deviceId: Schema.Union(ActivityDeviceIdSchema, Schema.Null),
});

export const ActivitySurfaceScopeSchema = withParser(
  ActivitySurfaceScopeBaseSchema.pipe(
    Schema.filter(
      (scope) =>
        (scope.scopeKind === 'family' && scope.familyId !== null && scope.deviceId === null) ||
        (scope.scopeKind === 'device' && scope.familyId === null && scope.deviceId !== null) ||
        'Expected family scope with familyId or device scope with deviceId'
    )
  )
);

export const ActivitySurfaceRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
    scope: ActivitySurfaceScopeSchema,
    requestedAt: ActivityTimestampSchema,
    rangeStart: ActivityTimestampSchema,
    rangeEnd: ActivityTimestampSchema,
  })
);

export const ActivityReportRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
    frequency: ActivityReportFrequencySchema,
    scope: ActivitySurfaceScopeSchema,
    requestedAt: ActivityTimestampSchema,
    rangeStart: ActivityTimestampSchema,
    rangeEnd: ActivityTimestampSchema,
  })
);

export const ActivityReportSourceStateSchema = withParser(
  Schema.Struct({
    deviceId: ActivityDeviceIdSchema,
    reachabilityState: ActivityReportSourceReachabilityStateSchema,
    state: ActivityReadModelStateSchema,
    reason: Schema.Union(ActivityReportSummarySchema, Schema.Null),
    lastUpdatedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    custodyLabel: Schema.optionalWith(ActivityReportCustodyLabelSchema, {
      default: () => 'child-device-local-summary' as const,
    }),
    sourceLabel: Schema.optionalWith(ActivityReportSourceLabelSchema, {
      default: () => 'activity-query-store-summary' as const,
    }),
    rawChildEvidenceIncluded: Schema.optionalWith(Schema.Literal(false), { default: () => false as const }),
  })
);

export const ActivityReportSectionSchema = withParser(
  Schema.Struct({
    sectionKind: ActivityReportSectionKindSchema,
    title: ActivitySurfaceLabelSchema,
    state: ActivityReadModelStateSchema,
    summary: ActivityReportSummarySchema,
    itemCount: NonNegativeActivityCount,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const ActivitySavedReportMetadataSchema = withParser(
  Schema.Struct({
    reportId: ActivityReportIdSchema,
    fileName: ActivityReportFileNameSchema,
    savedState: ActivitySavedReportStateSchema,
    savedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    storageReason: Schema.Union(ActivityReportSummarySchema, Schema.Null),
    custodyLabel: Schema.optionalWith(ActivityReportCustodyLabelSchema, {
      default: () => 'parent-device-local-report-json' as const,
    }),
    sourceLabel: Schema.optionalWith(ActivityReportSourceLabelSchema, { default: () => 'saved-report-json' as const }),
    rawChildEvidenceIncluded: Schema.optionalWith(Schema.Literal(false), { default: () => false as const }),
  })
);

export const ActivityReportSourceStateSummarySchema = withParser(
  Schema.Struct({
    totalSources: NonNegativeActivityCount,
    readySources: NonNegativeActivityCount,
    offlineSources: NonNegativeActivityCount,
    staleSources: NonNegativeActivityCount,
    unavailableSources: NonNegativeActivityCount,
    unreachableSources: NonNegativeActivityCount,
    errorSources: NonNegativeActivityCount,
  })
);

export const ActivityReportDocumentSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
    reportId: ActivityReportIdSchema,
    frequency: ActivityReportFrequencySchema,
    scope: ActivitySurfaceScopeSchema,
    requestedAt: ActivityTimestampSchema,
    rangeStart: ActivityTimestampSchema,
    rangeEnd: ActivityTimestampSchema,
    generatedAt: ActivityTimestampSchema,
    savedMetadata: Schema.Union(ActivitySavedReportMetadataSchema, Schema.Null),
    sourceStates: Schema.Array(ActivityReportSourceStateSchema),
    sections: Schema.Array(ActivityReportSectionSchema),
  })
);

export const ActivityHistoricalReportListItemSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
    reportId: ActivityReportIdSchema,
    fileName: ActivityReportFileNameSchema,
    reportDate: ActivityTimestampSchema,
    rangeStart: ActivityTimestampSchema,
    rangeEnd: ActivityTimestampSchema,
    summary: ActivityReportSummarySchema,
    savedState: ActivitySavedReportStateSchema,
    savedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
    sourceStateSummary: ActivityReportSourceStateSummarySchema,
    parsedReport: ActivityReportDocumentSchema,
    custodyLabel: Schema.optionalWith(ActivityReportCustodyLabelSchema, {
      default: () => 'parent-device-local-history' as const,
    }),
    sourceLabel: Schema.optionalWith(ActivityReportSourceLabelSchema, {
      default: () => 'saved-report-history' as const,
    }),
    rawChildEvidenceIncluded: Schema.optionalWith(Schema.Literal(false), { default: () => false as const }),
  })
);

export const ActivityHistoricalReportListSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
    request: ActivitySurfaceRequestSchema,
    state: ActivityReadModelStateSchema,
    storageState: ActivitySavedReportStateSchema,
    storageReason: Schema.Union(ActivityReportSummarySchema, Schema.Null),
    reports: Schema.Array(ActivityHistoricalReportListItemSchema),
  })
);

const ActivityReadModelBaseFields = {
  schemaVersion: Schema.Literal(ActivitySurfaceSchemaVersion),
  request: ActivitySurfaceRequestSchema,
  state: ActivityReadModelStateSchema,
  generatedAt: ActivityTimestampSchema,
  summary: ActivityReportSummarySchema,
};

export const ActivityScreenReadModelSchema = withParser(
  Schema.Struct({
    ...ActivityReadModelBaseFields,
    rows: Schema.Array(
      Schema.Struct({
        rowId: ActivitySurfaceRowIdSchema,
        label: ActivitySurfaceLabelSchema,
        deviceId: ActivityDeviceIdSchema,
        state: ActivityReadModelStateSchema,
        totalMs: NonNegativeActivityDuration,
        foregroundMs: NonNegativeActivityDuration,
        backgroundMs: NonNegativeActivityDuration,
        evidence: Schema.Array(ActivityEvidenceRefSchema),
      })
    ),
  })
);

export const ActivityAppUseReadModelSchema = withParser(
  Schema.Struct({
    ...ActivityReadModelBaseFields,
    rows: Schema.Array(
      Schema.Struct({
        rowId: ActivitySurfaceRowIdSchema,
        appName: ActivitySubjectNameSchema,
        deviceId: ActivityDeviceIdSchema,
        state: ActivityReadModelStateSchema,
        productKind: AppGameProductKindSchema,
        classificationState: AppGameClassificationStateSchema,
        inventoryState: AppGameInventoryDetectionStateSchema,
        runtimeState: AppGameRuntimeStateSchema,
        foregroundState: AppGameForegroundStateSchema,
        capabilityStatus: AppGameCapabilityStatusSchema,
        lastObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
        totalMs: NonNegativeActivityDuration,
        launchCount: NonNegativeActivityCount,
        inventoryRowCount: NonNegativeActivityCount,
        runningRowCount: NonNegativeActivityCount,
        foregroundRowCount: NonNegativeActivityCount,
        dailyRollupCount: NonNegativeActivityCount,
        evidence: Schema.Array(ActivityEvidenceRefSchema),
      })
    ),
  })
);

export const ActivityBrowserReadModelSchema = withParser(
  Schema.Struct({
    ...ActivityReadModelBaseFields,
    rows: Schema.Array(
      Schema.Struct({
        rowId: ActivitySurfaceRowIdSchema,
        domainLabel: ActivitySurfaceLabelSchema,
        deviceId: ActivityDeviceIdSchema,
        state: ActivityReadModelStateSchema,
        visitCount: NonNegativeActivityCount,
        totalMs: NonNegativeActivityDuration,
        evidenceDigest: Schema.Union(ActivityEvidenceDigestSchema, Schema.Null),
      })
    ),
  })
);

export const ActivityGamesReadModelSchema = withParser(
  Schema.Struct({
    ...ActivityReadModelBaseFields,
    rows: Schema.Array(
      Schema.Struct({
        rowId: ActivitySurfaceRowIdSchema,
        displayName: ActivitySubjectNameSchema,
        deviceId: ActivityDeviceIdSchema,
        state: ActivityReadModelStateSchema,
        productKind: AppGameProductKindSchema,
        classificationState: AppGameClassificationStateSchema,
        inventoryState: AppGameInventoryDetectionStateSchema,
        runtimeState: AppGameRuntimeStateSchema,
        foregroundState: AppGameForegroundStateSchema,
        capabilityStatus: AppGameCapabilityStatusSchema,
        lastObservedAt: Schema.Union(ActivityTimestampSchema, Schema.Null),
        totalMs: NonNegativeActivityDuration,
        sessionCount: NonNegativeActivityCount,
        launcherRowCount: NonNegativeActivityCount,
        runningRowCount: NonNegativeActivityCount,
        foregroundRowCount: NonNegativeActivityCount,
        dailyRollupCount: NonNegativeActivityCount,
        evidence: Schema.Array(ActivityEvidenceRefSchema),
      })
    ),
  })
);

export const ActivityNetworkReadModelSchema = withParser(
  Schema.Struct({
    ...ActivityReadModelBaseFields,
    rows: Schema.Array(
      Schema.Struct({
        rowId: ActivitySurfaceRowIdSchema,
        destinationLabel: ActivitySurfaceLabelSchema,
        deviceId: ActivityDeviceIdSchema,
        state: ActivityReadModelStateSchema,
        connectionCount: NonNegativeActivityCount,
        totalBytes: NonNegativeActivityCount,
        evidenceDigest: Schema.Union(ActivityEvidenceDigestSchema, Schema.Null),
      })
    ),
  })
);

export type ActivitySurfaceScope = Infer<typeof ActivitySurfaceScopeSchema>;
export type ActivitySurfaceRequest = Infer<typeof ActivitySurfaceRequestSchema>;
export type ActivityReportRequest = Infer<typeof ActivityReportRequestSchema>;
export type ActivityReportSourceReachabilityState = Infer<typeof ActivityReportSourceReachabilityStateSchema>;
export type ActivityReportSourceState = Infer<typeof ActivityReportSourceStateSchema>;
export type ActivityReportSection = Infer<typeof ActivityReportSectionSchema>;
export type ActivitySavedReportMetadata = Infer<typeof ActivitySavedReportMetadataSchema>;
export type ActivityReportSourceStateSummary = Infer<typeof ActivityReportSourceStateSummarySchema>;
export type ActivityReportDocument = Infer<typeof ActivityReportDocumentSchema>;
export type ActivityHistoricalReportListItem = Infer<typeof ActivityHistoricalReportListItemSchema>;
export type ActivityHistoricalReportList = Infer<typeof ActivityHistoricalReportListSchema>;
export type ActivityScreenReadModel = Infer<typeof ActivityScreenReadModelSchema>;
export type ActivityAppUseReadModel = Infer<typeof ActivityAppUseReadModelSchema>;
export type ActivityBrowserReadModel = Infer<typeof ActivityBrowserReadModelSchema>;
export type ActivityGamesReadModel = Infer<typeof ActivityGamesReadModelSchema>;
export type ActivityNetworkReadModel = Infer<typeof ActivityNetworkReadModelSchema>;
