import path from 'node:path';

const screenshotDirectory = path.join(
  'output',
  'tracking-plan-proof',
  '30-parent-and-child-ui-ux-surfaces',
  '11-ui-snapshots'
);

const screenshotFiles = {
  desktop: 'hosted-policy-tracking-live-summary.png',
  familyDashboard: 'hosted-policy-tracking-family-dashboard-rollup.png',
  reportPolicyConsumer: 'hosted-policy-tracking-report-policy-consumer.png',
  reportExport: 'hosted-policy-tracking-report-export.png',
  notificationParentSurface: 'hosted-policy-tracking-notification-parent-surface.png',
  parentActionReadiness: 'hosted-policy-tracking-parent-action-readiness.png',
  missingDevice: 'hosted-policy-tracking-missing-device.png',
  evidenceDrawer: 'hosted-policy-tracking-evidence-drawer.png',
  citationDetail: 'hosted-policy-tracking-citation-detail.png',
  retentionSettings: 'hosted-policy-tracking-retention-settings.png',
  childCheckIn: 'hosted-policy-tracking-child-check-in.png',
  childRuntimeUi: 'hosted-policy-tracking-child-runtime-ui.png',
  parentOverviewShell: 'hosted-parent-overview-shell.png',
  parentDevicesShell: 'hosted-parent-devices-shell.png',
  unsupportedManualPlatform: 'hosted-policy-tracking-unsupported-manual.png',
  mobile: 'hosted-policy-tracking-live-summary-mobile.png',
} as const;

export function hostedTrackingScreenshotPaths(repoRoot: string): Record<keyof typeof screenshotFiles, string> {
  return Object.fromEntries(
    Object.entries(screenshotFiles).map(([name, fileName]) => [
      name,
      path.relative(repoRoot, path.join(repoRoot, screenshotDirectory, fileName)).replace(/\\/gu, '/'),
    ])
  ) as Record<keyof typeof screenshotFiles, string>;
}
