import { expect, type Page } from '@playwright/test';

const productRoutes = [
  { path: '/#/overview', nav: 'OVERVIEW', title: 'TODAY CONTROL SNAPSHOT', kind: 'control' },
  { path: '/#/activity', nav: 'ACTIVITY', title: 'PARENT CONTROL DETAIL', kind: 'control' },
  { path: '/#/browser', nav: 'WEB', title: 'MANAGED WEB CONTROL DETAIL', kind: 'control' },
  { path: '/#/browser-settings', nav: 'BROWSER SETUP', title: 'BROWSER SETUP CONTROL DETAIL', kind: 'control' },
  { path: '/#/policy', nav: 'RULES', title: 'RULES AND POLICY GUIDE', kind: 'guide' },
  { path: '/#/rule-management', nav: 'RULE SETUP', title: 'RULE SETUP CONTROL DETAIL', kind: 'control' },
  { path: '/#/schedules', nav: 'SCHEDULES', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
  { path: '/#/approvals', nav: 'APPROVALS', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
  { path: '/#/enforcement', nav: 'ENFORCE', title: 'ENFORCEMENT CONTROL DETAIL', kind: 'control' },
  { path: '/#/privacy-design', nav: 'PRIVATE', title: 'PRIVACY AND DATA GUIDE', kind: 'guide' },
  { path: '/#/memory', nav: 'MEMORY', title: 'CITED MEMORY GUIDE', kind: 'guide' },
  { path: '/#/memory-settings', nav: 'MEMORY SETUP', title: 'LOCAL AI AND MEMORY READINESS', kind: 'control' },
  { path: '/#/ai-runtime', nav: 'AI SETUP', title: 'LOCAL AI AND MEMORY READINESS', kind: 'control' },
  { path: '/#/api-providers', nav: 'API KEYS', title: 'LOCAL AI AND MEMORY READINESS', kind: 'control' },
  { path: '/#/report-settings', nav: 'REPORT SETUP', title: 'REPORT SETUP CONTROL DETAIL', kind: 'control' },
  { path: '/#/screen-analysis', nav: 'SCREEN', title: 'SCREEN ANALYSIS CONTROL DETAIL', kind: 'control' },
  { path: '/#/app-game-sessions', nav: 'APPS/GAMES', title: 'APP AND GAME SESSIONS CONTROL DETAIL', kind: 'control' },
  { path: '/#/network-activity', nav: 'NETWORK', title: 'NETWORK ACTIVITY CONTROL DETAIL', kind: 'control' },
  { path: '/#/devices', nav: 'DEVICES', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
  { path: '/#/notifications', nav: 'ALERTS', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
  { path: '/#/drive-connections', nav: 'DRIVES', title: 'DRIVE EXPORTS CONTROL DETAIL', kind: 'control' },
  { path: '/#/remote-access', nav: 'REMOTE', title: 'REMOTE ACCESS CONTROL DETAIL', kind: 'control' },
  { path: '/#/subscription', nav: 'SUBSCRIPTION', title: 'SUBSCRIPTION CONTROL DETAIL', kind: 'control' },
  { path: '/#/platforms-install', nav: 'PLATFORMS', title: 'PLATFORMS CONTROL DETAIL', kind: 'control' },
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT AND API STATUS CONTROL DETAIL', kind: 'control' },
  { path: '/#/settings-rules', nav: 'SETTINGS', title: 'DEVICE ROUTINE AND APPROVALS', kind: 'control' },
] as const;

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRoute(page, route.path, route.nav, route.title, route.kind);
  }
  await assertSidePanelFoldouts(page);
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

async function assertSidePanelFoldouts(page: Page): Promise<void> {
  await page.goto('/#/overview');
  await page.getByRole('button', { name: 'Expand GUIDE' }).click();
  await expect(page.getByRole('button', { name: 'Collapse GUIDE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open START HERE' })).toBeVisible();
  await page.getByRole('button', { name: 'Collapse GUIDE' }).click();
  await expect(page.getByRole('button', { name: 'Expand GUIDE' })).toBeVisible();
  await page.getByRole('button', { name: 'Expand MANAGE' }).click();
  await expect(page.getByRole('button', { name: 'Collapse MANAGE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open DEVICES' })).toBeVisible();
  await page.getByRole('button', { name: 'Collapse MANAGE' }).click();
  await expect(page.getByRole('button', { name: 'Expand MANAGE' })).toBeVisible();
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
  await expect(page.getByRole('button', { name: 'Open AI setup', exact: true })).toBeVisible();
  await page.getByRole('button', { name: 'Open AI setup', exact: true }).click();
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
