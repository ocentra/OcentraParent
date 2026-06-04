import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test } from '@playwright/test';
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

test('hosted policy tracking route renders service-backed tracking citations', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await page.goto('/#/commands');
  const commandRefresh = page.getByRole('button', { exact: true, name: 'Refresh tracking status' });
  await expect(commandRefresh).toBeEnabled({ timeout: portalShellReadyTimeoutMs });
  await commandRefresh.click();
  await commandRefresh.click();
  await expect(page.getByText('agent.activity.tracking.read-model.reported')).toBeVisible();
  await expect(page.getByText('tracking-hosted-expected-place-event')).toBeVisible();

  await page.goto('/#/policy-tracking');
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Tracking status proof' })).toBeVisible();

  const refresh = page.getByRole('button', { name: 'Refresh tracking status' });
  await expect(refresh).toBeEnabled();

  await expect(page.getByRole('heading', { name: 'Service read model' })).toBeVisible();
  await expect(page.getByText('tracking-hosted-expected-place-event')).toBeVisible();
  await expect(page.getByText('child-android-hosted-proof')).toBeVisible();
  await expect(page.getByText('android', { exact: true })).toBeVisible();
  await expect(page.getByText('tracking-engine')).toBeVisible();
  await expect(page.getByText('activity.tracking.expected-place.evaluated')).toBeVisible();
  await expect(page.getByText('School', { exact: true })).toBeVisible();
  await expect(page.getByText('expected-place-school')).toBeVisible();
  await expect(page.getByText('location-evidence-hosted-1 | location-evidence-hosted-2')).toBeVisible();
  await expect(page.getByText('No product claim').first()).toBeVisible();

  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Tracking status proof' })).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });

  const summary = await page.evaluate(() => {
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

  expect(browserFailures).toEqual([]);
});
