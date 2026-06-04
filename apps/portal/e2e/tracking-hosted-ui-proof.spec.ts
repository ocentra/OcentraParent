import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const screenshotDir = path.join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '30-parent-and-child-ui-ux-surfaces',
  '11-ui-snapshots'
);
const desktopScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-live-summary.png');
const mobileScreenshotPath = path.join(screenshotDir, 'hosted-policy-tracking-live-summary-mobile.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'tracking-plan-hosted-ui-proof',
  'accessibility-summary.json'
);

async function refreshCommandTrackingStatus(page: Page) {
  await page.goto('/#/commands');
  const commandRefresh = page.getByRole('button', { exact: true, name: 'Refresh tracking status' });
  await expect(commandRefresh).toBeEnabled({ timeout: portalShellReadyTimeoutMs });
  await commandRefresh.click();
  await expect(page.locator('strong', { hasText: 'agent.activity.tracking.read-model.reported' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText('tracking-hosted-expected-place-event')).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function assertHostedPolicyTrackingRoute(page: Page) {
  await page.goto('/#/policy-tracking');
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Tracking status proof' })).toBeVisible();

  const refresh = page.getByRole('button', { name: 'Refresh tracking status' });
  await expect(refresh).toBeEnabled();
  await refresh.click();

  await expect(page.getByRole('heading', { name: 'Service read model' })).toBeVisible();
  const trackingProofRegion = page.getByRole('region', { name: 'Tracking status proof' });
  await expect(trackingProofRegion.getByText('2', { exact: true })).toBeVisible();
  await expect(trackingProofRegion.getByText('2026-06-04T10:10:00.000Z')).toBeVisible();
  await expect(trackingProofRegion.getByText('tracking-hosted-expected-place-event')).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(trackingProofRegion.getByText('recent', { exact: true })).toBeVisible();
  await expect(trackingProofRegion.getByText('child-device-query-store', { exact: true })).toBeVisible();
  await expect(trackingProofRegion.getByText('location-evidence-hosted-1 | location-evidence-hosted-2')).toBeVisible();
  await expect(trackingProofRegion.getByText('No product claim').first()).toBeVisible();
}

async function captureHostedTrackingScreenshots(page: Page) {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
}

async function collectAccessibilitySummary(page: Page) {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Tracking status proof"]');
    const labels = Array.from(region?.querySelectorAll('dt') ?? []).map((element) => element.textContent ?? '');
    const values = Array.from(region?.querySelectorAll('dd') ?? []).map((element) => element.textContent ?? '');
    const buttons = Array.from(region?.querySelectorAll('button') ?? []).map((element) => ({
      text: element.textContent ?? '',
      disabled: element.hasAttribute('disabled'),
    }));
    return {
      hasNamedRegion: region !== null,
      labels,
      values,
      buttons,
    };
  });
}

async function writeAccessibilitySummary(summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>) {
  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/policy-tracking',
        assertions: [
          'named-region',
          'role-button-refresh',
          'heading-visible',
          'service-backed-row-citations-visible',
          'desktop-screenshot',
          'mobile-screenshot',
          'no-product-claim-visible',
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

test('hosted policy tracking route renders service-backed tracking citations', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await refreshCommandTrackingStatus(page);
  await assertHostedPolicyTrackingRoute(page);
  await captureHostedTrackingScreenshots(page);
  await writeAccessibilitySummary(await collectAccessibilitySummary(page));

  expect(browserFailures).toEqual([]);
});
