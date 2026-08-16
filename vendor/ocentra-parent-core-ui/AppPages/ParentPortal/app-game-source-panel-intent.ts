import type { ParentPortalAppGameDashboardTone, ParentPortalAppGameSourceStatusRow } from './app-game-dashboard-intent';

export type ParentPortalAppGameSourcePanelMetric = {
  readonly label: string;
  readonly value: string;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGameSourcePanelRow = {
  readonly key: string;
  readonly parentLabel: string;
  readonly sourceLabel: string;
  readonly sourceStatusLabel: string;
  readonly state: string;
  readonly capabilityStatus: string;
  readonly freshnessLabel: string;
  readonly rowCountLabel: string;
  readonly evidenceLabel: string;
  readonly lastObservedLabel: string;
  readonly tone: ParentPortalAppGameDashboardTone;
};

export type ParentPortalAppGameSourcePanelSection = {
  readonly sectionId: string;
  readonly title: string;
  readonly subtitle: string;
  readonly state: string;
  readonly tone: ParentPortalAppGameDashboardTone;
  readonly rowCount: number;
  readonly freshCount: number;
  readonly manualRequiredCount: number;
  readonly evidenceCount: number;
  readonly metrics: readonly ParentPortalAppGameSourcePanelMetric[];
  readonly rows: readonly ParentPortalAppGameSourcePanelRow[];
};

export function createParentPortalAppGameSourcePanelSections(
  sourceStatusRows: readonly ParentPortalAppGameSourceStatusRow[]
): readonly ParentPortalAppGameSourcePanelSection[] {
  const groupedRows = new Map<string, ParentPortalAppGameSourceStatusRow[]>();
  for (const row of sourceStatusRows) {
    const key = `${row.readModelKind}:${row.sourceLabel}`;
    groupedRows.set(key, [...(groupedRows.get(key) ?? []), row]);
  }

  return Array.from(groupedRows.entries())
    .map(([sectionId, rows]) => sourcePanelSection(sectionId, rows))
    .sort(sourcePanelSectionSort);
}

function sourcePanelSection(
  sectionId: string,
  rows: readonly ParentPortalAppGameSourceStatusRow[]
): ParentPortalAppGameSourcePanelSection {
  const title = `${rows[0]?.sourceLabel ?? 'App/game'} sources`;
  const rowCount = sumRows(rows, (row) => row.rowCount);
  const freshCount = rows.filter(sourceStatusRowFresh).length;
  const manualRequiredCount = rows.filter((row) => manualRequiredValue(row.state, row.capabilityStatus)).length;
  const evidenceCount = sumRows(rows, (row) => row.evidenceCount);
  const tone = sourcePanelTone(rows, rowCount, freshCount, manualRequiredCount);

  return {
    sectionId,
    title,
    subtitle: sourcePanelSubtitle(rowCount, freshCount, manualRequiredCount),
    state: manualRequiredCount > 0 ? 'manual-required' : freshCount > 0 ? 'fresh' : 'stale',
    tone,
    rowCount,
    freshCount,
    manualRequiredCount,
    evidenceCount,
    metrics: [
      { label: 'Fresh', value: String(freshCount), tone: freshCount > 0 ? 'cyan' : 'red' },
      { label: 'Rows', value: String(rowCount), tone: rowCount > 0 ? 'cyan' : 'red' },
      { label: 'Manual', value: String(manualRequiredCount), tone: manualRequiredCount > 0 ? 'gold' : 'cyan' },
      { label: 'Evidence', value: String(evidenceCount), tone: evidenceCount > 0 ? 'cyan' : 'gold' },
    ],
    rows: rows.map(sourcePanelRow).sort(sourcePanelRowSort),
  };
}

function sourcePanelRow(row: ParentPortalAppGameSourceStatusRow): ParentPortalAppGameSourcePanelRow {
  return {
    key: `${row.readModelKind}:${row.parentRowId}:${row.sourceStatusKind}`,
    parentLabel: row.parentLabel,
    sourceLabel: row.sourceLabel,
    sourceStatusLabel: row.sourceStatusLabel,
    state: row.state,
    capabilityStatus: row.capabilityStatus,
    freshnessLabel: sourceStatusRowFresh(row) ? 'Fresh source' : 'Needs review',
    rowCountLabel: `${row.rowCount} source rows`,
    evidenceLabel: `${row.evidenceCount} evidence refs`,
    lastObservedLabel: row.lastObservedLabel,
    tone: row.tone,
  };
}

function sourcePanelSubtitle(rowCount: number, freshCount: number, manualRequiredCount: number): string {
  if (rowCount <= 0) {
    return 'No source rows reported by the local service.';
  }
  const manualText = manualRequiredCount > 0 ? `; ${manualRequiredCount} manual-required` : '';
  return `${freshCount} fresh of ${rowCount} source rows${manualText}`;
}

function sourcePanelTone(
  rows: readonly ParentPortalAppGameSourceStatusRow[],
  rowCount: number,
  freshCount: number,
  manualRequiredCount: number
): ParentPortalAppGameDashboardTone {
  if (manualRequiredCount > 0) return 'gold';
  if (rowCount <= 0 || freshCount === 0 || rows.some((row) => row.tone === 'red')) return 'red';
  if (rows.some((row) => row.tone === 'purple')) return 'purple';
  return 'cyan';
}

function sourcePanelSectionSort(
  left: ParentPortalAppGameSourcePanelSection,
  right: ParentPortalAppGameSourcePanelSection
): number {
  return (
    right.manualRequiredCount - left.manualRequiredCount ||
    right.rowCount - left.rowCount ||
    left.title.localeCompare(right.title)
  );
}

function sourcePanelRowSort(left: ParentPortalAppGameSourcePanelRow, right: ParentPortalAppGameSourcePanelRow): number {
  return (
    Number(manualRequiredValue(right.state, right.capabilityStatus)) -
      Number(manualRequiredValue(left.state, left.capabilityStatus)) ||
    left.sourceStatusLabel.localeCompare(right.sourceStatusLabel) ||
    left.parentLabel.localeCompare(right.parentLabel)
  );
}

function sourceStatusRowFresh(row: ParentPortalAppGameSourceStatusRow): boolean {
  return (
    row.rowCount > 0 &&
    row.lastObservedLabel !== 'not observed' &&
    !manualRequiredValue(row.state, row.capabilityStatus)
  );
}

function manualRequiredValue(...values: readonly string[]): boolean {
  return values.some((value) =>
    /manual|required|permission|unsupported|unavailable|not-claimed|admin|supervised|degraded|stale/u.test(
      value.toLowerCase()
    )
  );
}

function sumRows<Row>(rows: readonly Row[], selector: (row: Row) => number): number {
  return rows.reduce((sum, row) => sum + selector(row), 0);
}
