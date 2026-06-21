import { expect, test } from '@playwright/test';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalRoute } from '@ocentra-parent/portal-domain/routes';
import { NetworkEvidenceDrawerProof } from '../tests/network-evidence-drawer-proof-fixture';

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
  await expect(commandResult.getByText(NetworkEvidenceDrawerProof.evidenceId)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  await page.evaluate(
    ([hashPrefix, route]) => {
      window.location.hash = `${hashPrefix}${route}`;
    },
    [PortalDom.HashPrefix, PortalRoute.NetworkActivity] as const
  );
  const networkPanel = page.getByRole('region', { name: 'Network activity' });
  await expect(networkPanel).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText(NetworkEvidenceDrawerProof.evidenceId)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(networkPanel.getByText(NetworkEvidenceDrawerProof.journalEvidenceId)).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(networkPanel.getByText('Exact URL claim')).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(networkPanel.getByText('Not reported').first()).toBeVisible({ timeout: shellReadyTimeoutMs });
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
