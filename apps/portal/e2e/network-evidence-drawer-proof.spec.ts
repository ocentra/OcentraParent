import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network evidence drawer renders service-backed refs without unsupported claims', async ({ page }) => {
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Controls' })).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  const networkCommand = page.getByRole('button', { exact: true, name: 'Refresh network activity' });
  await expect(networkCommand).toBeEnabled({ timeout: shellReadyTimeoutMs });

  await networkCommand.click();
  await networkCommand.click();
  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText('agent.network.flow.read-model.reported')).toHaveCount(1, {
    timeout: shellReadyTimeoutMs,
  });
  await expect(commandResult.getByText('network-ui-evidence-1')).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  await page.evaluate(() => {
    window.location.hash = '#/activity';
  });
  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('network-ui-evidence-1')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('network-ui-journal-1')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Exact URL claim')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Not reported').first()).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('example-network.test | domain-observed')).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(networkPanel.getByText('notepad.exe | 4242 | process-attributed')).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  const screenshotPath = process.env['NETWORK_EVIDENCE_DRAWER_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await networkPanel.screenshot({ path: screenshotPath });
  }
});
