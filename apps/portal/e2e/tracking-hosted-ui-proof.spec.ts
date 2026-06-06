import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Locator, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.skip(process.env['TRACKING_PLAN_HOSTED_UI_PROOF'] !== '1', 'Dedicated tracking hosted UI proof only.');
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
const citationDetailScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-citation-detail.png');
const childCheckInScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-child-check-in.png');
const childRuntimeUiScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-child-runtime-ui.png');
const unsupportedManualScreenshotPath = path.join(workpack31Root, '19-unsupported-manual-hosted-ui.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'tracking-plan-hosted-ui-proof',
  'accessibility-summary.json'
);

test('hosted policy tracking route renders real-service proof without product claims', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await assertHostedPolicyTrackingRoute(page);
  await captureHostedTrackingScreenshots(page);
  await writeAccessibilitySummary(await collectAccessibilitySummary(page));

  expect(browserFailures).toEqual([]);
});

async function assertHostedPolicyTrackingRoute(page: Page): Promise<void> {
  await page.goto('/#/policy-tracking');
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
  await assertHostedCitationDetailProof(trackingProofRegion);
  await expect(page.getByRole('heading', { name: 'Child check-in request' })).toBeVisible();
  await expect(trackingProofRegion.getByText('Your parent is asking you to check in. Are you safe?')).toBeVisible();
  await expect(trackingProofRegion.getByText("I'm safe")).toBeVisible();
  await expect(trackingProofRegion.getByText('Need help')).toBeVisible();
  await expect(trackingProofRegion.getByText('Share current location')).toBeVisible();
  await expect(trackingProofRegion.getByText('Call parent', { exact: true })).toBeVisible();
  await expect(trackingProofRegion.getByText('Child-device delivery not proved').first()).toBeVisible();
  await assertHostedChildRuntimeUiProof(trackingProofRegion);
  await assertHostedUnsupportedManualPlatformProof(trackingProofRegion);

  const routeText = await trackingProofRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:product ready|physical device proved|background geofence proved)/iu);
  expect(routeText ?? '').not.toMatch(/(?:trouble|lying|bad place|delivered to child device)/iu);
}

async function assertHostedCitationDetailProof(trackingProofRegion: Locator): Promise<void> {
  const citationDetailCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="service-backed-citation-detail"]')
    .first();
  await expect(citationDetailCard).toBeVisible();
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

async function assertHostedChildRuntimeUiProof(trackingProofRegion: Locator): Promise<void> {
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
  const familyDashboardCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="family-dashboard-rollup"]')
    .first();
  const citationDetailCard = trackingProofRegion
    .locator('[data-ocentra-tracking-proof="service-backed-citation-detail"]')
    .first();
  const childCheckInCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="child-check-in"]').first();
  const childRuntimeUiCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="child-runtime-ui"]').first();
  const unsupportedManualCard = trackingProofRegion
    .getByRole('heading', { name: 'Unsupported/manual tracking platform proof' })
    .locator('xpath=ancestor::article[1]');
  await page.evaluate(() => {
    const grid = document.querySelector('.tracking-status-overlay-grid');
    const familyDashboard = document.querySelector('[data-ocentra-tracking-proof="family-dashboard-rollup"]');
    if (grid instanceof HTMLElement && familyDashboard instanceof HTMLElement) {
      grid.scrollTop = Math.max(0, familyDashboard.offsetTop - 48);
    }
  });
  await page.waitForTimeout(250);
  await expect(familyDashboardCard).toBeVisible();
  await familyDashboardCard.screenshot({ path: familyDashboardScreenshotPath });
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="service-backed-citation-detail"]');
  await expect(citationDetailCard).toBeVisible();
  await citationDetailCard.screenshot({ path: citationDetailScreenshotPath });
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="child-check-in"]');
  await page.waitForTimeout(250);
  await expect(childCheckInCard).toBeVisible();
  await childCheckInCard.screenshot({ path: childCheckInScreenshotPath });
  await scrollTrackingProofCard(page, '[data-ocentra-tracking-proof="child-runtime-ui"]');
  await page.waitForTimeout(250);
  await expect(childRuntimeUiCard).toBeVisible();
  await childRuntimeUiCard.screenshot({ path: childRuntimeUiScreenshotPath });
  await unsupportedManualCard.scrollIntoViewIfNeeded();
  await page.waitForTimeout(250);
  await expect(unsupportedManualCard).toBeVisible();
  await unsupportedManualCard.screenshot({ path: unsupportedManualScreenshotPath });

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
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

async function collectAccessibilitySummary(page: Page): Promise<{
  readonly hasNamedRegion: boolean;
  readonly headings: readonly string[];
  readonly paragraphs: readonly string[];
  readonly labels: readonly string[];
  readonly values: readonly string[];
  readonly buttons: readonly { readonly text: string; readonly disabled: boolean }[];
  readonly unlabeledButtons: number;
}> {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Tracking status proof"]');
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    const buttons = Array.from(region?.querySelectorAll('button') ?? []).map((element) => ({
      text: text(element),
      disabled: element.hasAttribute('disabled'),
    }));
    return {
      hasNamedRegion: region !== null,
      headings: Array.from(region?.querySelectorAll('h2') ?? []).map(text),
      paragraphs: Array.from(region?.querySelectorAll('p') ?? []).map(text),
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
        route: '#/policy-tracking',
        assertions: hostedTrackingAssertions(),
        summary,
        screenshots: {
          desktop: path.relative(repoRoot, desktopScreenshotPath).replace(/\\/gu, '/'),
          familyDashboard: path.relative(repoRoot, familyDashboardScreenshotPath).replace(/\\/gu, '/'),
          citationDetail: path.relative(repoRoot, citationDetailScreenshotPath).replace(/\\/gu, '/'),
          childCheckIn: path.relative(repoRoot, childCheckInScreenshotPath).replace(/\\/gu, '/'),
          childRuntimeUi: path.relative(repoRoot, childRuntimeUiScreenshotPath).replace(/\\/gu, '/'),
          unsupportedManualPlatform: path.relative(repoRoot, unsupportedManualScreenshotPath).replace(/\\/gu, '/'),
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
  assertContainsAll(summary.headings, [
    'Tracking status proof',
    'Service read model',
    'Service data coverage',
    'Family dashboard tracking rollup',
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
  assertContainsAll(summary.values, [
    "I'm safe",
    'Family active summary',
    'Child attention summary',
    'Retention audit summary',
    'tracking-family-dashboard-evidence-active-summary',
    'tracking-hosted-expected-place-event',
    'location-evidence-hosted-1 | location-evidence-hosted-2',
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

function assertContainsAll(actualValues: readonly string[], expectedValues: readonly string[]): void {
  for (const expectedValue of expectedValues) {
    expect(actualValues).toContain(expectedValue);
  }
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
    'service-backed-citation-detail-visible',
    'service-backed-citation-detail-screenshot',
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
    'desktop-screenshot',
    'child-check-in-screenshot',
    'child-runtime-ui-screenshot',
    'mobile-screenshot',
  ];
}
