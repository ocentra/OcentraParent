import { expect, test, type Locator, type Page } from '@playwright/test';

test.setTimeout(180_000);

const shellReadyTimeoutMs = 90_000;
const devicesLanScreenshotPath = process.env['LAN_SOURCE_MATRIX_DEVICES_SCREENSHOT']?.trim() ?? '';
const policyNetworkTargetScreenshotPath = process.env['LAN_SOURCE_MATRIX_POLICY_TARGET_SCREENSHOT']?.trim() ?? '';
const manageTargetSelectionStorageKey = 'ocentra.parent.portal.manage-target-selection.v1';
const serviceBackedLanTargetName = /^Select (?!LAN |Parent Portal$).+/u;
const lanNeighborTargetName = /^Select LAN \d{1,3}(?:\.\d{1,3}){3}$/u;

type StoredManageTargetSelection = {
  readonly device: string;
  readonly deviceId: string;
};

test('devices and policy-network surfaces preserve the selected LAN policy target', async ({ page }) => {
  const surface = page.locator('svg.parent-portal-svg-surface');

  await page.goto('/#/devices');
  await expect(surface).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(surface.locator('text').filter({ hasText: 'SELECTED DEVICE CONTEXT' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SOURCE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'CONTROL' }).first()).toBeVisible();

  const scanButton = page.getByRole('button', { exact: true, name: 'Scan Local Area Network' });
  await expect(scanButton).toBeVisible({ timeout: shellReadyTimeoutMs });
  await scanButton.click({ force: true });
  await expect(surface.getByRole('button', { name: serviceBackedLanTargetName }).first()).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });

  const selectedTarget = await selectPolicyCapableLanTarget(page, surface);

  await expect(surface).toBeVisible({ timeout: shellReadyTimeoutMs });
  await expect(surface.locator('text').filter({ hasText: 'Per Device' }).first()).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(surface.locator('text').filter({ hasText: 'Network target' }).first()).toBeVisible({
    timeout: shellReadyTimeoutMs,
  });
  await expect(surface.locator('text').filter({ hasText: 'No device selected' })).toHaveCount(0);
  expect(await readStoredManageTargetSelection(page)).toEqual(selectedTarget);
  const policySurfaceText = (await surface.textContent()) ?? '';
  expect(policySurfaceText).toContain('Network target:');
  expect(policySurfaceText).not.toContain('Network target: No device selected');
  await captureOptionalFullPageScreenshot(page, policyNetworkTargetScreenshotPath);
});

async function captureOptionalFullPageScreenshot(page: Page, screenshotPath: string): Promise<void> {
  if (screenshotPath.length === 0) {
    return;
  }

  await page.screenshot({ fullPage: true, path: screenshotPath });
}

async function selectPolicyCapableLanTarget(page: Page, surface: Locator): Promise<StoredManageTargetSelection> {
  const candidateLabels = prioritizeCandidateLabels(await listLanChoiceLabels(surface));
  expect(candidateLabels.length).toBeGreaterThan(0);

  const attempts: string[] = [];
  for (const label of candidateLabels) {
    const choice = page.getByRole('button', { exact: true, name: label }).first();
    if ((await choice.count()) === 0) {
      attempts.push(`${label}: missing-on-devices-route`);
      continue;
    }

    await choice.click({ force: true });
    await captureOptionalFullPageScreenshot(page, devicesLanScreenshotPath);
    const selection = await readStoredManageTargetSelection(page);
    if (selection === null || selection.deviceId.length === 0) {
      attempts.push(`${label}: no-stored-selection`);
      continue;
    }

    await page.goto('/#/policy-network');
    await expect(surface).toBeVisible({ timeout: shellReadyTimeoutMs });
    const policySurfaceText = (await surface.textContent()) ?? '';
    if (!policySurfaceText.includes('Network target: No device selected')) {
      return selection;
    }

    attempts.push(`${label}: policy-route-rejected ${selection.deviceId}`);
    await page.goto('/#/devices');
    await expect(surface).toBeVisible({ timeout: shellReadyTimeoutMs });
  }

  throw new Error(`No policy-capable LAN target found. Attempts: ${attempts.join(' | ')}`);
}

async function listLanChoiceLabels(surface: Locator): Promise<readonly string[]> {
  const serviceBackedLabels = await accessibleButtonNames(
    surface.getByRole('button', { name: serviceBackedLanTargetName })
  );
  const lanLabels = await accessibleButtonNames(surface.getByRole('button', { name: lanNeighborTargetName }));
  return [...new Set([...serviceBackedLabels, ...lanLabels])].filter(
    (label) => label !== 'Select LAN' && label !== 'Select Scanning LAN'
  );
}

function prioritizeCandidateLabels(labels: readonly string[]): readonly string[] {
  return [...labels].sort((left, right) => candidateRank(left) - candidateRank(right) || left.localeCompare(right));
}

function candidateRank(label: string): number {
  if (/^Select LAN \d{1,3}(?:\.\d{1,3}){3}$/u.test(label)) {
    return 3;
  }
  if (/^Select LAN /u.test(label)) {
    return 2;
  }
  return 1;
}

async function readStoredManageTargetSelection(page: Page): Promise<StoredManageTargetSelection | null> {
  return page.evaluate((storageKey) => {
    const raw = globalThis.window.sessionStorage.getItem(storageKey);
    if (!raw) {
      return null;
    }
    try {
      const parsed = JSON.parse(raw) as { device?: unknown; deviceId?: unknown };
      return {
        device: typeof parsed.device === 'string' ? parsed.device : '',
        deviceId: typeof parsed.deviceId === 'string' ? parsed.deviceId : '',
      };
    } catch {
      return null;
    }
  }, manageTargetSelectionStorageKey);
}

async function accessibleButtonNames(locator: Locator): Promise<readonly string[]> {
  return locator.evaluateAll((buttons) =>
    buttons
      .map((button) => (button.getAttribute('aria-label') ?? button.textContent ?? '').trim())
      .filter((label) => label.length > 0)
  );
}
