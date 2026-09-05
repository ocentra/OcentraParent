import { expect, test, type Locator, type Page } from '@playwright/test';
import { PortalTheme } from '@ocentra-parent/portal-domain/contracts';
import { ParentAgentEvent } from '../../generated/parent-ui-bridge';
import { collectBrowserFailures } from './browser-failures';
import { assertLanRouteScaffolds, assertScheduleRouteUnavailable } from './portal-route-scaffold-assertions';
import {
  assertDevicesRoute,
  assertInvalidStoredDeviceContextFailsClosed,
  assertManageRoutesRenderTheirOwnedWorkspace,
  assertSelectedDeviceContextPersistsAcrossRoutes,
  manageTargetSelectionStorageKey,
} from './portal-ui-context-assertions';

test.setTimeout(420_000);

const portalShellReadyTimeoutMs = 90_000;
const defaultPortalPort = '4490';

test('portal UI connects to the real agent and renders command results', async ({ context, page }) => {
  const browserFailures = collectBrowserFailures(page);
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: portalOrigin() });
  await page.goto('/#/commands');
  await page.evaluate((storageKey) => window.localStorage.removeItem(storageKey), PortalTheme.LocalStorageKey);
  await page.evaluate((storageKey) => window.sessionStorage.removeItem(storageKey), manageTargetSelectionStorageKey);
  await page.reload();
  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByRole('button', { exact: true, name: 'Login' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  await assertHeaderThemeToggle(page);
  await assertAuthDialog(page);
  await assertCommandControls(page);
  await assertInitialCommandRouteDrain(page);
  await assertTabbedCommandResults(page);
  await assertRawEventLog(page);
  await assertOverview(page);
  await assertResponsiveParentSurface(page);
  await assertPolicyMutationControlsFailClosed(page);
  await assertAssistantRouteOwnershipAndUnavailableAffordances(page);
  await assertReportActionsReachRealAgent(page);
  await assertManageRoutesRenderTheirOwnedWorkspace(page);
  await assertSupportConnectorFailsClosed(page);
  await assertManageRouteRequiresExplicitDeviceSelection(page);
  const selectedDeviceLabel = await assertDevicesRoute(page);
  await assertSelectedDeviceContextPersistsAcrossRoutes(page, selectedDeviceLabel);
  await assertInvalidStoredDeviceContextFailsClosed(page, selectedDeviceLabel);
  await assertLanRouteScaffolds(page);

  expect(browserFailures).toEqual([]);
});

test('unavailable quick-glance routes render current fail-closed recovery', async ({ page }) => {
  await assertOverview(page);

  await page.goto('/#/browser');
  const browserStatus = page.getByRole('region', { exact: true, name: 'Browser activity status' });
  await expect(browserStatus).toBeVisible();
  await expect(browserStatus).toHaveAttribute('data-ocentra-browser-route-state', 'unavailable');
  await expect(browserStatus.getByRole('heading', { exact: true, name: 'Browser status unavailable' })).toBeVisible();
  await expect(browserStatus.getByRole('article')).toHaveCount(3);
  await expect(browserStatus.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
});

test('Capability Status is reachable from Quick Glance as a product status destination', async ({ page }) => {
  await page.goto('/#/activity');
  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  await page.getByRole('button', { exact: true, name: 'Expand QUICK GLANCE' }).click();
  await page.getByRole('button', { exact: true, name: 'Open CAPABILITY' }).click();

  await expect(page).toHaveURL(/#\/capability-status$/u);
  const capabilityStatus = page.getByRole('region', { exact: true, name: 'Capability status' });
  await expect(capabilityStatus).toBeVisible();
  await expect(
    capabilityStatus.getByRole('heading', { exact: true, name: 'Capability and service status' })
  ).toBeVisible();
  await expect(capabilityStatus.getByRole('article')).toHaveCount(12);
  await expect(capabilityStatus.getByRole('heading', { exact: true, name: 'Browser' })).toBeVisible();
  await expect(capabilityStatus.getByRole('heading', { exact: true, name: 'Network activity' })).toBeVisible();
});

test('unavailable assistant guides recovery through Start Here', async ({ page }) => {
  await assertAssistantRouteOwnershipAndUnavailableAffordances(page);
});

test('Start boundary status stays compact and leaves the setup map visible', async ({ page }) => {
  await page.goto('/#/start');
  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const boundary = page.getByRole('region', { exact: true, name: 'Setup-first-run boundary status' });
  const firstSetup = page.getByRole('button', { exact: true, name: 'Open Set Up Ocentra Parent' });
  const devicesAndPairing = page.getByRole('button', { exact: true, name: 'Open Devices And Pairing' });
  await expect(boundary).toBeVisible();
  await expect(boundary).toHaveAttribute('data-ocentra-setup-proof', 'first-run-route');
  await expect(firstSetup).toBeVisible();
  await expect(devicesAndPairing).toBeVisible();

  const boundaryBox = await requiredBoundingBox(boundary, 'Start boundary status');
  expect(boundaryBox.height).toBeLessThanOrEqual(360);
  expect(rectanglesOverlap(boundaryBox, await requiredBoundingBox(firstSetup, 'first setup topic'))).toBe(false);
  expect(
    rectanglesOverlap(boundaryBox, await requiredBoundingBox(devicesAndPairing, 'Devices And Pairing setup topic'))
  ).toBe(false);

  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/#/start');
  const compactBoundary = page.getByRole('region', { exact: true, name: 'Setup-first-run boundary status' });
  const compactFirstSetup = page.getByRole('button', { exact: true, name: 'Open Set Up Ocentra Parent' });
  await expect(compactBoundary).toBeVisible();
  await expect(compactFirstSetup).toBeVisible();
  expect((await requiredBoundingBox(compactBoundary, 'compact Start boundary status')).height).toBeLessThanOrEqual(180);
  expect(
    rectanglesOverlap(
      await requiredBoundingBox(compactBoundary, 'compact Start boundary status'),
      await requiredBoundingBox(compactFirstSetup, 'compact first setup topic')
    )
  ).toBe(false);
});

test('Platforms and Install Updates expose a useful fail-closed recovery path', async ({ page }) => {
  await page.goto('/#/platforms-install');
  const platformsPanel = page.getByRole('region', { exact: true, name: 'Platforms and install status' });
  await expect(platformsPanel).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(platformsPanel).toHaveAttribute('data-ocentra-desktop-distribution-state', 'runtime');
  await expect(platformsPanel).toHaveAttribute('data-ocentra-desktop-distribution-actions', 'unavailable');
  await expect(platformsPanel.getByRole('heading', { exact: true, name: 'Platform proof' })).toBeVisible();
  await expect(platformsPanel.getByRole('heading', { exact: true, name: 'Source and custody' })).toBeVisible();
  await expect(platformsPanel.getByText('Payload source', { exact: true })).toBeVisible();
  await expect(platformsPanel.getByText('rust parent runtime', { exact: true })).toBeVisible();
  await expect(platformsPanel.getByRole('button').allTextContents()).resolves.toEqual([
    'Open Start Here',
    'Review install updates',
  ]);

  await platformsPanel.getByRole('button', { exact: true, name: 'Review install updates' }).click();
  await expect(page).toHaveURL(/#\/install-updates$/u);
  const updatesPanel = page.getByRole('region', { exact: true, name: 'Install and update status' });
  await expect(updatesPanel).toHaveAttribute('data-ocentra-desktop-distribution-state', 'runtime');
  await expect(updatesPanel).toHaveAttribute('data-ocentra-desktop-distribution-actions', 'unavailable');
  await expect(updatesPanel.getByRole('heading', { exact: true, name: 'Update channel' })).toBeVisible();
  await expect(updatesPanel.getByRole('heading', { exact: true, name: 'Release proof' })).toBeVisible();
  await expect(updatesPanel.getByRole('button').allTextContents()).resolves.toEqual([
    'Open Start Here',
    'Review platform status',
  ]);

  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/#/platforms-install');
  const compactPanel = page.getByRole('region', { exact: true, name: 'Platforms and install status' });
  await expect(compactPanel).toBeVisible();
  const compactStatus = compactPanel.getByRole('heading', { exact: true, name: 'Desktop package' });
  const compactSource = compactPanel.getByRole('heading', { exact: true, name: 'Source and custody' });
  expect(
    rectanglesOverlap(
      await requiredBoundingBox(compactStatus, 'compact distribution status'),
      await requiredBoundingBox(compactSource, 'compact distribution source and custody card')
    )
  ).toBe(false);

  await compactPanel.getByRole('button', { exact: true, name: 'Open Start Here' }).click();
  await expect(page).toHaveURL(/#\/start$/u);
});

test('missing screen summary owns a dedicated recoverable route', async ({ page }) => {
  await page.goto('/#/screen-analysis');
  const panel = page.getByRole('region', { exact: true, name: 'Screen analysis' });
  const settingPreview = page.getByText('SETTING PREVIEW', { exact: true });
  await expect(panel).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(settingPreview).toHaveCount(0);
  await expect(panel.locator('details')).toHaveCount(0);
  await expect(panel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
  await expect(panel.getByText('No screen summary read model has been reported.').first()).toBeVisible();

  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/#/overview');
  await page.goto('/#/screen-analysis');
  const compactPanel = page.getByRole('region', { exact: true, name: 'Screen analysis' });
  await expect(compactPanel).toBeVisible();
  await expect(compactPanel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
  const familyScope = page.getByRole('button', { exact: true, name: 'Use Family scope' });
  await expect(familyScope).toHaveCount(0);
});

test('compact network status cards remain readable without overlap', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/#/network-activity');

  const networkStatus = page.getByRole('region', { exact: true, name: 'Network activity' });
  await expect(networkStatus).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(networkStatus).toHaveAttribute('data-ocentra-network-route-state', 'unavailable');
  const cards = networkStatus.getByRole('article');
  await expect(cards).toHaveCount(3);
  for (const title of ['Flow observations', 'Evidence custody', 'LAN discovery']) {
    await expect(networkStatus.getByRole('heading', { exact: true, name: title })).toBeVisible();
  }
  const cardBoxes = await Promise.all(
    [0, 1, 2].map((index) => requiredBoundingBox(cards.nth(index), `compact network card ${String(index + 1)}`))
  );
  const panelBox = await requiredBoundingBox(networkStatus, 'compact network status');
  for (const cardBox of cardBoxes) {
    expect(cardBox.width).toBeLessThanOrEqual(panelBox.width);
  }
  for (let index = 1; index < cardBoxes.length; index += 1) {
    expect(rectanglesOverlap(cardBoxes[index - 1]!, cardBoxes[index]!)).toBe(false);
  }
});

test('Portal settings use a balanced sparse-card grid', async ({ page }) => {
  await page.goto('/#/settings-rules');
  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const [familyDefaultsBox, parentSessionBox, privacyPostureBox, themeAndConsoleBox] = await Promise.all([
    requiredBoundingBox(page.getByText('FAMILY DEFAULTS', { exact: true }), 'FAMILY DEFAULTS'),
    requiredBoundingBox(page.getByText('PARENT SESSION', { exact: true }), 'PARENT SESSION'),
    requiredBoundingBox(page.getByText('PRIVACY POSTURE', { exact: true }), 'PRIVACY POSTURE'),
    requiredBoundingBox(page.getByText('THEME AND CONSOLE', { exact: true }), 'THEME AND CONSOLE'),
  ]);

  expect(Math.abs(familyDefaultsBox.y - parentSessionBox.y)).toBeLessThanOrEqual(2);
  expect(Math.abs(privacyPostureBox.y - themeAndConsoleBox.y)).toBeLessThanOrEqual(2);
  expect(privacyPostureBox.y - familyDefaultsBox.y).toBeGreaterThan(90);
});

test('diagnostics stay integrated with the Portal glass surface', async ({ page }) => {
  await page.goto('/#/diagnostics');
  const summary = page.locator('.portal-dev-route-content > .summary');
  const portalSurface = page.locator('svg.parent-portal-svg-surface');

  await expect(page.getByRole('heading', { exact: true, name: 'Device diagnostics' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Copy diagnostics' })).toBeVisible();
  await expect(portalSurface).toContainText('Copy redacted support diagnostics');
  await expect(portalSurface).toContainText('Local inspection');
  await expect(portalSurface).toContainText('Service authority stays fail-closed');
  await expect(portalSurface).toContainText('CURRENT AREA');
  await expect(portalSurface).toContainText('DIAGNOSTICS');
  await expect(portalSurface).not.toContainText('Support messages are parent-authored');
  await expect(summary).toHaveCount(1);
  const backgroundImage = await summary.evaluate((element) => getComputedStyle(element).backgroundImage);
  expect(backgroundImage).toContain('rgba(4, 20, 37, 0.96)');
  expect(backgroundImage).toContain('rgba(2, 12, 22, 0.98)');
  expect(await summary.evaluate((element) => getComputedStyle(element).color)).toBe('rgb(229, 247, 255)');
  expect(
    await page
      .getByRole('heading', { exact: true, name: 'Device diagnostics' })
      .evaluate((element) => getComputedStyle(element).color)
  ).toBe('rgb(244, 251, 255)');
});

test('overview identifies its active parent area instead of a stale manage selection', async ({ page }) => {
  await page.goto('/#/activity');
  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  await page.getByRole('button', { exact: true, name: 'Expand QUICK GLANCE' }).click();
  await page.getByRole('button', { exact: true, name: 'Open OVERVIEW' }).click();

  await expect(page).toHaveURL(/#\/overview$/u);
  const portalSurface = page.locator('svg.parent-portal-svg-surface');
  await expect(portalSurface).toContainText('CURRENT AREA');
  await expect(portalSurface).toContainText('OVERVIEW');
  await expect(portalSurface).not.toContainText('Activity Store');
});

test('policy routes render their own family and domain guidance', async ({ page }) => {
  await assertManageRoutesRenderTheirOwnedWorkspace(page);
});

test('unavailable schedules retain the parent dashboard shell', async ({ page }) => {
  await assertScheduleRouteUnavailable(page);
});

test('activity scope explains current device availability', async ({ page }) => {
  await page.goto('/#/activity');
  const familyScope = page.getByRole('status', { exact: true, name: 'Whole family activity report scope' });
  await expect(familyScope).toBeVisible();
  await expect(familyScope).toContainText('Family reports cover every current household device.');

  await page.getByRole('button', { exact: true, name: 'Select Per Device' }).click();
  const unavailableTargets = page.getByRole('status', { exact: true, name: 'No current activity device targets' });
  await expect(unavailableTargets).toBeVisible();
  await expect(unavailableTargets).toContainText('Connect the local service and load a current household device');
  const browserTab = page.getByRole('tab', { exact: true, name: 'Browser requires a current device selection' });
  await expect(browserTab).toHaveAttribute('aria-disabled', 'true');
  await expect(browserTab).toHaveAttribute('tabindex', '-1');
});

test('manage workspaces explain target scope before enabling device controls', async ({ page }) => {
  await page.goto('/#/approvals');
  const approvalsStatus = page.getByRole('region', { exact: true, name: 'Approvals status unavailable' });
  await expect(approvalsStatus).toBeVisible();
  await expect(approvalsStatus).toHaveAttribute('data-ocentra-policy-preview-state', 'unavailable');
  await expect(approvalsStatus.getByRole('listitem')).toHaveCount(3);
  await expect(approvalsStatus).toContainText('No owner-backed editing authority is available');
  await expect(approvalsStatus.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Select Per Device' })).toHaveCount(0);

  await page.goto('/#/api-providers');
  await page.getByRole('button', { exact: true, name: 'Select Portal' }).click();
  const portalTarget = page.getByRole('status', { exact: true, name: 'Parent console manage target scope' });
  await expect(portalTarget).toBeVisible();
  await expect(portalTarget).toContainText('This workspace is scoped to the parent console.');

  await page.goto('/#/devices');
  await expect(page.getByRole('status', { exact: true, name: 'Whole family manage target scope' })).toHaveCount(0);
});

test('unavailable device and Portal settings surfaces remain parent-facing', async ({ page }) => {
  await page.goto('/#/devices');
  const deviceDiscoveryUnavailable = page.getByRole('status', {
    exact: true,
    name: 'Device discovery unavailable',
  });
  await expect(deviceDiscoveryUnavailable).toBeVisible();
  await expect(page.getByText('DEVICE DISCOVERY UNAVAILABLE', { exact: true })).toBeVisible();
  await expect(deviceDiscoveryUnavailable).toContainText('Connect the local service');
  await expect(deviceDiscoveryUnavailable).toContainText('current LAN devices.');
  await expect(page.getByRole('button', { exact: true, name: 'Scan Local Area Network' })).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: 'Retry status' })).toBeEnabled();

  await page.goto('/#/settings-rules');
  await page.getByRole('tab', { exact: true, name: 'Show Channels' }).click();
  await expect(page.getByText('TEST MESSAGE', { exact: true })).toBeVisible();
  await expect(page.locator('svg.parent-portal-svg-surface')).toContainText(
    'A test cannot be sent until a verified channel and notification service report current state.'
  );
  await expect(page.getByText(/backend lands/iu)).toHaveCount(0);

  await page.getByRole('tab', { exact: true, name: 'Show Runtime' }).click();
  await expect(page.locator('svg.parent-portal-svg-surface')).toContainText(
    'Check, install, and rollback stay explicit parent actions.'
  );
});

test('leaving a policy route exposes the remaining Manage destinations without a hidden scroll gesture', async ({
  page,
}) => {
  await page.goto('/#/policy-tracking');
  const activeTrackingRoute = page.getByRole('button', { exact: true, name: 'Open TRACKING' });
  await expect(activeTrackingRoute).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(activeTrackingRoute).toHaveAttribute('aria-current', 'page');

  const devTools = page.getByRole('button', { exact: true, name: 'Expand DEV TOOLS' });
  await expect(devTools).toBeVisible();
  await devTools.click();
  await page.getByRole('button', { exact: true, name: 'Open LOGS' }).click();
  await page.getByRole('button', { exact: true, name: 'Expand MANAGE' }).click();
  await page.getByRole('tab', { name: /^Browser controls/u }).click();

  await expect(page).toHaveURL(/#\/browser-settings$/u);
  const policyGroup = page.getByRole('button', { exact: true, name: 'Collapse POLICY' });
  await expect(policyGroup).toBeVisible();
  await policyGroup.click();
  for (const destination of ['DATA', 'REMOTE', 'PLATFORMS', 'UPDATES', 'AI', 'ACCOUNT']) {
    await expect(page.getByRole('button', { exact: true, name: `Open ${destination}` })).toBeVisible();
  }
});

function portalOrigin(): string {
  const portalPort = process.env['OCENTRA_PARENT_PORTAL_PORT']?.trim() || defaultPortalPort;
  return `http://127.0.0.1:${portalPort}`;
}

interface ElementBox {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

async function requiredBoundingBox(locator: Locator, label: string): Promise<ElementBox> {
  const box = await locator.boundingBox();
  if (box === null) {
    throw new Error(`Expected a rendered bounding box for ${label}`);
  }
  return box;
}

function rectanglesOverlap(left: ElementBox, right: ElementBox): boolean {
  return !(
    left.x + left.width <= right.x ||
    right.x + right.width <= left.x ||
    left.y + left.height <= right.y ||
    right.y + right.height <= left.y
  );
}

function expectBoxInside(inner: ElementBox, outer: ElementBox): void {
  expect(inner.x).toBeGreaterThanOrEqual(outer.x);
  expect(inner.y).toBeGreaterThanOrEqual(outer.y);
  expect(inner.x + inner.width).toBeLessThanOrEqual(outer.x + outer.width);
  expect(inner.y + inner.height).toBeLessThanOrEqual(outer.y + outer.height);
}

async function assertSupportConnectorFailsClosed(page: Page): Promise<void> {
  await page.goto('/#/entitlements');
  await page.getByRole('tab', { exact: true, name: 'Show Support' }).click();
  const unavailable = page.getByRole('group', { exact: true, name: 'Support connector unavailable' });
  await expect(unavailable).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await expect(unavailable).toHaveAttribute('aria-disabled', 'true');
  await expect(unavailable).toContainText('SUPPORT CONNECTOR UNAVAILABLE');
  await expect(unavailable).toContainText('DRAFT UNAVAILABLE');
  await expect(unavailable).toContainText('SEND UNAVAILABLE');
  await expect(page.getByText('SAVE DRAFT', { exact: true })).toHaveCount(0);
  await expect(page.getByText('SEND MESSAGE', { exact: true })).toHaveCount(0);
}

function commandControlButton(page: Page, name: string): Locator {
  return page.getByRole('button', { exact: true, name });
}

async function expectCommandControlEnabled(page: Page, name: string): Promise<void> {
  await expect(commandControlButton(page, name)).toBeEnabled({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function clickCommandControl(page: Page, name: string): Promise<void> {
  await commandControlButton(page, name).click();
}

async function assertHeaderThemeToggle(page: Page): Promise<void> {
  await expect(page.getByRole('group', { exact: true, name: 'Display theme' })).toHaveCount(1);
  await expect(page.getByRole('button', { exact: true, name: 'Light' })).toHaveCount(1);
  await expect(page.getByRole('button', { exact: true, name: 'Dark' })).toHaveCount(1);
  await expect(page.getByRole('button', { exact: true, name: 'Select Light' })).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: 'Select Dark' })).toHaveCount(0);
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Dark);
  await page.getByRole('button', { exact: true, name: 'Light' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Light);
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Light);
  await page.getByRole('button', { exact: true, name: 'Dark' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Dark);
}

async function assertAuthDialog(page: Page): Promise<void> {
  const loginButton = page.getByRole('button', { exact: true, name: 'Login' });
  await loginButton.click();
  const dialog = page.getByRole('dialog', { name: /protect the family console/iu });
  await expect(dialog).toBeVisible();
  await expect(dialog).toHaveAttribute('aria-modal', 'true');
  await expect(page.locator('.portal-unified-shell')).toHaveAttribute('inert', '');
  await expect(dialog.getByRole('textbox', { exact: true, name: 'Email' })).toBeDisabled();
  await expect(dialog.getByLabel('Password', { exact: true })).toBeDisabled();
  const closeButton = dialog.getByRole('button', { exact: true, name: 'Close parent sign in' });
  await expect(closeButton).toBeVisible();
  await expect(page.getByText('PROTECT THE FAMILY CONSOLE')).toBeVisible();
  await expect(page.getByText('Parent identity is not connected on this device yet.')).toBeVisible();
  const signInButtons = dialog.getByRole('button', { exact: true, name: 'SIGN IN' });
  await expect(signInButtons.first()).toBeFocused();
  await expect(signInButtons.last()).toBeDisabled();
  await expect(dialog.getByRole('button', { exact: true, name: 'SIGN UP' })).toHaveCount(0);
  await expect(dialog.getByRole('button', { exact: true, name: 'Facebook' })).toHaveCount(0);
  await expect(dialog.getByRole('button', { exact: true, name: 'Google' })).toHaveCount(0);
  await expect(dialog.getByRole('button', { exact: true, name: 'Guest' })).toHaveCount(0);
  await page.keyboard.press('Tab');
  await expect(closeButton).toBeFocused();
  await page.keyboard.press('Shift+Tab');
  await expect(signInButtons.first()).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  await expect(loginButton).toBeFocused();
  await loginButton.click();
  await page.getByRole('button', { exact: true, name: 'Close parent sign in' }).click();
  await expect(dialog).toHaveCount(0);
  await expect(loginButton).toBeFocused();
}

async function assertCommandControls(page: Page): Promise<void> {
  await expectCommandControlEnabled(page, 'Check health');
  await expectCommandControlEnabled(page, 'Get log snapshot');
  await expectCommandControlEnabled(page, 'Send connectivity check');
  await expectCommandControlEnabled(page, 'Refresh browser watcher');
  await expectCommandControlEnabled(page, 'Refresh activity ingest');
  await expectCommandControlEnabled(page, 'Refresh recent activity');
  await expectCommandControlEnabled(page, 'Refresh web evidence');
  await expectCommandControlEnabled(page, 'Refresh memory links');
  await expectCommandControlEnabled(page, 'Build daily activity report');
  await expectCommandControlEnabled(page, 'Refresh activity report history');
  await expectCommandControlEnabled(page, 'Refresh activity screen');
  await expectCommandControlEnabled(page, 'Refresh activity app use');
  await expectCommandControlEnabled(page, 'Refresh activity browser');
  await expectCommandControlEnabled(page, 'Refresh activity games');
  await expectCommandControlEnabled(page, 'Refresh activity network');
  await expectCommandControlEnabled(page, 'Refresh browser protection');
  await expectCommandControlEnabled(page, 'Refresh managed browser');
  await expectCommandControlEnabled(page, 'Refresh network activity');
  await expectCommandControlEnabled(page, 'Refresh local AI');
  await expectCommandControlEnabled(page, 'Refresh policy decision');
  await expect(page.getByRole('heading', { name: 'Command result' })).toBeVisible();
  await expect(page.locator('.summary')).toHaveCount(1);
}

async function assertInitialCommandRouteDrain(page: Page): Promise<void> {
  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText(ParentAgentEvent.LogSnapshotReported)).toHaveCount(1, {
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertTabbedCommandResults(page: Page): Promise<void> {
  const commandResult = page.locator('.command-result-panel');
  await clickCommandControl(page, 'Check health');
  await clickCommandControl(page, 'Check health');
  await expectCommandResultEvent(commandResult, 'agent.health.reported');
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Get log snapshot');
  await clickCommandControl(page, 'Get log snapshot');
  await expectCommandResultEvent(commandResult, 'agent.log.snapshot.reported');
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(0);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Send connectivity check');
  await expectCommandResultEvent(commandResult, 'agent.dev.echoed');
  await clickCommandControl(page, 'Send connectivity check');
  await expectCommandResultEvent(commandResult, 'agent.dev.echoed');
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Refresh browser watcher');
  await clickCommandControl(page, 'Refresh browser watcher');
  await expectCommandResultEvent(commandResult, 'agent.watch.status.reported');
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Refresh activity ingest');
  await clickCommandControl(page, 'Refresh activity ingest');
  await expectCommandResultEvent(commandResult, 'agent.activity.ingest.status.reported');
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Refresh recent activity');
  await clickCommandControl(page, 'Refresh recent activity');
  await expectCommandResultEvent(commandResult, 'agent.activity.recent.summary.reported');
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await assertActivityReadModelResults(page, commandResult);
  await assertCommandResult(
    page,
    commandResult,
    'Refresh browser protection',
    'agent.browser.intervention.read-model.reported'
  );
  await assertCommandResult(page, commandResult, 'Refresh managed browser', 'agent.browser.managed.status.reported');
  await assertNetworkFlowResult(page, commandResult);
  await assertCommandResult(page, commandResult, 'Refresh local AI', 'agent.local-ai.runtime.status.reported');
  await assertCommandResult(page, commandResult, 'Refresh policy decision', 'agent.policy.preview.read-model.reported');
  await assertHealthResultForCopy(page, commandResult);
  await assertCopyButton(page, commandResult, 'agent.health.reported');
}

async function assertHealthResultForCopy(page: Page, commandResult: Locator): Promise<void> {
  await clickCommandControl(page, 'Check health');
  await expectCommandResultEvent(commandResult, 'agent.health.reported');
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertNetworkFlowResult(page: Page, commandResult: Locator): Promise<void> {
  await clickCommandControl(page, 'Refresh network activity');
  await expectCommandResultEvent(commandResult, 'agent.network.flow.read-model.reported');
  await clickCommandControl(page, 'Refresh network activity');
  await expectCommandResultEvent(commandResult, 'agent.network.flow.read-model.reported');
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertActivityReadModelResults(page: Page, commandResult: Locator): Promise<void> {
  await assertCommandResult(page, commandResult, 'Refresh web evidence', 'agent.browser.evidence.recent.reported');
  await assertCommandResult(page, commandResult, 'Refresh memory links', 'agent.activity.memory-graph.reported');
  await assertActivityCommandResult(
    page,
    commandResult,
    'Build daily activity report',
    'agent.activity.report.generated'
  );
  await assertActivityCommandResult(
    page,
    commandResult,
    'Refresh activity report history',
    'agent.activity.report.history.reported'
  );
  await assertActivityCommandResult(
    page,
    commandResult,
    'Refresh activity screen',
    'agent.activity.screen.read-model.reported'
  );
  await assertActivityCommandResult(
    page,
    commandResult,
    'Refresh activity app use',
    'agent.activity.app-use.read-model.reported'
  );
  await assertActivityCommandResult(
    page,
    commandResult,
    'Refresh activity browser',
    'agent.activity.browser.read-model.reported'
  );
  await assertActivityCommandResult(
    page,
    commandResult,
    'Refresh activity games',
    'agent.activity.games.read-model.reported'
  );
  await assertActivityCommandResult(
    page,
    commandResult,
    'Refresh activity network',
    'agent.activity.network.read-model.reported'
  );
}

async function assertRawEventLog(page: Page): Promise<void> {
  await page.goto('/#/events');
  await expect(page.getByRole('heading', { name: 'Device audit' })).toBeVisible();
  await assertRawEventPresent(page, 'agent.connection.ready');
  await assertRawEventPresent(page, 'agent.health.reported');
  await assertRawEventPresent(page, 'agent.log.snapshot.reported');
  await assertRawEventPresent(page, 'agent.dev.echoed');
  await assertRawEventPresent(page, 'agent.watch.status.reported');
  await assertRawEventPresent(page, 'agent.activity.ingest.status.reported');
  await assertRawEventPresent(page, 'agent.activity.recent.summary.reported');
  await assertRawEventPresent(page, 'agent.browser.evidence.recent.reported');
  await assertRawEventPresent(page, 'agent.activity.memory-graph.reported');
  await assertRawEventPresent(page, 'agent.activity.report.generated');
  await assertRawEventPresent(page, 'agent.activity.report.history.reported');
  await assertRawEventPresent(page, 'agent.activity.screen.read-model.reported');
  await assertRawEventPresent(page, 'agent.activity.app-use.read-model.reported');
  await assertRawEventPresent(page, 'agent.activity.browser.read-model.reported');
  await assertRawEventPresent(page, 'agent.activity.games.read-model.reported');
  await assertRawEventPresent(page, 'agent.activity.network.read-model.reported');
  await assertRawEventPresent(page, 'agent.browser.intervention.read-model.reported');
  await assertRawEventPresent(page, 'agent.browser.managed.status.reported');
  await assertRawEventPresent(page, 'agent.network.flow.read-model.reported');
  await assertRawEventPresent(page, 'agent.local-ai.runtime.status.reported');
  await assertRawEventPresent(page, 'agent.policy.preview.read-model.reported');
}

async function assertRawEventPresent(page: Page, eventName: string): Promise<void> {
  await expect(page.getByText(eventName).first()).toBeVisible();
}

async function assertCommandResult(
  page: Page,
  commandResult: Locator,
  commandName: string,
  eventName: string
): Promise<void> {
  await clickCommandControl(page, commandName);
  await expectCommandResultEvent(commandResult, eventName);
  await clickCommandControl(page, commandName);
  await expectCommandResultEvent(commandResult, eventName);
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function expectCommandResultEvent(commandResult: Locator, eventName: string): Promise<void> {
  await expect(commandResult.getByText(eventName)).toHaveCount(1, {
    timeout: portalShellReadyTimeoutMs,
  });
}

async function assertActivityCommandResult(
  page: Page,
  commandResult: Locator,
  commandName: string,
  eventName: string
): Promise<void> {
  await assertCommandResult(page, commandResult, commandName, eventName);
  await expect(commandResult.getByText('activitySurfaceState')).toHaveCount(1);
}

async function assertOverview(page: Page): Promise<void> {
  await page.goto('/#/overview');
  await expect(page.getByText('Current device state').first()).toBeVisible();
  await expect(page.getByRole('heading', { exact: true, name: 'OVERVIEW parent controls' })).toBeVisible();
  await expect(page.getByText('WHAT PARENTS CONTROL').first()).toBeVisible();
  await expect(page.getByText('DATA CUSTODY').first()).toBeVisible();
  await expect(page.getByText('CONTROL AUTHORITY').first()).toBeVisible();
  await expect(page.getByText('READ ONLY').first()).toBeVisible();

  await page.getByRole('button', { exact: true, name: 'OPEN START HERE' }).click();
  await expect(page).toHaveURL(/#\/start$/);
  await expect(page.getByText('START HERE').first()).toBeVisible();
  await page.goto('/#/overview');
}

async function assertResponsiveParentSurface(page: Page): Promise<void> {
  const originalViewport = page.viewportSize() ?? { width: 1280, height: 720 };
  const manageToggle = page.locator('[aria-label$="MANAGE"]');
  if ((await manageToggle.getAttribute('aria-label'))?.startsWith('Expand')) {
    await manageToggle.click();
  }
  const dataRoute = page.locator('[aria-label="Open DATA"]');
  await expect(dataRoute).not.toHaveAttribute('aria-hidden', 'true');
  await expect(dataRoute).toHaveAttribute('tabindex', '0');
  await dataRoute.click();
  await expect(page).toHaveURL(/#\/drive-connections$/u);

  await page.setViewportSize({ width: 319, height: 844 });
  await page.goto('/#/overview');
  await expect(page.getByRole('heading', { exact: true, name: 'OVERVIEW parent controls' })).toBeAttached();
  const sectionSelect = page.getByRole('combobox', { exact: true, name: 'Choose parent portal section' });
  await expect(sectionSelect).toBeVisible();
  await sectionSelect.selectOption('#/drive-connections');
  await expect(page).toHaveURL(/#\/drive-connections$/u);
  await expect(page.locator('.parent-portal-study-side-pane')).toHaveCount(0);

  await assertMobileHeaderFits(page);
  await assertMobilePolicySurface(page);
  await assertMobileRouteNavigationResetsScroll(page);
  await assertMobileDeviceSurface(page);
  await assertMobileActivityReportSurface(page);

  await page.setViewportSize(originalViewport);
  await page.goto('/#/overview');
  await expect(page.getByRole('button', { exact: true, name: 'Focus parent control selector' })).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: 'Expand parent detail panel' })).toBeVisible();
}

async function assertMobileHeaderFits(page: Page): Promise<void> {
  const headerBox = await requiredBoundingBox(page.locator('.portal-outline-header'), 'mobile header');
  const brandBox = await requiredBoundingBox(page.locator('.portal-outline-header__brand'), 'mobile header brand');
  const brandLogoBox = await requiredBoundingBox(
    page.locator('.portal-outline-header__brand-logo-mount'),
    'mobile header logo'
  );
  const themeBox = await requiredBoundingBox(page.locator('.portal-outline-header__theme'), 'mobile theme control');
  expectBoxInside(brandBox, headerBox);
  expectBoxInside(brandLogoBox, brandBox);
  expectBoxInside(themeBox, headerBox);
  expect(rectanglesOverlap(themeBox, brandBox)).toBe(false);
  await expect(page.locator('.portal-outline-header__brand-part')).toBeHidden();
  await expect(page.locator('.portal-outline-header__brand-part-muted')).toBeHidden();
}

async function assertMobilePolicySurface(page: Page): Promise<void> {
  await page.goto('/#/policy-apps');
  const route = page.locator('.parent-portal-route');
  const policyPanel = page.getByRole('region', { exact: true, name: 'App policy controls unavailable' });
  await expect(policyPanel).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  const routeScrollMetrics = await route.evaluate((element) => ({
    clientHeight: element.clientHeight,
    scrollHeight: element.scrollHeight,
  }));
  expect(routeScrollMetrics.scrollHeight).toBeGreaterThan(routeScrollMetrics.clientHeight);
  const routeBox = await requiredBoundingBox(route, 'mobile policy route');
  const policyPanelBox = await requiredBoundingBox(policyPanel, 'mobile policy panel');
  expectBoxInside(policyPanelBox, routeBox);
  const policyActionBoxes = await Promise.all(
    ['Open app activity', 'Open rules', 'Open approvals'].map((actionName) =>
      requiredBoundingBox(policyPanel.getByRole('button', { exact: true, name: actionName }), actionName)
    )
  );
  policyActionBoxes.forEach((actionBox) => expectBoxInside(actionBox, policyPanelBox));
}

async function assertMobileDeviceSurface(page: Page): Promise<void> {
  await page.goto('/#/devices');
  const deviceSurface = page.locator('svg.parent-portal-svg-surface');
  const deviceSurfaceBox = await requiredBoundingBox(deviceSurface, 'mobile device surface');
  const deviceStatus = page.getByRole('status', { exact: true, name: 'Device discovery unavailable' });
  await expect(deviceStatus).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  expectBoxInside(await requiredBoundingBox(deviceStatus, 'mobile device unavailable status'), deviceSurfaceBox);
  await expect(deviceSurface.locator('text').filter({ hasText: /^LAN$/u })).not.toHaveCount(0);
  const [lanScopeBox, parentScopeBox] = await Promise.all([
    requiredBoundingBox(page.getByRole('button', { exact: true, name: 'Select LAN Devices' }), 'Select LAN Devices'),
    requiredBoundingBox(
      page.getByRole('button', { exact: true, name: 'Select Parent Portal' }),
      'Select Parent Portal'
    ),
  ]);
  [lanScopeBox, parentScopeBox].forEach((scopeBox) => expectBoxInside(scopeBox, deviceSurfaceBox));
  expect(rectanglesOverlap(lanScopeBox, parentScopeBox)).toBe(false);
  const deviceTabBoxes = await Promise.all(
    ['Show LAN pairing Info', 'Show LAN pairing Pair', 'Show LAN pairing Update', 'Show LAN pairing Capability'].map(
      (tabName) => requiredBoundingBox(page.getByRole('tab', { exact: true, name: tabName }), tabName)
    )
  );
  deviceTabBoxes.forEach((tabBox) => expectBoxInside(tabBox, deviceSurfaceBox));
}

async function assertMobileActivityReportSurface(page: Page): Promise<void> {
  await page.goto('/#/activity');
  const activitySurface = page.locator('svg.parent-portal-svg-surface');
  const activitySurfaceBox = await requiredBoundingBox(activitySurface, 'mobile activity surface');
  await page.getByRole('tab', { exact: true, name: 'Show activity Reports' }).click();

  const reportListHeading = activitySurface
    .locator('text')
    .filter({ hasText: /^Reports$/u })
    .last();
  const reportViewerHeading = activitySurface.locator('text').filter({ hasText: /^Report viewer$/u });
  const selectedReportHeading = activitySurface.locator('text').filter({ hasText: /^SELECTED REPORT$/u });
  const generateReport = page.getByRole('button', {
    exact: true,
    name: 'Generate Daily activity report',
  });
  const saveReport = page.getByRole('button', {
    exact: true,
    name: 'Save generated activity report',
  });
  await expect(reportListHeading).toBeVisible();
  await expect(reportViewerHeading).toBeVisible();
  await expect(selectedReportHeading).toBeVisible();
  await expect(generateReport).toBeVisible();
  await expect(saveReport).toBeVisible();

  const [reportListBox, reportViewerBox, selectedReportBox, generateReportBox, saveReportBox] = await Promise.all([
    requiredBoundingBox(reportListHeading, 'mobile activity report list heading'),
    requiredBoundingBox(reportViewerHeading, 'mobile activity report viewer heading'),
    requiredBoundingBox(selectedReportHeading, 'mobile activity selected report heading'),
    requiredBoundingBox(generateReport, 'mobile activity generate report action'),
    requiredBoundingBox(saveReport, 'mobile activity save report action'),
  ]);
  [reportListBox, reportViewerBox, selectedReportBox, generateReportBox, saveReportBox].forEach((box) =>
    expectBoxInside(box, activitySurfaceBox)
  );
  expect(reportViewerBox.y).toBeGreaterThan(reportListBox.y);
  expect(selectedReportBox.y).toBeGreaterThan(reportViewerBox.y);
  expect(rectanglesOverlap(generateReportBox, saveReportBox)).toBe(false);
}

async function assertMobileRouteNavigationResetsScroll(page: Page): Promise<void> {
  const route = page.locator('.parent-portal-route');
  await route.evaluate((element) => {
    element.scrollTo({ behavior: 'auto', left: element.scrollWidth, top: element.scrollHeight });
  });
  await expect.poll(() => route.evaluate((element) => element.scrollTop)).toBeGreaterThan(0);

  await page.getByRole('combobox', { exact: true, name: 'Choose parent portal section' }).selectOption('#/devices');
  await expect(page).toHaveURL(/#\/devices$/u);
  await expect
    .poll(() => route.evaluate((element) => ({ left: element.scrollLeft, top: element.scrollTop })))
    .toEqual({ left: 0, top: 0 });
  await expect
    .poll(() => page.evaluate(() => ({ left: window.scrollX, top: window.scrollY })))
    .toEqual({
      left: 0,
      top: 0,
    });
}

async function assertManageRouteRequiresExplicitDeviceSelection(page: Page): Promise<void> {
  await page.goto('/#/browser-settings');
  await expect(page.getByText('Per Device').first()).toBeVisible();
  await expect(page.getByText('Browser target: Whole family').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  await page.evaluate(
    (storageKey) => window.sessionStorage.setItem(storageKey, '{"scope":'),
    manageTargetSelectionStorageKey
  );
  await page.reload();
  await expect(page.getByText('Per Device').first()).toBeVisible();
  await expect(page.getByText('Browser target: Whole family').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await page.evaluate((storageKey) => window.sessionStorage.removeItem(storageKey), manageTargetSelectionStorageKey);
  await page.reload();
}

async function assertPolicyMutationControlsFailClosed(page: Page): Promise<void> {
  await page.goto('/#/policy-apps');
  const policyPanel = page.getByRole('region', { exact: true, name: 'App policy controls unavailable' });
  await expect(policyPanel).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(policyPanel).toContainText('No service-reported app policy editor is connected.');

  for (const label of ['Off', 'On', 'Emergency allow', 'Emergency block', 'Observe']) {
    await expect(page.getByRole('button', { exact: true, name: label })).toHaveCount(0);
  }
  await expect(page.getByText('CURRENT POLICY NOT SHOWN HERE', { exact: true })).toHaveCount(0);
  await expect(policyPanel.getByRole('button', { exact: true, name: 'Open app activity' })).toBeVisible();
  await expect(policyPanel.getByRole('button', { exact: true, name: 'Open rules' })).toBeVisible();
}

async function assertAssistantRouteOwnershipAndUnavailableAffordances(page: Page): Promise<void> {
  await assertAssistantUnavailableControls(page);
  await assertCompactAssistantRecoveryLayout(page);
}

async function assertAssistantUnavailableControls(page: Page): Promise<void> {
  await page.goto('/#/assistant');
  await expect(page.getByRole('heading', { exact: true, level: 1, name: 'AI parent controls' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  for (const label of ['Attach context to MIA', 'Use voice input for MIA']) {
    const unavailableControl = page.getByRole('button', { exact: true, name: label });
    await expect(unavailableControl).toBeVisible();
    await expect(unavailableControl).toHaveAttribute('aria-disabled', 'true');
    await expect(unavailableControl).toHaveAttribute('tabindex', '-1');
  }

  const messageInput = page.getByRole('textbox', { exact: true, name: 'Message MIA' });
  await expect(messageInput).toBeDisabled();
  await expect(messageInput).toHaveAttribute('placeholder', 'Connect the local service to use MIA.');
  await expect(
    page
      .getByText('MIA is unavailable because the local service is not connected. Open Start Here to reconnect.')
      .last()
  ).toBeVisible();
  const sendControl = page.getByRole('button', { exact: true, name: 'Send message to MIA' });
  await expect(sendControl).toHaveAttribute('aria-disabled', 'true');
  await expect(sendControl).toHaveAttribute('tabindex', '-1');
  const newChatControl = page.getByRole('button', { exact: true, name: 'Start new MIA chat' });
  await expect(newChatControl).toHaveAttribute('aria-disabled', 'true');
  await expect(newChatControl).toHaveAttribute('tabindex', '-1');
  const quickAction = page.getByRole('button', { name: /^Ask MIA about /u }).first();
  await expect(quickAction).toHaveAttribute('aria-disabled', 'true');
  await expect(quickAction).toHaveAttribute('tabindex', '-1');
  await expect(page.getByText('I will pass this to MIA with the current chat context.')).toHaveCount(0);
  const recoveryControl = page.getByRole('button', { exact: true, name: 'Open Start Here to reconnect MIA' });
  await expect(recoveryControl).toBeVisible();
}

async function assertCompactAssistantRecoveryLayout(page: Page): Promise<void> {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/#/assistant');
  const assistantSurface = page.locator('svg.parent-portal-svg-surface');
  await expect(assistantSurface).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  const compactTitleBox = await requiredBoundingBox(
    assistantSurface.locator('[data-ocentra-assistant-header-title="true"]'),
    'compact assistant title'
  );
  const compactCloseBox = await requiredBoundingBox(
    page.getByRole('button', { exact: true, name: 'Close parent assistant' }),
    'compact assistant close control'
  );
  const compactRecoveryBox = await requiredBoundingBox(
    page.getByRole('button', { exact: true, name: 'Open Start Here to reconnect MIA' }),
    'compact assistant recovery control'
  );
  expect(rectanglesOverlap(compactTitleBox, compactCloseBox)).toBe(false);
  expect(rectanglesOverlap(compactTitleBox, compactRecoveryBox)).toBe(false);
  expect(rectanglesOverlap(compactCloseBox, compactRecoveryBox)).toBe(false);

  const compactRecoveryControl = page.getByRole('button', {
    exact: true,
    name: 'Open Start Here to reconnect MIA',
  });
  await compactRecoveryControl.click();
  await expect(page).toHaveURL(/#\/start$/u);
  await expect(page.getByRole('heading', { exact: true, name: 'Setup-first-run boundary status' })).toBeVisible();
}

async function assertReportActionsReachRealAgent(page: Page): Promise<void> {
  await page.goto('/#/activity');
  await expect(page.getByRole('button', { exact: true, name: 'Scan Local Area Network' })).toHaveCount(0);
  await page.getByRole('button', { exact: true, name: 'Select Weekly' }).click();

  const generateReport = page.getByRole('button', { exact: true, name: 'Generate Weekly activity report' });
  await expect(generateReport).toHaveAttribute('aria-disabled', 'false');
  await expect(generateReport).toHaveAttribute('tabindex', '0');
  await generateReport.click();
  await expect(page.getByRole('button', { name: /Open activity-report-weekly-local/iu }).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(/^Draft JSON: activity-report-weekly-local/iu)).toBeVisible();

  const saveReport = page.getByRole('button', { exact: true, name: 'Save generated activity report' });
  await expect(saveReport).toHaveAttribute('aria-disabled', 'false');
  await expect(saveReport).toHaveAttribute('tabindex', '0');
  await saveReport.click();
  await expect(page.getByText(/^Saved JSON: activity-report-weekly-local/iu)).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
}

async function assertCopyButton(page: Page, commandResult: Locator, eventName: string): Promise<void> {
  await commandResult.getByRole('button', { name: 'Copy result' }).click();
  await expect(commandResult.getByRole('button', { name: 'Copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain(eventName);
  expect(copiedText).toContain('"payload"');
}
