import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'browser-plan-proof', 'ai-20-parent-explanation-audit-ux');
const screenshotDir = path.join(proofRoot, '06-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'browser-parent-explanation-route.png');
const mobileScreenshotPath = path.join(screenshotDir, 'browser-parent-explanation-route-mobile.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'browser-ai-parent-explanation-rendered-proof',
  'accessibility-summary.json'
);

if (process.env['BROWSER_PARENT_EXPLANATION_UI_PROOF'] === '1') {
  test('browser route renders evidence-backed parent explanation bundle', async ({ page }) => {
    const browserFailures = collectBrowserFailures(page);

    await assertParentExplanationRoute(page);
    await captureParentExplanationScreenshots(page);
    await writeAccessibilitySummary(await collectAccessibilitySummary(page));

    expect(browserFailures).toEqual([]);
  });
}

async function assertParentExplanationRoute(page: Page): Promise<void> {
  await page.goto('/#/browser');
  const explanationRegion = page.getByRole('region', { name: 'Browser review' });
  await expect(explanationRegion).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(explanationRegion.getByRole('heading', { name: 'Browser review' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: '10 parent explanation rows' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'What happened' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'AI and model details' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Action taken' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Audit trail' })).toBeVisible();
  await expect(explanationRegion.getByText('browser-evidence-live-youtube-cdp', { exact: true }).first()).toBeVisible();
  await expect(explanationRegion.getByText('child-agent-endpoint-proof-warning').first()).toBeVisible();
  await expect(explanationRegion.getByText('Rendered parent explanation surface only').first()).toBeVisible();

  const routeText = await explanationRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:raw page content|raw prompt text|direct enforcement active)/iu);
  expect(routeText ?? '').not.toContain('https://www.youtube.com/watch?v=');
}

async function captureParentExplanationScreenshots(page: Page): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Browser review' })).toBeVisible();
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
    const region = document.querySelector('[aria-label="Browser review"]');
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
  expect(summary.headings).toContain('Browser review');
  expect(summary.headings).toContain('10 parent explanation rows');
  expect(summary.headings).toContain('What happened');
  expect(summary.headings).toContain('AI and model details');
  expect(summary.headings).toContain('Action taken');
  expect(summary.headings).toContain('Audit trail');
  expect(summary.labels).toContain('Rows returned');
  expect(summary.labels).toContain('Browser evidence');
  expect(summary.labels).toContain('Model');
  expect(summary.labels).toContain('Policy evaluation');
  expect(summary.labels).toContain('Intervention audit ID');
  expect(summary.values).toContain('10');
  expect(summary.values).toContain('ready');
  expect(summary.values).toContain('warn');

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/browser',
        assertions: [
          'named-browser-review-region',
          'schema-backed-parent-explanation-visible',
          'model-policy-action-audit-headings-visible',
          'live-ai19-evidence-ref-visible',
          'raw-youtube-url-not-rendered',
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
