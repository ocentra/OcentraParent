import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders service-backed platform claim manifest without execution claims', async ({ page }) => {
  await page.goto('/#/activity');

  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  const platformCard = networkPanel
    .getByRole('heading', { name: 'Platform claim manifest' })
    .locator('xpath=ancestor::article[1]');
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(platformCard).toContainText('WindowsFirewall', { timeout: shellReadyTimeoutMs });
  await expect(platformCard).toContainText('WindowsWfp');
  await expect(platformCard).toContainText('Ready');
  await expect(platformCard).toContainText('ManualRequired');
  await expect(platformCard).toContainText('event.policy.decision.completed.1');
  await expect(platformCard).toContainText('policy.rule.network-domain.1');
  await expect(platformCard).toContainText('evidence.network.flow.1');
  await expect(platformCard).toContainText('device.child.windows-1');
  await expect(platformCard).toContainText('network.live-capture.permission-proof.13');
  await expect(platformCard).toContainText('adapter.capability.network.dry-run.1');
  await expect(platformCard).toContainText('network.platform-claim.manual-followup.51a');
  await expect(platformCard).toContainText('event.audit.entry.committed.1');
  await expect(platformCard.locator('dt', { hasText: 'Adapter authorized by proof' })).toHaveCount(2);
  await expect(platformCard.locator('dt', { hasText: 'Enforcement command published' })).toHaveCount(2);
  await expect(
    platformCard.locator('dt', { hasText: 'Enforcement command published' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(['false', 'false']);
  await expect(platformCard).not.toContainText('exact URL');
  await expect(platformCard).not.toContainText('decrypted payload');
  await expect(platformCard).not.toContainText('host filtering');

  const screenshotPath = process.env['NETWORK_PLATFORM_CLAIM_MANIFEST_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await platformCard.screenshot({ path: screenshotPath });
  }
});
