import { expect, test, type Locator } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders local AI runtime result status without output or authority claims', async ({ page }) => {
  await page.goto('/#/activity');

  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  const localAiCard = networkPanel
    .getByRole('heading', { name: 'Local AI result' })
    .locator('xpath=ancestor::article[1]');
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expectLocalAiRefs(localAiCard);
  await expectLocalAiNoClaimRows(localAiCard);

  const screenshotPath = process.env['NETWORK_LOCAL_AI_RUNTIME_RESULT_STATUS_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await localAiCard.screenshot({ path: screenshotPath });
  }
});

async function expectLocalAiRefs(localAiCard: Locator): Promise<void> {
  await expect(localAiCard).toContainText('network.local-ai.runtime-result.status.33b', {
    timeout: shellReadyTimeoutMs,
  });
  await expect(localAiCard).toContainText('ResultReady');
  await expect(localAiCard).toContainText('Queued');
  await expect(localAiCard).toContainText('network.local-ai.trigger.33b');
  await expect(localAiCard).toContainText('network.local-ai.queue-job.33b');
  await expect(localAiCard).toContainText('network.local-ai.queue.33b');
  await expect(localAiCard).toContainText('network.local-ai.runtime-ref.33b');
  await expect(localAiCard).toContainText('network.local-ai.model-version.33b');
  await expect(localAiCard).toContainText('network.local-ai.prompt-template.33b');
  await expect(localAiCard).toContainText('network.local-ai.policy-context.33b');
  await expect(localAiCard).toContainText('policy.rule.network-domain.1');
  await expect(localAiCard).toContainText('network.local-ai.managed-browser-exact-url-evidence.33b');
  await expect(localAiCard).toContainText('network.local-ai.result.33b');
  await expect(localAiCard).toContainText('network.local-ai.output-summary.33b');
}

async function expectLocalAiNoClaimRows(localAiCard: Locator): Promise<void> {
  await expect(localAiDetailValue(localAiCard, 'Audit')).toHaveText('true');
  await expect(localAiDetailValue(localAiCard, 'Model output')).toHaveText('true');
  await expect(localAiDetailValue(localAiCard, 'Execution state')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Raw PCAP claim')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Exact URL claim')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Decrypted payload claim')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Page content claim')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Private message claim')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Search query claim')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Transport')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Policy authority')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Adapter dispatch')).toHaveText('false');
  await expect(localAiDetailValue(localAiCard, 'Enforcement')).toHaveText('0');
  await expect(localAiCard.getByText('raw model output', { exact: false })).toHaveCount(0);
}

function localAiDetailValue(card: Locator, label: string): Locator {
  return card.locator(`xpath=.//dt[normalize-space(.)="${label}"]/following-sibling::dd[1]`);
}
