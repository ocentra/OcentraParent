import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders service-backed product readiness status without action claims', async ({ page }) => {
  await page.goto('/#/activity');

  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  const readinessCard = networkPanel
    .getByRole('heading', { name: 'Readiness kind' })
    .locator('xpath=ancestor::article[1]');
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(readinessCard.getByText('network.live-capture.custody-status.13a')).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(readinessCard.getByText('CustodyReady').first()).toBeVisible();
  await expect(readinessCard.getByText('ProofReady')).toBeVisible();
  await expect(readinessCard.getByText('network.product-readiness.status.51a')).toBeVisible();
  await expect(readinessCard.getByText('ManualRequired')).toBeVisible();
  await expect(readinessCard.getByText('AskParentThreshold')).toBeVisible();
  await expect(readinessCard.getByText('AskParent', { exact: true })).toBeVisible();
  await expect(readinessCard.getByText('MeetsBenchmarkGate')).toBeVisible();
  await expect(readinessCard.getByText('DryRun')).toBeVisible();
  await expect(readinessCard.getByText('Platform ready claims')).toBeVisible();
  await expect(readinessCard.getByText('Platform dry-run claims')).toBeVisible();
  await expect(readinessCard.getByText('Platform research-only claims')).toBeVisible();
  await expect(readinessCard.getByText('Platform manual-required claims')).toBeVisible();
  await expect(readinessCard.getByText('Platform unavailable claims')).toBeVisible();
  await expect(readinessCard.getByText('WindowsWfp | network.platform-claim.manual-followup.51a')).toBeVisible();
  await expect(readinessCard.getByText('Adapter dispatch')).toBeVisible();
  await expect(
    readinessCard.locator('dt', { hasText: 'Adapter dispatch' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('false');
  await expect(readinessCard.getByText('exact URL', { exact: false })).toHaveCount(0);
  await expect(readinessCard.getByText('decrypted payload', { exact: false })).toHaveCount(0);

  const screenshotPath = process.env['NETWORK_PRODUCT_READINESS_STATUS_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await readinessCard.screenshot({ path: screenshotPath });
  }
});
