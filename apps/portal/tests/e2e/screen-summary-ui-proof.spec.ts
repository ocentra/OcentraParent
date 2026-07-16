import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Locator, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'screen-plan-proof', 'screen-parent-portal-summary-ui');
const testResultRoot = path.join(repoRoot, 'test-results', 'screen-parent-portal-summary-ui-proof');
const screenshotDir = path.join(proofRoot, 'screenshots');
const desktopScreenshotPath = path.join(screenshotDir, 'screen-analysis-route-desktop.png');
const mobileScreenshotPath = path.join(screenshotDir, 'screen-analysis-route-mobile.png');
const accessibilitySummaryPath = path.join(testResultRoot, 'accessibility-summary.json');
const productBoundaryCopy = 'No family setting is configured for this area yet.';

if (process.env['SCREEN_PARENT_PORTAL_SUMMARY_UI_PROOF'] === '1') {
  test('screen analysis route renders service-backed summary evidence without raw pixels', async ({ page }) => {
    const browserFailures = collectBrowserFailures(page);

    await refreshScreenReadModel(page);
    await page.goto('/#/screen-analysis');

    const screenRegion = page.getByRole('region', { name: 'Screen analysis' });
    await expect(screenRegion).toBeVisible({ timeout: shellReadyTimeoutMs });
    await expect(screenRegion.getByRole('heading', { name: 'Screen analysis' })).toBeVisible();
    await expect(screenRegion.getByText(productBoundaryCopy).first()).toBeVisible();
    await expect(screenRegion.getByText('Status').first()).toBeVisible();
    await expect(screenRegion.getByText('Product claim').first()).toBeVisible();
    await expect(screenRegion.getByText('Parent explanation refs').first()).toBeVisible();
    await expect(screenRegion.getByText('screen-summary-parent-explanation-service-explanation').first()).toBeVisible();
    await expect(screenRegion.getByText('Raw screenshot')).toHaveCount(0);
    await expect(screenRegion.getByText('Product ready')).toHaveCount(0);

    await captureScreenSummaryScreenshots(page, screenRegion);
    await writeAccessibilitySummary(await collectAccessibilitySummary(page));

    expect(browserFailures).toEqual([]);
  });
}

async function refreshScreenReadModel(page: Page): Promise<void> {
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  const command = page.getByRole('button', { exact: true, name: 'Refresh activity screen' });
  await expect(command).toBeEnabled({ timeout: shellReadyTimeoutMs });
  await command.click();
  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText('agent.activity.screen.read-model.reported')).toHaveCount(1, {
    timeout: shellReadyTimeoutMs,
  });
}

async function captureScreenSummaryScreenshots(page: Page, screenRegion: Locator): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(screenRegion).toBeVisible();
  await page.screenshot({ fullPage: true, path: mobileScreenshotPath });
}

async function collectAccessibilitySummary(page: Page): Promise<{
  readonly hasNamedRegion: boolean;
  readonly headings: readonly string[];
  readonly labels: readonly string[];
  readonly values: readonly string[];
}> {
  return page.evaluate(() => {
    const region = document.querySelector('[aria-label="Screen analysis"]');
    const text = (element: Element): string => element.textContent?.trim() ?? '';
    return {
      hasNamedRegion: region !== null,
      headings: Array.from(region?.querySelectorAll('h2') ?? []).map(text),
      labels: Array.from(region?.querySelectorAll('dt') ?? []).map(text),
      values: Array.from(region?.querySelectorAll('dd') ?? []).map(text),
    };
  });
}

async function writeAccessibilitySummary(
  summary: Awaited<ReturnType<typeof collectAccessibilitySummary>>
): Promise<void> {
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.headings).toContain('Screen analysis');
  expect(summary.labels).toContain('Status');
  expect(summary.labels).toContain('Product claim');
  expect(summary.labels).toContain('Parent explanation refs');
  expect(summary.values).toContain('screen-summary-parent-explanation-service-explanation');
  expect(summary.values).toContain(productBoundaryCopy);

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/screen-analysis',
        assertions: [
          'named-region',
          'visible-heading',
          'service-backed-screen-read-model-command',
          'screen-summary-route-visible',
          'parent-explanation-refs-visible',
          'product-claim-boundary-visible',
          'raw-screenshot-not-rendered',
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
