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

export type ParentPortalAppGamePolicyReadinessRow = {
  readonly rowId: string;
  readonly readinessKind: string;
  readonly readinessLabel: string;
  readonly readinessState: string;
  readonly rowCount: number;
  readonly evidenceCount: number;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGameDashboardIntent = {
  readonly state: string;
  readonly summary: string;
  readonly appRows: readonly ParentPortalAppGameDashboardRow[];
  readonly gameRows: readonly ParentPortalAppGameDashboardRow[];
  readonly rows: readonly ParentPortalAppGameDashboardRow[];
  readonly sourceStatusRows: readonly ParentPortalAppGameSourceStatusRow[];
  readonly sourcePanelSections: readonly ParentPortalAppGameSourcePanelSection[];
  readonly policyReadinessRows: readonly ParentPortalAppGamePolicyReadinessRow[];
  readonly metrics: readonly ParentPortalAppGameDashboardMetric[];
  readonly capabilityRows: readonly ParentPortalAppGameDashboardMetric[];
  readonly evidenceRows: readonly ParentPortalAppGameDashboardMetric[];
  readonly emptyMessage: string;
};

export function createParentPortalAppGameDashboardIntent(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null,
  policyReadinessReadModel: Record<string, unknown> | null = null
): ParentPortalAppGameDashboardIntent {
  const appRows = appDashboardRows(appUseReadModel);
  const gameRows = gameDashboardRows(gamesReadModel);
  const rows = [...appRows, ...gameRows].sort(dashboardRowSort);
  const sourceStatusRows = [
    ...appGameSourceStatusRows(appUseReadModel, 'app-use', 'App use', 'appName'),
    ...appGameSourceStatusRows(gamesReadModel, 'games', 'Game', 'displayName'),
  ].sort(sourceStatusRowSort);
  const sourcePanelSections = createParentPortalAppGameSourcePanelSections(sourceStatusRows);
  const policyReadinessRows = appGamePolicyReadinessRows(policyReadinessReadModel);
  const metrics = appGameDashboardMetrics(appRows, gameRows, rows, sourceStatusRows, policyReadinessRows);
  const state = dashboardState(appUseReadModel, gamesReadModel, rows, policyReadinessReadModel, policyReadinessRows);

  return {
    state,
    summary: dashboardSummary(appUseReadModel, gamesReadModel, rows, policyReadinessRows),
    appRows,
    gameRows,
    rows,
    sourceStatusRows,
    sourcePanelSections,
    policyReadinessRows,
    metrics,
    capabilityRows: capabilityRows(rows, policyReadinessRows),
    evidenceRows: evidenceRows(rows, sourceStatusRows, policyReadinessRows),
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
  policyReadinessRows: readonly ParentPortalAppGamePolicyReadinessRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  const readyPolicyInputs = policyReadinessRows.filter((row) => row.readinessState === 'ready').length;
  return [
    { label: 'Inventory', value: String(sumRows(rows, (row) => row.inventoryCount)), tone: 'cyan' },
    { label: 'Running', value: String(sumRows(rows, (row) => row.runningCount)), tone: 'gold' },
    { label: 'Foreground', value: String(sumRows(rows, (row) => row.foregroundCount)), tone: 'purple' },
    { label: 'Launcher', value: String(sumRows(rows, (row) => row.launcherCount)), tone: 'purple' },
    { label: 'Source rows', value: String(sumRows(sourceStatusRows, (row) => row.rowCount)), tone: 'cyan' },
    {
      label: 'Fresh sources',
      value: String(sourceStatusRows.filter(sourceStatusRowFresh).length),
      tone: sourceStatusRows.some((row) => row.tone === 'red') ? 'red' : 'cyan',
    },
    { label: 'Unknown review', value: String(rows.filter((row) => row.unknownApproval).length), tone: 'gold' },
    { label: 'Manual required', value: String(rows.filter((row) => row.manualRequired).length), tone: 'gold' },
    {
      label: 'Policy inputs',
      value:
        policyReadinessRows.length > 0 ? `${readyPolicyInputs}/${policyReadinessRows.length} ready` : 'not reported',
      tone: policyReadinessRows.some((row) => row.tone === 'red')
        ? 'red'
        : policyReadinessRows.some((row) => row.tone === 'gold')
          ? 'gold'
          : 'cyan',
    },
    { label: 'Evidence refs', value: String(sumRows(rows, (row) => row.evidenceCount)), tone: 'cyan' },
    {
      label: 'Game budgets',
      value: gameRows.length > 0 ? 'policy proof pending' : 'no game rows',
      tone: gameRows.length > 0 ? 'gold' : 'purple',
    },
  ];
}

function dashboardSummary(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null,
  rows: readonly ParentPortalAppGameDashboardRow[],
  policyReadinessRows: readonly ParentPortalAppGamePolicyReadinessRow[]
): string {
  const policySummary =
    policyReadinessRows.length > 0
      ? ` Policy readiness ${policyReadinessRows.filter((row) => row.readinessState === 'ready').length}/${policyReadinessRows.length} inputs ready.`
      : '';
  if (rows.length === 0) {
    return `Waiting for app-use and games read-model rows from the local service.${policySummary}`;
  }
  const states = [stringValue(appUseReadModel?.['state']), stringValue(gamesReadModel?.['state'])]
    .filter((state) => state.length > 0)
    .join(' / ');
  return states
    ? `${rows.length} service-backed app/game rows; read models ${states}.${policySummary}`
    : `${rows.length} service-backed app/game rows.${policySummary}`;
}

function dashboardState(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null,
  rows: readonly ParentPortalAppGameDashboardRow[],
  policyReadinessReadModel: Record<string, unknown> | null,
  policyReadinessRows: readonly ParentPortalAppGamePolicyReadinessRow[]
): string {
  if (rows.some((row) => row.manualRequired)) return 'manual-required';
  if (!booleanValue(policyReadinessReadModel?.['policyEvaluationReady']) && policyReadinessRows.length > 0) {
    return 'manual-required';
  }
  if (policyReadinessRows.some((row) => row.tone !== 'cyan')) return 'manual-required';
  if (rows.some((row) => row.riskCandidate || row.unknownApproval)) return 'review-required';
  return stringValue(appUseReadModel?.['state']) || stringValue(gamesReadModel?.['state']) || 'unavailable';
}

function capabilityRows(
  rows: readonly ParentPortalAppGameDashboardRow[],
  policyReadinessRows: readonly ParentPortalAppGamePolicyReadinessRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  const counts = new Map<string, number>();
  for (const row of rows) {
    const key = row.capabilityStatus || 'not-reported';
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  for (const row of policyReadinessRows) {
    const key = `policy ${row.readinessState}`;
    counts.set(key, (counts.get(key) ?? 0) + 1);
  }
  if (counts.size === 0) {
    return [{ label: 'Capability', value: 'No service rows', tone: 'gold' }];
  }
  return Array.from(counts.entries())
    .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))
    .slice(0, 6)
    .map(([label, value]) => ({
      label,
      value: `${value} rows`,
      tone: appGameManualRequired(label)
        ? 'gold'
        : /missing/u.test(label)
          ? 'red'
          : /ready/u.test(label)
            ? 'cyan'
            : 'purple',
    }));
}

function evidenceRows(
  rows: readonly ParentPortalAppGameDashboardRow[],
  sourceStatusRows: readonly ParentPortalAppGameSourceStatusRow[],
  policyReadinessRows: readonly ParentPortalAppGamePolicyReadinessRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  const sourceRows = sourceStatusRows.slice(0, 3).map((row) => ({
    label: `${row.sourceLabel} ${row.sourceStatusLabel}`,
    value: `${row.rowCount} source rows; ${row.capabilityStatus}; ${row.lastObservedLabel}; ${row.evidenceCount} refs`,
    tone: row.tone,
  }));
  const policyRows = policyReadinessRows.slice(0, 3).map((row) => ({
    label: `Policy ${row.readinessLabel}`,
    value: `${row.rowCount} source rows; ${row.readinessState}; ${row.evidenceCount} refs`,
    tone: row.tone,
  }));
  const serviceRows = rows
    .filter((row) => row.evidenceCount > 0 || row.lastObservedLabel !== 'not observed')
    .map((row) => ({
      label: row.label,
      value: `${row.evidenceCount} refs; ${row.lastObservedLabel}`,
      tone: row.tone,
    }));
  const visibleRows = [...policyRows, ...sourceRows, ...serviceRows].slice(0, 6);
  return visibleRows.length > 0
    ? visibleRows
    : [{ label: 'Evidence drawer', value: 'No evidence refs reported', tone: 'gold' }];
}

function appGamePolicyReadinessRows(
  readModel: Record<string, unknown> | null
): readonly ParentPortalAppGamePolicyReadinessRow[] {
  return readModelRows(readModel)
    .map((row, index) => appGamePolicyReadinessRow(row, index))
    .sort(policyReadinessRowSort);
}

function appGamePolicyReadinessRow(row: Record<string, unknown>, index: number): ParentPortalAppGamePolicyReadinessRow {
  const readinessKind = stringValue(row['readinessKind']) || 'not-reported';
  const readinessState = stringValue(row['readinessState']) || 'not-reported';
  const rowCount = numberValue(row['rowCount']);
  const evidenceCount = arrayCount(row['evidence']) || arrayCount(row['evidenceReferenceIds']);
  return {
    rowId: stringValue(row['rowId']) || `policy-readiness-${index + 1}`,
    readinessKind,
    readinessLabel: sourceKindLabel(readinessKind),
    readinessState,
    rowCount,
    evidenceCount,
    tone: policyReadinessTone(readinessState, rowCount),
  };
}

function policyReadinessTone(readinessState: string, rowCount: number): ParentPortalAppGameDashboardTone {
  if (readinessState === 'ready' && rowCount > 0) return 'cyan';
  if (readinessState === 'missing') return 'red';
  return 'gold';
}

function policyReadinessRowSort(
  left: ParentPortalAppGamePolicyReadinessRow,
  right: ParentPortalAppGamePolicyReadinessRow
): number {
  return (
    policyReadinessRank(right) - policyReadinessRank(left) ||
    right.rowCount - left.rowCount ||
    left.readinessLabel.localeCompare(right.readinessLabel)
  );
}

function policyReadinessRank(row: ParentPortalAppGamePolicyReadinessRow): number {
  if (row.readinessState === 'missing') return 3;
  if (row.readinessState === 'manual-required') return 2;
  return row.rowCount > 0 ? 0 : 1;
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

function booleanValue(value: unknown): boolean {
  return value === true;
}

function stringValue(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function arrayCount(value: unknown): number {
  return Array.isArray(value) ? value.length : 0;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
