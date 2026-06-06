import { expect, test, type Locator } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders broker and family-hub remote delivery status without live transport claims', async ({
  page,
}) => {
  await page.goto('/#/activity');

  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  const remoteDeliveryCard = networkPanel
    .getByRole('heading', { name: 'Remote delivery status' })
    .locator('xpath=ancestor::article[1]');
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(remoteDeliveryCard).toContainText('network.remote-delivery.status.10c', {
    timeout: shellReadyTimeoutMs,
  });
  await expect(remoteDeliveryCard).toContainText('RequirementsSatisfiedButNotImplemented');
  await expect(remoteDeliveryCard).toContainText('broker.network.custody-proof.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.publisher-auth.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.subscriber-auth.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.encryption.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.retention-policy.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.replay-plan.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.deletion-plan.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.offset-policy.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.dedupe-policy.1');
  await expect(remoteDeliveryCard).toContainText('broker.network.cross-process-replay.manual-required.10d');
  await expect(remoteDeliveryCard).toContainText('broker.network.remote-retention-delete-export.manual-required.10d');
  await expect(remoteDeliveryCard).toContainText('broker.network.config.1');
  await expect(remoteDeliveryCard).toContainText('family-hub.network.identity.1');
  await expect(remoteDeliveryCard).toContainText('family-hub.network.relay-policy.1');
  await expect(remoteDeliveryCard).toContainText('family-hub.network.delivery-ack.manual-required.10d');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Missing proof')).toHaveText('0 | 0 | 3');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Events')).toHaveText('3');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Local queue')).toHaveText('true');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Audit')).toHaveText(
    'network.remote-delivery.lifecycle-followup.10d'
  );
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Manual required')).toHaveText('true');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Transport')).toHaveText('false');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Child delivery')).toHaveText('false');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Remote sync')).toHaveText('false');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Policy authority')).toHaveText('false');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Side-effect authority')).toHaveText('false');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Enforcement command published')).toHaveText('0');
  await expect(remoteDeliveryDetailValue(remoteDeliveryCard, 'Adapter dispatch')).toHaveText('0');

  const screenshotPath = process.env['NETWORK_REMOTE_DELIVERY_STATUS_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await remoteDeliveryCard.screenshot({ path: screenshotPath });
  }
});

function remoteDeliveryDetailValue(card: Locator, label: string): Locator {
  return card.locator(`xpath=.//dt[normalize-space(.)="${label}"]/following-sibling::dd[1]`);
}
