import {
  createParentPortalAppGameSourcePanelSections,
  type ParentPortalAppGameSourcePanelSection,
} from './app-game-source-panel-intent';

export type ParentPortalAppGameDashboardTone = 'cyan' | 'gold' | 'purple' | 'red';

export type ParentPortalAppGameDashboardMetric = {
  readonly label: string;
  readonly value: string;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGameDashboardRow = {
  readonly sourceKind: 'app-use' | 'games';
  readonly sourceLabel: string;
  readonly rowId: string;
  readonly label: string;
  readonly deviceId: string;
  readonly state: string;
  readonly productKind: string;
  readonly classificationState: string;
  readonly inventoryState: string;
  readonly runtimeState: string;
  readonly foregroundState: string;
  readonly capabilityStatus: string;
  readonly lastObservedLabel: string;
  readonly totalDurationLabel: string;
  readonly eventCountLabel: string;
  readonly inventoryCount: number;
  readonly runningCount: number;
  readonly foregroundCount: number;
  readonly launcherCount: number;
  readonly dailyRollupCount: number;
  readonly evidenceCount: number;
  readonly evidenceClaimRowCount: number;
  readonly identityRowCount: number;
  readonly approvalAuthorityRowCount: number;
  readonly approvalActionResultRowCount: number;
  readonly platformAuthorityRowCount: number;
  readonly aiClassifierResultRowCount: number;
  readonly boundaryRowCount: number;
  readonly unknownApproval: boolean;
  readonly riskCandidate: boolean;
  readonly manualRequired: boolean;
  readonly launcherOnly: boolean;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGameSourceStatusRow = {
  readonly readModelKind: ParentPortalAppGameDashboardRow['sourceKind'];
  readonly sourceLabel: string;
  readonly parentRowId: string;
  readonly parentLabel: string;
  readonly sourceStatusKind: string;
  readonly sourceStatusLabel: string;
  readonly state: string;
  readonly rowCount: number;
  readonly lastObservedLabel: string;
  readonly capabilityStatus: string;
  readonly evidenceCount: number;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGamePlatformCapabilityRow = {
  readonly platform: string;
  readonly state: string;
  readonly adapterExecutionClaim: string;
  readonly proofPackState: string;
  readonly setupState: string;
  readonly authorityTier: string;
  readonly requiredProofCount: number;
  readonly broadBlockingClaimed: boolean;
  readonly privilegedMobileClaimed: boolean;
  readonly childDeviceDeliveryClaimed: boolean;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGameDashboardIntent = {
  readonly state: string;
  readonly summary: string;
  readonly metricsState: 'reported' | 'unavailable';
  readonly metricsUnavailableMessage: string | null;
  readonly appRows: readonly ParentPortalAppGameDashboardRow[];
  readonly gameRows: readonly ParentPortalAppGameDashboardRow[];
  readonly rows: readonly ParentPortalAppGameDashboardRow[];
  readonly sourceStatusRows: readonly ParentPortalAppGameSourceStatusRow[];
  readonly platformCapabilityRows: readonly ParentPortalAppGamePlatformCapabilityRow[];
  readonly sourcePanelSections: readonly ParentPortalAppGameSourcePanelSection[];
  readonly metrics: readonly ParentPortalAppGameDashboardMetric[];
  readonly capabilityRows: readonly ParentPortalAppGameDashboardMetric[];
  readonly evidenceRows: readonly ParentPortalAppGameDashboardMetric[];
  readonly emptyMessage: string;
};

export function createParentPortalAppGameDashboardIntent(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null,
  platformExtensionReadModel: Record<string, unknown> | null = null
): ParentPortalAppGameDashboardIntent {
  const appRows = appDashboardRows(appUseReadModel);
  const gameRows = gameDashboardRows(gamesReadModel);
  const rows = [...appRows, ...gameRows].sort(dashboardRowSort);
  const sourceStatusRows = [
    ...appGameSourceStatusRows(appUseReadModel, 'app-use', 'App use', 'appName'),
    ...appGameSourceStatusRows(gamesReadModel, 'games', 'Game', 'displayName'),
  ].sort(sourceStatusRowSort);
  const platformCapabilityRows = appGamePlatformCapabilityRows(platformExtensionReadModel);
  const sourcePanelSections = createParentPortalAppGameSourcePanelSections(sourceStatusRows);
  const metricsReported = combinedReadModelsReported(appUseReadModel, gamesReadModel);
  const metrics = metricsReported
    ? appGameDashboardMetrics(appRows, gameRows, rows, sourceStatusRows, platformCapabilityRows)
    : [];
  const state = dashboardState(appUseReadModel, gamesReadModel, rows, metricsReported);

  return {
    state,
    summary: dashboardSummary(appUseReadModel, gamesReadModel, rows),
    metricsState: metricsReported ? 'reported' : 'unavailable',
    metricsUnavailableMessage: metricsReported
      ? null
      : 'Measured totals are hidden until both app-use and games read models are reported by the local service.',
    appRows,
    gameRows,
    rows,
    sourceStatusRows,
    platformCapabilityRows,
    sourcePanelSections,
    metrics,
    capabilityRows: capabilityRows(rows, platformCapabilityRows),
    evidenceRows: evidenceRows(rows, sourceStatusRows, platformCapabilityRows),
    emptyMessage: 'No app/game read model rows reported by the local service.',
  };
}

function appDashboardRows(readModel: Record<string, unknown> | null): readonly ParentPortalAppGameDashboardRow[] {
  return readModelRows(readModel).map((row, index) =>
    dashboardRowFromRecord('app-use', row, index, {
      labelField: 'appName',
      countField: 'launchCount',
      countLabel: 'launches',
      launcherField: null,
    })
  );
}

function gameDashboardRows(readModel: Record<string, unknown> | null): readonly ParentPortalAppGameDashboardRow[] {
  return readModelRows(readModel).map((row, index) =>
    dashboardRowFromRecord('games', row, index, {
      labelField: 'displayName',
      countField: 'sessionCount',
      countLabel: 'sessions',
      launcherField: 'launcherRowCount',
    })
  );
}

function dashboardRowFromRecord(
  sourceKind: ParentPortalAppGameDashboardRow['sourceKind'],
  row: Record<string, unknown>,
  index: number,
  config: {
    readonly labelField: string;
    readonly countField: string;
    readonly countLabel: string;
    readonly launcherField: string | null;
  }
): ParentPortalAppGameDashboardRow {
  const classificationState = stringValue(row['classificationState']) || 'not-reported';
  const inventoryState = stringValue(row['inventoryState']) || 'not-reported';
  const runtimeState = stringValue(row['runtimeState']) || 'not-reported';
  const foregroundState = stringValue(row['foregroundState']) || 'not-reported';
  const capabilityStatus = stringValue(row['capabilityStatus']) || 'not-reported';
  const productKind = stringValue(row['productKind']) || 'not-reported';
  const inventoryCount = inventoryCountValue(row['inventoryRowCount'], inventoryState);
  const runningCount = numberValue(row['runningRowCount']);
  const foregroundCount = numberValue(row['foregroundRowCount']);
  const launcherCount = config.launcherField ? numberValue(row[config.launcherField]) : 0;
  const evidenceCount = arrayCount(row['evidence']);
  const evidenceClaimRowCount = numberValue(row['evidenceClaimRowCount']);
  const identityRowCount = numberValue(row['identityRowCount']);
  const approvalAuthorityRowCount = numberValue(row['approvalAuthorityRowCount']);
  const approvalActionResultRowCount = numberValue(row['approvalActionResultRowCount']);
  const platformAuthorityRowCount = numberValue(row['platformAuthorityRowCount']);
  const aiClassifierResultRowCount = numberValue(row['aiClassifierResultRowCount']);
  const boundaryRowCount =
    evidenceClaimRowCount +
    identityRowCount +
    approvalAuthorityRowCount +
    approvalActionResultRowCount +
    platformAuthorityRowCount +
    aiClassifierResultRowCount;
  const unknownApproval = appGameUnknownApproval(classificationState, inventoryState, runtimeState);
  const riskCandidate = appGameRiskCandidate(classificationState, productKind, stringValue(row[config.labelField]));
  const manualRequired = appGameManualRequired(capabilityStatus, inventoryState, runtimeState, foregroundState);
  const launcherOnly =
    sourceKind === 'games' && (launcherCount > 0 || appGameLauncherOnly(classificationState, productKind));
  const eventCount = numberValue(row[config.countField]);

  return {
    sourceKind,
    sourceLabel: sourceKind === 'app-use' ? 'App use' : 'Game',
    rowId: stringValue(row['rowId']) || `${sourceKind}-${index + 1}`,
    label: stringValue(row[config.labelField]) || 'Unlabeled app/game row',
    deviceId: stringValue(row['deviceId']) || 'not-reported',
    state: stringValue(row['state']) || 'not-reported',
    productKind,
    classificationState,
    inventoryState,
    runtimeState,
    foregroundState,
    capabilityStatus,
    lastObservedLabel: stringValue(row['lastObservedAt']) || 'not observed',
    totalDurationLabel: durationLabel(numberValue(row['totalMs'])),
    eventCountLabel: `${eventCount} ${config.countLabel}`,
    inventoryCount,
    runningCount,
    foregroundCount,
    launcherCount,
    dailyRollupCount: numberValue(row['dailyRollupCount']),
    evidenceCount,
    evidenceClaimRowCount,
    identityRowCount,
    approvalAuthorityRowCount,
    approvalActionResultRowCount,
    platformAuthorityRowCount,
    aiClassifierResultRowCount,
    boundaryRowCount,
    unknownApproval,
    riskCandidate,
    manualRequired,
    launcherOnly,
    tone: rowTone({
      unknownApproval,
      riskCandidate,
      manualRequired,
      launcherOnly,
      foregroundCount,
      runningCount,
      inventoryState,
      runtimeState,
      foregroundState,
    }),
  };
}

function appGameDashboardMetrics(
  appRows: readonly ParentPortalAppGameDashboardRow[],
  gameRows: readonly ParentPortalAppGameDashboardRow[],
  rows: readonly ParentPortalAppGameDashboardRow[],
  sourceStatusRows: readonly ParentPortalAppGameSourceStatusRow[],
  platformCapabilityRows: readonly ParentPortalAppGamePlatformCapabilityRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  return [
    { label: 'Inventory', value: String(sumRows(rows, (row) => row.inventoryCount)), tone: 'cyan' },
    { label: 'Running', value: String(sumRows(rows, (row) => row.runningCount)), tone: 'gold' },
    { label: 'Foreground', value: String(sumRows(rows, (row) => row.foregroundCount)), tone: 'purple' },
    { label: 'Launcher', value: String(sumRows(rows, (row) => row.launcherCount)), tone: 'purple' },
    {
      label: 'Game budgets',
      value: gameRows.length > 0 ? 'policy proof pending' : 'no game rows',
      tone: gameRows.length > 0 ? 'gold' : 'purple',
    },
    { label: 'Source rows', value: String(sumRows(sourceStatusRows, (row) => row.rowCount)), tone: 'cyan' },
    {
      label: 'Fresh sources',
      value: String(sourceStatusRows.filter(sourceStatusRowFresh).length),
      tone: sourceStatusRows.some((row) => row.tone === 'red') ? 'red' : 'cyan',
    },
    { label: 'Boundary rows', value: String(sumRows(rows, (row) => row.boundaryRowCount)), tone: 'cyan' },
    { label: 'AI classifier', value: String(sumRows(rows, (row) => row.aiClassifierResultRowCount)), tone: 'gold' },
    { label: 'Readiness blockers', value: String(readinessBlockerCount(rows)), tone: 'gold' },
    {
      label: 'Platform gaps',
      value: String(platformCapabilityRows.length),
      tone: platformCapabilityTone(platformCapabilityRows),
    },
    { label: 'Adapter executed', value: String(adapterExecutedCount(platformCapabilityRows)), tone: 'gold' },
    { label: 'Unknown review', value: String(rows.filter((row) => row.unknownApproval).length), tone: 'gold' },
    { label: 'Manual required', value: String(rows.filter((row) => row.manualRequired).length), tone: 'gold' },
    { label: 'Evidence refs', value: String(sumRows(rows, (row) => row.evidenceCount)), tone: 'cyan' },
  ];
}

function dashboardSummary(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null,
  rows: readonly ParentPortalAppGameDashboardRow[]
): string {
  if (rows.length === 0) {
    return 'Waiting for app-use and games read-model rows from the local service.';
  }
  const states = [stringValue(appUseReadModel?.['state']), stringValue(gamesReadModel?.['state'])]
    .filter((state) => state.length > 0)
    .join(' / ');
  return states
    ? `${rows.length} service-backed app/game rows; read models ${states}.`
    : `${rows.length} service-backed app/game rows.`;
}

function dashboardState(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null,
  rows: readonly ParentPortalAppGameDashboardRow[],
  metricsReported: boolean
): string {
  if (!metricsReported) return 'unavailable';
  if (rows.some((row) => row.manualRequired)) return 'manual-required';
  if (rows.some((row) => row.riskCandidate || row.unknownApproval)) return 'review-required';
  return stringValue(appUseReadModel?.['state']) || stringValue(gamesReadModel?.['state']) || 'unavailable';
}

function combinedReadModelsReported(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null
): boolean {
  return readModelReported(appUseReadModel) && readModelReported(gamesReadModel);
}

function readModelReported(readModel: Record<string, unknown> | null): boolean {
  if (readModel === null) return false;
  const state = stringValue(readModel['state']);
  return state.length > 0 && state !== 'unavailable';
}

function capabilityRows(
  rows: readonly ParentPortalAppGameDashboardRow[],
  platformCapabilityRows: readonly ParentPortalAppGamePlatformCapabilityRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  const counts = new Map<string, number>();
  for (const row of rows) {
    const key = row.capabilityStatus || 'not-reported';
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  if (counts.size === 0) {
    return platformCapabilityRows.length > 0
      ? platformCapabilitySummaryRows(platformCapabilityRows)
      : [{ label: 'Capability', value: 'No service rows', tone: 'gold' }];
  }
  return Array.from(counts.entries())
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .slice(0, 4)
    .map(
      ([label, value]): ParentPortalAppGameDashboardMetric => ({
        label,
        value: `${value} rows`,
        tone: appGameManualRequired(label) ? 'gold' : label === 'ready' ? 'cyan' : 'purple',
      })
    )
    .concat(platformCapabilitySummaryRows(platformCapabilityRows))
    .slice(0, 6);
}

function evidenceRows(
  rows: readonly ParentPortalAppGameDashboardRow[],
  sourceStatusRows: readonly ParentPortalAppGameSourceStatusRow[],
  platformCapabilityRows: readonly ParentPortalAppGamePlatformCapabilityRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  const sourceRows = sourceStatusRows.slice(0, 3).map((row) => ({
    label: `${row.sourceLabel} ${row.sourceStatusLabel}`,
    value: `${row.rowCount} source rows; ${row.capabilityStatus}; ${row.lastObservedLabel}; ${row.evidenceCount} refs`,
    tone: row.tone,
  }));
  const blockerRows = rows.flatMap((row) => readinessBlockerRows(row));
  const platformRows = platformCapabilityRows.map((row) => ({
    label: `${row.platform} platform proof`,
    value: platformCapabilityEvidenceValue(row),
    tone: row.tone,
  }));
  const boundaryRows = rows
    .filter((row) => row.boundaryRowCount > 0)
    .map((row) => ({
      label: `${row.label} boundary`,
      value: boundaryRowValue(row),
      tone: row.aiClassifierResultRowCount > 0 || row.approvalActionResultRowCount === 0 ? 'gold' : row.tone,
    }));
  const serviceRows = rows
    .filter((row) => row.evidenceCount > 0 || row.lastObservedLabel !== 'not observed')
    .map((row) => ({
      label: row.label,
      value: `${row.evidenceCount} refs; ${row.lastObservedLabel}`,
      tone: row.tone,
    }));
  const visibleRows = [...blockerRows, ...platformRows, ...sourceRows, ...boundaryRows, ...serviceRows].slice(0, 18);
  return visibleRows.length > 0
    ? visibleRows
    : [{ label: 'Evidence refs', value: 'No evidence refs reported', tone: 'gold' }];
}

function appGamePlatformCapabilityRows(
  readModel: Record<string, unknown> | null
): readonly ParentPortalAppGamePlatformCapabilityRow[] {
  return readModelRows(readModel).map((row) => {
    const adapterExecutionClaim = stringValue(row['adapterExecutionClaim']) || 'not-executed';
    const proofPackState = stringValue(row['proofPackState']) || 'not-reported';
    const setupState = stringValue(row['setupState']) || 'not-reported';
    const authorityTier = stringValue(row['authorityTier']) || 'not-reported';
    const broadBlockingClaimed = booleanValue(row['broadBlockingClaimed']);
    const privilegedMobileClaimed = booleanValue(row['privilegedMobileClaimed']);
    const childDeviceDeliveryClaimed = booleanValue(row['childDeviceDeliveryClaimed']);

    return {
      platform: stringValue(row['platform']) || 'platform',
      state: stringValue(row['state']) || 'not-reported',
      adapterExecutionClaim,
      proofPackState,
      setupState,
      authorityTier,
      requiredProofCount: arrayCount(row['requiredProofRefs']),
      broadBlockingClaimed,
      privilegedMobileClaimed,
      childDeviceDeliveryClaimed,
      tone: platformCapabilityRowTone({
        adapterExecutionClaim,
        proofPackState,
        setupState,
        authorityTier,
        broadBlockingClaimed,
        privilegedMobileClaimed,
        childDeviceDeliveryClaimed,
      }),
    };
  });
}

function platformCapabilitySummaryRows(
  rows: readonly ParentPortalAppGamePlatformCapabilityRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  if (rows.length === 0) return [];
  return [
    { label: 'Platform gaps', value: `${rows.length} rows`, tone: platformCapabilityTone(rows) },
    { label: 'Adapter executed', value: `${adapterExecutedCount(rows)} rows`, tone: 'gold' },
  ];
}

function platformCapabilityEvidenceValue(row: ParentPortalAppGamePlatformCapabilityRow): string {
  return [
    row.setupState,
    row.proofPackState,
    `${row.requiredProofCount} proof refs`,
    `adapter ${row.adapterExecutionClaim}`,
    row.broadBlockingClaimed ? 'broad block claimed' : 'broad block not claimed',
    row.childDeviceDeliveryClaimed ? 'delivery claimed' : 'delivery not claimed',
  ].join('; ');
}

function platformCapabilityTone(
  rows: readonly ParentPortalAppGamePlatformCapabilityRow[]
): ParentPortalAppGameDashboardTone {
  return rows.some((row) => row.tone === 'red') ? 'red' : rows.length > 0 ? 'gold' : 'cyan';
}

function adapterExecutedCount(rows: readonly ParentPortalAppGamePlatformCapabilityRow[]): number {
  return rows.filter((row) => row.adapterExecutionClaim !== 'not-executed').length;
}

function platformCapabilityRowTone(input: {
  readonly adapterExecutionClaim: string;
  readonly proofPackState: string;
  readonly setupState: string;
  readonly authorityTier: string;
  readonly broadBlockingClaimed: boolean;
  readonly privilegedMobileClaimed: boolean;
  readonly childDeviceDeliveryClaimed: boolean;
}): ParentPortalAppGameDashboardTone {
  if (input.broadBlockingClaimed || input.privilegedMobileClaimed || input.childDeviceDeliveryClaimed) return 'red';
  if (input.adapterExecutionClaim !== 'not-executed') return 'red';
  if (appGameManualRequired(input.proofPackState, input.setupState, input.authorityTier)) return 'gold';
  return 'purple';
}

function readinessBlockerCount(rows: readonly ParentPortalAppGameDashboardRow[]): number {
  return sumRows(rows, (row) => readinessBlockerRows(row).length);
}

function readinessBlockerRows(row: ParentPortalAppGameDashboardRow): readonly ParentPortalAppGameDashboardMetric[] {
  return [
    approvalActionResultBlocker(row),
    aiClassifierReviewBlocker(row),
    manualRequiredBlocker(row),
    unknownApprovalBlocker(row),
  ].filter((metric): metric is ParentPortalAppGameDashboardMetric => metric !== null);
}

function approvalActionResultBlocker(row: ParentPortalAppGameDashboardRow): ParentPortalAppGameDashboardMetric | null {
  if (row.approvalAuthorityRowCount <= row.approvalActionResultRowCount) {
    return null;
  }
  return {
    label: `${row.label} approval blocker`,
    value: `Approval action result missing; ${row.approvalAuthorityRowCount}/${row.approvalActionResultRowCount}; policy manual-required`,
    tone: 'gold',
  };
}

function aiClassifierReviewBlocker(row: ParentPortalAppGameDashboardRow): ParentPortalAppGameDashboardMetric | null {
  if (row.aiClassifierResultRowCount <= 0) {
    return null;
  }
  return {
    label: `${row.label} AI review`,
    value: `${row.aiClassifierResultRowCount} classifier rows; evidence-only; no direct action`,
    tone: 'gold',
  };
}

function manualRequiredBlocker(row: ParentPortalAppGameDashboardRow): ParentPortalAppGameDashboardMetric | null {
  if (!row.manualRequired) {
    return null;
  }
  return {
    label: `${row.label} manual blocker`,
    value: `${row.capabilityStatus}; ${row.state}; adapter dispatch not claimed`,
    tone: 'gold',
  };
}

function unknownApprovalBlocker(row: ParentPortalAppGameDashboardRow): ParentPortalAppGameDashboardMetric | null {
  if (!row.unknownApproval) {
    return null;
  }
  return {
    label: `${row.label} approval review`,
    value: `${row.classificationState}; ${row.inventoryState}; parent approval required`,
    tone: row.riskCandidate ? 'red' : 'gold',
  };
}

function boundaryRowValue(row: ParentPortalAppGameDashboardRow): string {
  return [
    `Evidence ${row.evidenceClaimRowCount}`,
    `Identity ${row.identityRowCount}`,
    `Approval ${row.approvalAuthorityRowCount}/${row.approvalActionResultRowCount}`,
    `Platform ${row.platformAuthorityRowCount}`,
    `AI ${row.aiClassifierResultRowCount}`,
  ].join(' / ');
}

function rowTone(input: {
  readonly unknownApproval: boolean;
  readonly riskCandidate: boolean;
  readonly manualRequired: boolean;
  readonly launcherOnly: boolean;
  readonly foregroundCount: number;
  readonly runningCount: number;
  readonly inventoryState: string;
  readonly runtimeState: string;
  readonly foregroundState: string;
}): ParentPortalAppGameDashboardTone {
  if (input.manualRequired) return 'gold';
  if (input.riskCandidate || input.unknownApproval) return 'red';
  if (input.launcherOnly) return 'purple';
  if (input.foregroundCount > 0 || positiveState(input.foregroundState)) return 'purple';
  if (input.runningCount > 0 || positiveState(input.runtimeState)) return 'gold';
  if (positiveState(input.inventoryState)) return 'cyan';
  return 'cyan';
}

function dashboardRowSort(left: ParentPortalAppGameDashboardRow, right: ParentPortalAppGameDashboardRow): number {
  return (
    Number(right.manualRequired) - Number(left.manualRequired) ||
    Number(right.riskCandidate) - Number(left.riskCandidate) ||
    Number(right.unknownApproval) - Number(left.unknownApproval) ||
    Number(right.foregroundCount > 0) - Number(left.foregroundCount > 0) ||
    Number(right.runningCount > 0) - Number(left.runningCount > 0) ||
    right.evidenceCount - left.evidenceCount ||
    left.label.localeCompare(right.label)
  );
}

function appGameUnknownApproval(...values: readonly string[]): boolean {
  return values.some((value) => /unknown|new|possible|candidate/u.test(value.toLowerCase()));
}

function appGameRiskCandidate(...values: readonly string[]): boolean {
  return values.some((value) =>
    /risk|restricted|vpn|proxy|torrent|installer|remote-desktop/u.test(value.toLowerCase())
  );
}

function appGameManualRequired(...values: readonly string[]): boolean {
  return values.some((value) =>
    /manual|required|permission|unsupported|unavailable|not-claimed|admin|supervised|degraded|stale/u.test(
      value.toLowerCase()
    )
  );
}

function appGameLauncherOnly(...values: readonly string[]): boolean {
  return values.some((value) => /launcher/u.test(value.toLowerCase()));
}

function appGameSourceStatusRows(
  readModel: Record<string, unknown> | null,
  readModelKind: ParentPortalAppGameDashboardRow['sourceKind'],
  sourceLabel: string,
  labelField: string
): readonly ParentPortalAppGameSourceStatusRow[] {
  return readModelRows(readModel).flatMap((row, index) => {
    const sourceRows = row['sourceStatusRows'];
    if (!Array.isArray(sourceRows)) return [];
    const parentRowId = stringValue(row['rowId']) || `${readModelKind}-${index + 1}`;
    const parentLabel = stringValue(row[labelField]) || 'Unlabeled app/game row';
    return sourceRows
      .filter(isRecord)
      .map((sourceRow) => appGameSourceStatusRow(readModelKind, sourceLabel, parentRowId, parentLabel, sourceRow));
  });
}

function appGameSourceStatusRow(
  readModelKind: ParentPortalAppGameDashboardRow['sourceKind'],
  sourceLabel: string,
  parentRowId: string,
  parentLabel: string,
  row: Record<string, unknown>
): ParentPortalAppGameSourceStatusRow {
  const sourceStatusKind = stringValue(row['sourceKind']) || 'not-reported';
  const state = stringValue(row['state']) || 'not-reported';
  const rowCount = numberValue(row['rowCount']);
  const lastObservedLabel = stringValue(row['lastObservedAt']) || 'not observed';
  const capabilityStatus = stringValue(row['capabilityStatus']) || 'not-reported';
  const evidenceCount = arrayCount(row['evidence']);
  return {
    readModelKind,
    sourceLabel,
    parentRowId,
    parentLabel,
    sourceStatusKind,
    sourceStatusLabel: sourceKindLabel(sourceStatusKind),
    state,
    rowCount,
    lastObservedLabel,
    capabilityStatus,
    evidenceCount,
    tone: sourceStatusTone(sourceStatusKind, state, capabilityStatus, rowCount),
  };
}

function sourceStatusTone(
  sourceStatusKind: string,
  state: string,
  capabilityStatus: string,
  rowCount: number
): ParentPortalAppGameDashboardTone {
  if (appGameManualRequired(state, capabilityStatus)) return 'gold';
  if (rowCount <= 0 || /error|missing|failed/u.test(`${state} ${capabilityStatus}`.toLowerCase())) return 'red';
  if (/foreground|launcher/u.test(sourceStatusKind.toLowerCase())) return 'purple';
  if (/process|runtime|start|exit/u.test(sourceStatusKind.toLowerCase())) return 'gold';
  return 'cyan';
}

function sourceStatusRowFresh(row: ParentPortalAppGameSourceStatusRow): boolean {
  return (
    row.rowCount > 0 &&
    row.lastObservedLabel !== 'not observed' &&
    !appGameManualRequired(row.state, row.capabilityStatus)
  );
}

function sourceStatusRowSort(
  left: ParentPortalAppGameSourceStatusRow,
  right: ParentPortalAppGameSourceStatusRow
): number {
  return (
    Number(appGameManualRequired(right.state, right.capabilityStatus)) -
      Number(appGameManualRequired(left.state, left.capabilityStatus)) ||
    right.rowCount - left.rowCount ||
    right.evidenceCount - left.evidenceCount ||
    left.sourceStatusLabel.localeCompare(right.sourceStatusLabel) ||
    left.parentLabel.localeCompare(right.parentLabel)
  );
}

function sourceKindLabel(value: string): string {
  const label = value
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/[-_]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
  return label ? label.toLowerCase().replace(/\b[a-z]/g, (character) => character.toUpperCase()) : 'Not reported';
}

function positiveState(value: string): boolean {
  return /ready|detected|installed|running|foreground|known|observed/u.test(value.toLowerCase());
}

function inventoryCountValue(value: unknown, inventoryState: string): number {
  const explicitCount = numberValue(value);
  if (explicitCount > 0) return explicitCount;
  if (/detected|installed|catalog|known/u.test(inventoryState.toLowerCase())) return 1;
  return 0;
}

function durationLabel(durationMs: number): string {
  if (durationMs <= 0) return '0 min';
  const minutes = Math.round(durationMs / 60000);
  if (minutes < 1) return '< 1 min';
  if (minutes < 60) return `${minutes} min`;
  const hours = Math.floor(minutes / 60);
  const remainder = minutes % 60;
  return remainder > 0 ? `${hours}h ${remainder}m` : `${hours}h`;
}

function readModelRows(readModel: Record<string, unknown> | null): readonly Record<string, unknown>[] {
  const rows = readModel?.['rows'];
  if (!Array.isArray(rows)) return [];
  return rows.filter(isRecord);
}

function sumRows<Row>(rows: readonly Row[], selector: (row: Row) => number): number {
  return rows.reduce((sum, row) => sum + selector(row), 0);
}

function numberValue(value: unknown): number {
  return typeof value === 'number' && Number.isFinite(value) ? value : 0;
}

function stringValue(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function booleanValue(value: unknown): boolean {
  return value === true;
}

function arrayCount(value: unknown): number {
  return Array.isArray(value) ? value.length : 0;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
