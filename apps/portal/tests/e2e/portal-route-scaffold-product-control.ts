import { expect, type Locator } from '@playwright/test';
import { expectSurfaceTextToMatch } from './portal-route-scaffold-common';

export async function assertControlRouteSurface(surface: Locator, path: string): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'WHAT PARENTS CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'DATA CUSTODY' }).first()).toBeVisible();
  if (path === '/#/browser') {
    await expectSurfaceTextToMatch(surface, /(?:Managed Web|Browser inventory)/);
    await expectSurfaceTextToMatch(surface, /(?:Browser Setup|Exact URL capability)/);
    await expectSurfaceTextToMatch(surface, /(?:Per-device browser choices|Browser review|Managed web ready)/);
  }
}
