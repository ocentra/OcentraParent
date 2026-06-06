import { expect, test } from '@playwright/test';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test('network drawer renders service-backed risk and performance readiness details without authority claims', async ({
  page,
}) => {
  await page.goto('/#/activity');

  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  const riskPerformanceCard = networkPanel
    .getByRole('heading', { name: 'Risk budget details' })
    .locator('xpath=ancestor::article[1]');
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(riskPerformanceCard).toContainText('network.risk-evaluation.51a', { timeout: shellReadyTimeoutMs });
  await expect(riskPerformanceCard).toContainText('child-profile.network.51a');
  await expect(riskPerformanceCard).toContainText('household-policy.network.51a');
  await expect(riskPerformanceCard).toContainText('network.cascade.51a');
  await expect(riskPerformanceCard).toContainText('UnderTwelve');
  await expect(riskPerformanceCard).toContainText('AskParentThreshold');
  await expect(riskPerformanceCard).toContainText('AskParent');
  await expect(riskPerformanceCard).toContainText('network.signal.51a');
  await expect(riskPerformanceCard).toContainText('network.audit.51a');
  await expect(riskPerformanceCard).toContainText('evidence.network.flow.1');
  await expect(riskPerformanceCard).toContainText('policy.rule.network-domain.1');
  await expect(riskPerformanceCard).toContainText('Ready');
  await expect(riskPerformanceCard).toContainText('network.performance.benchmark.51a');
  await expect(riskPerformanceCard).toContainText('network.performance.fixtures.51a');
  await expect(riskPerformanceCard).toContainText('network.performance.event-history.51a');
  await expect(riskPerformanceCard).toContainText('network.performance.resource-snapshot.51a');
  await expect(riskPerformanceCard).toContainText('MeetsBenchmarkGate');
  await expect(riskPerformanceCard).toContainText('2 | 20 | 2000 | 600 | 1200');
  await expect(riskPerformanceCard).toContainText('80 | 700 | 90 | Not reported');
  await expect(riskPerformanceCard).toContainText('3200 | 4 | 0 | 2100');
  await expect(riskPerformanceCard).toContainText('120 | 40000 | 20000');
  await expect(riskPerformanceCard).toContainText('DryRun');
  await expect(
    riskPerformanceCard.locator('dt', { hasText: 'Production SLO claim' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('false');
  await expect(
    riskPerformanceCard.locator('dt', { hasText: 'Adapter dispatch' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('false');
  await expect(
    riskPerformanceCard.locator('dt', { hasText: 'Host filtering' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('false');
  await expect(riskPerformanceCard).not.toContainText('exact URL');
  await expect(riskPerformanceCard).not.toContainText('decrypted payload');

  const screenshotPath = process.env['NETWORK_RISK_PERFORMANCE_READINESS_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await riskPerformanceCard.screenshot({ path: screenshotPath });
  }
});
