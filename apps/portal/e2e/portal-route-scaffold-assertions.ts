import { expect, type Page } from '@playwright/test';
import { PARENT_PORTAL_NAV_LABELS } from '@ocentra-parent/portal-domain/contracts';

const productRoutes = [
  { path: '/#/start', nav: 'START HERE', title: 'START HERE', kind: 'guideDashboard' },
  { path: '/#/overview', nav: 'OVERVIEW', title: 'TODAY CONTROL SNAPSHOT', kind: 'control' },
  { path: '/#/activity', nav: 'ACTIVITY', title: 'PARENT CONTROL DETAIL', kind: 'control' },
  { path: '/#/browser', nav: 'WEB', title: 'MANAGED WEB CONTROL DETAIL', kind: 'control' },
  {
    path: '/#/browser-settings',
    nav: PARENT_PORTAL_NAV_LABELS.Browser,
    title: 'BROWSER SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/policy', nav: 'RULES', title: 'RULES AND POLICY GUIDE', kind: 'guide' },
  {
    path: '/#/rule-management',
    nav: PARENT_PORTAL_NAV_LABELS.RuleSet,
    title: 'RULE SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/schedules', nav: 'SCHEDULES', title: 'SCHEDULES CONTROL DETAIL', kind: 'manage' },
  { path: '/#/approvals', nav: 'APPROVALS', title: 'APPROVALS CONTROL DETAIL', kind: 'manage' },
  { path: '/#/enforcement', nav: 'ENFORCE', title: 'ENFORCEMENT CONTROL DETAIL', kind: 'manage' },
  { path: '/#/privacy-design', nav: 'PRIVATE', title: 'PRIVACY AND DATA GUIDE', kind: 'guide' },
  { path: '/#/memory', nav: 'MEMORY', title: 'CITED MEMORY GUIDE', kind: 'guide' },
  {
    path: '/#/memory-settings',
    nav: PARENT_PORTAL_NAV_LABELS.MemorySet,
    title: 'MEMORY SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/ai-runtime', nav: 'AI SETUP', title: 'AI SETUP CONTROL DETAIL', kind: 'manage' },
  { path: '/#/api-providers', nav: 'API KEYS', title: 'API PROVIDERS CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/report-settings',
    nav: PARENT_PORTAL_NAV_LABELS.ReportSet,
    title: 'REPORT SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/screen-analysis', nav: 'SCREEN', title: 'SCREEN ANALYSIS CONTROL DETAIL', kind: 'manage' },
  { path: '/#/app-game-sessions', nav: 'APPS/GAMES', title: 'APPS AND GAMES CONTROL DETAIL', kind: 'manage' },
  { path: '/#/network-activity', nav: 'NETWORK', title: 'NETWORK ACTIVITY CONTROL DETAIL', kind: 'manage' },
  { path: '/#/devices', nav: 'DEVICES', title: 'DEVICES CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/lan-pairing',
    nav: PARENT_PORTAL_NAV_LABELS.Lan,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/capability-status', nav: 'CAPABILITY', title: 'CAPABILITY STATUS CONTROL DETAIL', kind: 'manage' },
  { path: '/#/notifications', nav: 'ALERTS', title: 'ALERTS CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/notification-channels',
    nav: 'CHANNELS',
    title: 'NOTIFICATION CHANNELS CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/drive-connections', nav: 'DRIVES', title: 'DRIVES CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/export-retention',
    nav: PARENT_PORTAL_NAV_LABELS.Export,
    title: 'EXPORT DELETE RETENTION CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/remote-access', nav: 'REMOTE', title: 'REMOTE ACCESS CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/report-compiler',
    nav: PARENT_PORTAL_NAV_LABELS.Builder,
    title: 'REPORT COMPILER CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/audit-history', nav: 'AUDIT', title: 'AUDIT HISTORY CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/subscription',
    nav: PARENT_PORTAL_NAV_LABELS.Plan,
    title: 'SUBSCRIPTION CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/entitlements',
    nav: PARENT_PORTAL_NAV_LABELS.Access,
    title: 'ENTITLEMENTS CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/platforms-install', nav: 'PLATFORMS', title: 'PLATFORMS CONTROL DETAIL', kind: 'manage' },
  { path: '/#/install-updates', nav: 'UPDATES', title: 'INSTALL UPDATES CONTROL DETAIL', kind: 'manage' },
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT CONTROL DETAIL', kind: 'manage' },
  { path: '/#/settings-rules', nav: 'SETTINGS', title: 'FAMILY SETTINGS CONTROL DETAIL', kind: 'manage' },
] as const;

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRoute(page, route.path, route.nav, route.title, route.kind);
  }
  await assertSidePanelFoldouts(page);
  await assertMockAssistantRemoved(page);
  await assertFrameTunerRoute(page);
}

async function assertProductRoute(
  page: Page,
  path: string,
  navLabel: string,
  panelTitle: string,
  kind: 'control' | 'guide' | 'guideDashboard' | 'manage'
): Promise<void> {
  await page.goto(path);
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible();
  await expect(page.getByRole('img', { name: 'Ocentra parent dashboard' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: navLabel }).first()).toBeVisible();
  if (kind === 'manage') {
    await expect(
      surface
        .locator('text')
        .filter({ hasText: /(?:SETTING|COMMAND) MODE/ })
        .first()
    ).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'SETTING CHOICES' }).first()).toBeVisible();
    return;
  }
  await expect(surface.locator('text').filter({ hasText: panelTitle }).first()).toBeVisible();
  if (kind === 'guideDashboard') {
    await expect(page.getByRole('button', { name: 'Open Set Up Ocentra Parent' })).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'GUIDES 17' }).first()).toBeVisible();
    return;
  }
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
  await clickSidePanelButton(page, 'Expand GUIDE');
  await expect(page.getByRole('button', { name: 'Collapse GUIDE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open START HERE' })).toBeVisible();
  await clickSidePanelButton(page, 'Collapse GUIDE');
  await expect(page.getByRole('button', { name: 'Expand GUIDE' })).toBeVisible();
  await clickSidePanelButton(page, 'Expand MANAGE');
  await expect(page.getByRole('button', { name: 'Collapse MANAGE' })).toBeVisible();
  await expect(page.getByRole('button', { name: `Collapse ${PARENT_PORTAL_NAV_LABELS.Policies}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Browser}` })).toBeVisible();
  await clickSidePanelButton(page, `Collapse ${PARENT_PORTAL_NAV_LABELS.Policies}`);
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Policies}` })).toBeVisible();
  await expect(
    page
      .locator('svg.parent-portal-svg-surface')
      .locator(`g[role="button"][aria-label="Open ${PARENT_PORTAL_NAV_LABELS.Browser}"]`)
  ).toHaveCount(0);
  await clickSidePanelButton(page, `Expand ${PARENT_PORTAL_NAV_LABELS.Policies}`);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Browser}` })).toBeVisible();
  await clickSidePanelButton(page, 'Collapse MANAGE');
  await expect(page.getByRole('button', { name: 'Expand MANAGE' })).toBeVisible();
}

async function clickSidePanelButton(page: Page, name: string): Promise<void> {
  const button = page.getByRole('button', { name });
  await expect(button).toBeVisible();
  await button.click({ force: true });
}

async function assertMockAssistantRemoved(page: Page): Promise<void> {
  await page.goto('/#/overview');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(page.getByRole('button', { name: 'Open AI assistant' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Close parent assistant' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Send message to MIA' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: /^Ask MIA/ })).toHaveCount(0);
  await expect(page.getByRole('button', { name: /Copy (?:MIA|YOU) message/ })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'AI ASSISTANT' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'MIA' })).toHaveCount(0);
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
