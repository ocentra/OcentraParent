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
  await expect(page.getByRole('heading', { name: 'Family dashboard rollup' })).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-hosted-expected-place-event').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(trackingProofRegion.getByText('2026-06-04T10:10:00.000Z').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('recent', { exact: true }).first()).toBeVisible();
  await expect(trackingProofRegion.getByText('child-device-query-store', { exact: true }).first()).toBeVisible();
  await expect(
    trackingProofRegion.getByText('location-evidence-hosted-1 | location-evidence-hosted-2').first()
  ).toBeVisible();
  await expect(
    trackingProofRegion.getByText('family-active-summary | child-attention-summary | retention-audit-summary').first()
  ).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-family-dashboard-child-attention-ready').first()).toBeVisible();
  await expect(
    trackingProofRegion.getByText('tracking-family-dashboard-evidence-retention-audit').first()
  ).toBeVisible();
  await expect(trackingProofRegion.getByText('Manual proof required').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('Physical device proof required').first()).toBeVisible();
  await expect(trackingProofRegion.getByText('No product claim').first()).toBeVisible();
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
  const childCheckInCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="child-check-in"]').first();
  const childRuntimeUiCard = trackingProofRegion.locator('[data-ocentra-tracking-proof="child-runtime-ui"]').first();
  const unsupportedManualCard = trackingProofRegion
    .getByRole('heading', { name: 'Unsupported/manual tracking platform proof' })
    .locator('xpath=ancestor::article[1]');
  await page.evaluate(() => {
    const grid = document.querySelector('.tracking-status-overlay-grid');
    const childCheckIn = document.querySelector('[data-ocentra-tracking-proof="child-check-in"]');
    if (grid instanceof HTMLElement && childCheckIn instanceof HTMLElement) {
      grid.scrollTop = Math.max(0, childCheckIn.offsetTop - 48);
    }
  });
  await page.waitForTimeout(250);
  await expect(childCheckInCard).toBeVisible();
  await childCheckInCard.screenshot({ path: childCheckInScreenshotPath });
  await page.evaluate(() => {
    const grid = document.querySelector('.tracking-status-overlay-grid');
    const childRuntimeUi = document.querySelector('[data-ocentra-tracking-proof="child-runtime-ui"]');
    if (grid instanceof HTMLElement && childRuntimeUi instanceof HTMLElement) {
      grid.scrollTop = Math.max(0, childRuntimeUi.offsetTop - 48);
    }
  });
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
  expect(summary.headings).toContain('Tracking status proof');
  expect(summary.headings).toContain('Service read model');
  expect(summary.headings).toContain('Service data coverage');
  expect(summary.labels).toContain('Evidence references');
  expect(summary.labels).toContain('Reason codes');
  expect(summary.labels).toContain('Product claim');
  assertFamilyDashboardRollupAccessibilitySummary(summary);
  assertChildTrackingAccessibilitySummary(summary);
  assertUnsupportedManualAccessibilitySummary(summary);
  expect(summary.values).toContain('No product claim');
}

function assertFamilyDashboardRollupAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>
): void {
  expect(summary.headings).toContain('Family dashboard rollup');
  expect(summary.values).toContain('family-active-summary | child-attention-summary | retention-audit-summary');
  expect(summary.values).toContain(
    'tracking-family-dashboard-evidence-active-summary | tracking-family-dashboard-evidence-child-attention | tracking-family-dashboard-evidence-retention-audit'
  );
  expect(summary.values).toContain(
    'tracking-family-dashboard-active-summary-ready | tracking-family-dashboard-child-attention-ready | tracking-family-dashboard-retention-audit-ready'
  );
}

function assertChildTrackingAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>
): void {
  expect(summary.headings).toContain('Child check-in request');
  expect(summary.headings).toContain('Child runtime UI proof');
  expect(summary.paragraphs).toContain('Your parent is asking you to check in. Are you safe?');
  expect(summary.paragraphs).toContain(
    'Child sees a clear tracking request, safe response, help response, and location-share consent copy.'
  );
  expect(summary.labels).toContain('Child copy');
  expect(summary.labels).toContain('Child delivery');
  expect(summary.values).toContain("I'm safe");
  expect(summary.values).toContain('Need help');
  expect(summary.values).toContain('Share current location');
  expect(summary.values).toContain('Call parent');
  expect(summary.values).toContain('Child-device delivery not proved');
  expect(summary.values).toContain('Tracking request disclosed');
  expect(summary.values).toContain('Safe response visible');
  expect(summary.values).toContain('Help response visible');
  expect(summary.values).toContain('Location share asks consent');
  expect(summary.values).toContain('Hosted proof only, not child-agent delivery');
}

function assertUnsupportedManualAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>
): void {
  expect(summary.headings).toContain('Unsupported/manual tracking platform proof');
  expect(summary.labels).toContain('Readiness kind');
  expect(summary.values).toContain('Android background location manual required');
  expect(summary.values).toContain('Web child agent location unavailable');
  expect(summary.values).toContain('Authority hard-control proof required');
  expect(summary.values).toContain('platform-unsupported');
  expect(summary.values).toContain('real-device-required');
  expect(summary.values).toContain('authority-required');
}

function hostedTrackingAssertions(): readonly string[] {
  return [
    'named-region',
    'visible-heading',
    'enabled-refresh-button',
    'service-backed-row-citation-visible',
    'service-data-coverage-visible',
    'family-dashboard-rollup-visible',
    'family-dashboard-rollup-evidence-visible',
    'family-dashboard-rollup-no-product-claim',
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
