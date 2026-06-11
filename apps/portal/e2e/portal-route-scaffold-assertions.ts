import { expect, type Page } from '@playwright/test';
import {
  PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION,
  PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS,
  PARENT_PORTAL_NAV_LABELS,
  PortalRouteSchema,
  parentPortalRouteContext,
} from '@ocentra-parent/portal-domain/contracts';

const productRoutes = [
  { path: '/#/start', nav: 'START HERE', title: 'START HERE', kind: 'guideDashboard' },
  { path: '/#/overview', nav: 'OVERVIEW', title: 'Current device state', kind: 'control' },
  { path: '/#/assistant', nav: 'AI ASSISTANT', title: 'Ask MIA about', kind: 'assistant' },
  { path: '/#/activity', nav: 'ACTIVITY', title: 'REPORTS CONTROL DETAIL', kind: 'activityManage' },
  { path: '/#/browser', nav: 'WEB', title: 'MANAGED WEB CONTROL DETAIL', kind: 'control' },
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
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'APP USE CONTROL DETAIL',
    kind: 'activityManage',
  },
  {
    path: '/#/network-activity',
    nav: PARENT_PORTAL_NAV_LABELS.Activity,
    title: 'NETWORK ACTIVITY CONTROL DETAIL',
    kind: 'activityManage',
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
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'lanPairing',
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
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
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
  {
    path: '/#/platforms-install',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'manage',
  },
  {
    path: '/#/install-updates',
    nav: PARENT_PORTAL_NAV_LABELS.Devices,
    title: 'LAN PAIRING CONTROL DETAIL',
    kind: 'manage',
  },
  { path: '/#/diagnostics', nav: 'SUPPORT', title: 'SUPPORT CONTROL DETAIL', kind: 'manage' },
  { path: '/#/settings-rules', nav: 'SETTINGS', title: 'FAMILY SETTINGS CONTROL DETAIL', kind: 'manage' },
] as const;

const lanManageRoutePaths = new Set(['/#/platforms-install', '/#/install-updates']);
const routeSurfaceReadyTimeoutMs = 30_000;
const assistantNewChatAction = requireAssistantNewChatAction();
const assistantRulesAction = requireAssistantRulesAction();
const assistantRulesExplainChoice = requireAssistantRulesExplainChoice();

function requireAssistantNewChatAction() {
  if (!PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION) {
    throw new Error('Assistant route scaffold requires the exported New Chat quick action.');
  }
  return PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION;
}

function requireAssistantRulesAction() {
  const action = PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS.find((candidate) => candidate.quickActionId === 'rules');
  if (!action) {
    throw new Error('Assistant route scaffold requires the exported Rules quick action.');
  }
  return action;
}

function requireAssistantRulesExplainChoice() {
  const choice = assistantRulesAction.choices.find((candidate) => candidate.choiceId === 'rules-explain');
  if (!choice) {
    throw new Error('Assistant route scaffold requires the exported Rules Explain choice.');
  }
  return choice;
}

export async function assertRouteScaffolds(page: Page): Promise<void> {
  for (const route of productRoutes) {
    await assertProductRoute(page, route.path, route.title, route.kind);
  }
  await assertSidePanelFoldouts(page);
  await assertDuplicateLabelSidePanelRoutes(page);
  await assertPolicyGuideDeepLinks(page);
  await assertManageTargetSelectorSemantics(page);
  await assertSupportContactRoute(page);
  await assertAssistantEntryAvailable(page);
  await assertFrameTunerRoute(page);
}

async function assertProductRoute(
  page: Page,
  path: string,
  panelTitle: string,
  kind: 'activityManage' | 'assistant' | 'control' | 'guide' | 'guideDashboard' | 'lanPairing' | 'manage'
): Promise<void> {
  await page.goto(path);
  const surface = page.locator('svg.parent-portal-svg-surface');
  const route = PortalRouteSchema.parse(path.slice('/#/'.length));
  const navLabel = parentPortalRouteContext(route).navLabel;
  await expect(surface).toBeVisible();
  await expect(page.getByRole('img', { name: 'Ocentra parent dashboard' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: navLabel }).first()).toBeVisible();
  if (kind === 'assistant') {
    await assertAssistantRouteSurface(page, surface, panelTitle);
    return;
  }
  if (kind === 'manage') {
    await assertManageRouteSurface(surface, path);
    return;
  }
  if (kind === 'activityManage') {
    await assertActivityManageSurface(page, surface, path);
    return;
  }
  if (kind === 'lanPairing') {
    await assertLanPairingRouteSurface(page, surface);
    return;
  }
  if (kind === 'control') {
    await assertControlRouteSurface(surface, path);
    return;
  }
  if (kind === 'guideDashboard') {
    await assertGuideDashboardRouteSurface(page, surface);
    return;
  }
  if (kind === 'guide') {
    await assertGuideRouteSurface(page);
    return;
  }
  await expect(surface.locator('text').filter({ hasText: panelTitle }).first()).toBeVisible();
}

async function assertAssistantRouteSurface(
  page: Page,
  _surface: ReturnType<Page['locator']>,
  _panelTitle: string
): Promise<void> {
  await expect(page.getByRole('button', { name: 'Close parent assistant' })).toBeVisible();
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await expect(page.getByRole('article', { name: `MIA: ${assistantNewChatAction.starterGuide}` })).toBeVisible();
}

async function assertManageRouteSurface(surface: ReturnType<Page['locator']>, path: string): Promise<void> {
  await expectSurfaceTextToMatch(
    surface,
    /(?:Family|Rules|Schedule|Approvals|Enforcement|Audit|Plan|Access|Support|Settings|Portal|Devices|Data|AI|Memory)/
  );
  if (lanManageRoutePaths.has(path)) {
    await expectSurfaceTextToContain(surface, 'Local Area Network');
    await expectSurfaceTextToContain(surface, 'SELECTED DEVICE CONTEXT');
    return;
  }
  if (path === '/#/browser-settings') {
    await expectSurfaceTextToContain(surface, 'ROUTE READINESS');
    await expectSurfaceTextToContain(surface, 'Browser activity');
    await expectSurfaceTextToMatch(surface, /(?:Managed web path|Browser inventory)/);
    await expectSurfaceTextToMatch(surface, /(?:Browser setup|Exact URL capability)/);
    await expectSurfaceTextToMatch(surface, /(?:Enforcement readiness|Active tab proof)/);
    await expectSurfaceTextToMatch(surface, /(?:Browser target|browser policy|Browser activity)/i);
    return;
  }
  if (path === '/#/enforcement') {
    await expectSurfaceTextToMatch(surface, /(?:Enforcement readiness|Browser target|browser policy)/i);
    return;
  }
  if (path === '/#/api-providers') {
    await expectSurfaceTextToContain(surface, 'API providers');
    return;
  }
  if (path === '/#/drive-connections') {
    await expectSurfaceTextToMatch(surface, /(?:Data custody|Drive exports)/);
  }
}

async function surfaceText(surface: ReturnType<Page['locator']>): Promise<string> {
  return (await surface.locator('text').allTextContents()).join(' ');
}

async function expectSurfaceTextToContain(surface: ReturnType<Page['locator']>, expected: string): Promise<void> {
  await expect.poll(() => surfaceText(surface), { timeout: routeSurfaceReadyTimeoutMs }).toContain(expected);
}

async function expectSurfaceTextToMatch(surface: ReturnType<Page['locator']>, expected: RegExp): Promise<void> {
  await expect.poll(() => surfaceText(surface), { timeout: routeSurfaceReadyTimeoutMs }).toMatch(expected);
}

async function closeParentPortalDetailIfOpen(page: Page): Promise<void> {
  const closeDetailButton = page.getByRole('button', { name: 'Close parent portal detail' });
  if ((await closeDetailButton.count()) === 0) {
    return;
  }
  await closeDetailButton.click({ force: true });
  await expect(closeDetailButton).toHaveCount(0, { timeout: routeSurfaceReadyTimeoutMs });
}

async function assertLanPairingRouteSurface(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  const viewport = page.viewportSize();
  await page.setViewportSize({
    width: Math.max(viewport?.width ?? 1280, 1600),
    height: Math.max(viewport?.height ?? 720, 960),
  });

  try {
    await closeParentPortalDetailIfOpen(page);
    await expect(surface.locator('text').filter({ hasText: 'Local Area Network' }).first()).toBeVisible();
    const scanButton = page.getByRole('button', { name: 'Scan Local Area Network' });
    await expect(scanButton).toBeVisible();
    await scanButton.click({ force: true });
    await expect(surface.locator('text').filter({ hasText: 'Info' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Update' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Capability' }).first()).toBeVisible();

    await expect(
      surface
        .locator('text')
        .filter({ hasText: /Device: (?!No device selected).+/ })
        .first()
    ).toBeVisible({ timeout: routeSurfaceReadyTimeoutMs });

    await closeParentPortalDetailIfOpen(page);
    const capabilityTab = page.getByRole('tab', { name: 'Show LAN pairing Capability' });
    await capabilityTab.focus();
    await page.keyboard.press('Enter');
    await expect(capabilityTab).toHaveAttribute('aria-selected', 'true', { timeout: routeSurfaceReadyTimeoutMs });
    await expect(surface.locator('text').filter({ hasText: 'Agent' }).first()).toBeVisible();
    const capabilityText = await surfaceText(surface);
    expect(capabilityText).toMatch(
      /(?:ocentra-(?:local-service|child-agent)|parent\s+local\s+service|agent\s+Not reported)/i
    );
    if (/ocentra-child-agent/i.test(capabilityText)) {
      await expect(surface.locator('text').filter({ hasText: 'CPU' }).first()).toBeVisible();
      await expect(surface.locator('text').filter({ hasText: 'Device ID' }).first()).toBeVisible();
    } else {
      expect(capabilityText).toMatch(/\b(?:DEVICE|ROUTE|EVIDENCE|CONTROL STATE)\b/);
    }
    expect(capabilityText).toMatch(/(?:Signed proof|Proof state|Requirement)/i);

    await assertOptionalLanNeighborRouteProof(page, surface);
    await assertOptionalRouterInfrastructureProof(page);
    await assertNoSyntheticLanDevices(page, surface);
  } finally {
    if (viewport) {
      await page.setViewportSize(viewport);
    }
  }
}

async function assertOptionalLanNeighborRouteProof(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  const neighborChoice = page.getByRole('button', { name: /^Select LAN \d{1,3}(?:\.\d{1,3}){3}$/ }).first();
  if ((await neighborChoice.count()) === 0) {
    return;
  }

  const ariaLabel = (await neighborChoice.getAttribute('aria-label')) ?? '';
  const neighborLabel = ariaLabel.replace(/^Select /, '');
  await neighborChoice.click({ force: true });
  await page.getByRole('tab', { name: 'Show LAN pairing Info' }).click({ force: true });
  const infoText = await surfaceText(surface);
  expect(infoText).toContain(neighborLabel);
  expect(infoText).toMatch(/\bIP\b/);
  expect(infoText).toMatch(/\b(?:HOST|SOURCE|STATE)\b/);
  expect(infoText).toMatch(/\b(?:LAN discovered|Not reported|Stale|Visible only)\b/);

  await page.getByRole('tab', { name: 'Show LAN pairing Capability' }).click({ force: true });
  const capabilityText = await surfaceText(surface);
  expect(capabilityText).toContain('Not reported');
}

async function assertOptionalRouterInfrastructureProof(page: Page): Promise<void> {
  const routerChoice = page.getByRole('button', { name: /^LAN \d{1,3}(?:\.\d{1,3}){3} is unsupported$/ }).first();
  if ((await routerChoice.count()) === 0) {
    return;
  }

  const ariaLabel = (await routerChoice.getAttribute('aria-label')) ?? '';
  const routerLabel = ariaLabel.replace(/ is unsupported$/, '');
  await expect(page.getByRole('button', { exact: true, name: `Select ${routerLabel}` })).toHaveCount(0);
}

async function assertNoSyntheticLanDevices(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  await expect(page.getByRole('button', { name: 'Select Aarav laptop' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Aarav laptop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'UI check device' })).toHaveCount(0);
}

async function assertControlRouteSurface(surface: ReturnType<Page['locator']>, path: string): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'WHAT PARENTS CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'DATA CUSTODY' }).first()).toBeVisible();
  if (path === '/#/browser') {
    await expect(surface.locator('text').filter({ hasText: 'Browser inventory' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Exact URL capability' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Active tab proof' }).first()).toBeVisible();
  }
}

async function assertGuideDashboardRouteSurface(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  await expect(page.getByRole('button', { name: 'Open Set Up Ocentra Parent' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Support Contact' }).first()).toBeVisible();
}

async function assertGuideRouteSurface(page: Page): Promise<void> {
  await expect(page.getByRole('button', { name: 'Show QUICK READ' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Show QUICK ACTION' })).toBeVisible();
}

async function assertPolicyGuideDeepLinks(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  await page.goto('/#/browser-settings');
  await expect(surface.locator('[aria-label="Open Browser Rules guide"]')).toBeVisible();
  await assertBrowserPolicyDeviceTargets(page, surface);
  await surface.locator('[aria-label="Open Browser Budget guide"]').focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/#\/policy\?guideTopic=browser-policy-guide&guidePage=2$/);
  await expect(surface.locator('text').filter({ hasText: 'BROWSER BUDGET' }).first()).toBeVisible();
  await page.getByRole('button', { name: 'Show QUICK ACTION' }).click({ force: true });
  await page.getByRole('button', { name: 'Open Browser setup' }).click({ force: true });
  await expect(page).toHaveURL(/#\/browser-settings$/);
  await page.goto('/#/policy-apps');
  await expect(surface.locator('[aria-label="Open Apps Rules guide"]')).toBeVisible();
}

async function assertBrowserPolicyDeviceTargets(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  const viewport = page.viewportSize();
  await page.setViewportSize({
    width: Math.max(viewport?.width ?? 1280, 1600),
    height: Math.max(viewport?.height ?? 720, 960),
  });
  try {
    await expect(page.getByText('Per Device').first()).toBeVisible();
    await expect(page.getByRole('button', { name: /^Select (?!LAN ).+/ }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: /^LAN 192\.168\.2\.1$/ })).toHaveCount(0);
  } finally {
    if (viewport) {
      await page.setViewportSize(viewport);
    }
  }
}

async function assertActivityManageSurface(
  page: Page,
  surface: ReturnType<Page['locator']>,
  path: string
): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'Family' }).first()).toBeVisible();
  await expect(page.getByText('Per Device').first()).toBeVisible();
  if (path === '/#/app-game-sessions') {
    await assertAppGameDashboardRouteSurface(page, surface);
    await assertCollapsedActivitySubsurfaceRemoved(page, surface);
    return;
  }
  if (path === '/#/network-activity') {
    await expect(surface.locator('text').filter({ hasText: 'NETWORK ACTIVITY' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Network' }).first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'Scan Local Area Network' })).toBeVisible();
    await assertCollapsedActivitySubsurfaceRemoved(page, surface);
    return;
  }
  await expect(surface.locator('text').filter({ hasText: 'Reports' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Screen' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'App Use' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Browser' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Games' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Network' }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: 'Scan Local Area Network' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Connected' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Offline' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Empty' }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: 'Select Aarav laptop' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Aarav laptop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'D001' })).toHaveCount(0);
  await assertActivityReportSurface(page, surface);
  await assertCollapsedActivitySubsurfaceRemoved(page, surface);
}

async function assertAppGameDashboardRouteSurface(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'APP/GAME READ MODEL DASHBOARD' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SERVICE ROWS' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'CAPABILITY MATRIX' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'EVIDENCE DRAWER' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'GAME BUDGETS' }).first()).toBeVisible();
  const visibleText = await surfaceText(surface);
  expect(visibleText).toMatch(/\b(?:INVENTORY|Inventory)\b/);
  expect(visibleText).toMatch(/\b(?:RUNNING|Running)\b/);
  expect(visibleText).toMatch(/\b(?:FOREGROUND|Foreground)\b/);
  expect(visibleText).toMatch(/\b(?:LAUNCHER|Launcher)\b/);
  expect(visibleText).toMatch(/\bSOURCE ROWS\b/);
  expect(visibleText).toMatch(/\bFRESH SOURCES\b/);
  await expect(page.getByRole('button', { name: 'Select Aarav laptop' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'D001' })).toHaveCount(0);
}

async function assertActivityReportSurface(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  await expect(
    surface
      .locator('text')
      .filter({ hasText: /Report (device|target): .+/ })
      .first()
  ).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Frequency' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Report viewer' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SELECTED REPORT' }).first()).toBeVisible();
  const savedReportButton = page.getByRole('button', { name: /^Open activity-report-.+\.json$/ });
  if ((await savedReportButton.count()) > 0) {
    await expect(savedReportButton.first()).toBeVisible();
  } else {
    await expect(
      surface.locator('text').filter({ hasText: 'No saved activity reports reported' }).first()
    ).toBeVisible();
  }
  await expect(page.getByRole('button', { name: 'Generate Daily activity report' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save generated activity report' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Daily' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Weekly' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Monthly' }).first()).toBeVisible();
}

async function assertSupportContactRoute(page: Page): Promise<void> {
  await page.goto('/#/diagnostics');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface.locator('text').filter({ hasText: 'Support / Contact' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'NEW MESSAGE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'REPLY EMAIL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SUBJECT' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'MESSAGE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SEND MESSAGE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SAVE DRAFT' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'diagnostic' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Response path' })).toHaveCount(0);
}

async function assertCollapsedActivitySubsurfaceRemoved(
  page: Page,
  surface: ReturnType<Page['locator']>
): Promise<void> {
  await expect(page.getByRole('tab', { name: 'Show History activity monitor' })).toHaveCount(0);
  await expect(page.getByRole('tab', { name: 'Show Live activity monitor' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'MONITOR' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT CADENCE' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'PER DEVICE BEHAVIOR' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'PAST REPORTS' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'LIVE REPORT' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Open report' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Family Defaults' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT TYPE' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT TARGET' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT MODE' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Family activity selected' })).toHaveCount(0);
}

async function assertSidePanelFoldouts(page: Page): Promise<void> {
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand GUIDE');
  await expect(page.getByRole('button', { name: 'Collapse GUIDE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open START HERE' })).toBeVisible();
  await clickSidePanelButton(page, 'Collapse GUIDE');
  await expect(page.getByRole('button', { name: 'Expand GUIDE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open START HERE' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Expand MANAGE' })).toBeVisible();
  await page.goto('/#/settings-rules');
  await expect(page.getByRole('button', { name: 'Collapse MANAGE' })).toBeVisible();
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Portal}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Portal}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Settings}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Devices}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Devices}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Lan}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Capability}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Remote}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Platforms}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Updates}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Activity}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.ReportSet}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.AppsGames}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Builder}` })).toHaveCount(0);
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.Devices}`);
  await expect(page).toHaveURL(/#\/devices$/);
  await page.goto('/#/overview');
  await expect(page.getByRole('button', { name: 'Expand MANAGE' })).toBeVisible();
}

async function clickSidePanelButton(page: Page, name: string): Promise<void> {
  const button = page.getByRole('button', { exact: true, name });
  await expect(button).toBeVisible();
  await button.dispatchEvent('click');
}

async function expandSidePanelGroup(page: Page, label: string): Promise<void> {
  const expandButton = page.getByRole('button', { name: `Expand ${label}` });
  if ((await expandButton.count()) > 0) {
    await expandButton.click({ force: true });
  }
  await expect(page.getByRole('button', { name: `Collapse ${label}` })).toBeVisible();
}

async function assertDuplicateLabelSidePanelRoutes(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand GUIDE');
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.RulesGuide}`);
  await expect(page).toHaveURL(/#\/policy$/);
  await expect(surface.locator('text').filter({ hasText: 'Rules' }).first()).toBeVisible();
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand MANAGE');
  await expandSidePanelGroup(page, PARENT_PORTAL_NAV_LABELS.Policies);
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.Browser}`);
  await expect(page).toHaveURL(/#\/browser-settings$/);
  await expect(page.getByRole('button', { name: 'Open Browser guide' })).toBeVisible();
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand GUIDE');
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.ReportsGuide}`);
  await expect(page).toHaveURL(/#\/reports-guide$/);
  await expect(surface.locator('text').filter({ hasText: 'Reports And Summaries' }).first()).toBeVisible();
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand MANAGE');
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Activity}` })).toHaveCount(0);
  await page.goto('/#/activity');
  await expect(page.getByRole('button', { name: 'Open Report guide' })).toBeVisible();
}

async function assertManageTargetSelectorSemantics(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  const targetSelector = page.getByRole('button', { exact: true, name: 'Focus parent control selector' });

  await page.goto('/#/settings-rules');
  await expect(surface).toBeVisible();
  await expect(targetSelector).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'PORTAL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Parent owned' }).first()).toBeVisible();

  await page.goto('/#/lan-pairing');
  await expect(targetSelector).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Local Area Network' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SELECTED DEVICE CONTEXT' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SOURCE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'ROUTE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Local device' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'This parent portal' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'New child device' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Aarav laptop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Family tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'AI / New child device' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select D001' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'D004 is unsupported' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'UI check device 1' })).toHaveCount(0);

  await page.goto('/#/platforms-install');
  await expect(targetSelector).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Local Area Network' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Info' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Platforms / Parent desktop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Platforms / Parent profile' })).toHaveCount(0);
}

async function assertAssistantEntryAvailable(page: Page): Promise<void> {
  await page.goto('/#/overview');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(page.getByRole('button', { name: 'Open AI assistant' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'AI ASSISTANT' }).first()).toBeVisible();
  await page.getByRole('button', { name: 'Open AI assistant' }).click({ force: true });
  await expect(page).toHaveURL(/#\/assistant$/);
  await expect(page.getByRole('button', { name: 'Close parent assistant' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Hide action panel' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Send message to MIA' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Use voice input for MIA' })).toBeVisible();
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await page.getByRole('tab', { name: 'History' }).click({ force: true });
  await expect(page.getByRole('button', { name: /^Report history$/ })).toBeVisible();
  await expect(page.getByRole('button', { name: /^Rules history$/ })).toBeVisible();
  await page.getByRole('tab', { name: 'Quick Action' }).click({ force: true });
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await expect(page.getByRole('button', { name: /^Ask MIA: Give me the overall report$/ })).toBeVisible();
  await expect(page.getByRole('button', { name: /^Copy MIA message$/ }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'AI assisted view' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Ask AI Assistant to update a setting' })).toHaveCount(0);
  await assistantQuickActionButton(page, assistantRulesAction.title).click({ force: true });
  await expectAssistantQuickActionChoice(page, assistantRulesAction.title, assistantRulesExplainChoice.label);
  await assistantQuickActionChoiceButton(page, assistantRulesAction.title, assistantRulesExplainChoice.label).click({
    force: true,
  });
  await expect(page).toHaveURL(/#\/assistant$/);
  await expect(page.getByRole('button', { name: /^Copy YOU message$/ }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: /^Ask MIA: Change a rule$/ })).toBeVisible();
  await expect(page.getByRole('article', { name: `YOU: ${assistantRulesExplainChoice.label}` })).toBeVisible();
  await page.getByRole('button', { name: 'Hide action panel' }).click({ force: true });
  await expect(page.getByRole('button', { name: 'Show action panel' })).toBeVisible();
  await expect(assistantQuickActionButton(page, assistantRulesAction.title)).toHaveCount(0);
  await page.getByRole('button', { name: 'Show action panel' }).click({ force: true });
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await page.getByRole('button', { name: 'Close parent assistant' }).click({ force: true });
  await expect(page).toHaveURL(/#\/overview$/);
  await expect(page.getByRole('button', { name: 'Open AI assistant' })).toBeVisible();
}

function assistantQuickActionButton(page: Page, actionTitle: string) {
  return page.getByRole('button', { exact: true, name: `Ask MIA about ${actionTitle}` });
}

async function expectAssistantQuickAction(page: Page, actionTitle: string): Promise<void> {
  await expect(assistantQuickActionButton(page, actionTitle)).toBeVisible();
}

function assistantQuickActionChoiceButton(page: Page, _actionTitle: string, choiceLabel: string) {
  return page.getByRole('button', { exact: true, name: `Ask MIA: ${choiceLabel}` });
}

async function expectAssistantQuickActionChoice(page: Page, actionTitle: string, choiceLabel: string): Promise<void> {
  await expect(assistantQuickActionChoiceButton(page, actionTitle, choiceLabel)).toBeVisible();
}

async function assertFrameTunerRoute(page: Page): Promise<void> {
  await page.goto('/#/app-layout');
  await expect(page.getByRole('heading', { name: 'App layout' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save JSON' })).toBeVisible();
  await expect(page.locator('.app-sidebar')).toHaveCount(0);
  await expect(page.locator('svg.portal-frame-backdrop-svg')).toHaveCount(0);
  await assertAppLayoutTopTabs(page);
  await assertMainAppLayoutHierarchy(page);
  await assertChatLayoutHierarchy(page);
}

async function assertAppLayoutTopTabs(page: Page): Promise<void> {
  await expect(page.getByRole('tab', { name: 'Main App' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Chat Interface' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Carousel' })).toHaveCount(0);
  await expect(page.getByRole('tab', { name: 'Golden card' })).toHaveCount(0);
  await expect(page.getByRole('tab', { name: 'Save and JSON' })).toHaveCount(0);
}

async function assertMainAppLayoutHierarchy(page: Page): Promise<void> {
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Main App');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Side panel');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Sidepanel top');
  await expect(page.getByRole('tab', { name: 'Side panel' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Main panel' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Sidepanel top' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Sidepanel bottom' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Chrome' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Colors' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Content' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Reset surface' })).toBeVisible();
  await page.getByRole('tab', { name: 'Content' }).click();
  await expect(page.getByRole('button', { name: 'Add foldout' })).toBeVisible();
  await expect(page.getByText('Sidepanel foldouts')).toBeVisible();
}

async function assertChatLayoutHierarchy(page: Page): Promise<void> {
  await page.getByRole('tab', { name: 'Chat Interface' }).click();
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Chat Interface');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Side panel');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Sidepanel top');
  await expect(page.getByRole('tab', { name: 'Sidepanel top' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Sidepanel bottom' })).toBeVisible();
  await page.getByRole('tab', { name: 'Main panel' }).click();
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Chat Interface');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Main panel');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Top choices');
  await expect(page.getByRole('tab', { name: 'Top choices' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Main bottom' })).toBeVisible();
  await page.getByRole('tab', { name: 'Content' }).click();
  await expect(page.getByRole('tab', { name: 'Top choices' })).toHaveCount(2);
  await expect(page.getByText('Main bottom content')).toBeVisible();
}
