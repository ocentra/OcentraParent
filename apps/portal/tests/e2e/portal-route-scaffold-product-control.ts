import { expect, type Locator } from '@playwright/test';
import { expectSurfaceTextToContain } from './portal-route-scaffold-common';

export async function assertControlRouteSurface(surface: Locator, path: string): Promise<void> {
  await expect(surface.locator('text').filter({ hasText: 'WHAT PARENTS CONTROL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'DATA CUSTODY' }).first()).toBeVisible();
  if (path === '/#/browser') {
    await expectSurfaceTextToContain(surface, 'Managed Web');
    await expectSurfaceTextToContain(surface, 'Browser Setup');
    await expect(surface.getByRole('button', { name: 'Show Local agent control row' })).toHaveCount(0);
  }
}
