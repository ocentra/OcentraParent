import { expect, test, type Locator, type Page } from '@playwright/test';

test('portal UI connects to the real agent and renders command results', async ({ context, page }) => {
  const browserFailures = collectBrowserFailures(page);

  await context.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: 'http://127.0.0.1:4490' });
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { name: 'Ocentra Parent' })).toBeVisible();
  await expect(page.getByText('Local agent WebSocket command and event scaffold')).toBeVisible();

  await assertCommandControls(page);
  await assertTabbedCommandResults(page);
  await assertRawEventLog(page);
  await assertOverview(page);

  expect(browserFailures).toEqual([]);
});

async function assertCommandControls(page: Page): Promise<void> {
  await expect(page.getByRole('button', { name: 'Reconnect' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Check health' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get log snapshot' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Echo portal ping' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get watcher status' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get activity ingest status' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get recent activity summary' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get browser evidence' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Poll managed browser bridge' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get network flow' })).toBeEnabled();
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
  await page.getByRole('button', { name: 'Echo portal ping' }).click();
  await page.getByRole('button', { name: 'Echo portal ping' }).click();
  await expect(commandResult.getByText('agent.dev.echoed')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Get watcher status' }).click();
  await page.getByRole('button', { name: 'Get watcher status' }).click();
  await expect(commandResult.getByText('agent.watch.status.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Get activity ingest status' }).click();
  await page.getByRole('button', { name: 'Get activity ingest status' }).click();
  await expect(commandResult.getByText('agent.activity.ingest.status.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await page.getByRole('button', { name: 'Get recent activity summary' }).click();
  await page.getByRole('button', { name: 'Get recent activity summary' }).click();
  await expect(commandResult.getByText('agent.activity.recent.summary.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await assertCommandResult(page, commandResult, 'Get browser evidence', 'agent.browser.evidence.recent.reported');
  await assertCommandResult(
    page,
    commandResult,
    'Poll managed browser bridge',
    'agent.browser.managed.status.reported'
  );
  await assertNetworkFlowResult(page, commandResult);
  await page.getByRole('button', { name: 'Check health' }).click();
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await assertCopyButton(page, commandResult, 'agent.health.reported');
}

async function assertNetworkFlowResult(page: Page, commandResult: Locator): Promise<void> {
  await page.getByRole('button', { name: 'Get network flow' }).click();
  await page.getByRole('button', { name: 'Get network flow' }).click();
  await expect(commandResult.getByText('agent.network.flow.read-model.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertRawEventLog(page: Page): Promise<void> {
  await page.getByRole('link', { name: 'events' }).click();
  await expect(page.getByRole('heading', { name: 'Agent events' })).toBeVisible();
  await expect(page.getByText('agent.connection.ready')).toHaveCount(1);
  await expect(page.getByText('agent.health.reported')).toHaveCount(4);
  await expect(page.getByText('agent.log.snapshot.reported')).toHaveCount(3);
  await expect(page.getByText('agent.dev.echoed')).toHaveCount(2);
  await expect(page.getByText('agent.watch.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.ingest.status.reported')).toHaveCount(3);
  await expect(page.getByText('agent.activity.recent.summary.reported')).toHaveCount(3);
  await expect(page.getByText('agent.browser.evidence.recent.reported')).toHaveCount(3);
  await expect(page.getByText('agent.browser.managed.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.network.flow.read-model.reported')).toHaveCount(3);
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

async function assertOverview(page: Page): Promise<void> {
  await page.getByRole('link', { name: 'overview' }).click();
  await expect(page.getByRole('heading', { name: 'Live activity' })).toBeVisible();
  await expect(page.getByText('Agent WebSocket connected')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Evidence store' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Managed browser' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Browser evidence' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Network flow' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Recent activity' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Device diagnostics' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Activity timeline' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Service dev log' })).toBeVisible();
  await expect(
    page.locator('dt').filter({ hasText: 'Events stored' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/\d+/u);
  const recentActivity = page
    .locator('section.summary')
    .filter({ has: page.getByRole('heading', { name: 'Recent activity' }) });
  await expect(
    recentActivity.locator('dt').filter({ hasText: 'Rows returned' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/\d+/u);
  await expect(page.getByRole('heading', { name: 'Latest agent snapshot' })).toBeVisible();
  const snapshotPanel = page
    .locator('.summary')
    .filter({ has: page.getByRole('heading', { name: 'Latest agent snapshot' }) });
  await expect(
    snapshotPanel.locator('dt').filter({ hasText: 'Device' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('local-dev-agent');
  await expect(
    snapshotPanel.locator('dt').filter({ hasText: 'Version' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/\b\d+\.\d+\.\d+\b/u);
  await assertDiagnosticsCopy(page);
}

async function assertCopyButton(page: Page, commandResult: Locator, eventName: string): Promise<void> {
  await commandResult.getByRole('button', { name: 'Copy result' }).click();
  await expect(commandResult.getByRole('button', { name: 'Copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain(eventName);
  expect(copiedText).toContain('"payload"');
}

async function assertDiagnosticsCopy(page: Page): Promise<void> {
  await page.getByRole('button', { name: 'Copy diagnostics' }).click();
  await expect(page.getByRole('button', { name: 'Diagnostics copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain('"agentUrl"');
  expect(copiedText).toContain('"connectionState"');
  expect(copiedText).toContain('"events"');
  expect(copiedText).toContain('"recentSummary"');
  expect(copiedText).toContain('"networkFlowReadModel"');
}

function collectBrowserFailures(page: Page): string[] {
  const failures: string[] = [];
  page.on('console', (message) => {
    if (message.type() === 'error') {
      failures.push(message.text());
    }
  });
  page.on('pageerror', (error) => {
    failures.push(error.message);
  });
  return failures;
}
