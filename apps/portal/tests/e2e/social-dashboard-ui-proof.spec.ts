import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

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

if (process.env['SOCIAL_DASHBOARD_UI_PROOF'] === '1') {
  test('browser route renders service-backed honest social dashboard rows', async ({ page }) => {
    const browserFailures = collectBrowserFailures(page);

    await assertSocialDashboardRoute(page);
    await requestSocialDashboardReadModel(page);
    await assertServiceBackedSocialRows(page);
    await captureSocialDashboardScreenshots(page);
    await writeAccessibilitySummary(await collectAccessibilitySummary(page));

    expect(browserFailures).toEqual([]);
  });
}

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

async function requestSocialDashboardReadModel(page: Page): Promise<void> {
  const socialRegion = page.getByRole('region', { name: 'Social review' });
  await socialRegion.getByRole('button', { name: 'Social review' }).click();
  await expect(socialRegion.getByRole('heading', { name: '7 social dashboard rows' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function assertServiceBackedSocialRows(page: Page): Promise<void> {
  const socialRegion = page.getByRole('region', { name: 'Social review' });
  await expect(socialRegion.getByRole('heading', { name: 'Account approvals' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: 'Feed and video route gates' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: 'Native app capability' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: 'Connected account boundaries' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: 'Remembered decisions' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: 'Settings and custody' })).toBeVisible();
  await expect(socialRegion.getByRole('heading', { name: 'Needs manual proof' })).toBeVisible();
  await expect(socialRegion.getByText('Ready for parent review').first()).toBeVisible();
  await expect(socialRegion.getByText('Manual proof required').first()).toBeVisible();
  await expect(socialRegion.getByText('Contract proof only').first()).toBeVisible();
  await expect(socialRegion.getByText('social-13-managed-browser-account-creation-gate').first()).toBeVisible();
  await expect(socialRegion.getByText('social-14-managed-browser-feed-video-route-gate').first()).toBeVisible();

  const routeText = await socialRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:policy execution proved|enforcement active|native app controlled)/iu);
  expect(routeText ?? '').not.toMatch(/(?:connector authorized|notification delivered|runtime feed fetched)/iu);
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
  expect(summary.headings).toContain('7 social dashboard rows');
  expect(summary.headings).toContain('Account approvals');
  expect(summary.headings).toContain('Feed and video route gates');
  expect(summary.headings).toContain('Native app capability');
  expect(summary.headings).toContain('Connected account boundaries');
  expect(summary.headings).toContain('Remembered decisions');
  expect(summary.headings).toContain('Settings and custody');
  expect(summary.headings).toContain('Needs manual proof');
  expect(summary.labels).toContain('Rows returned');
  expect(summary.labels).toContain('Generated at');
  expect(summary.labels).toContain('Status');
  expect(summary.labels).toContain('Product claim');
  expect(summary.values).toContain('7');
  expect(summary.values).toContain('Ready for parent review');
  expect(summary.values).toContain('Manual proof required');
  expect(summary.values).toContain('Contract proof only');

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/browser',
        assertions: [
          'named-region',
          'visible-social-review-heading',
          'zero-row-before-command-visible',
          'service-backed-seven-row-summary-visible',
          'account-approval-row-visible',
          'feed-video-gate-row-visible',
          'native-app-manual-required-visible',
          'connector-boundary-manual-required-visible',
          'decision-memory-contract-only-visible',
          'settings-custody-manual-required-visible',
          'manual-required-gap-visible',
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
