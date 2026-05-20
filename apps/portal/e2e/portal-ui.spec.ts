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
  await expect(page.getByRole('heading', { name: 'Command result' })).toBeVisible();
  await expect(page.locator('.summary')).toHaveCount(1);

  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText('Run a command to see the latest response.')).toBeVisible();
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
  await page.getByRole('button', { name: 'Check health' }).click();
  await expect(commandResult.getByText('agent.health.reported')).toHaveCount(1);
  await expect(commandResult.locator('.log')).toHaveCount(1);
  await assertCopyButton(page, commandResult, 'agent.health.reported');
}

async function assertRawEventLog(page: Page): Promise<void> {
  await page.getByRole('link', { name: 'events' }).click();
  await expect(page.getByRole('heading', { name: 'Agent events' })).toBeVisible();
  await expect(page.getByText('agent.connection.ready')).toHaveCount(1);
  await expect(page.getByText('agent.health.reported')).toHaveCount(3);
  await expect(page.getByText('agent.log.snapshot.reported')).toHaveCount(2);
  await expect(page.getByText('agent.dev.echoed')).toHaveCount(2);
  await expect(page.getByText('agent.watch.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.ingest.status.reported')).toHaveCount(2);
  await expect(page.getByText('agent.activity.recent.summary.reported')).toHaveCount(2);
}

async function assertOverview(page: Page): Promise<void> {
  await page.getByRole('link', { name: 'overview' }).click();
  await expect(page.getByRole('heading', { name: 'Agent WebSocket connected' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Latest agent snapshot' })).toBeVisible();
  await expect(page.getByText('local-dev-agent')).toBeVisible();
  await expect(page.locator('dt').filter({ hasText: 'Version' }).locator('xpath=following-sibling::dd[1]')).toHaveText(
    /\b\d+\.\d+\.\d+\b/u
  );
}

async function assertCopyButton(page: Page, commandResult: Locator, eventName: string): Promise<void> {
  await commandResult.getByRole('button', { name: 'Copy result' }).click();
  await expect(commandResult.getByRole('button', { name: 'Copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain(eventName);
  expect(copiedText).toContain('"payload"');
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
