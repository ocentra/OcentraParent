import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network evidence drawer renders service-backed digest indicators without unsupported claims', async ({
  page,
}) => {
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Controls' })).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  const networkCommand = page.getByRole('button', { exact: true, name: 'Refresh network activity' });
  await expect(networkCommand).toBeEnabled({ timeout: shellReadyTimeoutMs });

  await networkCommand.click();
  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText('agent.network.flow.read-model.reported')).toHaveCount(1, {
    timeout: shellReadyTimeoutMs,
  });

  await page.goto('/#/activity');
  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('network-digest-evidence-1')).toHaveCount(2);
  await expect(networkPanel.getByText('vpn-proxy-tunnel')).toBeVisible();
  await expect(networkPanel.getByText('encrypted-content-unavailable')).toBeVisible();
  await expect(networkPanel.getByText('unusual-unknown-process')).toBeVisible();
  await expect(networkPanel.getByText('repeated-failure')).toBeVisible();
  await expect(networkPanel.getByText('Exact URL claim')).toHaveCount(2);
  await expect(networkPanel.getByText('Not reported').first()).toBeVisible();
  await expect(networkPanel.getByText('203.0.113.80 | 8080')).toBeVisible();

  const screenshotPath = process.env['NETWORK_DIGEST_INDICATOR_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await networkPanel.screenshot({ path: screenshotPath });
  }
});
