import { expect, type Locator, type Page } from '@playwright/test';

const portalShellReadyTimeoutMs = 90_000;
const devicesLanScreenshotPath = process.env['LAN_SOURCE_MATRIX_DEVICES_SCREENSHOT']?.trim() ?? '';
const policyNetworkTargetScreenshotPath = process.env['LAN_SOURCE_MATRIX_POLICY_TARGET_SCREENSHOT']?.trim() ?? '';

export const manageTargetSelectionStorageKey = 'ocentra.parent.portal.manage-target-selection.v1';

export async function assertManageRoutesRenderTheirOwnedWorkspace(page: Page): Promise<void> {
  const exactRoutes = [
    {
      path: '/#/approvals',
      text: 'Approvals status unavailable',
      rejectedText: 'Approvals: request matrix',
    },
    {
      path: '/#/enforcement',
      text: 'Enforcement status unavailable',
      rejectedText: '1. Should enforcement policy be active?',
    },
    {
      path: '/#/rule-management',
      text: 'Rule management status unavailable',
      rejectedText: '1. Should family rules be active?',
    },
    {
      path: '/#/api-providers',
      text: 'No owner-backed family external AI provider, key, budget, or raw-evidence policy state is available.',
      rejectedText: 'Configure family external AI providers, budgets, and no-raw-evidence controls.',
    },
    {
      path: '/#/memory-settings',
      text: 'No service-reported family cited-memory registry, review state, export state, or audit state is available.',
      rejectedText: 'Review family cited memory, revoke/export controls, and memory audit state.',
    },
    {
      path: '/#/drive-connections',
      text: 'CONNECTOR STATE',
      rejectedText: 'JSON first',
    },
    {
      path: '/#/export-retention',
      text: 'RETENTION SNAPSHOT',
      rejectedText: 'Parent policy',
    },
    {
      path: '/#/audit-history',
      text: 'AUDIT HISTORY',
      rejectedText: 'Logged',
    },
    {
      path: '/#/remote-access',
      text: 'Remote access unavailable',
      rejectedText: 'CURRENT SELECTION',
    },
  ] as const;

  for (const route of exactRoutes) {
    await page.goto(route.path);
    await expect(page.getByText(route.text, { exact: true }).first()).toBeVisible({
      timeout: portalShellReadyTimeoutMs,
    });
    await expect(page.getByText(route.rejectedText, { exact: true })).toHaveCount(0);
  }

  await page.goto('/#/rule-management');
  const ruleStatus = page.getByRole('region', { exact: true, name: 'Rule management status unavailable' });
  await expect(ruleStatus).toBeVisible();
  await expect(ruleStatus.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
  await expect(page.getByText('CURRENT POLICY NOT SHOWN HERE', { exact: true })).toHaveCount(0);
  await expect(page.getByText('3. Which policy areas should family rules cover?', { exact: true })).toHaveCount(0);
  await expect(page.locator('[role="button"][aria-label="Apps"]')).toHaveCount(0);
  await expect(page.locator('[role="button"][aria-label="Parent approval"]')).toHaveCount(0);
  await expect(page.getByText('Domain', { exact: true })).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: 'Collapse rule question' })).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: 'Open Family Rules guide' })).toHaveCount(0);

  await page.goto('/#/policy-apps');
  const appPolicy = page.getByRole('region', { exact: true, name: 'App policy controls unavailable' });
  await expect(appPolicy).toBeVisible();
  await expect(appPolicy.getByRole('button', { exact: true, name: 'Open app activity' })).toBeVisible();
  await expect(page.getByText('CURRENT POLICY NOT SHOWN HERE', { exact: true })).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: 'Open apps rules guide' })).toHaveCount(0);
}

export async function assertDevicesRoute(page: Page): Promise<string> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  await page.goto('/#/devices');
  await expect(page.getByText('SELECTED DEVICE CONTEXT').first()).toBeVisible();
  await expect(page.getByText('SELECTED DEVICE').first()).toBeVisible();
  await expect(page.getByText('SOURCE').first()).toBeVisible();
  await expect(page.getByText('CONTROL').first()).toBeVisible();
  for (const tabName of ['Show LAN pairing Info', 'Show LAN pairing Update', 'Show LAN pairing Capability']) {
    await expect(page.getByRole('tab', { exact: true, name: tabName })).toBeVisible();
  }
  const pairTab = page.getByRole('tab', { exact: true, name: 'Show LAN pairing Pair' });
  await expect(pairTab.or(page.getByText('Policy target').first()).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const selectedDeviceLabel = await selectLanDeviceForContextProof(page, surface);
  await captureOptionalFullPageScreenshot(page, devicesLanScreenshotPath);
  return selectedDeviceLabel;
}

export async function assertSelectedDeviceContextPersistsAcrossRoutes(
  page: Page,
  selectedDeviceLabel: string
): Promise<void> {
  await assertSelectedDeviceContextOnManageRoute(page, '/#/browser-settings', selectedDeviceLabel, 'Browser target');
  await assertSelectedDeviceContextOnManageRoute(page, '/#/ai-runtime', selectedDeviceLabel, 'AI device');
  await assertSelectedDeviceContextOnManageRoute(page, '/#/entitlements', selectedDeviceLabel, 'Account device');
  await assertSelectedDeviceContextOnManageRoute(
    page,
    '/#/policy-network',
    selectedDeviceLabel,
    'Network target',
    policyNetworkTargetScreenshotPath
  );
  await assertSelectedDeviceContextOnActivityRoute(page, selectedDeviceLabel);
}

export async function assertInvalidStoredDeviceContextFailsClosed(
  page: Page,
  selectedDeviceLabel: string
): Promise<void> {
  const missingDeviceId = 'missing-persisted-child-device';
  const missingDeviceLabel = 'Removed persisted target';
  await page.goto('/#/browser-settings');
  await storeSelectedDeviceContext(page, missingDeviceId, missingDeviceLabel);
  await page.reload();
  await expect(page.getByText('Browser target: No device selected').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(`Browser target: ${missingDeviceLabel}`)).toHaveCount(0);
  await expect(page.getByRole('button', { exact: true, name: `Select ${missingDeviceLabel}` })).toHaveCount(0);

  await storeSelectedDeviceContext(page, missingDeviceId, selectedDeviceLabel);
  await page.reload();
  await expect(page.getByText('Browser target: No device selected').first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(`Browser target: ${selectedDeviceLabel}`)).toHaveCount(0);

  await page.evaluate((storageKey) => window.sessionStorage.removeItem(storageKey), manageTargetSelectionStorageKey);
  await page.reload();
}

async function selectLanDeviceForContextProof(page: Page, surface: Locator): Promise<string> {
  const scanButton = page.getByRole('button', { name: 'Scan Local Area Network' });
  await expect(scanButton).toBeVisible();
  await scanButton.click({ force: true });
  const deviceChoice = surface.getByRole('button', { name: /^Select (?!LAN ).+/ }).first();
  await expect(deviceChoice).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  const ariaLabel = (await deviceChoice.getAttribute('aria-label')) ?? '';
  const selectedDeviceLabel = ariaLabel.replace(/^Select /u, '');
  await deviceChoice.click({ force: true });
  await expect(surface.locator('text').filter({ hasText: selectedDeviceLabel }).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  return selectedDeviceLabel;
}

async function assertSelectedDeviceContextOnManageRoute(
  page: Page,
  path: string,
  selectedDeviceLabel: string,
  expectedTargetLabel: string,
  screenshotPath = ''
): Promise<void> {
  await page.goto(path);
  await expect(page.getByText('Per Device').first()).toBeVisible();
  await expect(page.getByText(`${expectedTargetLabel}: ${selectedDeviceLabel}`).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText(`${expectedTargetLabel}: No device selected`)).toHaveCount(0);
  await captureOptionalFullPageScreenshot(page, screenshotPath);
}

async function assertSelectedDeviceContextOnActivityRoute(page: Page, selectedDeviceLabel: string): Promise<void> {
  await page.goto('/#/activity');
  await expect(page.getByText(`Report device: ${selectedDeviceLabel}`).first()).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(page.getByText('Report device: Whole family')).toHaveCount(0);
  await expect(page.getByText('Report device: No device selected')).toHaveCount(0);
}

async function storeSelectedDeviceContext(page: Page, deviceId: string, deviceLabel: string): Promise<void> {
  await page.evaluate(
    ([storageKey, selectedDeviceId, selectedDeviceLabel]) => {
      if (typeof storageKey !== 'string' || storageKey.length === 0) {
        throw new Error('Portal manage-target selection storage key is required.');
      }
      window.sessionStorage.setItem(
        storageKey,
        JSON.stringify({
          scope: 'perDevice',
          device: selectedDeviceLabel,
          deviceId: selectedDeviceId,
          browser: 'Chrome',
        })
      );
    },
    [manageTargetSelectionStorageKey, deviceId, deviceLabel]
  );
}

async function captureOptionalFullPageScreenshot(page: Page, screenshotPath: string): Promise<void> {
  if (screenshotPath.length > 0) {
    await page.screenshot({ fullPage: true, path: screenshotPath });
  }
}
