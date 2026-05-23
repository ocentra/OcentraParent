import { expect, type Page } from '@playwright/test';

const productRoutes = [
  { path: '/#/overview', nav: 'TODAY', title: 'TODAY CONTROL SNAPSHOT' },
  { path: '/#/activity', nav: 'TODAY', title: 'TODAY CONTROL SNAPSHOT' },
  { path: '/#/browser', nav: 'BROWSERS', title: 'MANAGED WEB CONTROL DETAIL' },
  { path: '/#/policy', nav: 'RULES', title: 'POLICY ACTION CONTROL DETAIL' },
  { path: '/#/privacy-design', nav: 'PRIVATE', title: 'LOCAL AI AND MEMORY READINESS' },
  { path: '/#/memory', nav: 'MEMORY', title: 'LOCAL AI AND MEMORY READINESS' },
  { path: '/#/ai-runtime', nav: 'LOCAL AI', title: 'LOCAL AI AND MEMORY READINESS' },
  { path: '/#/devices', nav: 'DEVICES', title: 'DEVICE ROUTINE AND APPROVALS' },
  { path: '/#/notifications', nav: 'ALERTS', title: 'DEVICE ROUTINE AND APPROVALS' },
  { path: '/#/drive-connections', nav: 'DRIVES', title: 'SUPPORT EXPORTS AND DRIVE CONNECTIONS' },
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT EXPORTS AND DRIVE CONNECTIONS' },
  { path: '/#/settings-rules', nav: 'SETTINGS', title: 'DEVICE ROUTINE AND APPROVALS' },
] as const;

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRoute(page, route.path, route.nav, route.title);
  }
  await assertFrameTunerRoute(page);
}

async function assertProductRoute(page: Page, path: string, navLabel: string, panelTitle: string): Promise<void> {
  await page.goto(path);
  const surface = page.locator('svg.leaderboard-page-svg-surface');
  await expect(surface).toBeVisible();
  await expect(surface).toHaveAttribute('aria-label', 'Ocentra parent dashboard');
  await expect(surface.locator('text').filter({ hasText: navLabel }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: panelTitle }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'WHAT PARENTS CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'DATA CUSTODY' }).first()).toBeVisible();
}

async function assertFrameTunerRoute(page: Page): Promise<void> {
  await page.goto('/#/frame-tuner');
  await expect(page.getByRole('heading', { name: 'Frame tuner' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save JSON' })).toBeVisible();
  await expect(page.locator('.app-sidebar')).toHaveCount(0);
  await expect(page.locator('svg.portal-frame-backdrop-svg')).toHaveCount(0);
  await page.getByRole('tab', { name: 'Side panel' }).click();
  await expect(page.getByRole('tab', { name: 'Top' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Bottom' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Reset top' })).toBeVisible();
  await page.getByRole('tab', { name: 'Bottom' }).click();
  await expect(page.getByRole('button', { name: 'Reset bottom' })).toBeVisible();
  await page.getByRole('tab', { name: 'Main panel' }).click();
  await expect(page.getByRole('button', { name: 'Reset main' })).toBeVisible();
  await page.getByRole('tab', { name: 'Golden card' }).click();
  await expect(page.getByRole('button', { name: 'Reset golden card' })).toBeVisible();
  await expect(page.getByText('Golden card frame')).toBeVisible();
  await expect(page.getByText('Golden card content')).toBeVisible();
  await page.getByRole('tab', { name: 'Save and JSON' }).click();
  await expect(page.getByText('Saved JSON preview')).toBeVisible();
}
