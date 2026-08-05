import { expect, type Page } from '@playwright/test';

export async function assertLanRouteSurface(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  const viewport = page.viewportSize();
  await page.setViewportSize({
    width: Math.max(viewport?.width ?? 1280, 1600),
    height: Math.max(viewport?.height ?? 720, 960),
  });

  try {
    await closeParentPortalDetailIfOpen(page);
    await expect(surface.locator('text').filter({ hasText: 'Local Area Network' }).first()).toBeVisible();
    const scanButton = page.getByRole('button', { name: 'Scan Local Area Network' });
    await expect(scanButton).toBeVisible();
    await scanButton.click({ force: true });
    await expect(surface.locator('text').filter({ hasText: 'Info' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Update' }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: 'Capability' }).first()).toBeVisible();
    await selectFreshServiceBackedLanDevice(page, surface);

    await closeParentPortalDetailIfOpen(page);
    const capabilityTab = page.getByRole('tab', { name: 'Show LAN pairing Capability' });
    await capabilityTab.focus();
    await page.keyboard.press('Enter');
    await expect(capabilityTab).toHaveAttribute('aria-selected', 'true', { timeout: 30_000 });
    await expect(surface.locator('text').filter({ hasText: 'Agent' }).first()).toBeVisible();
    const capabilityText = await surfaceText(surface);
    expect(capabilityText).toMatch(
      /(?:ocentra-(?:local-service|child-agent)|parent\s+local\s+service|agent\s+Not reported)/i
    );
    if (/ocentra-child-agent/i.test(capabilityText)) {
      await expect(surface.locator('text').filter({ hasText: 'CPU' }).first()).toBeVisible();
      await expect(surface.locator('text').filter({ hasText: 'Device ID' }).first()).toBeVisible();
    } else {
      expect(capabilityText).toMatch(/\b(?:DEVICE|ROUTE|EVIDENCE|CONTROL STATE)\b/);
    }
    expect(capabilityText).toMatch(/(?:Signed proof|Proof state|Requirement)/i);

    await assertOptionalLanNeighborRouteProof(page, surface);
    await assertOptionalRouterInfrastructureProof(page);
    await assertNoSyntheticLanDevices(page, surface);
  } finally {
    if (viewport) {
      await page.setViewportSize(viewport);
    }
  }
}

async function selectFreshServiceBackedLanDevice(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  const lanScopeChoice = surface.getByRole('button', { exact: true, name: 'Select LAN Devices' });
  await expect(lanScopeChoice).toBeVisible({ timeout: 30_000 });
  await lanScopeChoice.click({ force: true });

  const deviceChoice = surface.getByRole('button', { name: /^Select (?!LAN |Parent Portal$|Portal$).+/ }).first();
  await expect(deviceChoice).toBeVisible({ timeout: 30_000 });
  const deviceLabel = ((await deviceChoice.getAttribute('aria-label')) ?? '').replace(/^Select /, '');
  await deviceChoice.click({ force: true });
  await expect(
    surface
      .locator('text')
      .filter({ hasText: `Device: ${deviceLabel}` })
      .first()
  ).toBeVisible({
    timeout: 30_000,
  });
}

async function assertOptionalLanNeighborRouteProof(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  const neighborChoice = page.getByRole('button', { name: /^Select LAN \d{1,3}(?:\.\d{1,3}){3}$/ }).first();
  if ((await neighborChoice.count()) === 0) {
    return;
  }

  const ariaLabel = (await neighborChoice.getAttribute('aria-label')) ?? '';
  const neighborLabel = ariaLabel.replace(/^Select /, '');
  await neighborChoice.click({ force: true });
  await page.getByRole('tab', { name: 'Show LAN pairing Info' }).click({ force: true });
  const infoText = await surfaceText(surface);
  expect(infoText).toContain(neighborLabel);
  expect(infoText).toMatch(/\bIP\b/);
  expect(infoText).toMatch(/\b(?:HOST|SOURCE|STATE)\b/);
  expect(infoText).toMatch(/\b(?:LAN discovered|Not reported|Stale|Visible only)\b/);

  await page.getByRole('tab', { name: 'Show LAN pairing Capability' }).click({ force: true });
  const capabilityText = await surfaceText(surface);
  expect(capabilityText).toContain('Not reported');
}

async function assertOptionalRouterInfrastructureProof(page: Page): Promise<void> {
  const routerChoice = page.getByRole('button', { name: /^LAN \d{1,3}(?:\.\d{1,3}){3} is unsupported$/ }).first();
  if ((await routerChoice.count()) === 0) {
    return;
  }

  const ariaLabel = (await routerChoice.getAttribute('aria-label')) ?? '';
  const routerLabel = ariaLabel.replace(/ is unsupported$/, '');
  await expect(page.getByRole('button', { exact: true, name: `Select ${routerLabel}` })).toHaveCount(0);
}

async function assertNoSyntheticLanDevices(page: Page, surface: ReturnType<Page['locator']>): Promise<void> {
  await expect(page.getByRole('button', { name: 'Select Aarav laptop' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Aarav laptop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'UI check device' })).toHaveCount(0);
}

async function closeParentPortalDetailIfOpen(page: Page): Promise<void> {
  const closeDetailButton = page.getByRole('button', { name: 'Close parent portal detail' });
  if ((await closeDetailButton.count()) === 0) {
    return;
  }
  await closeDetailButton.click({ force: true });
  await expect(closeDetailButton).toHaveCount(0, { timeout: 30_000 });
}

async function surfaceText(surface: ReturnType<Page['locator']>): Promise<string> {
  return (await surface.locator('text').allTextContents()).join(' ');
}
