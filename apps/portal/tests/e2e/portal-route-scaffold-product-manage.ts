import { type Locator } from '@playwright/test';
import { expectSurfaceTextToContain, expectSurfaceTextToMatch } from './portal-route-scaffold-common';

export async function assertManageRouteSurface(surface: Locator, path: string): Promise<void> {
  await expectSurfaceTextToMatch(
    surface,
    /(?:Family|Rules|Schedule|Approvals|Enforcement|Audit|Plan|Access|Support|Settings|Portal|Devices|Data|AI|Memory)/
  );
  if (path === '/#/browser-settings') {
    await expectSurfaceTextToMatch(surface, /(?:Browser target|Browser activity)/i);
    return;
  }
  if (path === '/#/settings-rules') {
    await expectSurfaceTextToMatch(surface, /(?:Rules|Managed web path|Browser inventory)/);
    await expectSurfaceTextToMatch(
      surface,
      /(?:Schedule|Budget|Approvals|Audit|Exact URL capability|Active tab proof)/
    );
    await expectSurfaceTextToMatch(surface, /(?:browser policy|Observe|Enforce)/i);
    return;
  }
  if (path === '/#/enforcement') {
    await expectSurfaceTextToMatch(surface, /(?:Enforcement readiness|Browser target|browser policy)/i);
    return;
  }
  if (path === '/#/api-providers') {
    await expectSurfaceTextToContain(surface, 'API providers');
    return;
  }
  if (path === '/#/drive-connections') {
    await expectSurfaceTextToMatch(surface, /(?:Data custody|Drive exports)/);
  }
}
