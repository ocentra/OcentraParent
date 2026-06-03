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

export type ParentPortalAppGameDashboardIntent = {
  readonly state: string;
  readonly summary: string;
  readonly appRows: readonly ParentPortalAppGameDashboardRow[];
  readonly gameRows: readonly ParentPortalAppGameDashboardRow[];
  readonly rows: readonly ParentPortalAppGameDashboardRow[];
  readonly metrics: readonly ParentPortalAppGameDashboardMetric[];
  readonly capabilityRows: readonly ParentPortalAppGameDashboardMetric[];
  readonly evidenceRows: readonly ParentPortalAppGameDashboardMetric[];
  readonly emptyMessage: string;
};

export function createParentPortalAppGameDashboardIntent(
  appUseReadModel: Record<string, unknown> | null,
  gamesReadModel: Record<string, unknown> | null
): ParentPortalAppGameDashboardIntent {
  const appRows = appDashboardRows(appUseReadModel);
  const gameRows = gameDashboardRows(gamesReadModel);
  const rows = [...appRows, ...gameRows].sort(dashboardRowSort);
  const metrics = appGameDashboardMetrics(appRows, gameRows, rows);
  const state = dashboardState(appUseReadModel, gamesReadModel, rows);

  return {
    state,
    summary: dashboardSummary(appUseReadModel, gamesReadModel, rows),
    appRows,
    gameRows,
    rows,
    metrics,
    capabilityRows: capabilityRows(rows),
    evidenceRows: evidenceRows(rows),
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
  rows: readonly ParentPortalAppGameDashboardRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  return [
    { label: 'App rows', value: String(appRows.length), tone: appRows.length > 0 ? 'cyan' : 'gold' },
    { label: 'Game rows', value: String(gameRows.length), tone: gameRows.length > 0 ? 'purple' : 'gold' },
    { label: 'Inventory', value: String(sumRows(rows, (row) => row.inventoryCount)), tone: 'cyan' },
    { label: 'Running', value: String(sumRows(rows, (row) => row.runningCount)), tone: 'gold' },
    { label: 'Foreground', value: String(sumRows(rows, (row) => row.foregroundCount)), tone: 'purple' },
    { label: 'Launcher', value: String(sumRows(rows, (row) => row.launcherCount)), tone: 'purple' },
    { label: 'Unknown review', value: String(rows.filter((row) => row.unknownApproval).length), tone: 'gold' },
    { label: 'Manual required', value: String(rows.filter((row) => row.manualRequired).length), tone: 'gold' },
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
  rows: readonly ParentPortalAppGameDashboardRow[]
): string {
  if (rows.some((row) => row.manualRequired)) return 'manual-required';
  if (rows.some((row) => row.riskCandidate || row.unknownApproval)) return 'review-required';
  return stringValue(appUseReadModel?.['state']) || stringValue(gamesReadModel?.['state']) || 'unavailable';
}

function capabilityRows(
  rows: readonly ParentPortalAppGameDashboardRow[]
): readonly ParentPortalAppGameDashboardMetric[] {
  const counts = new Map<string, number>();
  for (const row of rows) {
    const key = row.capabilityStatus || 'not-reported';
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
      tone: appGameManualRequired(label) ? 'gold' : label === 'ready' ? 'cyan' : 'purple',
    }));
}

function evidenceRows(rows: readonly ParentPortalAppGameDashboardRow[]): readonly ParentPortalAppGameDashboardMetric[] {
  const visibleRows = rows
    .filter((row) => row.evidenceCount > 0 || row.lastObservedLabel !== 'not observed')
    .slice(0, 6)
    .map((row) => ({
      label: row.label,
      value: `${row.evidenceCount} refs; ${row.lastObservedLabel}`,
      tone: row.tone,
    }));
  return visibleRows.length > 0
    ? visibleRows
    : [{ label: 'Evidence drawer', value: 'No evidence refs reported', tone: 'gold' }];
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

function sumRows(
  rows: readonly ParentPortalAppGameDashboardRow[],
  selector: (row: ParentPortalAppGameDashboardRow) => number
): number {
  return rows.reduce((sum, row) => sum + selector(row), 0);
}

function numberValue(value: unknown): number {
  return typeof value === 'number' && Number.isFinite(value) ? value : 0;
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
