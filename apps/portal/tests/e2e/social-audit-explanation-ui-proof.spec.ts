import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { expect, test, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';

test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 90_000;
const repoRoot = path.resolve(process.cwd(), '..', '..');
const proofRoot = path.join(repoRoot, 'output', 'browser-plan-proof', 'social-22-audit-explanation-read-model');
const screenshotDir = path.join(proofRoot, '06-ui-snapshots');
const desktopScreenshotPath = path.join(screenshotDir, 'social-audit-explanation-route.png');
const mobileScreenshotPath = path.join(screenshotDir, 'social-audit-explanation-route-mobile.png');
const accessibilitySummaryPath = path.join(
  repoRoot,
  'test-results',
  'social-audit-explanation-ui-proof',
  'accessibility-summary.json'
);

if (process.env['SOCIAL_AUDIT_EXPLANATION_UI_PROOF'] === '1') {
  test('browser route renders schema-backed social audit explanations', async ({ page }) => {
    const browserFailures = collectBrowserFailures(page);

    await assertSocialAuditExplanationRoute(page);
    await captureSocialAuditExplanationScreenshots(page);
    await writeAccessibilitySummary(await collectAccessibilitySummary(page));

    expect(browserFailures).toEqual([]);
  });
}

async function assertSocialAuditExplanationRoute(page: Page): Promise<void> {
  await page.goto('/#/browser?agent.browser.social-audit-explanation.read-model.reported');
  const explanationRegion = page.getByRole('region', { name: 'Social explanations' });
  await expect(explanationRegion).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(explanationRegion.getByRole('heading', { name: 'Social explanations' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: '6 social explanation rows' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Account approval explanation' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Feed and video gate explanation' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Native app gap explanation' })).toBeVisible();
  await expect(
    explanationRegion.getByRole('heading', { name: 'Connected account boundary explanation' })
  ).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Remembered decision explanation' })).toBeVisible();
  await expect(explanationRegion.getByRole('heading', { name: 'Manual proof gap explanation' })).toBeVisible();
  await expect(explanationRegion.getByText('route-evidence:parent-evidence-route-evidence').first()).toBeVisible();
  await expect(explanationRegion.getByText('parent-evidence-audit-ref-rendered').first()).toBeVisible();
  await expect(explanationRegion.getByText('Rendered parent explanation surface only').first()).toBeVisible();

  const routeText = await explanationRegion.textContent();
  expect(routeText ?? '').not.toMatch(/(?:connector authorized|native app controlled|notification delivered)/iu);
  expect(routeText ?? '').not.toMatch(/(?:final policy execution proved|enforcement active|raw message content)/iu);
}

async function captureSocialAuditExplanationScreenshots(page: Page): Promise<void> {
  await mkdir(screenshotDir, { recursive: true });
  await page.screenshot({ fullPage: true, path: desktopScreenshotPath });
  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.getByRole('region', { name: 'Social explanations' })).toBeVisible();
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
    const region = document.querySelector('[aria-label="Social explanations"]');
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
  expect(summary.headings).toContain('Social explanations');
  expect(summary.headings).toContain('6 social explanation rows');
  expect(summary.headings).toContain('Account approval explanation');
  expect(summary.headings).toContain('Feed and video gate explanation');
  expect(summary.labels).toContain('Rows returned');
  expect(summary.labels).toContain('Generated at');
  expect(summary.labels).toContain('Decision action');
  expect(summary.labels).toContain('Intervention audit ID');
  expect(summary.values).toContain('6');
  expect(summary.values).toContain('warn-candidate');
  expect(summary.values).toContain('manual-review-candidate');

  await mkdir(path.dirname(accessibilitySummaryPath), { recursive: true });
  await writeFile(
    accessibilitySummaryPath,
    `${JSON.stringify(
      {
        route: '#/browser',
        assertions: [
          'named-social-explanations-region',
          'schema-backed-social-explanation-visible',
          'six-subject-explanation-headings-visible',
          'evidence-and-audit-refs-visible',
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
