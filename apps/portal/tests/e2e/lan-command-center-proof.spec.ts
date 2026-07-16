import { expect, test, type Locator, type Page } from '@playwright/test';

test.setTimeout(180_000);

const shellReadyTimeoutMs = 90_000;
const commandCenterScreenshotPath = process.env['LAN_COMMAND_CENTER_SCREENSHOT']?.trim() ?? '';
const devicesScreenshotPath = process.env['LAN_COMMAND_CENTER_DEVICES_SCREENSHOT']?.trim() ?? '';
const proofPanelsScreenshotPath = process.env['LAN_COMMAND_CENTER_PROOF_PANELS_SCREENSHOT']?.trim() ?? '';
const serviceBackedLanTargetName = /^Select (?!LAN |Parent Portal$).+/u;

test('existing command center and devices surfaces expose Rust-backed LAN state', async ({ page }) => {
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  const lanStatusCommand = page.getByRole('button', { exact: true, name: 'Refresh LAN pairing status' });
  await expect(lanStatusCommand).toBeEnabled({ timeout: shellReadyTimeoutMs });
  await lanStatusCommand.click();
  await lanStatusCommand.click();

  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText('agent.lan-pairing.status.reported')).toHaveCount(1, {
    timeout: shellReadyTimeoutMs,
  });
  await expect(commandResult.getByText(/"addDeviceState"|"discoveryState"|"selectedRouteId"/u)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await captureOptionalFullPageScreenshot(page, commandCenterScreenshotPath);

  await page.goto('/#/devices');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(surface.locator('text').filter({ hasText: 'SELECTED DEVICE CONTEXT' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SOURCE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'CONTROL' }).first()).toBeVisible();

  const scanButton = page.getByRole('button', { exact: true, name: 'Scan Local Area Network' });
  await expect(scanButton).toBeEnabled({ timeout: shellReadyTimeoutMs });
  await scanButton.click({ force: true });
  await expect(surface.getByRole('button', { name: serviceBackedLanTargetName }).first()).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await captureOptionalFullPageScreenshot(page, devicesScreenshotPath);

  await page.goto('/#/proof-panels');
  await page.getByRole('button', { exact: true, name: 'Network activity' }).click();
  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('LAN source matrix')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Policy targets')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Recent LAN events')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await captureOptionalPanelScreenshot(networkPanel, proofPanelsScreenshotPath);
});

async function captureOptionalFullPageScreenshot(page: Page, screenshotPath: string): Promise<void> {
  if (screenshotPath.length === 0) {
    return;
  }

  await page.screenshot({ fullPage: true, path: screenshotPath });
}

async function captureOptionalPanelScreenshot(panel: Locator, screenshotPath: string): Promise<void> {
  if (screenshotPath.length === 0) {
    return;
  }

  await panel.screenshot({ path: screenshotPath });
}
