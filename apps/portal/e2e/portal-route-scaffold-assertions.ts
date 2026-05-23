import { expect, type Page } from '@playwright/test';

const productRoutes = [
  { path: '/#/overview', nav: 'OVERVIEW', title: 'TODAY CONTROL SNAPSHOT', kind: 'control' },
  { path: '/#/activity', nav: 'ACTIVITY', title: 'PARENT CONTROL DETAIL', kind: 'control' },
  { path: '/#/browser', nav: 'WEB', title: 'MANAGED WEB CONTROL DETAIL', kind: 'control' },
  { path: '/#/policy', nav: 'RULES', title: 'RULES AND POLICY GUIDE', kind: 'guide' },
  { path: '/#/privacy-design', nav: 'PRIVATE', title: 'PRIVACY AND DATA GUIDE', kind: 'guide' },
  { path: '/#/memory', nav: 'MEMORY', title: 'CITED MEMORY GUIDE', kind: 'guide' },
  { path: '/#/ai-runtime', nav: 'AI SETUP', title: 'LOCAL AI AND MEMORY READINESS', kind: 'control' },
  { path: '/#/devices', nav: 'DEVICES', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
  { path: '/#/notifications', nav: 'ALERTS', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
  { path: '/#/drive-connections', nav: 'DRIVES', title: 'SUPPORT EXPORTS AND DRIVE CONNECTIONS', kind: 'control' },
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT EXPORTS AND DRIVE CONNECTIONS', kind: 'control' },
  { path: '/#/settings-rules', nav: 'SETTINGS', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
] as const;

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRoute(page, route.path, route.nav, route.title, route.kind);
  }
  await assertChatDock(page);
  await assertFrameTunerRoute(page);
}

async function assertProductRoute(
  page: Page,
  path: string,
  navLabel: string,
  panelTitle: string,
  kind: 'control' | 'guide'
): Promise<void> {
  await page.goto(path);
  const surface = page.locator('svg.leaderboard-page-svg-surface');
  await expect(surface).toBeVisible();
  await expect(surface).toHaveAttribute('aria-label', 'Ocentra parent dashboard');
  await expect(surface.locator('text').filter({ hasText: navLabel }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: panelTitle }).first()).toBeVisible();
  if (kind === 'guide') {
    await expect(page.getByRole('button', { name: 'Show QUICK READ' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Show QUICK ACTION' })).toBeVisible();
    return;
  }
  await expect(surface.locator('text').filter({ hasText: 'WHAT PARENTS CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'DATA CUSTODY' }).first()).toBeVisible();
}

async function assertChatDock(page: Page): Promise<void> {
  await page.goto('/#/overview');
  await page.getByRole('button', { name: 'Open parent chat' }).click();
  await expect(
    page
      .locator('svg.leaderboard-page-svg-surface')
      .locator('text')
      .filter({ hasText: 'PARENT ASSISTANT CHAT' })
      .first()
  ).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open AI setup' })).toBeVisible();
  await page.getByRole('button', { name: 'Open AI setup' }).click();
  await expect(page).toHaveURL('/#/ai-runtime');
  await expect(
    page.locator('svg.leaderboard-page-svg-surface').locator('text').filter({ hasText: 'AI SETUP' }).first()
  ).toBeVisible();
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
