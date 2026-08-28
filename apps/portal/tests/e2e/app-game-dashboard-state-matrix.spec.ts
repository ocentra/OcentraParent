import { expect, test, type Locator, type Page } from '@playwright/test';
import { ParentAgentEvent } from '../../generated/parent-ui-bridge';
import { collectBrowserFailures } from './browser-failures';
import { assertAppGameDashboardRouteSurface } from './portal-route-scaffold-product-activity';

test.setTimeout(180_000);

const portalShellReadyTimeoutMs = 90_000;
const appGameRoute = '/#/app-game-sessions';

test('app/game dashboard renders the real service state matrix without unsupported claims', async ({ page }) => {
  const browserFailures = collectBrowserFailures(page);

  await refreshActivityReadModel(page, 'Refresh activity app use', ParentAgentEvent.ActivityAppUseReadModelReported);
  await refreshActivityReadModel(page, 'Refresh activity games', ParentAgentEvent.ActivityGamesReadModelReported);

  await page.goto(appGameRoute);
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(surface).toBeVisible({ timeout: portalShellReadyTimeoutMs });
  await assertAppGameDashboardRouteSurface(page, surface);
  await assertDashboardStateMatrix(surface);
  await assertSafeRenderedMetadata(surface);

  expect(browserFailures).toEqual([]);
});

async function refreshActivityReadModel(page: Page, commandName: string, eventName: string): Promise<void> {
  await page.goto('/#/commands');
  await expect(page.getByRole('heading', { exact: true, name: 'Device controls' })).toBeVisible({
    timeout: portalShellReadyTimeoutMs,
  });

  const command = page.getByRole('button', { exact: true, name: commandName });
  await expect(command).toBeEnabled({ timeout: portalShellReadyTimeoutMs });
  await command.click();

  const commandResult = page.locator('.command-result-panel');
  await expect(commandResult.getByText(eventName)).toHaveCount(1, {
    timeout: portalShellReadyTimeoutMs,
  });
  await expect(commandResult.getByText('activitySurfaceState')).toHaveCount(1, {
    timeout: portalShellReadyTimeoutMs,
  });
}

async function assertDashboardStateMatrix(surface: Locator): Promise<void> {
  const textNodes = await surface.locator('text').allTextContents();
  const dashboardText = textNodes.join(' ');
  const statusLines = textNodes.filter((text) => /^(?:App use|Game)\s+\/\s+/u.test(text));
  const countLines = textNodes.filter((text) => /^Inventory\s+\d+\s+\/\s+Running\s+\d+/u.test(text));

  expect(dashboardText).toContain('APP/GAME READ MODEL DASHBOARD');
  expect(dashboardText).toMatch(/\bSTATE\s+[A-Z][A-Z0-9-]*\b/u);
  expect(dashboardText).not.toMatch(
    /(?:providerDispatchTarget|rawPlatformDiagnostics|provider delivery|enforcement active|product ready)/iu
  );

  if (statusLines.length === 0) {
    expect(dashboardText).toContain('No app/game read model rows reported by the local service.');
    expect(dashboardText).toMatch(/\bGAME BUDGETS\b/u);
    expect(dashboardText).toContain('no game rows');
    expect(countLines).toHaveLength(0);
    return;
  }

  expect(countLines).toHaveLength(statusLines.length);
  for (const countLine of countLines) {
    expect(countLine).toMatch(
      /^Inventory\s+\d+\s+\/\s+Running\s+\d+\s+\/\s+Foreground\s+\d+\s+\/\s+Launcher\s+\d+/u
    );
    expect(countLine).toMatch(/\/\s+Evidence\s+\d+$/u);
  }

  const warningLines = textNodes.filter((text) =>
    /^(?:SERVICE-BACKED ROW|MANUAL-REQUIRED CAPABILITY|UNKNOWN REVIEW CANDIDATE|LAUNCHER-ONLY, NOT ACTIVE GAME)/u.test(
      text
    )
  );
  expect(warningLines).toHaveLength(statusLines.length);
  expect(new Set(statusLines.map((text) => text.split('/')[1]?.trim() ?? '')).size).toBeGreaterThan(0);
}

async function assertSafeRenderedMetadata(surface: Locator): Promise<void> {
  const serializedSurface = await surface.evaluate((element) => element.outerHTML);
  expect(serializedSurface).not.toMatch(/<(?:script|iframe|object|img)\b/iu);
  expect(serializedSurface).not.toMatch(/(?:javascript:|onerror\s*=|onload\s*=)/iu);

  const textGeometry = await surface.locator('text').evaluateAll((elements) =>
    elements.map((element) => {
      const box = (element as SVGGraphicsElement).getBBox();
      return {
        text: element.textContent?.trim() ?? '',
        x: box.x,
        y: box.y,
        width: box.width,
        height: box.height,
      };
    })
  );
  expect(textGeometry.length).toBeGreaterThan(0);
  expect(
    textGeometry.every(({ x, y, width, height }) =>
      [x, y, width, height].every((value) => Number.isFinite(value))
    )
  ).toBe(true);
  expect(textGeometry.every(({ text }) => text.length <= 256)).toBe(true);
  expect(textGeometry.every(({ text }) => !/<(?:script|iframe|object|img)\b/iu.test(text))).toBe(true);
}
