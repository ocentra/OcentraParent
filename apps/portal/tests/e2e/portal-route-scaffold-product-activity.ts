import { expect, type Locator, type Page } from '@playwright/test';
import { closeParentPortalDetailIfOpen, surfaceText } from './portal-route-scaffold-common';

export async function assertActivityManageRouteSurface(page: Page, surface: Locator, path: string): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'Family' }).first()).toBeVisible();
  await expect(page.getByText('Per Device').first()).toBeVisible();
  if (path === '/#/app-game-sessions') {
    await assertAppGameDashboardRouteSurface(page, surface);
    await assertCollapsedActivitySubsurfaceRemoved(page, surface);
    return;
  }
  if (path === '/#/network-activity') {
    await expect(page.getByRole('tab', { exact: true, name: 'Show activity Network' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Scan Local Area Network' })).toBeVisible();
    await assertActivityReportSurface(page, surface);
    await assertCollapsedActivitySubsurfaceRemoved(page, surface);
    return;
  }
  await expect(surface.locator('text').filter({ hasText: 'Reports' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Screen' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'App Use' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Browser' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Games' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Network' }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: 'Scan Local Area Network' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Connected' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Offline' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Empty' }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: 'Select Aarav laptop' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Aarav laptop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'D001' })).toHaveCount(0);
  await assertActivityReportSurface(page, surface);
  await assertCollapsedActivitySubsurfaceRemoved(page, surface);
}

export async function assertAppGameDashboardRouteSurface(page: Page, surface: Locator): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'APP/GAME READ MODEL DASHBOARD' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SERVICE ROWS' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'CAPABILITY MATRIX' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'EVIDENCE DRAWER' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'GAME BUDGETS' }).first()).toBeVisible();
  const visibleText = await surfaceText(surface);
  expect(visibleText).toMatch(/\b(?:INVENTORY|Inventory)\b/);
  expect(visibleText).toMatch(/\b(?:RUNNING|Running)\b/);
  expect(visibleText).toMatch(/\b(?:FOREGROUND|Foreground)\b/);
  expect(visibleText).toMatch(/\b(?:LAUNCHER|Launcher)\b/);
  expect(visibleText).toMatch(/\bSOURCE ROWS\b/);
  expect(visibleText).toMatch(/\bFRESH SOURCES\b/);
  await expect(page.getByRole('button', { name: 'Select Aarav laptop' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select Mina tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'D001' })).toHaveCount(0);
}

export async function assertActivityReportSurface(page: Page, surface: Locator): Promise<void> {
  await expect(
    surface
      .locator('text')
      .filter({ hasText: /Report (device|target): .+/ })
      .first()
  ).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Frequency' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Report viewer' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SELECTED REPORT' }).first()).toBeVisible();
  const savedReportButton = page.getByRole('button', { name: /^Open activity-report-.+\.json$/ });
  if ((await savedReportButton.count()) > 0) {
    await expect(savedReportButton.first()).toBeVisible();
  } else {
    await expect(
      surface.locator('text').filter({ hasText: 'No saved activity reports reported' }).first()
    ).toBeVisible();
  }
  await expect(page.getByRole('button', { name: 'Generate Daily activity report' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save generated activity report' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Daily' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Weekly' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Monthly' }).first()).toBeVisible();
}

export async function assertCollapsedActivitySubsurfaceRemoved(page: Page, surface: Locator): Promise<void> {
  await expect(page.getByRole('tab', { name: 'Show History activity monitor' })).toHaveCount(0);
  await expect(page.getByRole('tab', { name: 'Show Live activity monitor' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'MONITOR' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT CADENCE' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'PER DEVICE BEHAVIOR' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'PAST REPORTS' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'LIVE REPORT' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Open report' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Family Defaults' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT TYPE' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT TARGET' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'REPORT MODE' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Family activity selected' })).toHaveCount(0);
  await closeParentPortalDetailIfOpen(page);
}
