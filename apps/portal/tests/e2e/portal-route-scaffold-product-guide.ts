import { expect, type Locator, type Page } from '@playwright/test';

export async function assertGuideDashboardRouteSurface(page: Page, surface: Locator): Promise<void> {
  await expect(page.getByRole('button', { name: 'Open Set Up Ocentra Parent' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Support Contact' }).first()).toBeVisible();
}

export async function assertGuideRouteSurface(page: Page): Promise<void> {
  await expect(page.getByRole('button', { name: 'Show QUICK READ' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Show QUICK ACTION' })).toBeVisible();
}
