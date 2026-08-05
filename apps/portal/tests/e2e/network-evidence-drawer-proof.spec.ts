import { expect, test } from '@playwright/test';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentAgentEvent, ParentRoute } from '../../generated/parent-ui-bridge';
import { NetworkEvidenceDrawerProof } from '../fixtures/network/network-evidence-drawer-proof-fixture';
import { seedPortalNetworkActivityStore } from '../../../../scripts/test/portal-network-activity-seed.mjs';

test.setTimeout(120_000);

const shellReadyTimeoutMs = 90_000;

test.beforeEach('refreshes the network evidence fixture after prior live capture', () => {
  const activityDbPath = process.env['OCENTRA_PARENT_ACTIVITY_DB_PATH'];
  if (activityDbPath === undefined || activityDbPath.trim().length === 0) {
    throw new Error('Portal network evidence E2E requires OCENTRA_PARENT_ACTIVITY_DB_PATH.');
  }
  seedPortalNetworkActivityStore(activityDbPath);
});

test('network evidence drawer renders service-backed refs without unsupported claims', async ({ page }) => {
  const evidenceRefs = `${NetworkEvidenceDrawerProof.evidenceId} | ${NetworkEvidenceDrawerProof.journalEvidenceId}`;

  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText(ParentAgentEvent.LogSnapshotReported)).toHaveCount(1, {
    timeout: shellReadyTimeoutMs,
  });
  const networkCommand = page.getByRole('button', { exact: true, name: 'Refresh network activity' });
  await expect(networkCommand).toBeEnabled({ timeout: shellReadyTimeoutMs });

  await networkCommand.click();
  await expect(commandResult.getByText(ParentAgentEvent.NetworkFlowReadModelReported)).toHaveCount(1, {
    timeout: shellReadyTimeoutMs,
  });
  await expect(commandResult.getByText(NetworkEvidenceDrawerProof.evidenceId)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  await page.evaluate(
    ([hashPrefix, route]) => {
      window.location.hash = `${hashPrefix}${route}`;
    },
    [PortalDom.HashPrefix, ParentRoute.ProofPanels] as const
  );
  await page.getByRole('button', { exact: true, name: 'Network activity' }).click();
  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(
    networkPanel.getByText('Evidence references', { exact: true }).locator('xpath=following-sibling::dd[1]')
  ).toContainText(evidenceRefs, {
    timeout: shellReadyTimeoutMs,
  });
  await expect(networkPanel.getByText('Exact URL claim')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Not reported').first()).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('LAN source matrix')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Matrix coverage')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Policy targets')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Relay cache')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Recent LAN events')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText(NetworkEvidenceDrawerProof.expected.domainEvidenceRef)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(networkPanel.getByText(NetworkEvidenceDrawerProof.expected.processRef)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  const screenshotPath = process.env['NETWORK_EVIDENCE_DRAWER_SCREENSHOT'];
  if (screenshotPath !== undefined && screenshotPath.trim().length > 0) {
    await networkPanel.screenshot({ path: screenshotPath });
  }
});
