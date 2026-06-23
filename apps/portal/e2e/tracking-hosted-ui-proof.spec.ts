import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Locator, type Page } from '@playwright/test';
import { AgentTrackingRetentionSettingsWriteDefaults } from '@ocentra-parent/schema-domain/agent-tracking-retention-settings-write-command';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const workpack31Root = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const screenshotDir = path.join(proofRoot, '11-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-live-summary.png');
const mobileScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-live-summary-mobile.png');
const familyDashboardScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-family-dashboard-rollup.png');
const reportPolicyConsumerScreenshotPath = path.join(
  screenshotDir,
  'hosted-policy-tracking-report-policy-consumer.png'
);
const reportExportScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-report-export.png');
const notificationParentSurfaceScreenshotPath = path.join(
  screenshotDir,
  'hosted-policy-tracking-notification-parent-surface.png'
);
const parentActionReadinessScreenshotPath = path.join(
  screenshotDir,
  'hosted-policy-tracking-parent-action-readiness.png'
);
const missingDeviceScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-missing-device.png');
const evidenceDrawerScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-evidence-drawer.png');
const citationDetailScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-citation-detail.png');
const retentionSettingsScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-retention-settings.png');
const childCheckInScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-child-check-in.png');
const childRuntimeUiScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-child-runtime-ui.png');
const parentOverviewShellScreenshotPath = path.join(screenshotDir, 'hosted-parent-overview-shell.png');
const parentDevicesShellScreenshotPath = path.join(screenshotDir, 'hosted-parent-devices-shell.png');
const unsupportedManualScreenshotPath = path.join(workpack31Root, '19-unsupported-manual-hosted-ui.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'tracking-plan-hosted-ui-proof',
  'accessibility-summary.json'
);

type HostedTrackingLayoutBox = {
  readonly proofId: string;
  readonly left: number;
  readonly top: number;
  readonly right: number;
  readonly bottom: number;
  readonly width: number;
  readonly height: number;
};

type HostedTrackingProofCards = {
  readonly familyDashboard: Locator;
  readonly reportPolicyConsumer: Locator;
  readonly reportExport: Locator;
  readonly notificationParentSurface: Locator;
  readonly parentActionReadiness: Locator;
  readonly missingDevice: Locator;
  readonly evidenceDrawer: Locator;
  readonly citationDetail: Locator;
  readonly retentionSettings: Locator;
  readonly childCheckIn: Locator;
  readonly childRuntimeUi: Locator;
  readonly unsupportedManual: Locator;
};

test('proof panels route renders hosted policy tracking proof without product claims', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await assertHostedPolicyTrackingRoute(page);
  await captureHostedTrackingScreenshots(page);
  const accessibilitySummary = await collectAccessibilitySummary(page);
  const parentPortalShellSummary = await captureParentPortalShellScreenshots(page);
  await writeAccessibilitySummary(accessibilitySummary, parentPortalShellSummary);

  expect(browserFailures).toEqual([]);
});

async function assertHostedPolicyTrackingRoute(page: Page): Promise<void> {
  await page.goto('/#/proof-panels');
  await page.getByRole('button', { name: 'Tracking status' }).click();
  const trackingProofRegion = page.getByRole('region', { name: 'Tracking status proof' });
  await expect(trackingProofRegion).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(page.getByRole('heading', { name: 'Tracking status proof' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  await refreshHostedTrackingStatus(page, trackingProofRegion);

  await expect(page.getByRole('heading', { name: 'Service read model' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Service data coverage' })).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-hosted-expected-place-event').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(trackingProofRegion.getByText('2026-06-04T10:10:00.000Z').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('recent', { exact: true }).first()).toBeVisible();
  await expect(trackingProofRegion.getByText('child-device-query-store', { exact: true }).first()).toBeVisible();
  await expect(
    trackingProofRegion.getByText('location-evidence-hosted-1 | location-evidence-hosted-2').first()
  ).toBeVisible();
  await expect(trackingProofRegion.getByText('Manual proof required').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('Physical device proof required').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('No product claim').first()).toBeVisible();
  await assertHostedFamilyDashboardRollupProof(trackingProofRegion);
  await assertHostedReportPolicyConsumerProof(trackingProofRegion);
  await assertHostedReportExportProof(trackingProofRegion);
  await assertHostedNotificationParentSurfaceProof(trackingProofRegion);
  await assertHostedParentActionReadinessProof(page, trackingProofRegion);
  await assertHostedMissingDeviceProof(page, trackingProofRegion);
  await assertHostedEvidenceDrawerProof(page, trackingProofRegion);
  await assertHostedCitationDetailProof(page, trackingProofRegion);
  await assertHostedRetentionSettingsProof(page, trackingProofRegion);
  await assertHostedChildCheckInProof(page, trackingProofRegion);
  await assertHostedChildRuntimeUiProof(page, trackingProofRegion);
  await assertHostedUnsupportedManualPlatformProof(trackingProofRegion);

  const routeText = await trackingProofRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:product ready|physical device proved|background geofence proved)/iu);
  expect(routeText ?? '').not.toMatch(/(?:trouble|lying|bad place|delivered to child device)/iu);
}

async function assertHostedChildCheckInProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="child-check-in"]');
  await expect(page.getByRole('heading', { name: 'Child check-in request' })).toBeVisible();
  await expect(trackingProofRegion.getByText('Your parent is asking you to check in. Are you safe?')).toBeVisible();
  await expect(trackingProofRegion.getByText("I'm safe")).toBeVisible();
  await expect(trackingProofRegion.getByText('Need help')).toBeVisible();
  await expect(trackingProofRegion.getByText('Share current location')).toBeVisible();
  await expect(trackingProofRegion.getByText('Call parent', { exact: true })).toBeVisible();
  await expect(trackingProofRegion.getByText('Child-device delivery not proved').first()).toBeVisible();
}

async function assertHostedReportExportProof(trackingProofRegion: Locator): Promise<void> {
  const reportExportCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="report-export-ui"]').first();
  await expect(reportExportCard).toBeVisible();
  await expect(reportExportCard.getByRole('heading', { name: 'Report export read-model UI' })).toBeVisible();
  await expect(reportExportCard.getByText('Redacted report packet')).toBeVisible();
  await expect(reportExportCard.getByText('Retention audit export packet')).toBeVisible();
  await expect(reportExportCard.getByText('Family dashboard summary packet')).toBeVisible();
  await expect(reportExportCard.getByText('Policy drill-in export packet')).toBeVisible();
  await expect(reportExportCard.getByText('report-export-read-model-ready').first()).toBeVisible();
  await expect(reportExportCard.getByText('tracking-report-export-evidence-redacted-report')).toBeVisible();
  await expect(reportExportCard.getByText('tracking-report-export-evidence-policy-drill-in')).toBeVisible();
  await expect(reportExportCard.getByText('28-report-export-read-model-proof.json')).toBeVisible();
  await expect(reportExportCard.getByText('Hosted report/export packet rendering only')).toBeVisible();
  await expect(reportExportCard.getByText('No product claim')).toBeVisible();
}

async function assertHostedReportPolicyConsumerProof(trackingProofRegion: Locator): Promise<void> {
  const consumerCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="report-policy-consumer-ui"]').first();
  await expect(consumerCard).toBeVisible();
  await expect(consumerCard.getByRole('heading', { name: 'Report policy consumer UI' })).toBeVisible();
  await expect(consumerCard.getByText('Parent report summary consumer')).toBeVisible();
  await expect(consumerCard.getByText('Policy evidence drill-in consumer')).toBeVisible();
  await expect(consumerCard.getByText('Retention audit export consumer')).toBeVisible();
  await expect(consumerCard.getByText('consumer-ready').first()).toBeVisible();
  await expect(consumerCard.getByText('tracking-journal-row-report-summary')).toBeVisible();
  await expect(consumerCard.getByText('tracking-read-model-row-policy-drill-in')).toBeVisible();
  await expect(consumerCard.getByText('tracking-report-policy-evidence-decision')).toBeVisible();
  await expect(consumerCard.getByText('22-report-policy-consumer-proof.json')).toBeVisible();
  await expect(consumerCard.getByText('Hosted report/policy consumer rendering only')).toBeVisible();
  await expect(consumerCard.getByText('No product claim')).toBeVisible();
}

async function assertHostedNotificationParentSurfaceProof(trackingProofRegion: Locator): Promise<void> {
  const notificationCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="notification-parent-surface-history-ui"]')
    .first();
  await expect(notificationCard).toBeVisible();
  await expect(notificationCard.getByRole('heading', { name: 'Notification history intent UI' })).toBeVisible();
  await expect(notificationCard.getByText('Notification history ready')).toBeVisible();
  await expect(notificationCard.getByText('Manual notification action required')).toBeVisible();
  await expect(notificationCard.getByText('Notification provider unavailable')).toBeVisible();
  await expect(notificationCard.getByText('history-intent-ready')).toBeVisible();
  await expect(notificationCard.getByText('manual-action-required')).toBeVisible();
  await expect(notificationCard.getByText('provider-unavailable', { exact: true })).toBeVisible();
  await expect(notificationCard.getByText('tracking-provider-attempt-home-arrival')).toBeVisible();
  await expect(notificationCard.getByText('receipt-ingestion-required-home-arrival')).toBeVisible();
  await expect(notificationCard.getByText('quiet-hours-requirement-left-school')).toBeVisible();
  await expect(notificationCard.getByText('provider-adapter-unavailable')).toBeVisible();
  await expect(notificationCard.getByText('26-notification-parent-surface-history-proof.json')).toBeVisible();
  await expect(notificationCard.getByText('Hosted notification history rendering only')).toBeVisible();
  await expect(notificationCard.getByText('No product claim')).toBeVisible();
}

async function assertHostedParentActionReadinessProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  const parentActionCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="parent-action-readiness-ui"]')
    .first();
  await expect(parentActionCard).toBeVisible();
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="parent-action-readiness-ui"]');
  await expect(parentActionCard.getByRole('heading', { name: 'Parent action readiness UI' })).toBeVisible();
  await expect(parentActionCard.getByText('Expected-place parent alert ready')).toBeVisible();
  await expect(parentActionCard.getByText('Expected-place child check-in ready')).toBeVisible();
  await expect(parentActionCard.getByText('Parent acknowledgement recorded')).toBeVisible();
  await expect(parentActionCard.getByText('Critical escalation review ready')).toBeVisible();
  await expect(parentActionCard.getByText('alert-policy-ready')).toBeVisible();
  await expect(parentActionCard.getByText('acknowledgement-recorded')).toBeVisible();
  await expect(parentActionCard.getByText('escalation-review-ready')).toBeVisible();
  await expect(parentActionCard.getByText('expected-place-evidence-school-arrival')).toBeVisible();
  await expect(parentActionCard.getByText('tracking-parent-action-evidence-5')).toBeVisible();
  await expect(parentActionCard.getByText('29-expected-place-alert-policy-proof.json')).toBeVisible();
  await expect(parentActionCard.getByText('30-parent-acknowledgement-action-readiness-proof.json')).toBeVisible();
  await expect(parentActionCard.getByText('Hosted parent action readiness rendering only')).toBeVisible();
  await expect(parentActionCard.getByText('No product claim')).toBeVisible();
}

async function assertHostedMissingDeviceProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  const missingDeviceCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="missing-device-ui"]').first();
  await expect(missingDeviceCard).toBeVisible();
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="missing-device-ui"]');
  await expect(missingDeviceCard.getByRole('heading', { name: 'Missing-device state UI' })).toBeVisible();
  await expect(missingDeviceCard.getByText('Last-known only state')).toBeVisible();
  await expect(missingDeviceCard.getByText('Powered-off offline state')).toBeVisible();
  await expect(missingDeviceCard.getByText('Contact requested state')).toBeVisible();
  await expect(missingDeviceCard.getByText('Manual platform proof state')).toBeVisible();
  await expect(missingDeviceCard.getByText('location-evidence-last-known-stale')).toBeVisible();
  await expect(missingDeviceCard.getByText('device-status-powered-off')).toBeVisible();
  await expect(missingDeviceCard.getByText('device-status-contact-action-queued')).toBeVisible();
  await expect(missingDeviceCard.getByText('device-status-platform-proof-required')).toBeVisible();
  await expect(missingDeviceCard.getByText('powered-off-current-location-proof-forbidden')).toBeVisible();
  await expect(missingDeviceCard.getByText('os-lost-mode-api-proof-required')).toBeVisible();
  await expect(missingDeviceCard.getByText('29-missing-device-mode/proof.json')).toBeVisible();
  await expect(missingDeviceCard.getByText('Hosted missing-device rendering only')).toBeVisible();
  await expect(missingDeviceCard.getByText('No product claim')).toBeVisible();
}

async function assertHostedEvidenceDrawerProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  const evidenceDrawerCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="service-backed-evidence-drawer"]')
    .first();
  await expect(evidenceDrawerCard).toBeVisible();
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="service-backed-evidence-drawer"]');
  await expect(evidenceDrawerCard.getByRole('heading', { name: 'Evidence drawer proof' })).toBeVisible();
  await expect(evidenceDrawerCard.getByText('read-only evidence drawer', { exact: true })).toBeVisible();
  await expect(evidenceDrawerCard.getByText('tracking-hosted-expected-place-event')).toBeVisible();
  await expect(evidenceDrawerCard.getByText('location-evidence-hosted-1 | location-evidence-hosted-2')).toBeVisible();
  await expect(evidenceDrawerCard.getByText('Display-only evidence drill-in')).toBeVisible();
  await expect(evidenceDrawerCard.getByText('20-evidence-drawer-hosted-ui-proof.json')).toBeVisible();
  await expect(evidenceDrawerCard.getByText('No product claim')).toBeVisible();
  await expect(evidenceDrawerCard.locator('dd').filter({ hasText: /^0$/u }).first()).toBeVisible();
}

async function assertHostedCitationDetailProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  const citationDetailCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="service-backed-citation-detail"]')
    .first();
  await expect(citationDetailCard).toBeVisible();
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="service-backed-citation-detail"]');
  await expect(citationDetailCard.locator('h2').first()).toBeVisible();
  await expect(citationDetailCard.getByText('tracking-hosted-expected-place-event').first()).toBeVisible();
  await expect(citationDetailCard.getByText('location-evidence-hosted-1 | location-evidence-hosted-2')).toBeVisible();
  await expect(citationDetailCard.getByText('No product claim')).toBeVisible();
}

async function assertHostedFamilyDashboardRollupProof(trackingProofRegion: Locator): Promise<void> {
  await expect(trackingProofRegion.getByRole('heading', { name: 'Family dashboard tracking rollup' })).toBeVisible();
  await expect(trackingProofRegion.getByText('Family active summary')).toBeVisible();
  await expect(trackingProofRegion.getByText('Child attention summary')).toBeVisible();
  await expect(trackingProofRegion.getByText('Retention audit summary')).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-family-dashboard-evidence-active-summary')).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-family-dashboard-evidence-child-attention')).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-family-dashboard-evidence-retention-audit')).toBeVisible();
  await expect(trackingProofRegion.getByText('23-family-dashboard-rollup-proof.json')).toBeVisible();
  await expect(
    trackingProofRegion.getByText(
      'Hosted dashboard rollup rendering only; child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.'
    )
  ).toBeVisible();
}

async function assertHostedRetentionSettingsProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  const retentionSettingsCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="retention-settings-ui"]')
    .first();
  const localWrite = retentionSettingsCard.getByRole('button', { name: 'Send retention local write' });

  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="retention-settings-ui"]');
  await expect(trackingProofRegion.getByRole('heading', { name: 'Retention settings read-model UI' })).toBeVisible();
  await expect(trackingProofRegion.getByText('Retention window setting')).toBeVisible();
  await expect(trackingProofRegion.getByText('Delete-after-alert setting')).toBeVisible();
  await expect(trackingProofRegion.getByText('Parent export setting')).toBeVisible();
  await expect(trackingProofRegion.getByText('Remote sync disabled setting')).toBeVisible();
  await expect(trackingProofRegion.getByText('Remote AI disabled setting')).toBeVisible();
  await expect(trackingProofRegion.getByText('settings-read-model-ready').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-retention-settings-evidence-window')).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-retention-settings-evidence-remote-ai-disabled')).toBeVisible();
  await expect(retentionSettingsCard.getByText('24-retention-settings-read-model-proof.json')).toBeVisible();
  await expect(retentionSettingsCard.getByText('22-retention-local-service-state-proof.json')).toBeVisible();
  await ensureHostedTrackingCommandEnabled(page, trackingProofRegion, localWrite);
  await localWrite.click();
  await expect(page.getByText(AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted)).toBeVisible();
  await expect(
    page.getByText(AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow).first()
  ).toBeVisible();
  await expect(page.getByText('20-retention-settings-mutation-proof.json', { exact: false }).first()).toBeVisible();
  await expect(
    page.getByText(AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef).first()
  ).toBeVisible();
  await expect(
    page.getByText('Portal command/result rendering proves local service mutation execution').first()
  ).toBeVisible();
  await expect(
    trackingProofRegion.getByText(
      'Hosted retention settings rendering proves local service write execution and durable local persistence only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.'
    )
  ).toBeVisible();
}

async function ensureHostedTrackingCommandEnabled(
  _page: Page,
  _trackingProofRegion: Locator,
  command: Locator
): Promise<void> {
  await expect(command).toBeEnabled({ timeout: portalShellReadyTimeoutMs });
}

async function assertHostedChildRuntimeUiProof(page: Page, trackingProofRegion: Locator): Promise<void> {
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="child-runtime-ui"]');
  await expect(trackingProofRegion.getByRole('heading', { name: 'Child runtime UI proof' })).toBeVisible();
  await expect(trackingProofRegion.getByText('Tracking request disclosed')).toBeVisible();
  await expect(trackingProofRegion.getByText('Safe response visible')).toBeVisible();
  await expect(trackingProofRegion.getByText('Help response visible')).toBeVisible();
  await expect(trackingProofRegion.getByText('Location share asks consent')).toBeVisible();
  await expect(trackingProofRegion.getByText('Hosted proof only, not child-agent delivery')).toBeVisible();
  await expect(trackingProofRegion.getByText('19-child-runtime-ui-proof.json')).toBeVisible();
}

async function assertHostedUnsupportedManualPlatformProof(trackingProofRegion: Locator): Promise<void> {
  await expect(
    trackingProofRegion.getByRole('heading', { name: 'Unsupported/manual tracking platform proof' })
  ).toBeVisible();
  await expect(trackingProofRegion.getByText('Android background location manual required')).toBeVisible();
  await expect(trackingProofRegion.getByText('Android geofence transition manual required')).toBeVisible();
  await expect(trackingProofRegion.getByText('iOS background location manual required')).toBeVisible();
  await expect(trackingProofRegion.getByText('iOS geofence transition manual required')).toBeVisible();
  await expect(trackingProofRegion.getByText('Windows desktop OS location manual required')).toBeVisible();
  await expect(trackingProofRegion.getByText('Web child agent location unavailable')).toBeVisible();
  await expect(trackingProofRegion.getByText('Authority hard-control proof required')).toBeVisible();
  await expect(trackingProofRegion.getByText('platform-unsupported')).toBeVisible();
  await expect(trackingProofRegion.getByText('real-device-required')).toBeVisible();
  await expect(trackingProofRegion.getByText('authority-required')).toBeVisible();
  await expect(trackingProofRegion.getByText('unsupported-platform-manual-proof/proof.json')).toBeVisible();
  await expect(trackingProofRegion.getByText('No product claim').first()).toBeVisible();
}

async function refreshHostedTrackingStatus(page: Page, trackingProofRegion: Locator): Promise<void> {
  const refresh = page.getByRole('button', { exact: true, name: 'Refresh tracking status' });
  const expectedEvent = trackingProofRegion.getByText('tracking-hosted-expected-place-event').first();
  const deadline = Date.now() + portalShellReadyTimeoutMs;
  let lastError: unknown = undefined;

  while (Date.now() < deadline) {
    try {
      await expect(refresh).toBeEnabled({ timeout: 5_000 });
      await refresh.click();
      await expect(expectedEvent).toBeVisible({ timeout: 15_000 });
      return;
    } catch (error) {
      lastError = error;
      await expect(trackingProofRegion).toBeVisible({ timeout: 5_000 });
      await page.waitForTimeout(1_000);
    }
  }

  if (lastError instanceof Error) {
    throw lastError;
  }
  throw new Error('Timed out waiting for hosted tracking route refresh.');
}

async function captureHostedTrackingScreenshots(page: Page): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await mkdir(workpack31Root, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 1280, height: 960 });
  const trackingProofRegion = page.getByRole('region', { name: 'Tracking status proof' });
  await expect(trackingProofRegion).toBeVisible();
  await captureHostedTrackingProofCards(page, locateHostedTrackingProofCards(trackingProofRegion));

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
}

async function captureParentPortalShellScreenshots(page: Page): Promise<{
  readonly routes: readonly {
    readonly route: string;
    readonly screenshot: string;
    readonly assertions: readonly string[];
  }[];
}> {
  await page.setViewportSize({ width: 1280, height: 960 });
  await assertAndCaptureParentPortalShellRoute(page, {
    route: '#/overview',
    screenshotPath: parentOverviewShellScreenshotPath,
    expectedSvgText: ['Current device state', 'WHAT PARENTS CONTROL', 'DATA CUSTODY'],
  });
  await assertAndCaptureParentPortalShellRoute(page, {
    route: '#/devices',
    screenshotPath: parentDevicesShellScreenshotPath,
    expectedSvgText: [
      'SELECTED DEVICE CONTEXT',
      'SELECTED DEVICE',
      'SOURCE',
      'CONTROL',
      'Info',
      'Pair',
      'Update',
      'Capability',
    ],
  });
  return {
    routes: [
      {
        route: '#/overview',
        screenshot: path.relative(repoRoot, parentOverviewShellScreenshotPath).replace(/\\/gu, '/'),
        assertions: [
          'parent-overview-shell-visible',
          'parent-overview-custody-copy-visible',
          'parent-overview-tracking-summary-visible',
          'parent-overview-no-product-claim-visible',
        ],
      },
      {
        route: '#/devices',
        screenshot: path.relative(repoRoot, parentDevicesShellScreenshotPath).replace(/\\/gu, '/'),
        assertions: [
          'parent-devices-shell-visible',
          'parent-devices-context-copy-visible',
          'parent-devices-tracking-summary-visible',
          'parent-devices-no-product-claim-visible',
        ],
      },
    ],
  };
}

async function assertAndCaptureParentPortalShellRoute(
  page: Page,
  route: {
    readonly route: string;
    readonly screenshotPath: string;
    readonly expectedSvgText: readonly string[];
  }
): Promise<void> {
  await page.goto(route.route);
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  for (const expectedText of route.expectedSvgText) {
    await expect(surface.locator('text').filter({ hasText: expectedText }).first()).toBeVisible();
  }
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await expect(page.getByText('Service read model').first()).toBeVisible();
  await expect(page.getByText('Service data coverage').first()).toBeVisible();
  await expect(page.getByText('No product claim').first()).toBeVisible();
  await page.screenshot({ fullPage: true, path: route.screenshotPath });
}

function locateHostedTrackingProofCards(trackingProofRegion: Locator): HostedTrackingProofCards {
  const familyDashboardCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="family-dashboard-rollup"]')
    .first();
  const reportPolicyConsumerCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="report-policy-consumer-ui"]')
    .first();
  const reportExportCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="report-export-ui"]').first();
  const notificationParentSurfaceCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="notification-parent-surface-history-ui"]')
    .first();
  const parentActionReadinessCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="parent-action-readiness-ui"]')
    .first();
  const missingDeviceCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="missing-device-ui"]').first();
  const evidenceDrawerCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="service-backed-evidence-drawer"]')
    .first();
  const citationDetailCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="service-backed-citation-detail"]')
    .first();
  const retentionSettingsCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="retention-settings-ui"]')
    .first();
  const childCheckInCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="child-check-in"]').first();
  const childRuntimeUiCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="child-runtime-ui"]').first();
  const unsupportedManualCard = trackingProofRegion
    .getByRole('heading', { name: 'Unsupported/manual tracking platform proof' })
    .locator('xpath=ancestor::article[1]');
  return {
    familyDashboard: familyDashboardCard,
    reportPolicyConsumer: reportPolicyConsumerCard,
    reportExport: reportExportCard,
    notificationParentSurface: notificationParentSurfaceCard,
    parentActionReadiness: parentActionReadinessCard,
    missingDevice: missingDeviceCard,
    evidenceDrawer: evidenceDrawerCard,
    citationDetail: citationDetailCard,
    retentionSettings: retentionSettingsCard,
    childCheckIn: childCheckInCard,
    childRuntimeUi: childRuntimeUiCard,
    unsupportedManual: unsupportedManualCard,
  };
}

async function captureHostedTrackingProofCards(page: Page, cards: HostedTrackingProofCards): Promise<void> {
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.familyDashboard,
    '[data-ocentra-tracking-proof="family-dashboard-rollup"]',
    familyDashboardScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.reportPolicyConsumer,
    '[data-ocentra-tracking-proof="report-policy-consumer-ui"]',
    reportPolicyConsumerScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.reportExport,
    '[data-ocentra-tracking-proof="report-export-ui"]',
    reportExportScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.notificationParentSurface,
    '[data-ocentra-tracking-proof="notification-parent-surface-history-ui"]',
    notificationParentSurfaceScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.parentActionReadiness,
    '[data-ocentra-tracking-proof="parent-action-readiness-ui"]',
    parentActionReadinessScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.missingDevice,
    '[data-ocentra-tracking-proof="missing-device-ui"]',
    missingDeviceScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.evidenceDrawer,
    '[data-ocentra-tracking-proof="service-backed-evidence-drawer"]',
    evidenceDrawerScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.citationDetail,
    '[data-ocentra-tracking-proof="service-backed-citation-detail"]',
    citationDetailScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.retentionSettings,
    '[data-ocentra-tracking-proof="retention-settings-ui"]',
    retentionSettingsScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.childCheckIn,
    '[data-ocentra-tracking-proof="child-check-in"]',
    childCheckInScreenshotPath
  );
  await captureScrolledTrackingProofCardScreenshot(
    page,
    cards.childRuntimeUi,
    '[data-ocentra-tracking-proof="child-runtime-ui"]',
    childRuntimeUiScreenshotPath
  );
  await scrollTrackingProofCardByHeading(page, 'Unsupported/manual tracking platform proof');
  await captureTrackingProofCardScreenshot(page, cards.unsupportedManual, unsupportedManualScreenshotPath);
}

async function captureScrolledTrackingProofCardScreenshot(
  page: Page,
  proofCard: Locator,
  proofSelector: string,
  screenshotPath: string
): Promise<void> {
  await scrollTrackingProofCard(page, proofSelector);
  await captureTrackingProofCardScreenshot(page, proofCard, screenshotPath);
}

async function captureTrackingProofCardScreenshot(
  page: Page,
  proofCard: Locator,
  screenshotPath: string
): Promise<void> {
  await pageSettledForScreenshot();
  await expect(proofCard).toBeVisible();
  const box = await proofCard.boundingBox();
  if (box === null) {
    throw new Error('Tracking proof card screenshot target did not expose a bounding box.');
  }
  await page.screenshot({
    clip: {
      height: box.height,
      width: box.width,
      x: box.x,
      y: box.y,
    },
    path: screenshotPath,
  });
}

async function pageSettledForScreenshot(): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, 250));
}

async function scrollTrackingProofCard(page: Page, proofSelector: string): Promise<void> {
  await page.evaluate((selector) => {
    const grid = document.querySelector('.tracking-status-overlay-grid');
    const proofCard = document.querySelector(selector);
    if (grid instanceof HTMLElement && proofCard instanceof HTMLElement) {
      grid.scrollTop = Math.max(0, proofCard.offsetTop - 48);
    }
  }, proofSelector);
}

async function scrollTrackingProofCardByHeading(page: Page, heading: string): Promise<void> {
  await page.evaluate((expectedHeading) => {
    const grid = document.querySelector('.tracking-status-overlay-grid');
    const proofCard = Array.from(document.querySelectorAll('article')).find((article) => {
      const headingElement = article.querySelector('h2');
      return headingElement?.textContent === expectedHeading;
    });
    if (grid instanceof HTMLElement && proofCard instanceof HTMLElement) {
      grid.scrollTop = Math.max(0, proofCard.offsetTop - 48);
    }
  }, heading);
}

async function collectAccessibilitySummary(page: Page): Promise<{
  readonly hasNamedRegion: boolean;
  readonly headings: readonly string[];
  readonly paragraphs: readonly string[];
  readonly labels: readonly string[];
  readonly values: readonly string[];
  readonly buttons: readonly { readonly text: string; readonly disabled: boolean }[];
  readonly layoutBoxes: readonly HostedTrackingLayoutBox[];
  readonly unlabeledButtons: number;
}> {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Tracking status proof"]');
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    const buttons = Array.from(region?.querySelectorAll('button') ?? []).map((element) => ({
      text: text(element),
      disabled: element.hasAttribute('disabled'),
    }));
    const requiredProofIds = [
      'family-dashboard-rollup',
      'missing-device-ui',
      'notification-parent-surface-history-ui',
      'parent-action-readiness-ui',
      'report-policy-consumer-ui',
      'report-export-ui',
      'service-backed-evidence-drawer',
      'service-backed-citation-detail',
      'retention-settings-ui',
      'child-check-in',
      'child-runtime-ui',
    ];
    const layoutBoxes = requiredProofIds.flatMap((proofId) => {
      const element = region?.querySelector<HTMLElement>(`[data-ocentra-tracking-proof="${proofId}"]`);
      if (element === null || element === undefined) {
        return [];
      }
      const rect = element.getBoundingClientRect();
      return [
        {
          proofId,
          left: Math.round(rect.left),
          top: Math.round(rect.top),
          right: Math.round(rect.right),
          bottom: Math.round(rect.bottom),
          width: Math.round(rect.width),
          height: Math.round(rect.height),
        },
      ];
    });
    return {
      hasNamedRegion: region !== null,
      headings: Array.from(region?.querySelectorAll('h2') ?? []).map(text),
      paragraphs: Array.from(region?.querySelectorAll('p') ?? []).map(text),
      labels: Array.from(region?.querySelectorAll('dt') ?? []).map(text),
      values: Array.from(region?.querySelectorAll('dd') ?? []).map(text),
      buttons,
      layoutBoxes,
      unlabeledButtons: buttons.filter((button) => button.text.length === 0).length,
    };
  });
}

async function writeAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>,
  parentPortalShellSummary: Awaited<ReturnType<typeof captureParentPortalShellScreenshots>>
): Promise<void> {
  assertAccessibilitySummary(summary);
  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/proof-panels',
        assertions: hostedTrackingAssertions(),
        summary,
        screenshots: {
          desktop: path.relative(repoRoot, desktopScreenshotPath).replace(/\\/gu, '/'),
          familyDashboard: path.relative(repoRoot, familyDashboardScreenshotPath).replace(/\\/gu, '/'),
          reportPolicyConsumer: path.relative(repoRoot, reportPolicyConsumerScreenshotPath).replace(/\\/gu, '/'),
          reportExport: path.relative(repoRoot, reportExportScreenshotPath).replace(/\\/gu, '/'),
          notificationParentSurface: path
            .relative(repoRoot, notificationParentSurfaceScreenshotPath)
            .replace(/\\/gu, '/'),
          parentActionReadiness: path.relative(repoRoot, parentActionReadinessScreenshotPath).replace(/\\/gu, '/'),
          missingDevice: path.relative(repoRoot, missingDeviceScreenshotPath).replace(/\\/gu, '/'),
          evidenceDrawer: path.relative(repoRoot, evidenceDrawerScreenshotPath).replace(/\\/gu, '/'),
          citationDetail: path.relative(repoRoot, citationDetailScreenshotPath).replace(/\\/gu, '/'),
          retentionSettings: path.relative(repoRoot, retentionSettingsScreenshotPath).replace(/\\/gu, '/'),
          childCheckIn: path.relative(repoRoot, childCheckInScreenshotPath).replace(/\\/gu, '/'),
          childRuntimeUi: path.relative(repoRoot, childRuntimeUiScreenshotPath).replace(/\\/gu, '/'),
          parentOverviewShell: path.relative(repoRoot, parentOverviewShellScreenshotPath).replace(/\\/gu, '/'),
          parentDevicesShell: path.relative(repoRoot, parentDevicesShellScreenshotPath).replace(/\\/gu, '/'),
          unsupportedManualPlatform: path.relative(repoRoot, unsupportedManualScreenshotPath).replace(/\\/gu, '/'),
          mobile: path.relative(repoRoot, mobileScreenshotPath).replace(/\\/gu, '/'),
        },
        parentPortalShellSummary,
      },
      null,
      2
    )}\n`
  );
}

function assertAccessibilitySummary(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  assertAccessibilityBasics(summary);
  assertAccessibilityHeadingsAndLabels(summary);
  assertAccessibilityValues(summary);
}

function assertAccessibilityHeadingsAndLabels(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  assertContainsAll(summary.headings, [
    'Tracking status proof',
    'Service read model',
    'Service data coverage',
    'Family dashboard tracking rollup',
    'Report policy consumer UI',
    'Report export read-model UI',
    'Notification history intent UI',
    'Parent action readiness UI',
    'Missing-device state UI',
    'Retention settings read-model UI',
    'Evidence drawer proof',
    'Child check-in request',
    'Child runtime UI proof',
    'Unsupported/manual tracking platform proof',
  ]);
  assertContainsAll(summary.paragraphs, [
    'Your parent is asking you to check in. Are you safe?',
    'Child sees a clear tracking request, safe response, help response, and location-share consent copy.',
  ]);
  assertContainsAll(summary.labels, [
    'Evidence references',
    'Row count',
    'Child copy',
    'Child delivery',
    'Readiness kind',
    'Product claim',
  ]);
}

function assertAccessibilityValues(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  assertAccessibilityReportAndNotificationValues(summary.values);
  assertAccessibilityParentActionAndMissingDeviceValues(summary.values);
  assertAccessibilityRetentionAndEvidenceValues(summary.values);
  assertAccessibilityChildAndPlatformValues(summary.values);
}

function assertAccessibilityReportAndNotificationValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    "I'm safe",
    'Family active summary',
    'Child attention summary',
    'Retention audit summary',
    'Parent report summary consumer',
    'Policy evidence drill-in consumer',
    'Retention audit export consumer',
    'consumer-ready',
    'tracking-journal-row-report-summary',
    'tracking-read-model-row-policy-drill-in',
    'tracking-report-policy-evidence-decision',
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    'Hosted report/policy consumer rendering only; AI execution, product policy mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, production, and product readiness remain unclaimed.',
    'Redacted report packet',
    'Retention audit export packet',
    'Family dashboard summary packet',
    'Policy drill-in export packet',
    'report-export-read-model-ready',
    'tracking-report-export-evidence-redacted-report',
    'tracking-report-export-evidence-policy-drill-in',
    'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/28-report-export-read-model-proof.json',
    'Hosted report/export packet rendering only; raw location payload export, service mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.',
    'Notification history ready',
    'Manual notification action required',
    'Notification provider unavailable',
    'history-intent-ready',
    'manual-action-required',
    'provider-unavailable',
    'tracking-provider-attempt-home-arrival',
    'receipt-ingestion-required-home-arrival',
    'quiet-hours-requirement-left-school',
    'provider-adapter-unavailable | manual-parent-history-review-required',
    'output/tracking-plan-proof/26-alert-severity-and-notification-model/26-notification-parent-surface-history-proof.json',
    'Hosted notification history rendering only; preference mutation, quiet-hours runtime, provider delivery, receipt ingestion, child-device delivery, physical-device proof, authority, production storage, adapter dispatch, and product readiness remain unclaimed.',
  ]);
}

function assertAccessibilityParentActionAndMissingDeviceValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    'Expected-place parent alert ready',
    'Expected-place child check-in ready',
    'Parent acknowledgement recorded',
    'Critical escalation review ready',
    'alert-policy-ready',
    'acknowledgement-recorded',
    'escalation-review-ready',
    'expected-place-evidence-school-arrival',
    'tracking-parent-action-evidence-5',
    'output/tracking-plan-proof/16-expected-place-schedule-engine/29-expected-place-alert-policy-proof.json',
    'output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/30-parent-acknowledgement-action-readiness-proof.json',
    'Hosted parent action readiness rendering only; live service mutation, alert delivery, provider delivery, receipt ingestion, child-device runtime, physical-device proof, authority, production workers, adapter dispatch, and product readiness remain unclaimed.',
    'Last-known only state',
    'Powered-off offline state',
    'Contact requested state',
    'Manual platform proof state',
    'location-evidence-last-known-stale',
    'device-status-powered-off',
    'device-status-contact-action-queued',
    'device-status-platform-proof-required',
    'powered-off-current-location-proof-forbidden | hosted-read-only-missing-device-proof',
    'os-lost-mode-api-proof-required | physical-device-proof-required',
    'output/tracking-plan-proof/29-missing-device-mode/proof.json',
    'Hosted missing-device rendering only; current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode APIs, authority, production workers, and product readiness remain unclaimed.',
  ]);
}

function assertAccessibilityRetentionAndEvidenceValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    'Retention window setting',
    'Delete-after-alert setting',
    'Parent export setting',
    'Remote sync disabled setting',
    'Remote AI disabled setting',
    'settings-read-model-ready',
    'tracking-family-dashboard-evidence-active-summary',
    'tracking-hosted-expected-place-event',
    'location-evidence-hosted-1 | location-evidence-hosted-2',
    'tracking-retention-settings-evidence-window',
    'tracking-retention-settings-evidence-remote-ai-disabled',
    'Retention local service write result',
    AgentTrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
    AgentTrackingRetentionSettingsWriteDefaults.CommandId,
    AgentTrackingRetentionSettingsWriteDefaults.WriterIntentRef,
    AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
    AgentTrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
    'output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json',
    AgentTrackingRetentionSettingsWriteDefaults.MutationProofRef,
    'Portal command/result rendering proves local service mutation execution, local durable settings persistence, and local state revision only; product-ready writable settings, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
    'read-only evidence drawer',
    'Display-only evidence drill-in; policy evaluation, action dispatch, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
    'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/20-evidence-drawer-hosted-ui-proof.json',
  ]);
}

function assertAccessibilityChildAndPlatformValues(actualValues: readonly string[]): void {
  assertContainsAll(actualValues, [
    'Need help',
    'Share current location',
    'Call parent',
    'Child-device delivery not proved',
    'Tracking request disclosed',
    'Safe response visible',
    'Help response visible',
    'Location share asks consent',
    'Hosted proof only, not child-agent delivery',
    'Android background location manual required',
    'Web child agent location unavailable',
    'Authority hard-control proof required',
    'platform-unsupported',
    'real-device-required',
    'authority-required',
    'No product claim',
  ]);
}

function assertAccessibilityBasics(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.unlabeledButtons).toBe(0);
  assertHostedTrackingLayoutBoxes(summary.layoutBoxes);
}

function assertContainsAll(actualValues: readonly string[], expectedValues: readonly string[]): void {
  for (const expectedValue of expectedValues) {
    expect(actualValues).toContain(expectedValue);
  }
}

function assertHostedTrackingLayoutBoxes(layoutBoxes: readonly HostedTrackingLayoutBox[]): void {
  expect(layoutBoxes.map((box) => box.proofId).sort()).toEqual(
    [
      'child-check-in',
      'child-runtime-ui',
      'family-dashboard-rollup',
      'missing-device-ui',
      'notification-parent-surface-history-ui',
      'parent-action-readiness-ui',
      'report-policy-consumer-ui',
      'report-export-ui',
      'retention-settings-ui',
      'service-backed-citation-detail',
      'service-backed-evidence-drawer',
    ].sort()
  );
  for (const layoutBox of layoutBoxes) {
    expect(layoutBox.width).toBeGreaterThan(0);
    expect(layoutBox.height).toBeGreaterThan(0);
  }
  for (const [index, layoutBox] of layoutBoxes.entries()) {
    for (const otherBox of layoutBoxes.slice(index + 1)) {
      expect(layoutBoxesOverlap(layoutBox, otherBox)).toBe(false);
    }
  }
}

function layoutBoxesOverlap(first: HostedTrackingLayoutBox, second: HostedTrackingLayoutBox): boolean {
  return (
    first.left < second.right && first.right > second.left && first.top < second.bottom && first.bottom > second.top
  );
}

function hostedTrackingAssertions(): readonly string[] {
  return [
    'named-region',
    'visible-heading',
    'enabled-refresh-button',
    'service-backed-row-citation-visible',
    'service-data-coverage-visible',
    'family-dashboard-rollup-visible',
    'family-dashboard-rollup-screenshot',
    'report-policy-consumer-visible',
    'report-policy-consumer-screenshot',
    'report-export-read-model-visible',
    'report-export-read-model-screenshot',
    'notification-parent-surface-history-visible',
    'notification-parent-surface-history-screenshot',
    'parent-action-readiness-visible',
    'parent-action-readiness-screenshot',
    'missing-device-visible',
    'missing-device-screenshot',
    'service-backed-evidence-drawer-visible',
    'service-backed-evidence-drawer-screenshot',
    'service-backed-citation-detail-visible',
    'service-backed-citation-detail-screenshot',
    'retention-settings-read-model-visible',
    'retention-settings-local-write-clicked',
    'retention-settings-local-write-result-visible',
    'retention-settings-screenshot',
    'manual-required-visible',
    'physical-device-required-visible',
    'no-product-claim-visible',
    'child-check-in-copy-visible',
    'child-check-in-actions-visible',
    'child-device-delivery-not-claimed',
    'child-runtime-disclosure-visible',
    'child-runtime-safe-help-response-visible',
    'child-runtime-location-share-consent-visible',
    'child-runtime-hosted-only-boundary-visible',
    'unsupported-manual-platform-render-state-visible',
    'unsupported-manual-platform-screenshot',
    'no-unlabeled-buttons',
    'no-proof-card-overlap',
    'desktop-screenshot',
    'child-check-in-screenshot',
    'child-runtime-ui-screenshot',
    'parent-overview-shell-screenshot',
    'parent-devices-shell-screenshot',
    'mobile-screenshot',
  ];
}
