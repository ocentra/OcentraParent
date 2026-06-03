import { expect, test, type Locator, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';
import { assertRouteScaffolds } from './portal-route-scaffold-assertions';

test.setTimeout(180_000);

const portalShellReadyTimeoutMs = 90_000;
const defaultPortalPort = '4490';

test('portal UI connects to the real agent and renders command results', async ({ context, page }) => {
  const browserFailures = collectBrowserFailures(page);
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: portalOrigin() });
  await page.goto('/#/commands');
  await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByRole('button', { exact: true, name: 'Login' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByRole('heading', { name: 'Controls' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  await assertAuthDialog(page);
  await assertCommandControls(page);
  await assertInitialOverviewCommandDrain(page);
  await assertTabbedCommandResults(page);
  await assertRawEventLog(page);
  await assertOverview(page);
  await assertDevicesRoute(page);
  await assertRouteScaffolds(page);

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
  await expect(commandControlButton(page, 'Reconnect')).toBeVisible();
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

async function assertInitialOverviewCommandDrain(page: Page): Promise<void> {
  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText('agent.policy.preview.read-model.reported')).toHaveCount(1, {
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertTabbedCommandResults(page: Page): Promise<void> {
  const commandResult = page.locator('.command-result-panel');
  await clickCommandControl(page, 'Check health');
  await clickCommandControl(page, 'Check health');
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Get log snapshot');
  await clickCommandControl(page, 'Get log snapshot');
  await expect(commandResult.getByText('agent.log.snapshot.reported')).toHaveCount(1);
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(0);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Send connectivity check');
  await clickCommandControl(page, 'Send connectivity check');
  await expect(commandResult.getByText('agent.dev.echoed')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Refresh browser watcher');
  await clickCommandControl(page, 'Refresh browser watcher');
  await expect(commandResult.getByText('agent.watch.status.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Refresh activity ingest');
  await clickCommandControl(page, 'Refresh activity ingest');
  await expect(commandResult.getByText('agent.activity.ingest.status.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await clickCommandControl(page, 'Refresh recent activity');
  await clickCommandControl(page, 'Refresh recent activity');
  await expect(commandResult.getByText('agent.activity.recent.summary.reported')).toHaveCount(1);
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
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertNetworkFlowResult(page: Page, commandResult: Locator): Promise<void> {
  await clickCommandControl(page, 'Refresh network activity');
  await clickCommandControl(page, 'Refresh network activity');
  await expect(commandResult.getByText('agent.network.flow.read-model.reported')).toHaveCount(1);
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
  await assertRawEventPresent(page, 'agent.lan-pairing.status.reported');
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
  await clickCommandControl(page, commandName);
  await expect(commandResult.getByText(eventName)).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
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
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Current device state' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'WHAT PARENTS CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'DATA CUSTODY' }).first()).toBeVisible();
}

async function assertDevicesRoute(page: Page): Promise<void> {
  await page.goto('/#/devices');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SELECTED DEVICE CONTEXT' }).first()).toBeVisible();
  await expect(
    surface
      .locator('text')
      .filter({ hasText: /SELECTED DEVICE/i })
      .first()
  ).toBeVisible();
  await expect(
    surface
      .locator('text')
      .filter({ hasText: /SOURCE/i })
      .first()
  ).toBeVisible();
  await expect(
    surface
      .locator('text')
      .filter({ hasText: /CONTROL/i })
      .first()
  ).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: /ROUTE/i }).first()).toBeVisible();
}

async function assertCopyButton(page: Page, commandResult: Locator, eventName: string): Promise<void> {
  await commandResult.getByRole('button', { name: 'Copy result' }).click();
  await expect(commandResult.getByRole('button', { name: 'Copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain(eventName);
  expect(copiedText).toContain('"payload"');
}
