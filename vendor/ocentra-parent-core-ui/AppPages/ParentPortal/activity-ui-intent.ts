import type { DeviceSlot } from './DeviceChoiceGrid/DeviceChoiceGridTypes';

type ActivityAdapterResultLike = {
  readonly ok?: unknown;
  readonly state?: unknown;
  readonly value?: unknown;
  readonly reason?: unknown;
};

type ParentPortalServiceRowLike = {
  readonly label?: unknown;
  readonly primaryArea?: unknown;
  readonly readyCount?: unknown;
  readonly trend?: unknown;
};

export type ParentPortalActivityStateLike = {
  readonly activityReport?: ActivityAdapterResultLike | null;
  readonly activityReportHistory?: ActivityAdapterResultLike | null;
  readonly activityScreenReadModel?: ActivityAdapterResultLike | null;
  readonly activityAppUseReadModel?: ActivityAdapterResultLike | null;
  readonly activityBrowserReadModel?: ActivityAdapterResultLike | null;
  readonly activityGamesReadModel?: ActivityAdapterResultLike | null;
  readonly activityNetworkReadModel?: ActivityAdapterResultLike | null;
};

export type ParentPortalActivityReportFile = {
  readonly id: string;
  readonly fileName: string;
  readonly dateLabel: string;
  readonly rangeLabel: string;
  readonly summary: string;
  readonly saved: boolean;
  readonly report: ParentPortalActivityReportView;
};

export type ParentPortalActivityReportView = {
  readonly title: string;
  readonly summary: string;
  readonly targetLabel: string;
  readonly saved: boolean;
  readonly fileName: string;
  readonly sections: readonly ParentPortalActivityReportSection[];
};

export type ParentPortalActivityReportSection = {
  readonly title: string;
  readonly lines: readonly string[];
};

export type ParentPortalActivityUiIntent = {
  readonly hasServiceBackedDeviceRows: boolean;
  readonly deviceSlots: readonly DeviceSlot[];
  readonly reportFiles: readonly ParentPortalActivityReportFile[];
  readonly reportDocument: Record<string, unknown> | null;
  readonly reportHistory: Record<string, unknown> | null;
  readonly screenReadModel: Record<string, unknown> | null;
  readonly appUseReadModel: Record<string, unknown> | null;
  readonly browserReadModel: Record<string, unknown> | null;
  readonly gamesReadModel: Record<string, unknown> | null;
  readonly networkReadModel: Record<string, unknown> | null;
};

export function createParentPortalActivityUiIntent(
  activityState: ParentPortalActivityStateLike | null | undefined,
  planSeatLimit: number
): ParentPortalActivityUiIntent {
  const reportDocument = parentPortalActivityAdapterRecord(activityState?.activityReport);
  const reportHistory = parentPortalActivityAdapterRecord(activityState?.activityReportHistory);
  const screenReadModel = parentPortalActivityAdapterRecord(activityState?.activityScreenReadModel);
  const appUseReadModel = parentPortalActivityAdapterRecord(activityState?.activityAppUseReadModel);
  const browserReadModel = parentPortalActivityAdapterRecord(activityState?.activityBrowserReadModel);
  const gamesReadModel = parentPortalActivityAdapterRecord(activityState?.activityGamesReadModel);
  const networkReadModel = parentPortalActivityAdapterRecord(activityState?.activityNetworkReadModel);
  const serviceDeviceStates = collectActivityDeviceStates([
    reportDocument,
    reportHistory,
    screenReadModel,
    appUseReadModel,
    browserReadModel,
    gamesReadModel,
    networkReadModel,
  ]);
  const hasServiceBackedDeviceRows = serviceDeviceStates.size > 0;
  const seatCount = Math.max(0, Math.floor(planSeatLimit));

  return {
    hasServiceBackedDeviceRows,
    deviceSlots: activityDeviceSlots(serviceDeviceStates, seatCount),
    reportFiles: activityReportFiles(reportDocument, reportHistory),
    reportDocument,
    reportHistory,
    screenReadModel,
    appUseReadModel,
    browserReadModel,
    gamesReadModel,
    networkReadModel,
  };
}

export function parentPortalActivityAdapterRecord(
  result: ActivityAdapterResultLike | null | undefined
): Record<string, unknown> | null {
  if (!isRecord(result) || result.ok !== true || !isRecord(result.value)) {
    return null;
  }
  return result.value;
}

export function createParentPortalLanPairingUiSlots(
  rows: readonly ParentPortalServiceRowLike[]
): readonly DeviceSlot[] {
  const deviceRow = serviceRow(rows, 'Current device', 'Device pairing');
  const discoveryRow = serviceRow(rows, 'LAN', 'LAN discovery');
  if (!deviceRow && !discoveryRow) return [];
  const state = stringValue(deviceRow?.trend) || stringValue(discoveryRow?.trend) || 'unavailable';
  return [lanServiceSlot(state)];
}

function serviceRow(
  rows: readonly ParentPortalServiceRowLike[],
  primaryArea: string,
  label: string
): ParentPortalServiceRowLike | null {
  return (
    rows.find((row) => stringValue(row.primaryArea) === primaryArea) ??
    rows.find((row) => stringValue(row.label) === label) ??
    null
  );
}

function lanServiceSlot(state: string): DeviceSlot {
  const status = activityDeviceChoiceStatus(state);
  return {
    value: 'lan-pairing-service-state',
    label: 'LAN',
    status,
    slotIndex: 0,
    badge: state,
  };
}

function activityDeviceSlots(deviceStates: ReadonlyMap<string, string>, planSeatLimit: number): readonly DeviceSlot[] {
  const devices = Array.from(deviceStates.entries())
    .slice(0, planSeatLimit)
    .map(([deviceId, state], index) => activityDeviceSlot(deviceId, state, index));

  while (devices.length < planSeatLimit) {
    devices.push(emptyActivityDeviceSlot(devices.length));
  }

  return devices;
}

function collectActivityDeviceStates(readModels: readonly (Record<string, unknown> | null)[]): Map<string, string> {
  const deviceStates = new Map<string, string>();
  for (const readModel of readModels) {
    if (!readModel) continue;
    collectRequestScope(readModel, deviceStates);
    collectReportSourceStates(readModel, deviceStates);
    collectReportHistorySourceStates(readModel, deviceStates);
    collectReadModelRows(readModel, deviceStates);
  }
  return deviceStates;
}

function collectRequestScope(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const request = recordValue(readModel['request']);
  const scope = recordValue(request?.['scope']);
  const deviceId = stringValue(scope?.['deviceId']);
  if (deviceId) {
    mergeDeviceState(deviceStates, deviceId, stringValue(readModel['state']) || 'unavailable');
  }
}

function collectReportSourceStates(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const sourceStates = readModel['sourceStates'];
  if (!Array.isArray(sourceStates)) return;
  for (const sourceState of sourceStates) {
    if (!isRecord(sourceState)) continue;
    const deviceId = stringValue(sourceState['deviceId']);
    if (!deviceId) continue;
    const state = stringValue(sourceState['state']) || stringValue(sourceState['reachabilityState']) || 'unavailable';
    mergeDeviceState(deviceStates, deviceId, state);
  }
}

function collectReportHistorySourceStates(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const reports = readModel['reports'];
  if (!Array.isArray(reports)) return;
  for (const item of reports) {
    const report = recordValue(recordValue(item)?.['parsedReport']);
    if (report) {
      collectReportSourceStates(report, deviceStates);
    }
  }
}

function collectReadModelRows(readModel: Record<string, unknown>, deviceStates: Map<string, string>): void {
  const rows = readModel['rows'];
  if (!Array.isArray(rows)) return;
  for (const row of rows) {
    if (!isRecord(row)) continue;
    const deviceId = stringValue(row['deviceId']);
    if (!deviceId) continue;
    mergeDeviceState(
      deviceStates,
      deviceId,
      stringValue(row['state']) || stringValue(readModel['state']) || 'unavailable'
    );
  }
}

function mergeDeviceState(deviceStates: Map<string, string>, deviceId: string, state: string): void {
  const existing = deviceStates.get(deviceId);
  if (!existing || activityDeviceStateRank(state) > activityDeviceStateRank(existing)) {
    deviceStates.set(deviceId, state);
  }
}

function activityDeviceStateRank(state: string): number {
  switch (state) {
    case 'ready':
    case 'reachable':
      return 5;
    case 'empty':
      return 4;
    case 'stale':
      return 3;
    case 'offline':
    case 'unreachable':
      return 2;
    case 'permission-required':
    case 'scaffold-only':
    case 'unavailable':
    case 'error':
      return 1;
    default:
      return 0;
  }
}

function activityDeviceSlot(deviceId: string, state: string, slotIndex: number): DeviceSlot {
  const status = activityDeviceChoiceStatus(state);
  return {
    value: deviceId,
    label: activityDeviceShortLabel(deviceId, slotIndex),
    status,
    slotIndex,
    badge: state,
    device: {
      id: deviceId,
      name: deviceId,
      type: 'unknown',
      platform: 'unknown',
      status,
    },
  };
}

function activityDeviceChoiceStatus(state: string): 'connected' | 'available' | 'offline' | 'unsupported' {
  if (state === 'ready' || state === 'reachable' || state === 'online' || state === 'paired') return 'connected';
  if (state === 'empty' || state === 'stale') return 'available';
  if (state === 'offline' || state === 'unreachable') return 'offline';
  return 'unsupported';
}

function emptyActivityDeviceSlot(slotIndex: number): DeviceSlot {
  return {
    value: `activity-empty-seat-${slotIndex + 1}`,
    label: '',
    status: 'empty',
    slotIndex,
  };
}

function activityDeviceShortLabel(deviceId: string, slotIndex: number): string {
  const suffix = deviceId
    .replace(/[^A-Za-z0-9]/g, '')
    .slice(-3)
    .toUpperCase();
  return suffix || `D${String(slotIndex + 1).padStart(3, '0')}`;
}

function activityReportFiles(
  reportDocument: Record<string, unknown> | null,
  reportHistory: Record<string, unknown> | null
): readonly ParentPortalActivityReportFile[] {
  const currentReport = reportDocument ? [activityReportFileFromDocument(reportDocument, null)] : [];
  const reports = reportHistory?.['reports'];
  if (!Array.isArray(reports)) return currentReport;
  return currentReport.concat(
    reports.flatMap((item) => {
      if (!isRecord(item)) return [];
      const report = recordValue(item['parsedReport']);
      if (!report) return [];
      return [activityReportFileFromDocument(report, item)];
    })
  );
}

function activityReportFileFromDocument(
  report: Record<string, unknown>,
  savedItem: Record<string, unknown> | null
): ParentPortalActivityReportFile {
  const reportId = stringValue(report['reportId']) || stringValue(savedItem?.['reportId']) || 'activity-report';
  const fileName = stringValue(savedItem?.['fileName']) || `${reportId}.json`;
  const saved = savedItem !== null || isRecord(report['savedMetadata']);
  return {
    id: reportId,
    fileName,
    dateLabel: dateLabel(stringValue(savedItem?.['reportDate']) || stringValue(report['generatedAt'])),
    rangeLabel: rangeLabel(stringValue(report['rangeStart']), stringValue(report['rangeEnd'])),
    summary: reportSummary(report),
    saved,
    report: activityReportView(report, fileName, saved),
  };
}

function activityReportView(
  report: Record<string, unknown>,
  fileName: string,
  saved: boolean
): ParentPortalActivityReportView {
  return {
    title: stringValue(report['reportId']) || fileName,
    summary: reportSummary(report),
    targetLabel: reportTargetLabel(report),
    saved,
    fileName,
    sections: reportSections(report),
  };
}

function reportSummary(report: Record<string, unknown>): string {
  const sections = report['sections'];
  if (!Array.isArray(sections)) return 'Activity report returned by the local service';
  for (const section of sections) {
    const summary = stringValue(recordValue(section)?.['summary']);
    if (summary) return summary;
  }
  return 'Activity report returned by the local service';
}

function reportTargetLabel(report: Record<string, unknown>): string {
  const scope = recordValue(report['scope']);
  const scopeKind = stringValue(scope?.['scopeKind']);
  const deviceId = stringValue(scope?.['deviceId']);
  if (scopeKind === 'device' && deviceId) return `Device ${deviceId}`;
  return 'Family';
}

function reportSections(report: Record<string, unknown>): readonly ParentPortalActivityReportSection[] {
  const sections = report['sections'];
  if (!Array.isArray(sections)) return [];
  return sections.flatMap((section) => {
    if (!isRecord(section)) return [];
    const title = stringValue(section['title']) || stringValue(section['sectionKind']) || 'Report section';
    const state = stringValue(section['state']) || 'not-reported';
    const itemCount = numberValue(section['itemCount']);
    const summary = stringValue(section['summary']) || `${title} is ${state}`;
    return [
      {
        title,
        lines: [summary, itemCount === null ? `State ${state}` : `State ${state}; ${itemCount} items`],
      },
    ];
  });
}

function dateLabel(value: string): string {
  return value ? value.slice(0, 10) : 'No date';
}

function rangeLabel(start: string, end: string): string {
  const startLabel = dateLabel(start);
  const endLabel = dateLabel(end);
  if (startLabel === 'No date' && endLabel === 'No date') return 'No range';
  if (startLabel === endLabel) return startLabel;
  return `${startLabel} to ${endLabel}`;
}

function numberValue(value: unknown): number | null {
  return typeof value === 'number' && Number.isFinite(value) ? value : null;
}

function stringValue(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function recordValue(value: unknown): Record<string, unknown> | null {
  return isRecord(value) ? value : null;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
