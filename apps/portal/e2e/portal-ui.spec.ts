import { expect, test, type Locator, type Page } from '@playwright/test';
import { collectBrowserFailures } from './browser-failures';
import { assertRouteScaffolds } from './portal-route-scaffold-assertions';

test.setTimeout(120_000);

const portalShellReadyTimeoutMs = 30_000;

test('portal UI connects to the real agent and renders command results', async ({ context, page }) => {
  const browserFailures = collectBrowserFailures(page);
  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'http://127.0.0.1:4490' });
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
  await assertTabbedCommandResults(page);
  await assertRawEventLog(page);
  await assertOverview(page);
  await assertRouteScaffolds(page);

  expect(browserFailures).toEqual([]);
});

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
  await expect(page.getByRole('button', { name: 'Reconnect' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Check health' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get log snapshot' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Send connectivity check' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh browser watcher' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity ingest' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh recent activity' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh web evidence' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh memory links' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Build daily activity report' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity report history' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity screen' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity app use' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity browser' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity games' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh activity network' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh browser protection' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh managed browser' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh network activity' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh local AI' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Refresh policy decision' })).toBeEnabled();
  await expect(page.getByRole('heading', { name: 'Command result' })).toBeVisible();
  await expect(page.locator('.summary')).toHaveCount(1);
}

async function assertTabbedCommandResults(page: Page): Promise<void> {
  const commandResult = page.locator('.command-result-panel');
  await page.getByRole('button', { name: 'Check health' }).click();
  await page.getByRole('button', { name: 'Check health' }).click();
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Get log snapshot' }).click();
  await page.getByRole('button', { name: 'Get log snapshot' }).click();
  await expect(commandResult.getByText('agent.log.snapshot.reported')).toHaveCount(1);
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(0);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Send connectivity check' }).click();
  await page.getByRole('button', { name: 'Send connectivity check' }).click();
  await expect(commandResult.getByText('agent.dev.echoed')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Refresh browser watcher' }).click();
  await page.getByRole('button', { name: 'Refresh browser watcher' }).click();
  await expect(commandResult.getByText('agent.watch.status.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Refresh activity ingest' }).click();
  await page.getByRole('button', { name: 'Refresh activity ingest' }).click();
  await expect(commandResult.getByText('agent.activity.ingest.status.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Refresh recent activity' }).click();
  await page.getByRole('button', { name: 'Refresh recent activity' }).click();
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
  await page.getByRole('button', { name: 'Check health' }).click();
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertNetworkFlowResult(page: Page, commandResult: Locator): Promise<void> {
  await page.getByRole('button', { name: 'Refresh network activity' }).click();
  await page.getByRole('button', { name: 'Refresh network activity' }).click();
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
  await expect(page.getByText('agent.connection.ready')).toHaveCount(1);
  await expect(page.getByText('agent.health.reported')).toHaveCount(4);
  await expect(page.getByText('agent.log.snapshot.reported')).toHaveCount(3);
  await expect(page.getByText('agent.dev.echoed')).toHaveCount(2);
  await expect(page.getByText('agent.watch.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.ingest.status.reported')).toHaveCount(3);
  await expect(page.getByText('agent.activity.recent.summary.reported')).toHaveCount(3);
  await expect(page.getByText('agent.browser.evidence.recent.reported')).toHaveCount(3);
  await expect(page.getByText('agent.activity.memory-graph.reported')).toHaveCount(3);
  await expect(page.getByText('agent.activity.report.generated')).toHaveCount(2);
  await expect(page.getByText('agent.activity.report.history.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.screen.read-model.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.app-use.read-model.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.browser.read-model.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.games.read-model.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.network.read-model.reported')).toHaveCount(2);
  await expect(page.getByText('agent.browser.intervention.read-model.reported')).toHaveCount(3);
  await expect(page.getByText('agent.browser.managed.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.network.flow.read-model.reported')).toHaveCount(3);
  await expect(page.getByText('agent.local-ai.runtime.status.reported')).toHaveCount(3);
  await expect(page.getByText('agent.policy.preview.read-model.reported')).toHaveCount(3);
}

async function assertCommandResult(
  page: Page,
  commandResult: Locator,
  commandName: string,
  eventName: string
): Promise<void> {
  await page.getByRole('button', { name: commandName }).click();
  await page.getByRole('button', { name: commandName }).click();
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

async function assertCopyButton(page: Page, commandResult: Locator, eventName: string): Promise<void> {
  await commandResult.getByRole('button', { name: 'Copy result' }).click();
  await expect(commandResult.getByRole('button', { name: 'Copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain(eventName);
  expect(copiedText).toContain('"payload"');
}
