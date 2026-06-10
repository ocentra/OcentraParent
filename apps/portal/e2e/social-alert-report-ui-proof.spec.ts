import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.skip(process.env['SOCIAL_ALERT_REPORT_UI_PROOF'] !== '1', 'Dedicated social alert/report UI proof only.');
test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'browser-plan-proof', 'social-alert-report-intent-ui-proof');
const screenshotDir = path.join(proofRoot, '06-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'social-alert-report-browser-route.png');
const mobileScreenshotPath = path.join(screenshotDir, 'social-alert-report-browser-route-mobile.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'social-alert-report-intent-ui-proof',
  'accessibility-summary.json'
);

test('browser route renders service-backed social alert and report intent rows', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await assertSocialAlertReportRoute(page);
  await requestSocialAlertReportReadModel(page);
  await assertServiceBackedSocialAlertReportRows(page);
  await requestSocialParentNotificationDeliveryReadModel(page);
  await assertSocialParentNotificationDeliveryRows(page);
  await requestSocialAlertReportParentSurfaceReadModel(page);
  await assertSocialAlertReportParentSurfaceRows(page);
  await assertBrowserActionIntentStatusRows(page);
  await assertBrowserReceiptStatusRows(page);
  await captureSocialAlertReportScreenshots(page);
  await writeAccessibilitySummary(await collectAccessibilitySummary(page));

  expect(browserFailures).toEqual([]);
});

async function assertSocialAlertReportRoute(page: Page): Promise<void> {
  await page.goto('/#/browser?agent.browser.social-alert-report.read-model.reported');
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(alertRegion.getByRole('heading', { name: 'Social alerts and reports' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: '0 social alert/report rows' })).toBeVisible();
  await expect(
    alertRegion.getByRole('heading', { name: 'No social alert/report read model has been reported yet.' })
  ).toBeVisible();
  await expect(alertRegion.getByText('unavailable', { exact: true }).first()).toBeVisible();
  await expect(alertRegion.getByText('not reported', { exact: true }).first()).toBeVisible();
  await expect(alertRegion.getByText('Rendered parent alert/report intent surface only').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:provider delivery complete|report delivered|notification ui delivered)/iu);
  expect(routeText ?? '').not.toMatch(/(?:final policy execution proved|enforcement active)/iu);
}

async function requestSocialParentNotificationDeliveryReadModel(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await alertRegion.getByRole('button', { name: 'Social parent notification delivery readiness' }).click();
  await expect(alertRegion.getByRole('heading', { name: '3 parent notification readiness rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function requestSocialAlertReportParentSurfaceReadModel(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await alertRegion.getByRole('button', { name: 'Social parent surface status' }).click();
  await expect(alertRegion.getByRole('heading', { name: '3 parent surface rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function requestSocialAlertReportReadModel(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await alertRegion.getByRole('button', { name: 'Social alerts and reports' }).click();
  await expect(alertRegion.getByRole('heading', { name: '4 social alert/report rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function assertSocialParentNotificationDeliveryRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(
    alertRegion.getByRole('button', { name: 'Social parent notification delivery readiness' })
  ).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent report status ready' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent notification manual proof required' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent notification delivery unavailable' })).toBeVisible();
  await expect(alertRegion.getByText('parent-report-status-ready').first()).toBeVisible();
  await expect(alertRegion.getByText('parent-owned-report-ready').first()).toBeVisible();
  await expect(alertRegion.getByText('external-report-delivery-runtime-unavailable').first()).toBeVisible();
  await expect(alertRegion.getByText('manual-parent-notification-ui-runtime-proof-required').first()).toBeVisible();
  await expect(alertRegion.getByText('Parent report readiness projection only').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:parent notification ui delivered|provider delivery complete)/iu);
  expect(routeText ?? '').not.toMatch(/(?:external runtime report delivered|final policy execution proved)/iu);
  expect(routeText ?? '').not.toMatch(/(?:enforcement active|provider receipt ingested)/iu);
}

async function assertSocialAlertReportParentSurfaceRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('button', { name: 'Social parent surface status' })).toBeVisible();
  await expect(
    alertRegion.getByRole('heading', { name: 'Parent surface manual action required' }).first()
  ).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Parent surface unavailable' })).toBeVisible();
  await expect(alertRegion.getByText('manual-action-required').first()).toBeVisible();
  await expect(alertRegion.getByText('unavailable-visible').first()).toBeVisible();
  await expect(alertRegion.getByText('preference-setup-required').first()).toBeVisible();
  await expect(alertRegion.getByText('preference-disabled-visible').first()).toBeVisible();
  await expect(alertRegion.getByText('manual-parent-surface-runtime-proof-required').first()).toBeVisible();
  await expect(alertRegion.getByText('Parent-surface status projection only').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:notification ui delivered|provider delivery complete)/iu);
  expect(routeText ?? '').not.toMatch(/(?:receipt ingested|final policy execution proved|enforcement active)/iu);
}

async function assertServiceBackedSocialAlertReportRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('heading', { name: 'High-risk social alert intent' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Manual alert/report proof required' })).toBeVisible();
  await expect(alertRegion.getByRole('heading', { name: 'Provider status manual required' }).first()).toBeVisible();
  await expect(alertRegion.getByText('local-outbox-only').first()).toBeVisible();
  await expect(alertRegion.getByText('manual-required').first()).toBeVisible();
  await expect(alertRegion.getByText('provider-adapter-required').first()).toBeVisible();
  await expect(
    alertRegion.getByText('provider-adapter-required-social-alert-report-high-risk-service').first()
  ).toBeVisible();
  await expect(
    alertRegion.getByText('provider-credentials-required-social-alert-report-high-risk-service').first()
  ).toBeVisible();
  await expect(
    alertRegion.getByText('provider-smoke-proof-required-social-alert-report-high-risk-service').first()
  ).toBeVisible();
  await expect(alertRegion.getByText('not-observed').first()).toBeVisible();
  await expect(alertRegion.getByText('social-high-risk-signal').first()).toBeVisible();
  await expect(alertRegion.getByText('social-manual-review-required').first()).toBeVisible();
  await expect(alertRegion.getByText('evidence-social-route-gate').first()).toBeVisible();
  await expect(alertRegion.getByText('evidence-social-manual-gap').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:provider delivery complete|provider observed|receipt ingested)/iu);
  expect(routeText ?? '').not.toMatch(/(?:report delivered|notification ui delivered|enforcement active)/iu);
}

async function assertBrowserActionIntentStatusRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('heading', { name: 'Browser action-intent stream status' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(alertRegion.getByText('0 action candidates').first()).toBeVisible();
  await expect(alertRegion.getByText('not-claimed').first()).toBeVisible();
  await expect(alertRegion.getByText('not-observed').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:adapter dispatch complete|browser mutation complete)/iu);
  expect(routeText ?? '').not.toMatch(/(?:child intervention executed|final policy execution proved)/iu);
  expect(routeText ?? '').not.toMatch(/(?:unmanaged exact url supported|enforcement active)/iu);
}

async function assertBrowserReceiptStatusRows(page: Page): Promise<void> {
  const alertRegion = page.getByRole('region', { name: 'Social alerts and reports' });
  await expect(alertRegion.getByRole('heading', { name: 'Social provider receipt stream status' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(alertRegion.getByRole('heading', { name: 'Social provider receipt ingestion readiness' })).toBeVisible();
  await expect(alertRegion.getByText('0 provider receipts observed').first()).toBeVisible();
  await expect(alertRegion.getByText('not-observed').first()).toBeVisible();
  await expect(alertRegion.getByText('unavailable').first()).toBeVisible();
  await expect(alertRegion.getByText('not-claimed').first()).toBeVisible();

  const routeText = await alertRegion.textContent();
  expect(routeText ?? '').not.toMatch(
    /(?:provider delivery complete|provider receipt ingested|webhook runtime ready)/iu
  );
  expect(routeText ?? '').not.toMatch(/(?:browser mutation complete|child intervention executed|enforcement active)/iu);
}

async function captureSocialAlertReportScreenshots(page: Page): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Social alerts and reports' })).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
}

async function collectAccessibilitySummary(page: Page): Promise<{
  readonly hasNamedRegion: boolean;
  readonly headings: readonly string[];
  readonly labels: readonly string[];
  readonly values: readonly string[];
  readonly buttons: readonly { readonly text: string; readonly disabled: boolean }[];
  readonly unlabeledButtons: number;
}> {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Social alerts and reports"]');
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    const buttons = Array.from(region?.querySelectorAll('button') ?? []).map((element) => ({
      text: text(element),
      disabled: element.hasAttribute('disabled'),
    }));
    return {
      hasNamedRegion: region !== null,
      headings: Array.from(region?.querySelectorAll('h2') ?? []).map(text),
      labels: Array.from(region?.querySelectorAll('dt') ?? []).map(text),
      values: Array.from(region?.querySelectorAll('dd') ?? []).map(text),
      buttons,
      unlabeledButtons: buttons.filter((button) => button.text.length === 0).length,
    };
  });
}

async function writeAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>
): Promise<void> {
  assertAccessibilitySummary(summary);

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/browser',
        assertions: [
          'named-social-alert-report-region',
          'zero-row-before-command-visible',
          'service-backed-four-row-summary-visible',
          'high-risk-local-outbox-row-visible',
          'manual-required-row-visible',
          'provider-status-manual-required-row-visible',
          'parent-notification-delivery-readiness-command-visible',
          'parent-report-status-ready-row-visible',
          'parent-notification-manual-required-row-visible',
          'parent-notification-unavailable-row-visible',
          'parent-notification-delivery-non-claims-visible',
          'parent-surface-status-command-visible',
          'parent-surface-manual-row-visible',
          'parent-surface-unavailable-row-visible',
          'parent-surface-non-claims-visible',
          'action-intent-stream-status-visible',
          'action-intent-zero-candidates-visible',
          'receipt-stream-status-visible',
          'receipt-ingestion-readiness-visible',
          'receipt-zero-provider-receipts-visible',
          'non-claim-copy-visible',
          'provider-report-notification-final-policy-enforcement-claims-not-visible',
          'action-intent-adapter-child-intervention-browser-mutation-enforcement-claims-not-visible',
          'receipt-ingestion-runtime-provider-delivery-enforcement-claims-not-visible',
          'no-unlabeled-buttons',
          'desktop-screenshot',
          'mobile-screenshot',
        ],
        summary,
        screenshots: {
          desktop: path.relative(repoRoot, desktopScreenshotPath).replace(/\\/gu, '/'),
          mobile: path.relative(repoRoot, mobileScreenshotPath).replace(/\\/gu, '/'),
        },
      },
      null,
      2
    )}\n`
  );
}

function assertAccessibilitySummary(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>): void {
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.unlabeledButtons).toBe(0);
  for (const heading of expectedAccessibilityHeadings()) {
    expect(summary.headings).toContain(heading);
  }
  for (const label of ['Rows returned', 'Generated at', 'Capability', 'Product claim']) {
    expect(summary.labels).toContain(label);
  }
  for (const value of expectedAccessibilityValues()) {
    expect(summary.values).toContain(value);
  }
}

function expectedAccessibilityHeadings(): readonly string[] {
  return [
    'Social alerts and reports',
    '4 social alert/report rows',
    'High-risk social alert intent',
    'Manual alert/report proof required',
    'Provider status manual required',
    '3 parent notification readiness rows',
    'Parent report status ready',
    'Parent notification manual proof required',
    'Parent notification delivery unavailable',
    '3 parent surface rows',
    'Parent surface manual action required',
    'Parent surface unavailable',
    'Browser action-intent stream status',
    'Social provider receipt stream status',
    'Social provider receipt ingestion readiness',
  ];
}

function expectedAccessibilityValues(): readonly string[] {
  return [
    '4',
    '3',
    'parent-report-status-ready',
    'parent-owned-report-ready',
    'manual-action-required',
    'unavailable-visible',
    'preference-setup-required',
    '0 action candidates',
    '0 provider receipts observed',
    'local-outbox-only',
    'manual-required',
    'not-observed',
  ];
}
