import { expect, type Locator, type Page } from '@playwright/test';
import { assertLanRouteSurface } from './portal-route-scaffold-lan';
import { assertActivityManageRouteSurface } from './portal-route-scaffold-product-activity';
import { assertAssistantRouteSurface } from './portal-route-scaffold-product-assistant';
import { assertControlRouteSurface } from './portal-route-scaffold-product-control';
import { assertGuideDashboardRouteSurface, assertGuideRouteSurface } from './portal-route-scaffold-product-guide';
import { assertManageRouteSurface } from './portal-route-scaffold-product-manage';

type RouteKind = 'activityManage' | 'assistant' | 'control' | 'guide' | 'guideDashboard' | 'lanPairing' | 'manage';
type RouteSurfaceHandler = (page: Page, surface: Locator, path: string, panelTitle: string) => Promise<void>;

const routeSurfaceHandlers = {
  assistant: async (page: Page, _surface: Locator, _path: string, _panelTitle: string): Promise<void> => {
    await assertAssistantRouteSurface(page);
  },
  manage: async (_page: Page, surface: Locator, path: string, _panelTitle: string): Promise<void> => {
    await assertManageRouteSurface(surface, path);
  },
  activityManage: async (page: Page, surface: Locator, path: string, _panelTitle: string): Promise<void> => {
    await assertActivityManageRouteSurface(page, surface, path);
  },
  lanPairing: async (page: Page, _surface: Locator, _path: string, _panelTitle: string): Promise<void> => {
    await assertLanRouteSurface(page);
  },
  control: async (_page: Page, surface: Locator, path: string, _panelTitle: string): Promise<void> => {
    await assertControlRouteSurface(surface, path);
  },
  guideDashboard: async (page: Page, surface: Locator, _path: string, _panelTitle: string): Promise<void> => {
    await assertGuideDashboardRouteSurface(page, surface);
  },
  guide: async (page: Page, _surface: Locator, _path: string, _panelTitle: string): Promise<void> => {
    await assertGuideRouteSurface(page);
  },
} satisfies Record<RouteKind, RouteSurfaceHandler>;

export async function assertProductRouteSurface(
  page: Page,
  path: string,
  navLabel: string,
  panelTitle: string,
  kind: RouteKind
): Promise<void> {
  await page.goto(path);
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible();
  await expect(page.getByRole('img', { name: 'Ocentra parent dashboard' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: navLabel }).first()).toBeVisible();
  const handler = routeSurfaceHandlers[kind];
  if (handler) {
    await handler(page, surface, path, panelTitle);
    return;
  }
  await expect(surface.locator('text').filter({ hasText: panelTitle }).first()).toBeVisible();
}
