import { expect, type Page } from '@playwright/test';

const sidebarGroupLabel = {
  Guide: 'Guide',
  Manage: 'Manage',
} as const;

type SidebarGroupLabel = (typeof sidebarGroupLabel)[keyof typeof sidebarGroupLabel];

export async function assertRouteScaffolds(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Activity/u }).click();
  await expect(page.getByRole('heading', { name: 'Network activity' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'App and game sessions' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Screen analysis' })).toBeVisible();

  await page.getByRole('tab', { name: /^Web/u }).click();
  await expect(page.getByRole('heading', { name: 'Managed browser' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Browser evidence' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Browser protection' })).toBeVisible();

  await ensureSidebarGroupOpen(page, sidebarGroupLabel.Guide);
  await page.getByRole('tab', { name: /^Policy/u }).click();
  await expect(page.getByRole('heading', { name: 'Policy decision' })).toBeVisible();
  await assertParentControls(page);
  await assertDeviceRuleScope(page);

  await assertPrivacyDesignRoute(page);

  await page.getByRole('tab', { name: /^Memory/u }).click();
  await expect(page.getByRole('heading', { name: 'Memory links' })).toBeVisible();
  await expect(page.locator('.control-deck-header h2').filter({ hasText: /^Memory$/u })).toBeVisible();

  await page.getByRole('tab', { name: /^Local AI/u }).click();
  await expect(page.locator('section.summary h2').filter({ hasText: /^Local AI$/u })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Private by design' })).toBeVisible();

  await ensureSidebarGroupOpen(page, sidebarGroupLabel.Manage);
  await assertNotificationsRoute(page);
  await assertDriveConnectionsRoute(page);
  await assertDeviceRoute(page);
  await assertDiagnosticsRoute(page);
  await assertSettingsRoute(page);
  await assertFrameTunerRoute(page);
}

async function ensureSidebarGroupOpen(page: Page, label: SidebarGroupLabel): Promise<void> {
  const groupLabel = page.locator('summary.route-group-label').filter({ hasText: label });
  const isOpen = await groupLabel.evaluate(
    (node) => node.parentElement instanceof HTMLDetailsElement && node.parentElement.open
  );
  if (!isOpen) {
    await groupLabel.click();
  }
}

async function assertParentControls(page: Page): Promise<void> {
  const parentControls = page.locator('.control-deck').filter({
    has: page.locator('.control-deck-header h2').filter({ hasText: /^Parent controls$/u }),
  });
  await expect(parentControls.getByRole('heading', { name: 'Rule builder' })).toBeVisible();
  await expect(parentControls.getByRole('heading', { name: 'Schedules and budgets' })).toBeVisible();
  await expect(parentControls.locator('.control-card h2').filter({ hasText: /^Approvals$/u })).toHaveCount(1);
}

async function assertDeviceRuleScope(page: Page): Promise<void> {
  const deviceScope = page
    .locator('.summary')
    .filter({ has: page.getByRole('heading', { name: 'Device rule scope' }) });
  await expect(deviceScope.getByRole('heading', { name: 'Device rule scope' })).toBeVisible();
  await expect(deviceScope.getByRole('heading', { name: 'Managed web' })).toBeVisible();
  await expect(deviceScope.locator('.product-badge').filter({ hasText: /^Child device/u })).toBeVisible();
}

async function assertPrivacyDesignRoute(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Private by design/u }).click();
  await expect(page.locator('.control-deck-header h2').filter({ hasText: /^Private by design$/u })).toBeVisible();
}

async function assertNotificationsRoute(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Notifications/u }).click();
  await expect(page.locator('.control-deck-header h2').filter({ hasText: /^Notifications$/u })).toBeVisible();
}

async function assertDriveConnectionsRoute(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Connect your drives/u }).click();
  await expect(page.locator('.control-deck-header h2').filter({ hasText: /^Connect your drives$/u })).toBeVisible();
}

async function assertDeviceRoute(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Devices/u }).click();
  await expect(page.getByRole('heading', { name: 'Device diagnostics' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Latest device snapshot' })).toBeVisible();
  const deviceDeck = page.locator('.control-deck').filter({
    has: page.locator('.control-deck-header h2').filter({ hasText: /^Devices$/u }),
  });
  await expect(deviceDeck.getByRole('heading', { name: 'Device inventory' })).toBeVisible();
  await expect(deviceDeck.getByRole('heading', { name: 'Pairing' })).toBeVisible();
  await expect(deviceDeck.locator('.control-card h2').filter({ hasText: /^Mobile app$/u })).toHaveCount(1);

  const snapshotPanel = page
    .locator('.summary')
    .filter({ has: page.getByRole('heading', { name: 'Latest device snapshot' }) });
  await expect(
    snapshotPanel.locator('dt').filter({ hasText: 'Device' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText('local-dev-agent');
  await expect(
    snapshotPanel.locator('dt').filter({ hasText: 'Version' }).locator('xpath=following-sibling::dd[1]')
  ).toHaveText(/\b\d+\.\d+\.\d+\b/u);
}

async function assertDiagnosticsRoute(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Support/u }).click();
  await expect(page.getByRole('heading', { name: 'Service log' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Device controls' })).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Device audit' })).toBeVisible();
  await expect(page.locator('dt').filter({ hasText: 'Events' }).locator('xpath=following-sibling::dd[1]')).toHaveText(
    /\d+/u
  );
  await assertDiagnosticsCopy(page);
}

async function assertSettingsRoute(page: Page): Promise<void> {
  await page.getByRole('tab', { name: /^Settings/u }).click();
  await expect(page.getByRole('heading', { name: 'Display theme' })).toBeVisible();
}

async function assertFrameTunerRoute(page: Page): Promise<void> {
  await expect(page.getByRole('tab', { name: /^Frame tuner/u })).toHaveCount(0);
  await page.goto('/#/frame-tuner');
  await expect(page.getByRole('heading', { name: 'Frame tuner' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save JSON' })).toBeVisible();
  await expect(page.locator('.app-sidebar')).toHaveCount(0);
  await expect(page.locator('svg.portal-frame-backdrop-svg')).toHaveCount(0);
  await page.getByRole('tab', { name: 'Side panel' }).click();
  await expect(page.getByRole('tab', { name: 'Top' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Bottom' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Reset top' })).toBeVisible();
  await page.getByRole('tab', { name: 'Bottom' }).click();
  await expect(page.getByRole('button', { name: 'Reset bottom' })).toBeVisible();
  await page.getByRole('tab', { name: 'Main panel' }).click();
  await expect(page.getByRole('button', { name: 'Reset main' })).toBeVisible();
  await page.getByRole('tab', { name: 'Golden card' }).click();
  await expect(page.getByRole('button', { name: 'Reset golden card' })).toBeVisible();
  await expect(page.getByText('Golden card frame')).toBeVisible();
  await expect(page.getByText('Golden card content')).toBeVisible();
  await page.getByRole('tab', { name: 'Save and JSON' }).click();
  await expect(page.getByText('Saved JSON preview')).toBeVisible();
}

async function assertDiagnosticsCopy(page: Page): Promise<void> {
  await page.getByRole('button', { name: 'Copy diagnostics' }).click();
  await expect(page.getByRole('button', { name: 'Diagnostics copied' })).toBeVisible();

  const copiedText = await page.evaluate(() => navigator.clipboard.readText());
  expect(copiedText).toContain('"agentUrl"');
  expect(copiedText).toContain('"connectionState"');
  expect(copiedText).toContain('"events"');
  expect(copiedText).toContain('"recentSummary"');
  expect(copiedText).toContain('"networkFlowReadModel"');
  expect(copiedText).toContain('"activityMemoryGraphReadModel"');
}
