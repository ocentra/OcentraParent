import { expect, test, type Locator, type Page } from '@playwright/test';
import { PortalTheme } from '@ocentra-parent/portal-domain/contracts';
import { ParentAgentEvent } from '../../generated/parent-ui-bridge';
import { collectBrowserFailures } from './browser-failures';
import { assertLanRouteScaffolds } from './portal-route-scaffold-assertions';

test.setTimeout(420_000);

const portalShellReadyTimeoutMs = 90_000;
const defaultPortalPort = '4490';
const manageTargetSelectionStorageKey = 'ocentra.parent.portal.manage-target-selection.v1';
const devicesLanScreenshotPath = process.env['LAN_SOURCE_MATRIX_DEVICES_SCREENSHOT']?.trim() ?? '';
const policyNetworkTargetScreenshotPath = process.env['LAN_SOURCE_MATRIX_POLICY_TARGET_SCREENSHOT']?.trim() ?? '';

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
  await assertManageRouteRequiresExplicitDeviceSelection(page);
  const selectedDeviceLabel = await assertDevicesRoute(page);
  await assertSelectedDeviceContextPersistsAcrossRoutes(page, selectedDeviceLabel);
  await assertInvalidStoredDeviceContextFailsClosed(page, selectedDeviceLabel);
  await assertLanRouteScaffolds(page);

  expect(browserFailures).toEqual([]);
});

function portalOrigin(): string {
  const portalPort = process.env['OCENTRA_PARENT_PORTAL_PORT']?.trim() || defaultPortalPort;
  return `http://127.0.0.1:${portalPort}`;
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
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Dark);
  await page.getByRole('button', { exact: true, name: 'Light' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Light);
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Light);
  await page.getByRole('button', { exact: true, name: 'Dark' }).click();
  await expect(page.locator('html')).toHaveAttribute('data-theme', PortalTheme.Dark);
}

async function assertAuthDialog(page: Page): Promise<void> {
  await page.getByRole('button', { exact: true, name: 'Login' }).click();
  await expect(page.getByRole('button', { exact: true, name: 'Close parent sign in' })).toBeVisible();
  await expect(page.getByText('PROTECT THE FAMILY CONSOLE')).toBeVisible();
  await expect(page.getByText('OR CONTINUE WITH')).toBeVisible();
  await page.getByRole('button', { exact: true, name: 'Guest' }).click();
  await expect(page.getByText('Parent identity is not connected on this device yet.')).toBeVisible();
  await page.getByRole('button', { exact: true, name: 'Close parent sign in' }).dispatchEvent('click');
  await expect(page.getByRole('button', { exact: true, name: 'Close parent sign in' })).toHaveCount(0);
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
  await expect(page.getByText('WHAT PARENTS CONTROL').first()).toBeVisible();
  await expect(page.getByText('DATA CUSTODY').first()).toBeVisible();
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

async function assertDevicesRoute(page: Page): Promise<string> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  await page.goto('/#/devices');
  await expect(page.getByText('SELECTED DEVICE CONTEXT').first()).toBeVisible();
  await expect(page.getByText('SELECTED DEVICE').first()).toBeVisible();
  await expect(page.getByText('SOURCE').first()).toBeVisible();
  await expect(page.getByText('CONTROL').first()).toBeVisible();
  for (const tabName of ['Show LAN pairing Info', 'Show LAN pairing Update', 'Show LAN pairing Capability']) {
    await expect(page.getByRole('tab', { exact: true, name: tabName })).toBeVisible();
  }
  const pairTab = page.getByRole('tab', { exact: true, name: 'Show LAN pairing Pair' });
  await expect(pairTab.or(page.getByText('Policy target').first()).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const selectedDeviceLabel = await selectLanDeviceForContextProof(page, surface);
  await captureOptionalFullPageScreenshot(page, devicesLanScreenshotPath);
  return selectedDeviceLabel;
}

async function assertSelectedDeviceContextPersistsAcrossRoutes(page: Page, selectedDeviceLabel: string): Promise<void> {
  await assertSelectedDeviceContextOnManageRoute(page, '/#/browser-settings', selectedDeviceLabel, 'Browser target');
  await assertSelectedDeviceContextOnManageRoute(page, '/#/ai-runtime', selectedDeviceLabel, 'AI device');
  await assertSelectedDeviceContextOnManageRoute(page, '/#/entitlements', selectedDeviceLabel, 'Account device');
  await assertSelectedDeviceContextOnManageRoute(
    page,
    '/#/policy-network',
    selectedDeviceLabel,
    'Network target',
    policyNetworkTargetScreenshotPath
  );
  await assertSelectedDeviceContextOnActivityRoute(page, selectedDeviceLabel);
}

async function selectLanDeviceForContextProof(page: Page, surface: Locator): Promise<string> {
  const scanButton = page.getByRole('button', { name: 'Scan Local Area Network' });
  await expect(scanButton).toBeVisible();
  await scanButton.click({ force: true });
  const deviceChoice = surface.getByRole('button', { name: /^Select (?!LAN ).+/ }).first();
  await expect(deviceChoice).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  const ariaLabel = (await deviceChoice.getAttribute('aria-label')) ?? '';
  const selectedDeviceLabel = ariaLabel.replace(/^Select /u, '');
  await deviceChoice.click({ force: true });
  await expect(surface.locator('text').filter({ hasText: selectedDeviceLabel }).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  return selectedDeviceLabel;
}

async function assertSelectedDeviceContextOnManageRoute(
  page: Page,
  path: string,
  selectedDeviceLabel: string,
  expectedTargetLabel: string,
  screenshotPath = ''
): Promise<void> {
  await page.goto(path);
  await expect(page.getByText('Per Device').first()).toBeVisible();
  await expect(page.getByText(`${expectedTargetLabel}: ${selectedDeviceLabel}`).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(`${expectedTargetLabel}: No device selected`)).toHaveCount(0);
  await captureOptionalFullPageScreenshot(page, screenshotPath);
}

async function assertSelectedDeviceContextOnActivityRoute(page: Page, selectedDeviceLabel: string): Promise<void> {
  await page.goto('/#/activity');
  await expect(page.getByText(`Report device: ${selectedDeviceLabel}`).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText('Report device: Whole family')).toHaveCount(0);
  await expect(page.getByText('Report device: No device selected')).toHaveCount(0);
}

async function assertInvalidStoredDeviceContextFailsClosed(page: Page, selectedDeviceLabel: string): Promise<void> {
  const missingDeviceId = 'missing-persisted-child-device';
  const missingDeviceLabel = 'Removed persisted target';
  await page.goto('/#/browser-settings');
  await page.evaluate(
    ([storageKey, deviceId, deviceLabel]) =>
      window.sessionStorage.setItem(
        storageKey,
        JSON.stringify({ scope: 'perDevice', device: deviceLabel, deviceId, browser: 'Chrome' })
      ),
    [manageTargetSelectionStorageKey, missingDeviceId, missingDeviceLabel]
  );
  await page.reload();
  await expect(page.getByText('Browser target: No device selected').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(`Browser target: ${missingDeviceLabel}`)).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: `Select ${missingDeviceLabel}` })).toHaveCount(0);

  await page.evaluate(
    ([storageKey, deviceId, deviceLabel]) =>
      window.sessionStorage.setItem(
        storageKey,
        JSON.stringify({ scope: 'perDevice', device: deviceLabel, deviceId, browser: 'Chrome' })
      ),
    [manageTargetSelectionStorageKey, missingDeviceId, selectedDeviceLabel]
  );
  await page.reload();
  await expect(page.getByText('Browser target: No device selected').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(`Browser target: ${selectedDeviceLabel}`)).toHaveCount(0);

  await page.evaluate((storageKey) => window.sessionStorage.removeItem(storageKey), manageTargetSelectionStorageKey);
  await page.reload();
}

async function assertCopyButton(page: Page, commandResult: Locator, eventName: string): Promise<void> {
  await commandResult.getByRole('button', { name: 'Copy result' }).click();
  await expect(commandResult.getByRole('button', { name: 'Copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain(eventName);
  expect(copiedText).toContain('"payload"');
}

async function captureOptionalFullPageScreenshot(page: Page, screenshotPath: string): Promise<void> {
  if (screenshotPath.length === 0) {
    return;
  }

  await page.screenshot({ fullPage: true, path: screenshotPath });
}
