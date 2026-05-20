import { expect, test, type Page } from '@playwright/test';

test('portal UI connects to the real agent and renders command results', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { name: 'Ocentra Parent' })).toBeVisible();
  await expect(page.getByText('Local agent WebSocket command and event scaffold')).toBeVisible();

  await expect(page.getByRole('button', { name: 'Reconnect' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Check health' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get log snapshot' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Echo portal ping' })).toBeEnabled();
  await expect(page.getByRole('button', { name: 'Get watcher status' })).toBeEnabled();

  await page.getByRole('button', { name: 'Check health' }).click();
  await expect(page.getByText('agent.health.reported')).toBeVisible();
  await page.getByRole('button', { name: 'Get log snapshot' }).click();
  await expect(page.getByText('agent.log.snapshot.reported')).toBeVisible();
  await page.getByRole('button', { name: 'Get watcher status' }).click();
  await expect(page.getByText('agent.watch.status.reported')).toBeVisible();

  await page.getByRole('link', { name: 'events' }).click();
  await expect(page.getByRole('heading', { name: 'Agent events' })).toBeVisible();
  await expect(page.getByText('agent.connection.ready')).toBeVisible();
  await expect(page.getByText('agent.health.reported')).toBeVisible();
  await expect(page.getByText('agent.log.snapshot.reported')).toBeVisible();
  await expect(page.getByText('agent.watch.status.reported')).toBeVisible();

  await page.getByRole('link', { name: 'overview' }).click();
  await expect(page.getByRole('heading', { name: 'Agent WebSocket connected' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Latest agent snapshot' })).toBeVisible();
  await expect(page.getByText('local-dev-agent')).toBeVisible();
  await expect(page.locator('dt').filter({ hasText: 'Version' }).locator('xpath=following-sibling::dd[1]')).toHaveText(
    /\b\d+\.\d+\.\d+\b/u
  );

  expect(browserFailures).toEqual([]);
});

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
