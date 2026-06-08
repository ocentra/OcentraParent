import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { ActivitySurfaceSchemaVersion } from '@ocentra-parent/activity-domain/activity-surface';
import { createParentPortalActivityUiIntent } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';

type ActivityUiIntent = ReturnType<typeof createParentPortalActivityUiIntent>;

const ActivityRequest = {
  schemaVersion: ActivitySurfaceSchemaVersion,
  scope: {
    scopeKind: 'device',
    familyId: null,
    deviceId: 'child-device-1',
  },
  requestedAt: '2026-06-01T15:00:00Z',
  rangeStart: '2026-06-01T00:00:00Z',
  rangeEnd: '2026-06-01T15:00:00Z',
} as const;

describe('parent portal app/game dashboard intent', () => {
  it('separates inventory, running, foreground, launcher, unknown, manual, and evidence state', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: adapterResult(appUseReadModel()),
        activityAppGamePlatformExtensionReadModel: adapterResult(appGamePlatformExtensionReadModel()),
        activityGamesReadModel: adapterResult(gamesReadModel()),
      },
      3
    );

    expectPopulatedDashboard(intent.appGameDashboard);
  });

  it('keeps evidence boundary gates from upgrading weaker app/game evidence', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: adapterResult(evidenceBoundaryAppUseReadModel()),
        activityGamesReadModel: adapterResult(evidenceBoundaryGamesReadModel()),
      },
      3
    );

    expectEvidenceBoundarySafetyGates(intent.appGameDashboard);
  });

  it('renders malicious app/game metadata as escaped bounded text', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: adapterResult(appUseReadModel()),
        activityGamesReadModel: adapterResult(gamesReadModel()),
      },
      3
    );
    const maliciousLabel = String(
      intent.appGameDashboard.rows.find((row) => row.rowId === 'app-row-malicious-name')?.label
    );
    const markup = renderToStaticMarkup(createElement('svg', null, createElement('text', null, maliciousLabel)));
    const rendererSource = readFileSync(
      new URL(
        '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface.tsx',
        import.meta.url
      ),
      'utf8'
    );

    expect(markup).toContain('VPN Proxy Portable');
    expect(markup).toContain('&lt;script&gt;alert(1)&lt;/script&gt;');
    expect(markup).not.toContain('<script>alert(1)</script>');
    expect(markup).not.toContain('<script');
    expect(markup).not.toContain('dangerouslySetInnerHTML');
    expect(rendererSource).toContain('{truncateTextForWidth(row.label, w - 28, titleSize, 0.58)}');
    expect(rendererSource).not.toContain('dangerouslySetInnerHTML');
    expect(markup).not.toContain('C:\\Users\\child\\AppData\\Local\\Study Timer\\study-timer.exe');
    expect(markup).not.toContain('C:\\Program Files\\VoxelQuest\\VoxelQuest.exe');
  });

  it('does not create dashboard rows when app/game adapter data is absent or failed', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: { ok: false, reason: 'invalid-json', state: 'unavailable' },
        activityGamesReadModel: null,
      },
      2
    );

    expect(intent.appGameDashboard.rows).toEqual([]);
    expect(intent.appGameDashboard.emptyMessage).toContain('No app/game read model rows');
    expect(intent.appGameDashboard.metrics.map((metric) => [metric.label, metric.value])).toContainEqual([
      'Manual required',
      '0',
    ]);
  });
});

function expectPopulatedDashboard(dashboard: ActivityUiIntent['appGameDashboard']) {
  const metricPairs = dashboard.metrics.map((metric) => [metric.label, metric.value]);
  expect(dashboard.summary).toContain('4 service-backed app/game rows');
  expect(metricPairs).toContainEqual(['Inventory', '3']);
  expect(metricPairs).toContainEqual(['Running', '4']);
  expect(metricPairs).toContainEqual(['Foreground', '2']);
  expect(metricPairs).toContainEqual(['Launcher', '1']);
  expect(metricPairs).toContainEqual(['Source rows', '6']);
  expect(metricPairs).toContainEqual(['Fresh sources', '4']);
  expectBoundaryCountVisibility(dashboard, metricPairs);
  expect(dashboard.rows.map((row) => [row.label, row.inventoryCount, row.runningCount])).toContainEqual([
    'Study Timer',
    1,
    1,
  ]);
  expect(dashboard.rows.map((row) => [row.label, row.foregroundCount, row.evidenceCount])).toContainEqual([
    'Study Timer',
    1,
    2,
  ]);
  expect(dashboard.rows.map((row) => [row.label, row.launcherOnly, row.unknownApproval])).toContainEqual([
    'Steam Launcher',
    true,
    false,
  ]);
  expect(dashboard.rows.map((row) => [row.label, row.manualRequired, row.riskCandidate])).toContainEqual([
    'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
    true,
    true,
  ]);
  expect(
    dashboard.rows.map((row) => [row.label, row.inventoryState, row.capabilityStatus, row.manualRequired])
  ).toContainEqual([
    'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
    'stale',
    'manual-required',
    true,
  ]);
  expect(dashboard.rows.map((row) => [row.label, row.capabilityStatus, row.manualRequired, row.tone])).toContainEqual([
    'Voxel Quest Candidate',
    'permission-required',
    true,
    'gold',
  ]);
  expect(dashboard.rows.some((row) => row.label.includes('<script>alert(1)</script>'))).toBe(true);
  expect(dashboard.capabilityRows.map((row) => row.label)).toContain('manual-required');
  expect(dashboard.capabilityRows.map((row) => row.label)).toContain('permission-required');
  expectReadinessBlockerCards(dashboard, metricPairs);
  expectPlatformCapabilityLimitations(dashboard, metricPairs);
  expectSourceStatusRows(dashboard);
  expectSourcePanelSections(dashboard);
  expectNoRawExecutablePathLeak(dashboard);
  expectMaliciousMetadataStaysTextOnly(dashboard);
}

function expectBoundaryCountVisibility(
  dashboard: ActivityUiIntent['appGameDashboard'],
  metricPairs: (string | number)[][]
) {
  expect(metricPairs).toContainEqual(['Boundary rows', '7']);
  expect(metricPairs).toContainEqual(['AI classifier', '1']);
  expect(
    dashboard.rows.map((row) => [
      row.label,
      row.evidenceClaimRowCount,
      row.identityRowCount,
      row.approvalAuthorityRowCount,
      row.approvalActionResultRowCount,
      row.platformAuthorityRowCount,
      row.aiClassifierResultRowCount,
      row.boundaryRowCount,
    ])
  ).toContainEqual(['Study Timer', 1, 1, 1, 0, 1, 1, 5]);
}

function expectReadinessBlockerCards(
  dashboard: ActivityUiIntent['appGameDashboard'],
  metricPairs: (string | number)[][]
) {
  const evidenceRows = dashboard.evidenceRows.map((row) => [row.label, row.value, row.tone]);
  expect(metricPairs).toContainEqual(['Readiness blockers', '7']);
  expect(evidenceRows).toContainEqual([
    'Study Timer approval blocker',
    'Approval action result missing; 1/0; policy manual-required',
    'gold',
  ]);
  expect(evidenceRows).toContainEqual([
    'Study Timer AI review',
    '1 classifier rows; evidence-only; no direct action',
    'gold',
  ]);
  expect(evidenceRows).toContainEqual([
    'Steam Launcher manual blocker',
    'manual-required; manual-required; adapter dispatch not claimed',
    'gold',
  ]);
  expect(evidenceRows.map((row) => row.join(' ')).join(' ')).toContain('approval review');
}

function expectPlatformCapabilityLimitations(
  dashboard: ActivityUiIntent['appGameDashboard'],
  metricPairs: (string | number)[][]
) {
  const platformRows = dashboard.platformCapabilityRows.map((row) => [
    row.platform,
    row.setupState,
    row.proofPackState,
    row.adapterExecutionClaim,
    row.requiredProofCount,
    row.broadBlockingClaimed,
    row.childDeviceDeliveryClaimed,
  ]);
  const evidenceRows = dashboard.evidenceRows.map((row) => [row.label, row.value, row.tone]);

  expect(metricPairs).toContainEqual(['Platform gaps', '4']);
  expect(metricPairs).toContainEqual(['Adapter executed', '0']);
  expect(platformRows).toContainEqual([
    'macos',
    'manual-required',
    'manual-proof-pack-required',
    'not-executed',
    4,
    false,
    false,
  ]);
  expect(platformRows).toContainEqual([
    'android',
    'manual-required',
    'manual-proof-pack-required',
    'not-executed',
    4,
    false,
    false,
  ]);
  expect(evidenceRows).toContainEqual([
    'macos platform proof',
    'manual-required; manual-proof-pack-required; 4 proof refs; adapter not-executed; broad block not claimed; delivery not claimed',
    'gold',
  ]);
  expect(evidenceRows.map((row) => row.join(' ')).join(' ')).toContain('ios platform proof');
  expect(dashboard.capabilityRows.map((row) => [row.label, row.value])).toContainEqual(['Platform gaps', '4 rows']);
  expect(JSON.stringify(dashboard)).not.toContain('providerDispatchTarget');
  expect(JSON.stringify(dashboard)).not.toContain('rawPlatformDiagnostics');
}

function expectSourceStatusRows(dashboard: ActivityUiIntent['appGameDashboard']) {
  expect(
    dashboard.sourceStatusRows.map((row) => [row.parentLabel, row.sourceStatusKind, row.rowCount, row.evidenceCount])
  ).toContainEqual(['Study Timer', 'shortcut', 1, 1]);
  expect(dashboard.sourceStatusRows.map((row) => [row.parentLabel, row.sourceStatusLabel])).toContainEqual([
    'Steam Launcher',
    'Launcher Manifest',
  ]);
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('refs');
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('source rows');
  expect(dashboard.evidenceRows.map((row) => row.label)).toContain('Study Timer boundary');
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('Approval 1/0');
  expect(dashboard.evidenceRows.map((row) => row.value).join(' ')).toContain('AI 1');
}

function expectSourcePanelSections(dashboard: ActivityUiIntent['appGameDashboard']) {
  expect(
    dashboard.sourcePanelSections.map((section) => [
      section.title,
      section.rowCount,
      section.freshCount,
      section.manualRequiredCount,
      section.evidenceCount,
    ])
  ).toEqual([
    ['App use sources', 4, 3, 1, 4],
    ['Game sources', 2, 1, 1, 2],
  ]);
  expect(dashboard.sourcePanelSections.map((section) => section.subtitle)).toEqual([
    '3 fresh of 4 source rows; 1 manual-required',
    '1 fresh of 2 source rows; 1 manual-required',
  ]);
  expect(
    dashboard.sourcePanelSections.flatMap((section) =>
      section.rows.map((row) => [section.title, row.parentLabel, row.sourceStatusLabel, row.freshnessLabel])
    )
  ).toContainEqual(['App use sources', 'Study Timer', 'Foreground Window', 'Fresh source']);
  expect(
    dashboard.sourcePanelSections.flatMap((section) =>
      section.rows.map((row) => [section.title, row.parentLabel, row.sourceStatusLabel, row.freshnessLabel])
    )
  ).toContainEqual(['Game sources', 'Steam Launcher', 'Launcher Manifest', 'Needs review']);
  expect(dashboard.sourcePanelSections.flatMap((section) => section.metrics.map((metric) => metric.label))).toEqual([
    'Fresh',
    'Rows',
    'Manual',
    'Evidence',
    'Fresh',
    'Rows',
    'Manual',
    'Evidence',
  ]);
}

function expectNoRawExecutablePathLeak(dashboard: ActivityUiIntent['appGameDashboard']) {
  const serializedDashboard = JSON.stringify(dashboard);
  expect(serializedDashboard).not.toContain('C:\\Users\\child\\AppData\\Local\\Study Timer\\study-timer.exe');
  expect(serializedDashboard).not.toContain('C:\\Program Files\\VoxelQuest\\VoxelQuest.exe');
  expect(serializedDashboard).not.toContain('executablePathRef');
}

function expectMaliciousMetadataStaysTextOnly(dashboard: ActivityUiIntent['appGameDashboard']) {
  const maliciousLabel =
    'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row';
  const maliciousRow = dashboard.rows.find((row) => row.rowId === 'app-row-malicious-name');
  expect(maliciousRow?.label).toBe(maliciousLabel);
  expect(maliciousRow?.manualRequired).toBe(true);
  expect(maliciousRow?.riskCandidate).toBe(true);
  expect(maliciousRow?.tone).toBe('gold');
  expect(dashboard.rows.length).toBe(4);
  expect(dashboard.rows.map((row) => row.rowId)).toContain('app-row-malicious-name');
}

function expectEvidenceBoundarySafetyGates(dashboard: ActivityUiIntent['appGameDashboard']) {
  expectDashboardRow(dashboard, 'app-row-inventory-only').toMatchObject({
    inventoryCount: 1,
    runningCount: 0,
    foregroundCount: 0,
    totalDurationLabel: '0 min',
    unknownApproval: false,
  });
  expectDashboardRow(dashboard, 'app-row-running-only').toMatchObject({
    inventoryCount: 0,
    runningCount: 1,
    foregroundCount: 0,
    totalDurationLabel: '0 min',
    manualRequired: false,
  });
  expectDashboardRow(dashboard, 'app-row-foreground-with-title-ref').toMatchObject({
    foregroundCount: 1,
    evidenceCount: 1,
    productKind: 'native-app',
  });
  expectDashboardRow(dashboard, 'game-row-launcher-only').toMatchObject({
    launcherOnly: true,
    launcherCount: 1,
    foregroundCount: 0,
    productKind: 'launcher',
  });
  expectDashboardRow(dashboard, 'app-row-unknown-process').toMatchObject({
    unknownApproval: true,
    classificationState: 'unknown-process',
    productKind: 'native-app',
    manualRequired: false,
  });
  expect(JSON.stringify(dashboard)).not.toContain('Secret homework window title');
  expect(JSON.stringify(dashboard)).not.toContain('privateForegroundContent');
  expect(dashboard.rows.some((row) => row.classificationState === 'known-game')).toBe(false);
}

function expectDashboardRow(dashboard: ActivityUiIntent['appGameDashboard'], rowId: string) {
  const row = dashboard.rows.find((candidate) => candidate.rowId === rowId);
  expect(row).toBeDefined();
  return expect(row);
}

function appUseReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'App-use read model from service projection',
    rows: [studyTimerAppRow(), manualRequiredAppRow()],
  } as const;
}

function evidenceBoundaryAppUseReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-06-01T15:10:01Z',
    summary: 'App-use read model with boundary rows',
    rows: [inventoryOnlyAppRow(), runningOnlyAppRow(), foregroundWithTitleRefAppRow(), unknownProcessAppRow()],
  } as const;
}

function evidenceBoundaryGamesReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'ready',
    generatedAt: '2026-06-01T15:10:01Z',
    summary: 'Games read model with launcher boundary rows',
    rows: [launcherOnlyGameRow()],
  } as const;
}

function inventoryOnlyAppRow() {
  return {
    rowId: 'app-row-inventory-only',
    appName: 'Installed Only Reference',
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-app',
    classificationState: 'known-app',
    inventoryState: 'installed',
    runtimeState: 'not-running',
    foregroundState: 'not-foreground',
    capabilityStatus: 'ready',
    lastObservedAt: '2026-06-01T15:09:00Z',
    totalMs: 0,
    launchCount: 0,
    inventoryRowCount: 1,
    runningRowCount: 0,
    foregroundRowCount: 0,
    dailyRollupCount: 0,
    evidence: [{ evidenceId: 'boundary-inventory-1', sourceId: 'inventory', capturedAt: '2026-06-01T15:09:00Z' }],
    sourceStatusRows: [
      sourceStatusRow('shortcut', 'ready', 'available', 'boundary-source-1', 'shortcut', '2026-06-01T15:09:00Z'),
    ],
  } as const;
}

function runningOnlyAppRow() {
  return {
    rowId: 'app-row-running-only',
    appName: 'Running Background Reference',
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-app',
    classificationState: 'known-app',
    inventoryState: 'not-reported',
    runtimeState: 'running',
    foregroundState: 'not-foreground',
    capabilityStatus: 'ready',
    lastObservedAt: '2026-06-01T15:08:00Z',
    totalMs: 0,
    launchCount: 0,
    inventoryRowCount: 0,
    runningRowCount: 1,
    foregroundRowCount: 0,
    dailyRollupCount: 0,
    evidence: [{ evidenceId: 'boundary-running-1', sourceId: 'runtime', capturedAt: '2026-06-01T15:08:00Z' }],
    sourceStatusRows: [
      sourceStatusRow('processSnapshot', 'ready', 'available', 'boundary-source-2', 'process', '2026-06-01T15:08:00Z'),
    ],
  } as const;
}

function foregroundWithTitleRefAppRow() {
  return {
    rowId: 'app-row-foreground-with-title-ref',
    appName: 'Foreground Ref Only',
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-app',
    classificationState: 'known-app',
    inventoryState: 'not-reported',
    runtimeState: 'running',
    foregroundState: 'foreground',
    capabilityStatus: 'ready',
    windowTitleRef: 'Secret homework window title',
    privateForegroundContent: 'privateForegroundContent',
    lastObservedAt: '2026-06-01T15:07:00Z',
    totalMs: 60000,
    launchCount: 1,
    inventoryRowCount: 0,
    runningRowCount: 1,
    foregroundRowCount: 1,
    dailyRollupCount: 0,
    evidence: [{ evidenceId: 'boundary-foreground-1', sourceId: 'foreground', capturedAt: '2026-06-01T15:07:00Z' }],
    sourceStatusRows: [
      sourceStatusRow(
        'foregroundWindow',
        'ready',
        'available',
        'boundary-source-3',
        'foreground',
        '2026-06-01T15:07:00Z'
      ),
    ],
  } as const;
}

function unknownProcessAppRow() {
  return {
    rowId: 'app-row-unknown-process',
    appName: 'Unknown Helper Process',
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-app',
    classificationState: 'unknown-process',
    inventoryState: 'not-reported',
    runtimeState: 'running',
    foregroundState: 'not-foreground',
    capabilityStatus: 'ready',
    lastObservedAt: '2026-06-01T15:06:00Z',
    totalMs: 0,
    launchCount: 0,
    inventoryRowCount: 0,
    runningRowCount: 1,
    foregroundRowCount: 0,
    dailyRollupCount: 0,
    evidence: [{ evidenceId: 'boundary-unknown-1', sourceId: 'runtime', capturedAt: '2026-06-01T15:06:00Z' }],
    sourceStatusRows: [
      sourceStatusRow('processSnapshot', 'ready', 'available', 'boundary-source-4', 'process', '2026-06-01T15:06:00Z'),
    ],
  } as const;
}

function launcherOnlyGameRow() {
  return {
    rowId: 'game-row-launcher-only',
    displayName: 'Launcher Only Reference',
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'launcher',
    classificationState: 'known-launcher',
    inventoryState: 'installed',
    runtimeState: 'running',
    foregroundState: 'not-foreground',
    capabilityStatus: 'ready',
    lastObservedAt: '2026-06-01T15:05:00Z',
    totalMs: 0,
    sessionCount: 0,
    launcherRowCount: 1,
    runningRowCount: 1,
    foregroundRowCount: 0,
    dailyRollupCount: 0,
    evidence: [{ evidenceId: 'boundary-launcher-1', sourceId: 'launcher', capturedAt: '2026-06-01T15:05:00Z' }],
    sourceStatusRows: [
      sourceStatusRow(
        'launcherManifest',
        'ready',
        'available',
        'boundary-source-5',
        'launcher',
        '2026-06-01T15:05:00Z'
      ),
    ],
  } as const;
}

function studyTimerAppRow() {
  return {
    rowId: 'app-row-study-timer',
    appName: 'Study Timer',
    deviceId: 'child-device-1',
    state: 'ready',
    productKind: 'native-app',
    classificationState: 'known-app',
    inventoryState: 'installed',
    runtimeState: 'running',
    foregroundState: 'foreground',
    capabilityStatus: 'ready',
    executablePathRef: 'C:\\Users\\child\\AppData\\Local\\Study Timer\\study-timer.exe',
    lastObservedAt: '2026-06-01T15:00:00Z',
    totalMs: 900000,
    launchCount: 2,
    inventoryRowCount: 1,
    runningRowCount: 1,
    foregroundRowCount: 1,
    dailyRollupCount: 1,
    evidenceClaimRowCount: 1,
    identityRowCount: 1,
    approvalAuthorityRowCount: 1,
    approvalActionResultRowCount: 0,
    platformAuthorityRowCount: 1,
    aiClassifierResultRowCount: 1,
    evidence: [
      { evidenceId: 'app-ev-1', sourceId: 'journal', capturedAt: '2026-06-01T15:00:00Z' },
      { evidenceId: 'app-ev-2', sourceId: 'sqlite', capturedAt: '2026-06-01T15:00:00Z' },
    ],
    sourceStatusRows: readyAppSourceStatusRows(),
  } as const;
}

function manualRequiredAppRow() {
  return {
    rowId: 'app-row-malicious-name',
    appName:
      'VPN Proxy Portable <script>alert(1)</script> with a display name that is deliberately too long for one row',
    deviceId: 'child-device-2',
    state: 'manual-required',
    productKind: 'native-app',
    classificationState: 'unknown-process',
    inventoryState: 'stale',
    runtimeState: 'running',
    foregroundState: 'not-foreground',
    capabilityStatus: 'manual-required',
    lastObservedAt: '2026-06-01T14:58:00Z',
    totalMs: 120000,
    launchCount: 1,
    inventoryRowCount: 0,
    runningRowCount: 1,
    foregroundRowCount: 0,
    dailyRollupCount: 0,
    evidenceClaimRowCount: 1,
    identityRowCount: 1,
    approvalAuthorityRowCount: 0,
    approvalActionResultRowCount: 0,
    platformAuthorityRowCount: 0,
    aiClassifierResultRowCount: 0,
    evidence: [{ evidenceId: 'app-ev-3', sourceId: 'sqlite', capturedAt: '2026-06-01T14:58:00Z' }],
    sourceStatusRows: [manualRequiredSourceStatusRow()],
  } as const;
}

function readyAppSourceStatusRows() {
  return [
    sourceStatusRow('shortcut', 'ready', 'available', 'app-source-1', 'shortcut', '2026-06-01T15:00:00Z'),
    sourceStatusRow('processSnapshot', 'ready', 'available', 'app-source-2', 'process', '2026-06-01T15:00:00Z'),
    sourceStatusRow('foregroundWindow', 'ready', 'available', 'app-source-3', 'foreground', '2026-06-01T15:00:00Z'),
  ] as const;
}

function manualRequiredSourceStatusRow(
  sourceKind = 'processSnapshot',
  evidenceId = 'app-source-4',
  sourceId = 'process',
  capturedAt = '2026-06-01T14:58:00Z'
) {
  return sourceStatusRow(sourceKind, 'manual-required', 'manualRequired', evidenceId, sourceId, capturedAt);
}

function sourceStatusRow(
  sourceKind: string,
  state: string,
  capabilityStatus: string,
  evidenceId: string,
  sourceId: string,
  capturedAt: string
) {
  return {
    sourceKind,
    state,
    rowCount: 1,
    lastObservedAt: capturedAt,
    capabilityStatus,
    evidence: [{ evidenceId, sourceId, capturedAt }],
  } as const;
}

function gamesReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    request: ActivityRequest,
    state: 'manual-required',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'Games read model from service projection',
    rows: [gameLauncherRow(), gameCandidateRow()],
  } as const;
}

function appGamePlatformExtensionReadModel() {
  return {
    schemaVersion: ActivitySurfaceSchemaVersion,
    state: 'manual-required',
    generatedAt: '2026-06-01T15:00:01Z',
    summary: 'App/game platform extension proof-pack readiness from service projection',
    rows: [
      platformCapabilityRow('macos', 'desktop-hard-control'),
      platformCapabilityRow('ios', 'mobile-supervised-shield'),
      platformCapabilityRow('android', 'device-owner-control'),
      platformCapabilityRow('linux', 'desktop-hard-control'),
    ],
  } as const;
}

function platformCapabilityRow(platform: string, authorityTier: string) {
  return {
    platform,
    state: 'manual-required',
    setupState: 'manual-required',
    proofPackState: 'manual-proof-pack-required',
    authorityTier,
    adapterExecutionClaim: 'not-executed',
    broadBlockingClaimed: false,
    privilegedMobileClaimed: false,
    childDeviceDeliveryClaimed: false,
    requiredProofRefs: [
      `${platform}-setup-proof`,
      `${platform}-inventory-proof`,
      `${platform}-blocking-proof`,
      `${platform}-rollback-proof`,
    ],
    providerDispatchTarget: `${platform}-private-provider-target`,
    rawPlatformDiagnostics: `${platform}-private-diagnostics`,
  } as const;
}

function gameLauncherRow() {
  return {
    rowId: 'game-row-launcher',
    displayName: 'Steam Launcher',
    deviceId: 'child-device-2',
    state: 'manual-required',
    productKind: 'launcher',
    classificationState: 'known-launcher',
    inventoryState: 'installed',
    runtimeState: 'running',
    foregroundState: 'not-foreground',
    capabilityStatus: 'manual-required',
    lastObservedAt: '2026-06-01T14:57:00Z',
    totalMs: 600000,
    sessionCount: 1,
    launcherRowCount: 1,
    runningRowCount: 1,
    foregroundRowCount: 0,
    dailyRollupCount: 1,
    evidence: [{ evidenceId: 'game-ev-1', sourceId: 'journal', capturedAt: '2026-06-01T14:57:00Z' }],
    sourceStatusRows: [
      manualRequiredSourceStatusRow('launcherManifest', 'game-source-1', 'launcher', '2026-06-01T14:57:00Z'),
    ],
  } as const;
}

function gameCandidateRow() {
  return {
    rowId: 'game-row-candidate',
    displayName: 'Voxel Quest Candidate',
    deviceId: 'child-device-2',
    state: 'manual-required',
    productKind: 'native-game',
    classificationState: 'possible-game',
    inventoryState: 'detected',
    runtimeState: 'running',
    foregroundState: 'foreground',
    capabilityStatus: 'permission-required',
    executablePathRef: 'C:\\Program Files\\VoxelQuest\\VoxelQuest.exe',
    lastObservedAt: '2026-06-01T14:56:00Z',
    totalMs: 300000,
    sessionCount: 1,
    launcherRowCount: 0,
    runningRowCount: 1,
    foregroundRowCount: 1,
    dailyRollupCount: 0,
    evidence: [{ evidenceId: 'game-ev-2', sourceId: 'sqlite', capturedAt: '2026-06-01T14:56:00Z' }],
    sourceStatusRows: [
      sourceStatusRow('foregroundWindow', 'ready', 'available', 'game-source-2', 'foreground', '2026-06-01T14:56:00Z'),
    ],
  } as const;
}

function adapterResult(value: Record<string, unknown>) {
  return {
    ok: true,
    state: value['state'] ?? 'ready',
    value,
  } as const;
}
