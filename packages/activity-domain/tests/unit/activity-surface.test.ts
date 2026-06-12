import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '../../src/kinds';
import {
  ActivityAppUseReadModelSchema,
  ActivityBrowserReadModelSchema,
  ActivityGamesReadModelSchema,
  ActivityHistoricalReportListSchema,
  ActivityNetworkReadModelSchema,
  ActivityReportDocumentSchema,
  ActivityReportFrequencySchema,
  ActivityReportRequestSchema,
  ActivityScreenReadModelSchema,
  ActivitySurfaceSchemaVersion,
  ActivitySurfaceScopeSchema,
} from '../../src/activity-surface';

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
  scope: DeviceScope,
  requestedAt: '2026-05-27T06:20:00Z',
  rangeStart: '2026-05-27T00:00:00Z',
  rangeEnd: '2026-05-27T06:20:00Z',
} as const;

const EvidenceRef = {
  evidenceId: 'journal-entry-activity-surface-1',
  kind: ActivityEvidenceKind.JournalEntry,
  digest: 'sha256:activity-surface',
  uri: null,
} as const;

const AppGameBoundaryCounts = {
  evidenceClaimRowCount: 1,
  identityRowCount: 1,
  approvalAuthorityRowCount: 1,
  approvalActionResultRowCount: 1,
  platformAuthorityMatrixCount: 1,
  platformAuthorityRowCount: 1,
  aiClassifierResultRowCount: 1,
} as const;

const AppGameSourceStatusRow = {
  sourceKind: 'osInstalledRecord',
  state: 'ready',
  rowCount: 1,
  lastObservedAt: '2026-05-27T06:19:00Z',
  capabilityStatus: 'available',
  evidence: [EvidenceRef],
} as const;

const ScreenChainFields = {
  captureReason: 'nativeAppForegroundStart',
  captureScope: 'activeWindow',
  capabilityStatus: 'ready',
  queueJobId: 'screen-queue-job-1',
  modelRuntimeRef: 'local-vision-runtime-1',
  modelId: 'local-vision-model-1',
  providerKind: 'localVision',
  promptOrTemplateVersion: 'screen-template-v1',
  primaryCategory: 'productivity',
  confidence: 0.91,
  imageDeletionState: 'deleted',
  rawImageRetained: false,
  policyEligible: true,
  imageDigest: 'sha256:screen-image-digest',
  custodyState: 'child-device-journal',
  policyDecisionRef: 'screen-policy-decision-1',
  policyAction: 'allow',
  policyReasonCodes: ['screen-summary-linked', 'parent-rule-linked'],
  parentRuleRefs: ['screen-parent-rule-school'],
  localModelRuntimeRefs: ['local-vision-runtime-1'],
  parentExplanationRefs: ['screen-parent-explanation-1'],
  explanationReasons: ['screen-summary-cited', 'policy-decision-cited'],
  deletionReasons: ['screen-image-deleted'],
  ocrTextSnippets: ['Homework research page [redacted]'],
  redactionNotes: ['credentialLikeTextRedacted', 'piiLikeTextRedacted'],
} as const;

function screenReadModelRow(overrides = {}) {
  return {
    rowId: 'screen-row-1',
    label: 'Visible productivity window',
    deviceId: 'child-device-1',
    state: 'ready',
    totalMs: 0,
    foregroundMs: 0,
    backgroundMs: 0,
    ...ScreenChainFields,
    evidence: [EvidenceRef],
    ...overrides,
  } as const;
}

function screenReadModel(row = screenReadModelRow()) {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-05-27T06:21:00Z',
    summary: 'Screen use ready',
    rows: [row],
  } as const;
}

const ReportDocument = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  reportId: 'activity-report-daily-1',
  frequency: 'daily',
  scope: FamilyScope,
  requestedAt: '2026-05-27T06:20:00Z',
  rangeStart: '2026-05-27T00:00:00Z',
  rangeEnd: '2026-05-27T06:20:00Z',
  generatedAt: '2026-05-27T06:21:00Z',
  savedMetadata: null,
  sourceStates: [
    {
      deviceId: 'child-device-1',
      reachabilityState: 'reachable',
      state: 'ready',
      reason: null,
      lastUpdatedAt: '2026-05-27T06:19:00Z',
      custodyLabel: 'child-device-local-summary',
      sourceLabel: 'activity-query-store-summary',
      rawChildEvidenceIncluded: false,
    },
    {
      deviceId: 'child-device-2',
      reachabilityState: 'offline',
      state: 'offline',
      reason: 'Device is offline for this family report',
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
      summary: 'One reachable device and one offline device',
      itemCount: 2,
      evidence: [EvidenceRef],
    },
    {
      sectionKind: 'network',
      title: 'Network',
      state: 'unavailable',
      summary: 'Network read model is not wired on this device',
      itemCount: 0,
      evidence: [],
    },
  ],
} as const;

const SourceStateSummary = {
  totalSources: 2,
  readySources: 1,
  offlineSources: 1,
  staleSources: 0,
  unavailableSources: 0,
  unreachableSources: 0,
  errorSources: 0,
} as const;

const SavedReportDocument = {
  ...ReportDocument,
  savedMetadata: {
    reportId: 'activity-report-daily-1',
    fileName: 'activity-report-daily-1.json',
    savedState: 'saved',
    savedAt: '2026-05-27T06:22:00Z',
    storageReason: null,
    custodyLabel: 'parent-device-local-report-json',
    sourceLabel: 'saved-report-json',
    rawChildEvidenceIncluded: false,
  },
} as const;

const SavedHistoryItem = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  reportId: 'activity-report-daily-1',
  fileName: 'activity-report-daily-1.json',
  reportDate: '2026-05-27T06:21:00Z',
  rangeStart: '2026-05-27T00:00:00Z',
  rangeEnd: '2026-05-27T06:20:00Z',
  summary: 'Daily report draft',
  savedState: 'saved',
  savedAt: '2026-05-27T06:22:00Z',
  sourceStateSummary: SourceStateSummary,
  parsedReport: SavedReportDocument,
  custodyLabel: 'parent-device-local-history',
  sourceLabel: 'saved-report-history',
  rawChildEvidenceIncluded: false,
} as const;

describe('activity surface contracts', () => {
  it('ActivitySurfaceScopeSchema: accepts family and device scopes with exact target ids', () => {
    expect(ActivitySurfaceScopeSchema.parse(FamilyScope).familyId).toBe('family-local-1');
    expect(ActivitySurfaceScopeSchema.parse(DeviceScope).deviceId).toBe('child-device-1');
  });

  it('ActivitySurfaceScopeSchema: rejects ambiguous family and device scope together', () => {
    expect(
      ActivitySurfaceScopeSchema.safeParse({
        scopeKind: 'family',
        familyId: 'family-local-1',
        deviceId: 'child-device-1',
      }).success
    ).toBe(false);
  });

  it('ActivityReportRequestSchema: accepts daily weekly and monthly report requests', () => {
    for (const frequency of ['daily', 'weekly', 'monthly'] as const) {
      const parsed = ActivityReportRequestSchema.parse({
        schemaVersion: ActivitySurfaceSchemaVersion,
        frequency,
        scope: FamilyScope,
        requestedAt: '2026-05-27T06:20:00Z',
        rangeStart: '2026-05-27T00:00:00Z',
        rangeEnd: '2026-05-27T06:20:00Z',
      });

      expect(ActivityReportFrequencySchema.parse(parsed.frequency)).toBe(frequency);
    }
  });

  it('ActivityReportDocumentSchema: accepts report sections and source availability states', () => {
    const parsed = ActivityReportDocumentSchema.parse(ReportDocument);

    expect(parsed.sourceStates[1]?.state).toBe('offline');
    expect(parsed.sourceStates[1]?.reachabilityState).toBe('offline');
    expect(parsed.sourceStates[1]?.sourceLabel).toBe('family-fanout-source-state');
    expect(parsed.sourceStates[1]?.rawChildEvidenceIncluded).toBe(false);
    expect(parsed.sections[1]?.state).toBe('unavailable');
  });

  it('ActivityReportDocumentSchema: rejects source records without reachability state', () => {
    expect(
      ActivityReportDocumentSchema.safeParse({
        ...ReportDocument,
        sourceStates: [
          {
            deviceId: 'child-device-1',
            state: 'ready',
            reason: null,
            lastUpdatedAt: '2026-05-27T06:19:00Z',
          },
        ],
      }).success
    ).toBe(false);
  });

  it('ActivityReportDocumentSchema: rejects source records that include raw child evidence', () => {
    expect(
      ActivityReportDocumentSchema.safeParse({
        ...ReportDocument,
        sourceStates: [
          {
            ...ReportDocument.sourceStates[0],
            rawChildEvidenceIncluded: true,
          },
        ],
      }).success
    ).toBe(false);
  });
});

describe('activity report history contracts', () => {
  it('ActivityHistoricalReportListSchema: carries parsed report documents with saved metadata', () => {
    const parsed = ActivityHistoricalReportListSchema.parse({
      schemaVersion: ActivitySurfaceSchemaVersion,
      request: ActivityRequest,
      state: 'ready',
      storageState: 'saved',
      storageReason: null,
      reports: [SavedHistoryItem],
    });

    expect(parsed.reports[0]?.parsedReport.savedMetadata?.savedState).toBe('saved');
    expect(parsed.reports[0]?.custodyLabel).toBe('parent-device-local-history');
    expect(parsed.reports[0]?.rawChildEvidenceIncluded).toBe(false);
    expect(parsed.reports[0]?.sourceStateSummary.offlineSources).toBe(1);
  });

  it('ActivityHistoricalReportListSchema: rejects malformed persisted report metadata', () => {
    expect(
      ActivityHistoricalReportListSchema.safeParse({
        schemaVersion: ActivitySurfaceSchemaVersion,
        request: ActivityRequest,
        state: 'ready',
        storageState: 'saved',
        storageReason: null,
        reports: [
          {
            ...SavedHistoryItem,
            fileName: '',
          },
        ],
      }).success
    ).toBe(false);
  });

  it('ActivityHistoricalReportListSchema: accepts typed storage-unavailable fallback state', () => {
    const parsed = ActivityHistoricalReportListSchema.parse({
      schemaVersion: ActivitySurfaceSchemaVersion,
      request: ActivityRequest,
      state: 'unavailable',
      storageState: 'storage-unavailable',
      storageReason: 'Local parent report storage is unavailable.',
      reports: [],
    });

    expect(parsed.storageState).toBe('storage-unavailable');
    expect(parsed.storageReason).toBe('Local parent report storage is unavailable.');
  });

  it('ActivityHistoricalReportListSchema: accepts degraded report storage with saved rows', () => {
    const parsed = ActivityHistoricalReportListSchema.parse({
      schemaVersion: ActivitySurfaceSchemaVersion,
      request: ActivityRequest,
      state: 'ready',
      storageState: 'degraded',
      storageReason: 'Some saved activity report files could not be read or parsed.',
      reports: [SavedHistoryItem],
    });

    expect(parsed.storageState).toBe('degraded');
    expect(parsed.reports[0]?.sourceStateSummary.totalSources).toBe(2);
  });
});

describe('activity screen and app-use read-model contracts', () => {
  specifyActivityScreenReadModelContracts();
  specifyActivityScreenLegacyReadModelContracts();
  specifyActivityAppUseReadModelContracts();
});

function specifyActivityScreenReadModelContracts() {
  it('ActivityScreenReadModelSchema: accepts foreground and background screen rows', () => {
    expect(
      ActivityScreenReadModelSchema.parse(
        screenReadModel(
          screenReadModelRow({
            label: 'Foreground use',
            totalMs: 3600000,
            foregroundMs: 2400000,
            backgroundMs: 1200000,
          })
        )
      ).rows[0]?.foregroundMs
    ).toBe(2400000);
  });

  it('ActivityScreenReadModelSchema: carries capture AI policy and deletion chain fields', () => {
    const parsed = ActivityScreenReadModelSchema.parse(screenReadModel());
    const row = parsed.rows[0];

    if (row === undefined) throw new Error('Expected parsed screen row');

    expect(row.captureReason).toBe('nativeAppForegroundStart');
    expect(row.modelId).toBe('local-vision-model-1');
    expect(row.providerKind).toBe('localVision');
    expect(row.promptOrTemplateVersion).toBe('screen-template-v1');
    expect(row.primaryCategory).toBe('productivity');
    expect(row.rawImageRetained).toBe(false);
    expect(row.policyDecisionRef).toBe('screen-policy-decision-1');
    expect(row.parentExplanationRefs).toEqual(['screen-parent-explanation-1']);
    expect(row.ocrTextSnippets).toEqual(['Homework research page [redacted]']);
    expect(row.redactionNotes).toEqual(['credentialLikeTextRedacted', 'piiLikeTextRedacted']);
  });

  it('ActivityScreenReadModelSchema: accepts service WinRT OCR and capture metadata rows', () => {
    const parsed = ActivityScreenReadModelSchema.parse({
      ...screenReadModel(),
      rows: [
        screenReadModelRow({
          rowId: 'screen-service-adapter-analysis-result-1',
          label: 'Windows WinRT OCR read a live page',
          captureReason: 'timedCadence',
          capabilityStatus: 'available',
          queueJobId: 'screen-service-queue-job-1',
          modelRuntimeRef: 'windows-winrt-ocr-local-runtime',
          modelId: 'windows-winrt-ocr',
          providerKind: 'localOcr',
          promptOrTemplateVersion: 'screen-ocr-worker-winrt-v1',
          primaryCategory: 'school',
          imageDigest: 'sha256:service-winrt-ocr-digest',
          rawImageRetained: false,
        }),
        screenReadModelRow({
          rowId: 'screen-service-analysis-result-1',
          label: 'Timed screen capture was queued by the local service cadence',
          captureReason: 'timedCadence',
          capabilityStatus: 'available',
          queueJobId: 'screen-service-queue-job-1',
          modelRuntimeRef: 'screen-service-deterministic-runtime',
          modelId: 'screen-service-cadence-metadata-v1',
          providerKind: 'serviceCaptureMetadata',
          promptOrTemplateVersion: 'screen-service-cadence-summary-v1',
          primaryCategory: 'unknown',
          confidence: 0.2,
          policyEligible: false,
          imageDigest: 'sha256:service-winrt-ocr-digest',
          rawImageRetained: false,
        }),
      ],
    });

    expect(parsed.rows[0]?.capabilityStatus).toBe('available');
    expect(parsed.rows[0]?.providerKind).toBe('localOcr');
    expect(parsed.rows[0]?.modelId).toBe('windows-winrt-ocr');
    expect(parsed.rows[0]?.promptOrTemplateVersion).toBe('screen-ocr-worker-winrt-v1');
    expect(parsed.rows[0]?.rawImageRetained).toBe(false);
    expect(parsed.rows[1]?.providerKind).toBe('serviceCaptureMetadata');
    expect(parsed.rows[1]?.policyEligible).toBe(false);
  });
}

function specifyActivityScreenLegacyReadModelContracts() {
  it('ActivityScreenReadModelSchema: defaults parent explanation refs for older rows', () => {
    const parsed = ActivityScreenReadModelSchema.parse(
      screenReadModel(
        screenReadModelRow({
          rowId: 'screen-row-legacy',
          policyDecisionRef: undefined,
          policyAction: undefined,
          policyReasonCodes: undefined,
          parentRuleRefs: undefined,
          localModelRuntimeRefs: undefined,
          parentExplanationRefs: undefined,
          explanationReasons: undefined,
          deletionReasons: undefined,
          ocrTextSnippets: undefined,
          redactionNotes: undefined,
        })
      )
    );

    expect(parsed.rows[0]?.policyDecisionRef).toBeNull();
    expect(parsed.rows[0]?.parentRuleRefs).toEqual([]);
    expect(parsed.rows[0]?.parentExplanationRefs).toEqual([]);
    expect(parsed.rows[0]?.ocrTextSnippets).toEqual([]);
    expect(parsed.rows[0]?.redactionNotes).toEqual([]);
  });

  it('ActivityScreenReadModelSchema: defaults raw image retention to false for older rows', () => {
    const parsed = ActivityScreenReadModelSchema.parse(
      screenReadModel(screenReadModelRow({ rowId: 'screen-row-legacy-retention', rawImageRetained: undefined }))
    );

    expect(parsed.rows[0]?.modelId).toBe('local-vision-model-1');
    expect(parsed.rows[0]?.promptOrTemplateVersion).toBe('screen-template-v1');
    expect(parsed.rows[0]?.rawImageRetained).toBe(false);
  });

  it('ActivityScreenReadModelSchema: rejects retained raw image rows', () => {
    expect(
      ActivityScreenReadModelSchema.safeParse(
        screenReadModel(screenReadModelRow({ rowId: 'screen-row-retained-image', rawImageRetained: true }))
      ).success
    ).toBe(false);
  });
}

function specifyActivityAppUseReadModelContracts() {
  it('ActivityAppUseReadModelSchema: accepts empty app-use read models', () => {
    expect(
      ActivityAppUseReadModelSchema.parse({
        schemaVersion: ActivitySurfaceSchemaVersion,
        request: ActivityRequest,
        state: 'empty',
        generatedAt: '2026-05-27T06:21:00Z',
        summary: 'No app rows',
        rows: [],
      }).state
    ).toBe('empty');
  });

  it('ActivityAppUseReadModelSchema: keeps inventory running and foreground states separate', () => {
    const parsed = ActivityAppUseReadModelSchema.parse({
      schemaVersion: ActivitySurfaceSchemaVersion,
      request: ActivityRequest,
      state: 'ready',
      generatedAt: '2026-05-27T06:21:00Z',
      summary: 'App use rows are projected from the service read model',
      rows: [
        {
          rowId: 'inventory-app-1',
          appName: 'Ocentra Fixture App',
          deviceId: 'child-device-1',
          state: 'ready',
          productKind: 'nativeApp',
          classificationState: 'knownApp',
          inventoryState: 'installed',
          runtimeState: 'running',
          foregroundState: 'foreground',
          capabilityStatus: 'available',
          lastObservedAt: '2026-05-27T06:19:00Z',
          totalMs: 60000,
          launchCount: 1,
          inventoryRowCount: 1,
          runningRowCount: 1,
          foregroundRowCount: 1,
          dailyRollupCount: 1,
          ...AppGameBoundaryCounts,
          sourceStatusRows: [AppGameSourceStatusRow],
          evidence: [EvidenceRef],
        },
      ],
    });

    expect(parsed.rows[0]?.runtimeState).toBe('running');
    expect(parsed.rows[0]?.foregroundState).toBe('foreground');
    expect(parsed.rows[0]?.inventoryRowCount).toBe(1);
    expect(parsed.rows[0]?.approvalAuthorityRowCount).toBe(1);
    expect(parsed.rows[0]?.aiClassifierResultRowCount).toBe(1);
    expect(parsed.rows[0]?.sourceStatusRows[0]?.sourceKind).toBe('osInstalledRecord');
    expect(parsed.rows[0]?.sourceStatusRows[0]?.rowCount).toBe(1);
  });
}

describe('activity browser games and network read-model contracts', () => {
  specifyActivityBrowserReadModelContracts();
  specifyActivityGamesReadModelContracts();
  specifyActivityNetworkReadModelContracts();
});

function specifyActivityBrowserReadModelContracts() {
  it('ActivityBrowserReadModelSchema: accepts permission-required browser state', () => {
    expect(
      ActivityBrowserReadModelSchema.parse({
        schemaVersion: ActivitySurfaceSchemaVersion,
        request: ActivityRequest,
        state: 'permission-required',
        generatedAt: '2026-05-27T06:21:00Z',
        summary: 'Browser bridge permission required',
        rows: [],
      }).state
    ).toBe('permission-required');
  });
}

function specifyActivityGamesReadModelContracts() {
  it('ActivityGamesReadModelSchema: accepts scaffold-only games state', () => {
    expect(
      ActivityGamesReadModelSchema.parse({
        schemaVersion: ActivitySurfaceSchemaVersion,
        request: ActivityRequest,
        state: 'scaffold-only',
        generatedAt: '2026-05-27T06:21:00Z',
        summary: 'Games catalog is scaffold-only',
        rows: [],
      }).state
    ).toBe('scaffold-only');
  });

  it('ActivityGamesReadModelSchema: accepts launcher and session source counts', () => {
    const parsed = ActivityGamesReadModelSchema.parse({
      schemaVersion: ActivitySurfaceSchemaVersion,
      request: ActivityRequest,
      state: 'ready',
      generatedAt: '2026-05-27T06:21:00Z',
      summary: 'Game rows are projected from launcher and session evidence',
      rows: [
        {
          rowId: 'game-session-1',
          displayName: 'Ocentra Fixture Game',
          deviceId: 'child-device-1',
          state: 'ready',
          productKind: 'nativeGame',
          classificationState: 'knownGame',
          inventoryState: 'detectable',
          runtimeState: 'running',
          foregroundState: 'notClaimed',
          capabilityStatus: 'available',
          lastObservedAt: '2026-05-27T06:19:00Z',
          totalMs: 120000,
          sessionCount: 2,
          launcherRowCount: 1,
          runningRowCount: 1,
          foregroundRowCount: 0,
          dailyRollupCount: 1,
          ...AppGameBoundaryCounts,
          sourceStatusRows: [
            {
              ...AppGameSourceStatusRow,
              sourceKind: 'launcherManifest',
            },
          ],
          evidence: [EvidenceRef],
        },
      ],
    });

    expect(parsed.rows[0]?.classificationState).toBe('knownGame');
    expect(parsed.rows[0]?.launcherRowCount).toBe(1);
    expect(parsed.rows[0]?.platformAuthorityRowCount).toBe(1);
    expect(parsed.rows[0]?.aiClassifierResultRowCount).toBe(1);
    expect(parsed.rows[0]?.sourceStatusRows[0]?.sourceKind).toBe('launcherManifest');
  });
}

function specifyActivityNetworkReadModelContracts() {
  it('ActivityNetworkReadModelSchema: accepts unavailable network state', () => {
    expect(
      ActivityNetworkReadModelSchema.parse({
        schemaVersion: ActivitySurfaceSchemaVersion,
        request: ActivityRequest,
        state: 'unavailable',
        generatedAt: '2026-05-27T06:21:00Z',
        summary: 'Network store unavailable',
        rows: [],
      }).state
    ).toBe('unavailable');
  });
}
