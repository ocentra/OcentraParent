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
  await expect(page.getByRole('button', { name: 'Get network flow digest' })).toBeEnabled();
  await expect(page.getByRole('heading', { name: 'Command result' })).toBeVisible();
  await expect(page.locator('.summary')).toHaveCount(1);
}

async function assertTabbedCommandResults(page: Page): Promise<void> {
  const commandResult = page.locator('.command-result-panel');
  await assertCommandResult(page, commandResult, 'Check health', 'agent.health.reported');
  await assertCommandResultReplacing(
    page,
    commandResult,
    'Get log snapshot',
    'agent.log.snapshot.reported',
    'agent.health.reported'
  );
  await assertCommandResult(page, commandResult, 'Echo portal ping', 'agent.dev.echoed');
  await assertCommandResult(page, commandResult, 'Get watcher status', 'agent.watch.status.reported');
  await assertActivityIngestStatusResult(page, commandResult);
  await assertRecentActivitySummaryResult(page, commandResult);
  await assertCommandResult(page, commandResult, 'Get browser evidence', 'agent.browser.evidence.recent.reported');
  await assertNetworkFlowResult(page, commandResult);
  await assertCommandResult(page, commandResult, 'Check health', 'agent.health.reported');
  await assertCopyButton(page, commandResult, 'agent.health.reported');
}

async function assertCommandResult(
  page: Page,
  commandResult: Locator,
  buttonName: string,
  eventName: string
): Promise<void> {
  await page.getByRole('button', { name: buttonName }).click();
  await page.getByRole('button', { name: buttonName }).click();
  await expect(commandResult.getByText(eventName)).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
}

async function assertCommandResultReplacing(
  page: Page,
  commandResult: Locator,
  buttonName: string,
  eventName: string,
  removedEventName: string
): Promise<void> {
  await assertCommandResult(page, commandResult, buttonName, eventName);
  await expect(commandResult.getByText(removedEventName)).toHaveCount(0);
}

async function assertActivityIngestStatusResult(page: Page, commandResult: Locator): Promise<void> {
  await assertCommandResult(page, commandResult, 'Get activity ingest status', 'agent.activity.ingest.status.reported');
  const ingestStatusText = await commandResult.locator('code').textContent();
  const ingestStatus = JSON.parse(ingestStatusText || '{}');
  expect(ingestStatus.eventsStored).toBeGreaterThanOrEqual(21);
}

async function assertRecentActivitySummaryResult(page: Page, commandResult: Locator): Promise<void> {
  await assertCommandResult(
    page,
    commandResult,
    'Get recent activity summary',
    'agent.activity.recent.summary.reported'
  );
  const recentSummaryText = await commandResult.locator('code').textContent();
  expect(recentSummaryText).toMatch(/"mostRecentObserver":\s*"(windows-window|windows-process|windows-network)"/u);
  expect(recentSummaryText).toMatch(
    /"mostRecentSubjectId":\s*"(window-|process-|network-destination-|network-status-)/u
  );
}

async function assertNetworkFlowResult(page: Page, commandResult: Locator): Promise<void> {
  await assertCommandResult(page, commandResult, 'Get network flow digest', 'agent.network.flow.reported');
  const networkFlowText = await commandResult.locator('code').textContent();
  const networkFlow = JSON.parse(networkFlowText || '{}');
  const digest = JSON.parse(networkFlow.activityDigest || '{}');
  expect(networkFlow.returned).toBeGreaterThanOrEqual(0);
  expect(networkFlow.custodyLabel).toBe('child-device-query-store');
  expect(digest.topProcesses.length).toBeGreaterThanOrEqual(0);
  expect(digest.topDestinations.length).toBeGreaterThanOrEqual(0);
}

async function assertRawEventLog(page: Page): Promise<void> {
  await page.getByRole('link', { name: 'events' }).click();
  await expect(page.getByRole('heading', { name: 'Agent events' })).toBeVisible();
  await expect(page.getByText('agent.connection.ready')).toHaveCount(1);
  await expect(page.getByText('agent.health.reported')).toHaveCount(5);
  await expect(page.getByText('agent.log.snapshot.reported')).toHaveCount(3);
  await expect(page.getByText('agent.dev.echoed')).toHaveCount(2);
  await expect(page.getByText('agent.watch.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.ingest.status.reported')).toHaveCount(3);
  await expect(page.getByText('agent.activity.recent.summary.reported')).toHaveCount(3);
  await expect(page.getByText('agent.browser.evidence.recent.reported')).toHaveCount(3);
  await expect(page.getByText('agent.network.flow.reported')).toHaveCount(3);
}

async function assertOverview(page: Page): Promise<void> {
  await page.getByRole('link', { name: 'overview' }).click();
  await expect(page.getByRole('heading', { name: 'Live activity' })).toBeVisible();
  await expect(page.getByText('Agent WebSocket connected')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Evidence store' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Browser evidence' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Recent activity' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Network flow' })).toBeVisible();
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
  await expect(
    recentActivity.locator('dt').filter({ hasText: 'Observer' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/^windows-window|^windows-process|^windows-network/u);
  await expect(
    recentActivity.locator('dt').filter({ hasText: 'Subject ID' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/^window-|^process-|^network-/u);
  const networkFlow = page
    .locator('section.summary')
    .filter({ has: page.getByRole('heading', { name: 'Network flow' }) });
  await expect(
    networkFlow.locator('dt').filter({ hasText: 'Rows returned' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/\d+/u);
  await expect(
    networkFlow.locator('dt').filter({ hasText: 'Custody' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('child-device-query-store');
  await expect(page.getByRole('heading', { name: 'Latest agent snapshot' })).toBeVisible();
  await expect(page.locator('dt').filter({ hasText: 'Device' }).locator('xpath=following-sibling::dd[1]')).toHaveText(
    'local-dev-agent'
  );
  await expect(page.locator('dt').filter({ hasText: 'Version' }).locator('xpath=following-sibling::dd[1]')).toHaveText(
    /\b\d+\.\d+\.\d+\b/u
  );
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
