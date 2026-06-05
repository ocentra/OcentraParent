import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Locator, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.skip(process.env['TRACKING_PLAN_HOSTED_UI_PROOF'] !== '1', 'Dedicated tracking hosted UI proof only.');
test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const screenshotDir = path.join(proofRoot, '11-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-live-summary.png');
const mobileScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-live-summary-mobile.png');
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
  await expect(trackingProofRegion).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Tracking status proof' })).toBeVisible();

  await refreshHostedTrackingStatus(page, trackingProofRegion);

  await expect(page.getByRole('heading', { name: 'Service read model' })).toBeVisible();
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

  const routeText = await trackingProofRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:product ready|physical device proved|background geofence proved)/iu);
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
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
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
    const region = document.querySelector('[aria-label="Tracking status proof"]');
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
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.unlabeledButtons).toBe(0);
  expect(summary.headings).toContain('Tracking status proof');
  expect(summary.headings).toContain('Service read model');
  expect(summary.labels).toContain('Evidence references');
  expect(summary.labels).toContain('Product claim');
  expect(summary.values).toContain('No product claim');

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/policy-tracking',
        assertions: [
          'named-region',
          'visible-heading',
          'enabled-refresh-button',
          'service-backed-row-citation-visible',
          'manual-required-visible',
          'physical-device-required-visible',
          'no-product-claim-visible',
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
