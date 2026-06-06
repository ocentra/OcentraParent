import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.skip(process.env['SOCIAL_DASHBOARD_UI_PROOF'] !== '1', 'Dedicated social dashboard UI proof only.');
test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'browser-plan-proof', 'social-20-parent-social-dashboard-ux');
const screenshotDir = path.join(proofRoot, '06-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'social-dashboard-browser-route.png');
const mobileScreenshotPath = path.join(screenshotDir, 'social-dashboard-browser-route-mobile.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'social-dashboard-ui-proof',
  'accessibility-summary.json'
);

test('browser route renders honest social dashboard unavailable state', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await assertSocialDashboardRoute(page);
  await captureSocialDashboardScreenshots(page);
  await writeAccessibilitySummary(await collectAccessibilitySummary(page));

  expect(browserFailures).toEqual([]);
});

async function assertSocialDashboardRoute(page: Page): Promise<void> {
  await page.goto('/#/browser');
  const socialRegion = page.getByRole('region', { name: 'Social review' });
  await expect(socialRegion).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(socialRegion.getByRole('heading', { name: 'Social review' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: '0 social dashboard rows' })).toBeVisible();
  await expect(
    socialRegion.getByRole('heading', { name: 'No social dashboard snapshot has been reported yet.' })
  ).toBeVisible();
  await expect(socialRegion.getByText('unavailable', { exact: true }).first()).toBeVisible();
  await expect(socialRegion.getByText('not reported', { exact: true }).first()).toBeVisible();
  await expect(socialRegion.getByText('Rendered parent surface only').first()).toBeVisible();
  await expect(socialRegion.getByText('runtime fetch, connector, native app').first()).toBeVisible();

  const routeText = await socialRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:product ready|policy execution proved|enforcement active)/iu);
  expect(routeText ?? '').not.toMatch(/(?:connector authorized|native app controlled|notification delivered)/iu);
}

async function captureSocialDashboardScreenshots(page: Page): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Social review' })).toBeVisible();
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
    const region = document.querySelector('[aria-label="Social review"]');
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
  expect(summary.hasNamedRegion).toBe(true);
  expect(summary.unlabeledButtons).toBe(0);
  expect(summary.headings).toContain('Social review');
  expect(summary.headings).toContain('0 social dashboard rows');
  expect(summary.headings).toContain('No social dashboard snapshot has been reported yet.');
  expect(summary.labels).toContain('Rows returned');
  expect(summary.labels).toContain('Status');
  expect(summary.labels).toContain('Product claim');
  expect(summary.values).toContain('0');
  expect(summary.values).toContain('not reported');
  expect(summary.values).toContain('unavailable');

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/browser',
        assertions: [
          'named-region',
          'visible-social-review-heading',
          'zero-row-summary-visible',
          'unavailable-state-visible',
          'no-service-backed-social-snapshot-visible',
          'runtime-social-claims-not-visible',
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
