import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders service-backed runtime event-chain refs', async ({ page }) => {
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Controls' })).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  const networkCommand = page.getByRole('button', { exact: true, name: 'Refresh network activity' });
  await expect(networkCommand).toBeEnabled({ timeout: shellReadyTimeoutMs });
  await networkCommand.click();
  await expect(page.locator('.command-result-panel').getByText('agent.network.flow.read-model.reported')).toHaveCount(
    1,
    { timeout: shellReadyTimeoutMs }
  );

  const eventChainCommand = page.getByRole('button', { exact: true, name: 'Refresh network event chain' });
  await expect(eventChainCommand).toBeEnabled({ timeout: shellReadyTimeoutMs });
  await eventChainCommand.click();
  await expect(
    page.locator('.command-result-panel').getByText('agent.network.runtime.event-chain.stream.reported')
  ).toHaveCount(1, { timeout: shellReadyTimeoutMs });

  await page.goto('/#/activity');
  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('network-runtime-ui-evidence-1')).toBeVisible();
  await expect(networkPanel.getByText(/ai\.analysis\.completed/u).first()).toBeVisible();
  await expect(networkPanel.getByText(/policy\.decision\.completed/u).first()).toBeVisible();
  await expect(networkPanel.getByText(/audit\.entry\.committed/u).first()).toBeVisible();
  await expect(networkPanel.getByText('manual-required-state')).toBeVisible();
  await expect(networkPanel.getByText('manual-required').first()).toBeVisible();
  await expect(networkPanel.getByText('Exact URL claim')).toBeVisible();
  await expect(networkPanel.getByText('Not reported').first()).toBeVisible();

  const screenshotPath = process.env['NETWORK_RUNTIME_EVENT_CHAIN_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await networkPanel.screenshot({ path: screenshotPath });
  }
});
