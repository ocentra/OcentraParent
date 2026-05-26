export type ActivityReportFrequency = 'daily' | 'weekly' | 'monthly';
export type ActivityReportScope = 'family' | 'device';
export type ActivityReportFileType = 'json';
export type ActivitySurfaceTab = 'screen' | 'apps' | 'browser' | 'games' | 'network';

export type ActivityReportDeviceInput = {
  readonly id: string;
  readonly label: string;
  readonly status?: string;
};

export type ActivityReportStorageTarget = {
  readonly id: string;
  readonly label: string;
};

export type ActivityReportRequest = {
  readonly frequency: ActivityReportFrequency;
  readonly scope: ActivityReportScope;
  readonly devices: readonly ActivityReportDeviceInput[];
  readonly selectedDeviceId?: string;
  readonly generatedAt?: string;
  readonly overrideMode?: string;
  readonly storageTarget?: ActivityReportStorageTarget;
};

export type ActivityReportSection = {
  readonly title: string;
  readonly lines: readonly string[];
};

export type ActivityReportDocument = {
  readonly id: string;
  readonly fileName: string;
  readonly fileType: ActivityReportFileType;
  readonly frequency: ActivityReportFrequency;
  readonly scope: ActivityReportScope;
  readonly title: string;
  readonly targetLabel: string;
  readonly generatedAt: string;
  readonly dateRangeLabel: string;
  readonly summary: string;
  readonly sourceDeviceCount: number;
  readonly saved: boolean;
  readonly storageTargetLabel: string;
  readonly sections: readonly ActivityReportSection[];
  readonly todo: string;
};

export type ActivityReportListItem = {
  readonly id: string;
  readonly fileName: string;
  readonly fileType: ActivityReportFileType;
  readonly dateLabel: string;
  readonly rangeLabel: string;
  readonly summary: string;
  readonly saved: boolean;
  readonly report: ActivityReportDocument;
};

export type ActivityTabView = {
  readonly tab: ActivitySurfaceTab;
  readonly title: string;
  readonly targetLabel: string;
  readonly summary: string;
  readonly rows: readonly ActivityReportSection[];
  readonly todo: string;
};

export const ACTIVITY_SURFACE_UI_CHECK_TAURI_TODO =
  'TODO(activity-surface-tauri): replace this UI-check adapter with Tauri/Rust parent portal commands that fan out to child agents, aggregate responses, and persist activity reports/views through the selected local data store.';

const DEFAULT_STORAGE_TARGET: ActivityReportStorageTarget = {
  id: 'local-parent-portal-json-store',
  label: 'Local parent portal JSON store',
};

const FREQUENCY_LABELS: Record<ActivityReportFrequency, string> = {
  daily: 'Daily',
  weekly: 'Weekly',
  monthly: 'Monthly',
};

const ACTIVITY_TAB_TITLES: Record<ActivitySurfaceTab, string> = {
  screen: 'Screen activity',
  apps: 'App use',
  browser: 'Browser activity',
  games: 'Game activity',
  network: 'Network activity',
};

function stableDate(input?: string): string {
  return input && input.length > 0 ? input : '2026-05-26T12:00:00Z';
}

function targetDevicesFor(request: ActivityReportRequest): readonly ActivityReportDeviceInput[] {
  if (request.scope === 'family') return request.devices;
  const selected = request.devices.find((device) => device.id === request.selectedDeviceId);
  return selected ? [selected] : [];
}

function targetLabelFor(request: ActivityReportRequest): string {
  if (request.scope === 'family') return 'Family';
  return targetDevicesFor(request)[0]?.label ?? 'No device selected';
}

function rangeLabelFor(frequency: ActivityReportFrequency): string {
  if (frequency === 'weekly') return 'May 20-26, 2026';
  if (frequency === 'monthly') return 'May 2026';
  return 'May 26, 2026';
}

function fileStemFor(frequency: ActivityReportFrequency, request: ActivityReportRequest): string {
  const scope = request.scope === 'family' ? 'family' : (targetDevicesFor(request)[0]?.id ?? 'device-unselected');
  return `activity-report-${frequency}-${scope}-2026-05-26`;
}

function reportSummaryFor(frequency: ActivityReportFrequency, sourceDeviceCount: number): string {
  const label = FREQUENCY_LABELS[frequency].toLowerCase();
  if (sourceDeviceCount <= 0) return `No device data is available for this ${label} report.`;
  if (sourceDeviceCount === 1) return `One device contributed to this ${label} activity summary.`;
  return `${sourceDeviceCount} devices contributed to this ${label} family activity summary.`;
}

function buildReport(
  frequency: ActivityReportFrequency,
  request: ActivityReportRequest,
  saved: boolean
): ActivityReportDocument {
  const generatedAt = stableDate(request.generatedAt);
  const devices = targetDevicesFor(request);
  const sourceDeviceCount = devices.length;
  const targetLabel = targetLabelFor(request);
  const storageTarget = request.storageTarget ?? DEFAULT_STORAGE_TARGET;
  const summary = reportSummaryFor(frequency, sourceDeviceCount);
  const fileStem = fileStemFor(frequency, request);

  return {
    id: fileStem,
    fileName: `${fileStem}.json`,
    fileType: 'json',
    frequency,
    scope: request.scope,
    title: `${FREQUENCY_LABELS[frequency]} activity report`,
    targetLabel,
    generatedAt,
    dateRangeLabel: rangeLabelFor(frequency),
    summary,
    sourceDeviceCount,
    saved,
    storageTargetLabel: storageTarget.label,
    todo: ACTIVITY_SURFACE_UI_CHECK_TAURI_TODO,
    sections: [
      {
        title: 'Summary',
        lines: [
          summary,
          request.scope === 'family'
            ? 'Family reports should request each reachable child device, merge available responses, and mark unavailable devices.'
            : 'Per-device reports should request only the selected child device.',
        ],
      },
      {
        title: 'Activity signals',
        lines: [
          'Screen, app, browser, game, and network signals are represented as report sections, not raw JSON output.',
          'Unsupported or offline devices should be listed as unavailable sources in the generated report.',
        ],
      },
      {
        title: 'Storage',
        lines: [
          saved
            ? `Saved to ${storageTarget.label}.`
            : `Draft is not saved. Save should write ${fileStem}.json to ${storageTarget.label}.`,
          'Data page wiring will choose the final local or drive-backed storage target.',
        ],
      },
    ],
  };
}

export async function getDailyReport(request: ActivityReportRequest): Promise<ActivityReportDocument> {
  return buildReport('daily', request, false);
}

export async function getWeeklyReport(request: ActivityReportRequest): Promise<ActivityReportDocument> {
  return buildReport('weekly', request, false);
}

export async function getMonthlyReport(request: ActivityReportRequest): Promise<ActivityReportDocument> {
  return buildReport('monthly', request, false);
}

export async function saveActivityReport(report: ActivityReportDocument): Promise<ActivityReportDocument> {
  return { ...report, saved: true };
}

export function listHistoricalActivityReportJsonFiles(
  request: ActivityReportRequest
): readonly ActivityReportListItem[] {
  const orderedFrequencies: readonly ActivityReportFrequency[] = [
    request.frequency,
    ...(['daily', 'weekly', 'monthly'] as const).filter((frequency) => frequency !== request.frequency),
  ];

  return orderedFrequencies.map((frequency) => {
    const report = buildReport(frequency, { ...request, frequency }, true);
    return {
      id: report.id,
      fileName: report.fileName,
      fileType: report.fileType,
      dateLabel: report.generatedAt.slice(0, 10),
      rangeLabel: report.dateRangeLabel,
      summary: report.summary,
      saved: report.saved,
      report,
    };
  });
}

export function getScreenActivity(request: ActivityReportRequest): ActivityTabView {
  return buildActivityTabView('screen', request);
}

export function getAppUseActivity(request: ActivityReportRequest): ActivityTabView {
  return buildActivityTabView('apps', request);
}

export function getBrowserActivity(request: ActivityReportRequest): ActivityTabView {
  return buildActivityTabView('browser', request);
}

export function getGamesActivity(request: ActivityReportRequest): ActivityTabView {
  return buildActivityTabView('games', request);
}

export function getNetworkActivity(request: ActivityReportRequest): ActivityTabView {
  return buildActivityTabView('network', request);
}

export const activityUiIntentAdapter = {
  getDailyReport,
  getWeeklyReport,
  getMonthlyReport,
  saveActivityReport,
  listHistoricalReports: listHistoricalActivityReportJsonFiles,
  getScreenActivity,
  getAppUseActivity,
  getBrowserActivity,
  getGamesActivity,
  getNetworkActivity,
} as const;

function buildActivityTabView(tab: ActivitySurfaceTab, request: ActivityReportRequest): ActivityTabView {
  const targetLabel = targetLabelFor(request);
  const deviceCount = targetDevicesFor(request).length;
  const title = ACTIVITY_TAB_TITLES[tab];
  return {
    tab,
    title,
    targetLabel,
    summary:
      deviceCount > 0
        ? `${title} is scoped to ${targetLabel} with ${deviceCount} available source device${deviceCount === 1 ? '' : 's'}.`
        : `${title} has no selected source device yet.`,
    todo: ACTIVITY_SURFACE_UI_CHECK_TAURI_TODO,
    rows: [
      {
        title: 'Intent',
        lines: [
          'The UI expects a Tauri/Rust command to return a user-facing activity view, not a raw transport dump.',
          request.scope === 'family'
            ? 'Family mode aggregates available child devices.'
            : 'Per-device mode reads only the selected child device.',
        ],
      },
      {
        title: 'State',
        lines: [
          'This is UI-check data until the activity read model is connected.',
          'The next wiring pass should replace this adapter without changing the tab layout.',
        ],
      },
    ],
  };
}
