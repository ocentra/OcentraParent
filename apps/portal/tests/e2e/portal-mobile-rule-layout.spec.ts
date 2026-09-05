import { expect, test, type Locator } from '@playwright/test';

const mobileViewport = { width: 319, height: 844 } as const;
const desktopViewport = { width: 1280, height: 800 } as const;

interface ElementBox {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

const carouselRoutes = ['policy', 'ai-guide', 'privacy-design'] as const;

test.describe('responsive parent portal layout', () => {
  registerRuleLayoutTests();
  registerCarouselLayoutTests();
  registerGuideAndNavigationTests();
  registerResponsiveCanvasStabilityTest();
  registerCapabilityStatusLayoutTest();
  registerBillingSurfaceTests();
  registerAiRuntimeSurfaceTests();
  registerPolicySurfaceTests();
  registerDataSurfaceTests();
  registerFrameTunerTest();
  registerBrowserUnavailableLayoutTest();
  registerScreenAnalysisLayoutTests();
  registerActivityReportLayoutTest();
});

function registerRuleLayoutTests(): void {
  test('unavailable policy overview stays inside the mobile viewport', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/rule-management');

    const unavailableOverview = page.getByRole('region', {
      exact: true,
      name: 'Rule management status unavailable',
    });
    await expect(unavailableOverview).toBeVisible();
    await expect(unavailableOverview).toContainText(
      'The local service has not reported current rules, approvals, or enforcement state. Nothing is inferred or presented as active.'
    );
    await expect(unavailableOverview.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();

    const overviewBox = await requiredBoundingBox(unavailableOverview, 'unavailable policy overview');
    expect(overviewBox.x).toBeGreaterThanOrEqual(0);
    expect(overviewBox.x + overviewBox.width).toBeLessThanOrEqual(mobileViewport.width);
    await expect(page.getByText('CURRENT POLICY NOT SHOWN HERE', { exact: true })).toHaveCount(0);
    await expect(page.getByText('3. Which policy areas should family rules cover?', { exact: true })).toHaveCount(0);
  });

  test('unavailable browser policy sections open real product routes instead of dead tabs', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/browser-settings');

    const openRules = page.getByRole('button', { exact: true, name: 'Open rules' });
    await expect(openRules).toBeVisible();
    await expect(page.getByRole('tab', { exact: true, name: 'Show Schedule' })).toHaveCount(0);

    await openRules.click();

    await expect(page).toHaveURL(/#\/rule-management$/u);
    await expect(page.getByRole('region', { exact: true, name: 'Rule management status unavailable' })).toBeVisible();
  });

  test('rule action icons use unique document identifiers', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/rule-management');

    const duplicateIds = await page.locator('[id]').evaluateAll((nodes) => {
      const ids = nodes.map((node) => node.id).filter((id) => id.length > 0);
      return ids
        .filter((id, index) => ids.indexOf(id) !== index)
        .filter((id, index, values) => values.indexOf(id) === index);
    });

    expect(duplicateIds).toEqual([]);
  });
}

function registerBillingSurfaceTests(): void {
  test('billing routes stay honest before an authenticated entitlement handoff exists', async ({ page }) => {
    await page.setViewportSize(desktopViewport);
    await page.goto('/#/subscription');

    const surfaceText = page.locator('svg.parent-portal-svg-surface text');
    await expect(surfaceText.filter({ hasText: /^CURRENT PLAN$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^SUBSCRIPTION STATUS$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^DEVICE SEATS$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^BILLING ACTIONS$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^Not reported$/u })).toHaveCount(3);
    await expect(surfaceText.filter({ hasText: /^Unavailable$/u })).toHaveCount(2);
    await expect(surfaceText.filter({ hasText: /^\$/u })).toHaveCount(0);
    await expect(surfaceText.filter({ hasText: /(?:trial target|extra seats|AI credits)/iu })).toHaveCount(0);

    await page.goto('/#/entitlements');
    await expect(surfaceText.filter({ hasText: /^ENTITLEMENT SNAPSHOT$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^FEATURE ACCESS$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^(?:Plan based|Credit based)$/u })).toHaveCount(0);
  });
}

function registerAiRuntimeSurfaceTests(): void {
  test('AI runtime uses the service-reported panel without fabricating a local runtime', async ({ page }) => {
    await page.setViewportSize(desktopViewport);
    await page.goto('/#/ai-runtime');

    const runtimePanel = page.getByRole('region', { exact: true, name: 'Local AI' });
    const overviewText = page.locator('svg.parent-portal-svg-surface text');
    const disclosure = runtimePanel.locator('details[data-ocentra-ai-runtime-disclosure]');

    await expect(runtimePanel).toBeVisible();
    await expect(disclosure).not.toHaveAttribute('open', '');
    await expect(runtimePanel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
    await disclosure.locator('summary').click();
    await expect(disclosure).toHaveAttribute('open', '');
    await expect(
      runtimePanel.getByRole('heading', { exact: true, name: 'AI jobs and runtime activity' })
    ).toBeVisible();
    await expect(runtimePanel.getByText('No local AI runtime or job event has been reported yet.')).toBeVisible();
    await expect(runtimePanel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
    await expect(overviewText.filter({ hasText: /^llama\.cpp$/u })).toHaveCount(0);
    await expect(overviewText.filter({ hasText: /^Queue$/u })).toHaveCount(0);

    await page.setViewportSize(mobileViewport);
    await page.goto('/#/overview');
    await page.goto('/#/ai-runtime');
    const compactRuntimePanel = page.getByRole('region', { exact: true, name: 'Local AI' });
    const compactDisclosure = compactRuntimePanel.locator('details[data-ocentra-ai-runtime-disclosure]');
    await expect(compactRuntimePanel).toBeVisible();
    await expect(compactDisclosure).not.toHaveAttribute('open', '');
    await expect(compactRuntimePanel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
    const compactRuntimePanelBox = await requiredBoundingBox(compactRuntimePanel, 'mobile AI runtime drawer');
    expect(compactRuntimePanelBox.width).toBeGreaterThan(280);
    expect(compactRuntimePanelBox.height).toBeGreaterThan(70);
    expect(compactRuntimePanelBox.height).toBeLessThan(130);
  });
}

function registerPolicySurfaceTests(): void {
  test('policy routes do not fabricate current rules, approvals, or enforcement state', async ({ page }) => {
    await page.setViewportSize(desktopViewport);

    for (const [route, statusTitle] of [
      ['rule-management', 'Rule management status unavailable'],
      ['approvals', 'Approvals status unavailable'],
      ['enforcement', 'Enforcement status unavailable'],
    ] as const) {
      await page.goto(`/#/${route}`);
      const surfaceText = page.locator('svg.parent-portal-svg-surface text');

      const unavailablePanel = page.getByRole('region', { exact: true, name: statusTitle });
      await expect(unavailablePanel).toBeVisible();
      await expect(unavailablePanel.getByRole('listitem')).toHaveCount(3);
      await expect(unavailablePanel.getByText('Not reported', { exact: true })).toBeVisible();
      await expect(unavailablePanel.getByText('Manual required', { exact: true })).toBeVisible();
      await expect(surfaceText.filter({ hasText: /^CURRENT POLICY NOT SHOWN HERE$/u })).toHaveCount(0);
      await expect(surfaceText.filter({ hasText: /request matrix|Should .* policy be active/iu })).toHaveCount(0);
      await expect(page.locator('svg.parent-portal-svg-surface')).not.toContainText('policy panel below');
    }

    for (const [route, statusTitle] of [
      ['browser-settings', 'Browser policy controls unavailable'],
      ['policy-apps', 'App policy controls unavailable'],
      ['policy-games', 'Game policy controls unavailable'],
    ] as const) {
      await page.goto(`/#/${route}`);
      const surfaceText = page.locator('svg.parent-portal-svg-surface text');

      const policyPanel = page.getByRole('region', { exact: true, name: statusTitle });
      await expect(policyPanel).toBeVisible();
      await expect(policyPanel.getByRole('listitem')).toHaveCount(3);
      await expect(policyPanel.getByText('Not reported', { exact: true })).toBeVisible();
      await expect(policyPanel.getByText('Manual required', { exact: true })).toBeVisible();
      await expect(surfaceText.filter({ hasText: /^CURRENT POLICY NOT SHOWN HERE$/u })).toHaveCount(0);
      await expect(surfaceText.filter({ hasText: /request matrix|Should .* policy be active/iu })).toHaveCount(0);
    }
  });
}

function registerDataSurfaceTests(): void {
  test('data routes show only reported connector, retention, and audit state', async ({ page }) => {
    await page.setViewportSize(desktopViewport);

    await page.goto('/#/drive-connections');
    await expect(page.getByRole('tab', { name: /Show Storage/u, selected: true })).toBeVisible();
    await expect(page.getByRole('button', { exact: true, name: 'Select Per Device' })).toHaveCount(0);
    let surfaceText = page.locator('svg.parent-portal-svg-surface text');
    await expect(surfaceText.filter({ hasText: /^CONNECTOR STATE$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^(?:Primary|Planned connector|Export only)$/u })).toHaveCount(0);

    await page.goto('/#/export-retention');
    surfaceText = page.locator('svg.parent-portal-svg-surface text');
    await expect(surfaceText.filter({ hasText: /^RETENTION SNAPSHOT$/u })).toBeVisible();
    await expect(
      surfaceText.filter({ hasText: /^(?:Parent policy|Long lived|Shorter window|Audit entry)$/u })
    ).toHaveCount(0);

    await page.goto('/#/audit-history');
    surfaceText = page.locator('svg.parent-portal-svg-surface text');
    await expect(surfaceText.filter({ hasText: /^AUDIT HISTORY$/u }).last()).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^0 reported$/u })).toBeVisible();
    await expect(surfaceText.filter({ hasText: /^Logged$/u })).toHaveCount(0);

    await page.goto('/#/remote-access');
    const remoteAccessStatus = page.getByRole('region', { exact: true, name: 'Remote access unavailable' });
    await expect(remoteAccessStatus).toContainText(
      'No owner-backed remote session, trusted target, transport route, or current authority is connected.'
    );
    for (const actionName of ['Open Start Here', 'Open devices', 'Review remote screen policy'] as const) {
      await expect(remoteAccessStatus.getByRole('button', { exact: true, name: actionName })).toBeVisible();
    }
    await expect(remoteAccessStatus.locator('article')).toHaveCount(3);
    await expect(remoteAccessStatus.getByText('Remote session', { exact: true })).toBeVisible();
    await expect(remoteAccessStatus.getByText('Trusted target', { exact: true })).toBeVisible();
    await expect(remoteAccessStatus.getByText('Control authority', { exact: true })).toBeVisible();
    await expect(page.getByRole('status', { exact: true, name: 'Remote access target unavailable' })).toHaveCount(0);
    await expect(page.getByRole('button', { exact: true, name: 'Use All devices scope' })).toHaveCount(0);
    await expect(page.getByText('CURRENT SELECTION', { exact: true })).toHaveCount(0);
    await expect(page.getByRole('button', { exact: true, name: 'Enable remote' })).toHaveCount(0);
  });
}

function registerCarouselLayoutTests(): void {
  for (const route of carouselRoutes) {
    test(`${route} carousel actions stay inside the mobile viewport`, async ({ page }) => {
      await page.setViewportSize(mobileViewport);
      await page.goto(`/#/${route}`);

      const nextActions = page.getByRole('button', { exact: true, name: 'Next parent portal carousel page' });
      await expect(nextActions.first()).toBeVisible();
      await expectVisibleControlsInsideViewport(nextActions, `${route} next carousel action`);
    });
  }
}

function registerGuideAndNavigationTests(): void {
  test('privacy guide preserves the full page heading on mobile', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/privacy-design');

    const pageTitle = page.getByRole('heading', {
      exact: true,
      name: 'Ocentra does not store child activity by default',
    });
    await expect(pageTitle).toBeVisible();
    const renderedLines = await pageTitle.locator('text').allTextContents();
    expect(renderedLines.join(' ')).toBe('Ocentra does not store child activity by default');
    expect(renderedLines.filter((line) => line.endsWith('...'))).toEqual([]);
  });

  test('AI guide steps stay clear of compact page navigation', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/ai-guide');

    const pageNavigation = page.getByRole('button', { exact: true, name: 'Show guide page 1' });
    const guideSteps = page.getByRole('list', { exact: true, name: 'AI And Evidence guide steps' });
    const stepLabels = guideSteps.getByRole('listitem');

    await expect(pageNavigation).toBeVisible();
    await expect(guideSteps).toBeVisible();
    await expect(stepLabels).toHaveCount(4);

    const pageNavigationBox = await requiredBoundingBox(pageNavigation, 'AI guide page navigation');
    for (let index = 0; index < (await stepLabels.count()); index += 1) {
      const stepBox = await requiredBoundingBox(stepLabels.nth(index), `AI guide step ${index + 1}`);
      expect(stepBox.y + stepBox.height).toBeLessThanOrEqual(pageNavigationBox.y - 2);
    }
  });

  test('light theme mobile section selector keeps the dark header material', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/subscription');
    await page.getByRole('button', { exact: true, name: 'Light' }).click();
    await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');

    const sectionSelector = page.getByRole('combobox', { exact: true, name: 'Choose parent portal section' });
    await expect(sectionSelector).toBeVisible();

    const colors = await sectionSelector.evaluate((element) => {
      const mobileNav = element.closest('nav');
      if (!(mobileNav instanceof HTMLElement))
        throw new Error('Expected the section selector inside mobile navigation');

      const selectStyle = getComputedStyle(element);
      return {
        foreground: selectStyle.color,
        selectBackground: selectStyle.backgroundColor,
        navigationBackground: getComputedStyle(mobileNav).backgroundColor,
      };
    });

    expect(colors.selectBackground).toBe(colors.navigationBackground);
    expect(colors.selectBackground).not.toBe('rgba(0, 0, 0, 0)');
    expect(colors.foreground).not.toBe(colors.selectBackground);
  });

  test('mobile section selector opens and identifies the AI Assistant route', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/overview');

    const sectionSelector = page.getByRole('combobox', { exact: true, name: 'Choose parent portal section' });
    await expect(sectionSelector).toBeVisible();

    await sectionSelector.selectOption({ label: 'AI Assistant' });

    await expect(page).toHaveURL(/#\/assistant$/u);
    await expect(page.getByRole('heading', { exact: true, name: 'AI parent controls' })).toBeVisible();
    await expect(sectionSelector).toHaveValue('#/assistant');

    await page.goto('/#assistant');
    await expect(page.getByRole('heading', { exact: true, name: 'AI parent controls' })).toBeVisible();
    await expect(sectionSelector).toHaveValue('#/assistant');
  });
}

function registerResponsiveCanvasStabilityTest(): void {
  test('mobile parent surface keeps a bounded intrinsic canvas after measurement', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/subscription');

    const surface = page.locator('svg.parent-portal-svg-surface');
    const main = page.locator('.parent-portal-svg-main');
    const route = page.locator('.parent-portal-route');
    await expect(surface).toBeVisible();
    await page.waitForTimeout(200);

    await expect(surface).toHaveAttribute('viewBox', '0 0 320 900');
    expect(await main.evaluate((element) => element.scrollHeight)).toBeLessThan(1_200);
    expect(await route.evaluate((element) => element.scrollHeight)).toBeLessThan(1_200);
  });
}

function registerCapabilityStatusLayoutTest(): void {
  test('reported shell status keeps unavailable capability domains readable in the light theme', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    await page.goto('/#/capability-status');
    await page.getByRole('button', { exact: true, name: 'Light' }).click();

    const panel = page.getByRole('region', { exact: true, name: 'Capability status' });
    const cards = panel.getByRole('article');
    const browserCard = cards.filter({ has: page.getByRole('heading', { exact: true, name: 'Browser' }) });
    const explanation = browserCard.getByText(
      'No browser capability is reported. Browser controls remain unavailable.',
      { exact: true }
    );

    await expect(panel).toBeVisible();
    await expect(panel).toHaveAttribute('data-ocentra-capability-status-state', 'reported');
    await expect(cards).toHaveCount(12);
    await expect(panel.locator('[data-ocentra-capability-card-state="unavailable"]')).toHaveCount(10);
    await expect(panel.locator('[data-ocentra-capability-card-state="reported"]')).toHaveCount(2);
    await expect(explanation).toBeVisible();
    const firstCardBox = await requiredBoundingBox(cards.nth(0), 'first unavailable capability status card');
    const fourthCardBox = await requiredBoundingBox(cards.nth(3), 'fourth unavailable capability status card');
    expect(firstCardBox.height).toBeGreaterThan(140);
    expect(firstCardBox.height).toBeLessThan(220);
    expect(fourthCardBox.y).toBeGreaterThan(firstCardBox.y);
    expect(await explanation.evaluate((element) => getComputedStyle(element).color)).toBe('rgb(223, 248, 255)');
  });
}

function registerFrameTunerTest(): void {
  test('mobile App Layout opens its dedicated tuner without replacing the parent portal', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/overview');

    const sectionSelector = page.getByRole('combobox', { exact: true, name: 'Choose parent portal section' });
    await expect(sectionSelector).toBeVisible();

    const frameTunerPromise = page.waitForEvent('popup');
    await sectionSelector.selectOption({ label: 'APP LAYOUT' });
    const frameTuner = await frameTunerPromise;

    await frameTuner.waitForLoadState('domcontentloaded');
    await expect(frameTuner).toHaveURL(/#\/frame-tuner$/u);
    await expect(frameTuner.getByRole('heading', { name: 'App layout' })).toBeVisible();
    await expect(page).toHaveURL(/#\/overview$/u);
    await expect(sectionSelector).toHaveValue('#/overview');

    await frameTuner.close();
  });

  test('mobile Background opens its dedicated tuner without replacing the parent portal', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/overview');

    const sectionSelector = page.getByRole('combobox', { exact: true, name: 'Choose parent portal section' });
    await expect(sectionSelector).toBeVisible();

    const backgroundTunerPromise = page.waitForEvent('popup');
    await sectionSelector.selectOption({ label: 'BG' });
    const backgroundTuner = await backgroundTunerPromise;

    await backgroundTuner.waitForLoadState('domcontentloaded');
    await expect(backgroundTuner).toHaveURL(/#\/frame-tuner\?bg-only=1$/u);
    await expect(backgroundTuner.getByText('BG tuner', { exact: true })).toBeVisible();
    await expect(page).toHaveURL(/#\/overview$/u);
    await expect(sectionSelector).toHaveValue('#/overview');

    await backgroundTuner.close();
  });
}

function registerBrowserUnavailableLayoutTest(): void {
  test('unavailable browser status stays inside the mobile viewport without inferred activity', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/browser');
    await page.getByRole('button', { exact: true, name: 'Light' }).click();

    const panel = page.getByRole('region', { exact: true, name: 'Browser activity status' });
    const card = panel.locator('.summary.product-status-card');
    await expect(panel).toHaveAttribute('data-ocentra-browser-route-state', 'unavailable');
    await expect(panel.getByRole('heading', { exact: true, name: 'Browser status unavailable' }).first()).toBeVisible();
    await expect(panel.getByRole('heading', { exact: true, name: 'Managed session' })).toBeVisible();
    await expect(panel.getByRole('heading', { exact: true, name: 'Evidence status' })).toBeVisible();
    await expect(panel.getByRole('heading', { exact: true, name: 'Activity rows' })).toBeVisible();
    await expect(card).toHaveCount(3);
    const explanation = panel.getByText(
      'No browser, domain, URL, session, or intervention state is inferred while the service snapshot is unavailable.',
      { exact: true }
    );
    await expect(explanation).toBeVisible();
    await expect(panel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
    await expect(explanation).toHaveCSS('color', 'rgb(184, 212, 229)');

    for (const [locator, label] of [
      [panel, 'browser status panel'],
      [card.first(), 'browser unavailable card'],
    ] as const) {
      const box = await requiredBoundingBox(locator, label);
      expect(box.x).toBeGreaterThanOrEqual(0);
      expect(box.x + box.width).toBeLessThanOrEqual(mobileViewport.width);
    }
    const panelBox = await requiredBoundingBox(panel, 'browser unavailable product panel');
    expect(panelBox.height).toBeGreaterThan(420);
    await expect(page.getByText('WHAT PARENTS CONTROL', { exact: true })).toHaveCount(0);
  });
}

function registerScreenAnalysisLayoutTests(): void {
  for (const [viewportLabel, viewportSize] of [
    ['mobile', mobileViewport],
    ['desktop', desktopViewport],
  ] as const) {
    test(`Screen Analysis owns a clear recoverable product surface on ${viewportLabel}`, async ({ page }) => {
      await page.setViewportSize(viewportSize);
      await page.goto('/#/screen-analysis');

      const parentRoute = page.locator('.parent-portal-route');
      const serviceBanner = page
        .locator('svg.parent-portal-svg-surface text')
        .filter({ hasText: /^PARENT SURFACE UNAVAILABLE$/u })
        .first();
      const unavailableActionHeading = page
        .locator('svg.parent-portal-svg-surface text')
        .filter({ hasText: /^CHANGES ARE UNAVAILABLE$/u })
        .first();
      const ownerJargon = page
        .locator('svg.parent-portal-svg-surface text')
        .filter({ hasText: /OWNER-BACKED MUTATION/u });
      const screenPanel = page.getByRole('region', { exact: true, name: 'Screen analysis' });
      const screenCards = screenPanel.locator('article');

      await expect(serviceBanner).toHaveCount(0);
      await expect(unavailableActionHeading).toHaveCount(0);
      await expect(ownerJargon).toHaveCount(0);
      await expect(screenPanel).toBeVisible();
      await expect(screenPanel.locator('details')).toHaveCount(0);
      await expect(screenPanel.getByRole('button', { exact: true, name: 'Retry status' })).toBeVisible();
      await expect(screenCards).toHaveCount(3);
      await expect(screenPanel.getByRole('heading', { exact: true, name: 'Activity rows' })).toBeVisible();
      await expect(screenPanel.getByRole('heading', { exact: true, name: 'Analysis capability' })).toBeVisible();
      await expect(screenPanel.getByRole('heading', { exact: true, name: 'Evidence custody' })).toBeVisible();

      const [parentRouteBox, screenPanelBox] = await Promise.all([
        requiredBoundingBox(parentRoute, 'parent portal route'),
        requiredBoundingBox(screenPanel, 'Screen Analysis panel'),
      ]);

      expect(screenPanelBox.y).toBeGreaterThanOrEqual(parentRouteBox.y);
      expect(screenPanelBox.x).toBeGreaterThanOrEqual(parentRouteBox.x);
      expect(screenPanelBox.x + screenPanelBox.width).toBeLessThanOrEqual(parentRouteBox.x + parentRouteBox.width + 1);
      expect(screenPanelBox.y + screenPanelBox.height).toBeLessThanOrEqual(
        parentRouteBox.y + parentRouteBox.height + 1
      );

      for (let index = 0; index < (await screenCards.count()); index += 1) {
        const card = screenCards.nth(index);
        await expect(card).toBeVisible();
        await card.scrollIntoViewIfNeeded();
        const cardBox = await requiredBoundingBox(card, `Screen Analysis card ${index + 1}`);
        expect(cardBox.y).toBeGreaterThanOrEqual(screenPanelBox.y);
        expect(cardBox.y + cardBox.height).toBeLessThanOrEqual(screenPanelBox.y + screenPanelBox.height + 1);
      }
    });
  }
}

function registerActivityReportLayoutTest(): void {
  test('activity report frequency options fit and remain selectable on mobile', async ({ page }) => {
    await page.setViewportSize(mobileViewport);
    await page.goto('/#/activity');

    const monthlyOption = page.getByRole('button', { exact: true, name: 'Select Monthly' });
    await expect(monthlyOption).toBeVisible();

    const monthlyBox = await requiredBoundingBox(monthlyOption, 'activity Monthly option');
    expect(monthlyBox.x).toBeGreaterThanOrEqual(0);
    expect(monthlyBox.x + monthlyBox.width).toBeLessThanOrEqual(mobileViewport.width);

    await monthlyOption.click();
    await expect(page.getByRole('group', { name: /^(?:Frequency|Freq\.): Monthly$/u }).first()).toBeVisible();
  });
}

async function expectVisibleControlsInsideViewport(controls: Locator, label: string): Promise<void> {
  const count = await controls.count();
  for (let index = 0; index < count; index += 1) {
    const control = controls.nth(index);
    if (!(await control.isVisible())) continue;

    const box = await requiredBoundingBox(control, `${label} ${index + 1}`);
    expect(box.x).toBeGreaterThanOrEqual(0);
    expect(box.x + box.width).toBeLessThanOrEqual(mobileViewport.width);
  }
}

async function requiredBoundingBox(locator: Locator, label: string): Promise<ElementBox> {
  const box = await locator.boundingBox();
  if (box === null) {
    throw new Error(`Expected a rendered bounding box for ${label}`);
  }
  return box;
}
