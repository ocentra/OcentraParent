import { expect, type Locator, type Page } from '@playwright/test';
import { PARENT_PORTAL_NAV_LABELS } from '@ocentra-parent/portal-domain/parent-portal-nav';
import {
  assertAssistantEntryAvailable,
  assertDuplicateLabelSidePanelRoutes,
  assertFrameTunerRoute,
  assertManageTargetSelectorSemantics,
  assertPolicyGuideDeepLinks,
  assertSidePanelFoldouts,
  assertDiagnosticsRoute,
} from './portal-route-scaffold-common';
import { assertLanRouteSurface } from './portal-route-scaffold-lan';
import { assertProductRouteSurface } from './portal-route-scaffold-product';

const productRoutes = [
  { path: '/#/start', nav: 'START HERE', title: 'START HERE', kind: 'guideDashboard' },
  { path: '/#/overview', nav: 'OVERVIEW', title: 'Current device state', kind: 'control' },
  { path: '/#/assistant', nav: 'AI ASSISTANT', title: 'Ask MIA about', kind: 'assistant' },
  { path: '/#/activity', nav: 'ACTIVITY', title: 'REPORTS CONTROL DETAIL', kind: 'activityManage' },
  { path: '/#/browser', nav: 'WEB', title: 'Browser activity status', kind: 'browserActivity' },
  {
    path: '/#/browser-settings',
    nav: PARENT_PORTAL_NAV_LABELS.Browser,
    title: 'BROWSER SETUP CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/policy', nav: 'RULES', title: 'Rules', kind: 'guide' },
  {
    path: '/#/rule-management',
    nav: PARENT_PORTAL_NAV_LABELS.RuleSet,
    title: 'RULES CONTROL DETAIL',
    kind: 'manage',
  },
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
  { path: '/#/ai-guide', nav: 'AI', title: 'AI AND EVIDENCE GUIDE', kind: 'guide' },
  { path: '/#/ai-runtime', nav: 'AI SETUP', title: 'AI SETUP CONTROL DETAIL', kind: 'manage' },
  { path: '/#/api-providers', nav: 'API KEYS', title: 'API PROVIDERS CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/reports-guide',
    nav: PARENT_PORTAL_NAV_LABELS.ReportsGuide,
    title: 'REPORTS AND SUMMARIES GUIDE',
    kind: 'guide',
  },
  {
    path: '/#/screen-analysis',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'SCREEN ANALYSIS CONTROL DETAIL',
    kind: 'activityManage',
  },
  {
    path: '/#/app-game-sessions',
    nav: PARENT_PORTAL_NAV_LABELS.AppsGames,
    title: 'APP/GAME CONTROL DETAIL',
    kind: 'activityManage',
  },
  {
    path: '/#/network-activity',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'NETWORK ACTIVITY CONTROL DETAIL',
    kind: 'networkActivity',
  },
  {
    path: '/#/devices',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'lanPairing',
  },
  {
    path: '/#/lan-pairing',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'lanPairing',
  },
  {
    path: '/#/capability-status',
    nav: PARENT_PORTAL_NAV_LABELS.Capability,
    title: 'Capability status',
    kind: 'capabilityStatus',
  },
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
  {
    path: '/#/remote-access',
    nav: PARENT_PORTAL_NAV_LABELS.Remote,
    title: 'REMOTE ACCESS CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/report-compiler',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'REPORT COMPILER CONTROL DETAIL',
    kind: 'activityManage',
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
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT CONTROL DETAIL', kind: 'manage' },
  {
    path: '/#/policy-screen',
    nav: PARENT_PORTAL_NAV_LABELS.Screen,
    title: 'Screen analysis settings',
    kind: 'screenSettings',
  },
  {
    path: '/#/settings-rules',
    nav: PARENT_PORTAL_NAV_LABELS.Portal,
    title: 'SETTINGS CONTROL DETAIL',
    kind: 'manage',
  },
] as const;

const lanRelevantScaffoldPaths = new Set([
  '/#/activity',
  '/#/browser',
  '/#/browser-settings',
  '/#/network-activity',
  '/#/devices',
  '/#/lan-pairing',
  '/#/capability-status',
]);

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRouteSurface(page, route.path, route.nav, route.title, route.kind);
  }
  await assertNotificationRoutesUnavailable(page);
  await assertDesktopDistributionRoutes(page);
  await assertScheduleRouteUnavailable(page);
  await assertSidePanelFoldouts(page);
  await assertDuplicateLabelSidePanelRoutes(page);
  await assertPolicyGuideDeepLinks(page);
  await assertManageTargetSelectorSemantics(page);
  await assertDiagnosticsRoute(page);
  await assertAssistantEntryAvailable(page);
  await assertFrameTunerRoute(page);
}

async function assertDesktopDistributionRoutes(page: Page): Promise<void> {
  const routes = [
    {
      path: '/#/platforms-install',
      region: 'Platforms and install status',
      expected: [
        'Desktop package',
        'built portal dist',
        'signing manual required',
        'Source and custody',
        'rust parent runtime',
        'source custody manual required',
      ],
    },
    {
      path: '/#/install-updates',
      region: 'Install and update status',
      expected: [
        'Update channel',
        'update channel scaffold',
        'rollback unavailable',
        'no installer updater rollback signing notarization store execution',
      ],
    },
  ] as const;

  for (const route of routes) {
    await page.goto(route.path);
    const panel = page.getByRole('region', { name: route.region });
    await expect(panel).toBeVisible();
    await expect(panel).toHaveAttribute('data-ocentra-desktop-distribution-actions', 'unavailable');
    for (const expected of route.expected) {
      await expect(panel).toContainText(expected);
    }
    await expect(page.locator('svg.parent-portal-svg-surface')).toHaveCount(0);
    await expect(page.getByRole('button', { name: /Validate|Apply|Revert|Install|Update|Rollback/u })).toHaveCount(0);
  }
}

async function assertNotificationRoutesUnavailable(page: Page): Promise<void> {
  const routeExpectations = [
    {
      path: '/#/notifications',
      expected: 'No service-reported notification intent, preference, or delivery state is available.',
    },
    {
      path: '/#/notification-channels',
      expected: 'No verified parent-owned notification channel registry or delivery receipt is available.',
    },
  ] as const;

  for (const route of routeExpectations) {
    await page.goto(route.path);
    const surface = page.locator('svg.parent-portal-svg-surface');
    await expect(surface).toBeVisible();
    await expect(surface).toContainText(route.expected);
    await expect(surface.locator('text').filter({ hasText: 'Enabled intent' })).toHaveCount(0);
    await expect(surface.locator('text').filter({ hasText: 'Configurable' })).toHaveCount(0);
    await expect(surface.locator('text').filter({ hasText: 'Local first' })).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Send test' })).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Verify channel' })).toHaveCount(0);
  }
}

export async function assertLanRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    if (!lanRelevantScaffoldPaths.has(route.path)) {
      continue;
    }
    await assertProductRouteSurface(page, route.path, route.nav, route.title, route.kind);
  }
  await assertScheduleRouteUnavailable(page);
  await assertLanRouteSurface(page);
}

export async function assertScheduleRouteUnavailable(page: Page): Promise<void> {
  await page.goto('/#/schedules');
  const panel = page.getByRole('region', { name: 'Schedules unavailable' });
  await expect(panel).toBeVisible();
  await expect(panel.getByRole('heading', { exact: true, name: 'Schedules unavailable' })).toBeVisible();
  await expect(panel).toContainText(
    'Ocentra has not received a current schedule or time-budget status from the local service, so schedule controls stay off instead of guessing.'
  );
  const availableAreas = panel.getByRole('navigation', { name: 'Available control areas' });
  await expect(availableAreas.getByRole('button', { name: 'Open rules' })).toBeVisible();
  await expect(availableAreas.getByRole('button', { name: 'Open approvals' })).toBeVisible();
  await expect(availableAreas.getByRole('button', { name: 'Open enforcement' })).toBeVisible();
  await expect(panel).toContainText('Manual required');
  await expect(panel).toContainText('Current/effective state');
  await expect(panel).toContainText('Not reported');
  await expect(panel).toContainText('Templates');
  await expect(panel).toContainText('Timezone/DST');
  await expect(panel).toContainText('Durability');
  await expect(panel.getByRole('listitem')).toHaveCount(3);
  await expect(panel.getByRole('heading', { exact: true, name: 'Review only' })).toBeVisible();
  const panelBox = await panel.boundingBox();
  expect(panelBox?.height ?? 0).toBeGreaterThan(360);
  await expect(panel).toHaveAttribute('data-ocentra-schedule-authority', 'manual-required');
  await expect(panel).toHaveAttribute('data-ocentra-schedule-state', 'unavailable');
  await expect(page.locator('svg.parent-portal-svg-surface')).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Open SCHEDULES' })).toBeVisible();
  await expect(page.getByRole('button', { name: /Quick preset/ })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Validate Draft' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: /Sync Family/ })).toHaveCount(0);

  await assertScheduleUnavailableResponsiveLayout(page, panel);
  await assertScheduleFallbackNavigation(page, availableAreas);
}

async function assertScheduleUnavailableResponsiveLayout(page: Page, panel: Locator): Promise<void> {
  const viewport = page.viewportSize();
  await page.setViewportSize({ width: 620, height: Math.max(viewport?.height ?? 720, 720) });
  try {
    await expect(panel).toBeVisible();
    await expect(panel.getByText('Current/effective state', { exact: true })).toBeVisible();
    await expect(panel.getByText('Local schedule service required', { exact: true })).toBeVisible();

    await page.setViewportSize({ width: 319, height: 513 });
    const actionsLabel = panel.getByText('Actions', { exact: true });
    await actionsLabel.scrollIntoViewIfNeeded();
    await expect(actionsLabel).toBeVisible();
    await expect(panel.getByText('Manual required', { exact: true }).last()).toBeVisible();
  } finally {
    if (viewport) {
      await page.setViewportSize(viewport);
    }
  }
}

async function assertScheduleFallbackNavigation(page: Page, availableAreas: Locator): Promise<void> {
  await availableAreas.getByRole('button', { name: 'Open rules' }).click();
  await expect(page).toHaveURL(/#\/rule-management$/);
}
