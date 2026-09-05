import { expect, test, type Locator } from '@playwright/test';

const mobileViewport = { width: 320, height: 844 } as const;

interface ElementBox {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

const developerRoutes = [
  { route: '/#/diagnostics', controlName: 'Copy diagnostics', controlRole: 'button' },
  { route: '/#/proof-panels', controlName: 'Proof panel', controlRole: 'combobox' },
  { route: '/#/commands', controlName: 'Check health', controlRole: 'button' },
  { route: '/#/events', controlName: 'Device audit', controlRole: 'heading' },
  { route: '/#/logs', controlName: 'Service log', controlRole: 'heading' },
] as const;

for (const developerRoute of developerRoutes) {
  test(`${developerRoute.route} keeps developer controls inside the mobile viewport`, async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto(developerRoute.route);
    await expect(page.getByRole('button', { exact: true, name: 'Home' })).toBeVisible();
    if (developerRoute.route === '/#/commands') {
      await page.getByText('Review unavailable device controls', { exact: true }).click();
    }

    const control = page.getByRole(developerRoute.controlRole, { exact: true, name: developerRoute.controlName });
    await expect(control).toBeVisible();
    await expect(control).toBeInViewport();
  });
}

test('proof panels use one compact mobile selector and switch real panel content', async ({ page }) => {
  await page.setViewportSize(mobileViewport);
  await page.goto('/#/proof-panels');

  const panel = page.locator('.portal-dev-route-panel');
  const picker = page.getByRole('combobox', { exact: true, name: 'Proof panel' });
  const desktopTabs = panel.locator('[data-ocentra-proof-panel-tabs]');
  const proofLayout = page.locator('[data-ocentra-proof-panels-layout="compact"]');

  await expect(panel).toBeVisible();
  await expect(picker).toBeVisible();
  await expect(desktopTabs).toBeHidden();
  await expect(proofLayout).toBeVisible();
  expect((await requiredBoundingBox(picker, 'Proof panel')).height).toBeLessThan(48);

  await picker.selectOption('network-activity');
  await expect(page.getByRole('region', { exact: true, name: 'Network activity' })).toBeVisible();
});

test('empty device audit is contained and reports its actual state on mobile', async ({ page }) => {
  await page.setViewportSize(mobileViewport);
  await page.goto('/#/events');

  const main = page.getByRole('main', { exact: true, name: 'Main body' });
  const auditHeading = main.getByRole('heading', { exact: true, name: 'Device audit' });
  await expect(main).toBeVisible();
  await expect(auditHeading).toBeVisible();
  await expect(auditHeading).toBeInViewport();
  await expect(main.getByText('No device audit events have been reported yet.')).toBeVisible();
  assertInsideMobileWidth(await requiredBoundingBox(main, 'Device audit main body'), 240);
});

test('unavailable device commands explain the gate and collapse the disabled control wall', async ({ page }) => {
  await page.setViewportSize(mobileViewport);
  await page.goto('/#/commands');

  const main = page.getByRole('main', { exact: true, name: 'Main body' });
  const disclosure = main.locator('details:has(.command-grid)');
  const disclosureSummary = main.getByText('Review unavailable device controls', { exact: true });
  const health = main.getByRole('button', { exact: true, name: 'Check health' });
  const logs = main.getByRole('button', { exact: true, name: 'Get log snapshot' });

  await expect(main.getByText('Start or reconnect the local service to enable these device controls.')).toBeVisible();
  await expect(disclosure).not.toHaveAttribute('open', '');
  await expect(health).toBeHidden();
  await disclosureSummary.click();
  await expect(disclosure).toHaveAttribute('open', '');
  await expect(health).toBeDisabled();
  await expect(logs).toBeDisabled();

  const healthBox = await requiredBoundingBox(health, 'Check health');
  const logsBox = await requiredBoundingBox(logs, 'Get log snapshot');
  assertInsideMobileWidth(healthBox, 200);
  assertInsideMobileWidth(logsBox, 200);
  expect(logsBox.y).toBeGreaterThanOrEqual(healthBox.y + healthBox.height);
  expect(Math.abs(healthBox.width - logsBox.width)).toBeLessThan(2);
});

function assertInsideMobileWidth(box: ElementBox, minimumWidth: number): void {
  expect(box.x).toBeGreaterThanOrEqual(0);
  expect(box.x + box.width).toBeLessThanOrEqual(mobileViewport.width);
  expect(box.width).toBeGreaterThanOrEqual(minimumWidth);
}

async function requiredBoundingBox(locator: Locator, label: string): Promise<ElementBox> {
  const box = await locator.boundingBox();
  if (box === null) {
    throw new Error(`Expected a rendered bounding box for ${label}`);
  }
  return box;
}
