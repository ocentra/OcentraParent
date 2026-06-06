import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders service-backed adapter capability status without action claims', async ({ page }) => {
  await page.goto('/#/activity');

  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('v0-8-supported-adapter-runtime-proof')).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(networkPanel.getByText(/windows-network-flow-observe-policy-handoff/u)).toBeVisible();
  await expect(networkPanel.getByText(/supported-boundary-proved/u)).toBeVisible();
  await expect(networkPanel.getByText(/windows-host-network-domain-blocking-manual-gate/u)).toBeVisible();
  await expect(networkPanel.getByText(/windows-host-network-domain-artifact-status/u)).toBeVisible();
  await expect(networkPanel.getByText(/windows-managed-exact-active-tab-not-claimed/u)).toBeVisible();
  await expect(networkPanel.getByText(/windows-adapter-permission-dependency-degraded/u)).toBeVisible();
  await expect(networkPanel.getByText(/linux-host-adapter-unavailable/u)).toBeVisible();
  await expect(networkPanel.getByText(/macos-host-adapter-unsupported/u)).toBeVisible();
  await expect(networkPanel.getByText(/manual-artifact-required/u).first()).toBeVisible();
  await expect(networkPanel.getByText(/unavailable-on-target/u)).toBeVisible();
  await expect(networkPanel.getByText(/unsupported-platform/u)).toBeVisible();
  await expect(networkPanel.getByText('false')).toBeVisible();

  const screenshotPath = process.env['NETWORK_ADAPTER_CAPABILITY_STATUS_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await networkPanel.screenshot({ path: screenshotPath });
  }
});
