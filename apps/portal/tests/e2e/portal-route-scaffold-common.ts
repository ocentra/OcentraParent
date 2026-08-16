import { expect, type Page } from '@playwright/test';
import {
  PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION,
  PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS,
} from '@ocentra-parent/portal-domain/parent-assistant-chat';
import { PARENT_PORTAL_NAV_LABELS } from '@ocentra-parent/portal-domain/parent-portal-nav';

export const routeSurfaceReadyTimeoutMs = 30_000;
const assistantRulesAction = requireAssistantRulesAction();
const assistantRulesExplainChoice = requireAssistantRulesExplainChoice();

export function requireAssistantNewChatAction() {
  if (!PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION) {
    throw new Error('Assistant route scaffold requires the exported New Chat quick action.');
  }
  return PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION;
}

export function requireAssistantRulesAction() {
  const action = PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS.find((candidate) => candidate.quickActionId === 'rules');
  if (!action) {
    throw new Error('Assistant route scaffold requires the exported Rules quick action.');
  }
  return action;
}

export function requireAssistantRulesExplainChoice() {
  const choice = assistantRulesAction.choices.find((candidate) => candidate.choiceId === 'rules-explain');
  if (!choice) {
    throw new Error('Assistant route scaffold requires the exported Rules Explain choice.');
  }
  return choice;
}

export async function assertPolicyGuideDeepLinks(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  await page.goto('/#/browser-settings');
  await expect(surface.locator('[aria-label="Open Browser Rules guide"]')).toBeVisible();
  await assertBrowserPolicyDeviceTargets(page, surface);
  await surface.locator('[aria-label="Open Browser Budget guide"]').focus();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/#\/policy\?guideTopic=browser-policy-guide&guidePage=2$/);
  await expect(surface.locator('text').filter({ hasText: 'BROWSER BUDGET' }).first()).toBeVisible();
  await page.getByRole('button', { name: 'Show QUICK ACTION' }).click({ force: true });
  await page.getByRole('button', { name: 'Open Browser setup' }).click({ force: true });
  await expect(page).toHaveURL(/#\/browser-settings$/);
  await page.goto('/#/policy-apps');
  await expect(surface.locator('[aria-label="Open Apps Rules guide"]')).toBeVisible();
}

export async function assertSidePanelFoldouts(page: Page): Promise<void> {
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand GUIDE');
  await expect(page.getByRole('button', { name: 'Collapse GUIDE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open START HERE' })).toBeVisible();
  await clickSidePanelButton(page, 'Collapse GUIDE');
  await expect(page.getByRole('button', { name: 'Expand GUIDE' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Open START HERE' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Expand MANAGE' })).toBeVisible();
  await page.goto('/#/settings-rules');
  await expect(page.getByRole('button', { name: 'Collapse MANAGE' })).toBeVisible();
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Portal}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Portal}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Settings}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Devices}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Devices}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Lan}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Capability}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Remote}` })).toBeVisible();
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Platforms}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Updates}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Activity}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.ReportSet}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.AppsGames}` })).toHaveCount(0);
  await expect(page.getByRole('button', { name: `Open ${PARENT_PORTAL_NAV_LABELS.Builder}` })).toHaveCount(0);
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.Devices}`);
  await expect(page).toHaveURL(/#\/devices$/);
  await page.goto('/#/overview');
  await expect(page.getByRole('button', { name: 'Expand MANAGE' })).toBeVisible();
}

export async function assertDuplicateLabelSidePanelRoutes(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand GUIDE');
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.RulesGuide}`);
  await expect(page).toHaveURL(/#\/policy$/);
  await expect(surface.locator('text').filter({ hasText: 'Rules' }).first()).toBeVisible();
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand MANAGE');
  await expandSidePanelGroup(page, PARENT_PORTAL_NAV_LABELS.Policies);
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.Browser}`);
  await expect(page).toHaveURL(/#\/browser-settings$/);
  await expect(page.getByRole('button', { name: 'Open Browser guide' })).toBeVisible();
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand GUIDE');
  await clickSidePanelButton(page, `Open ${PARENT_PORTAL_NAV_LABELS.ReportsGuide}`);
  await expect(page).toHaveURL(/#\/reports-guide$/);
  await expect(surface.locator('text').filter({ hasText: 'Reports And Summaries' }).first()).toBeVisible();
  await page.goto('/#/overview');
  await clickSidePanelButton(page, 'Expand MANAGE');
  await expect(page.getByRole('button', { name: `Expand ${PARENT_PORTAL_NAV_LABELS.Activity}` })).toHaveCount(0);
  await page.goto('/#/activity');
  await expect(page.getByRole('button', { name: 'Open Report guide' })).toBeVisible();
}

export async function assertManageTargetSelectorSemantics(page: Page): Promise<void> {
  const surface = page.locator('svg.parent-portal-svg-surface');
  const targetSelector = page.getByRole('button', { exact: true, name: 'Focus parent control selector' });

  await page.goto('/#/settings-rules');
  await expect(surface).toBeVisible();
  await expect(targetSelector).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'PORTAL' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Parent owned' }).first()).toBeVisible();

  await page.goto('/#/lan-pairing');
  await expect(targetSelector).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Local Area Network' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SELECTED DEVICE CONTEXT' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'SOURCE' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'CONTROL' }).first()).toBeVisible();
  for (const tabLabel of ['Info', 'Update', 'Capability']) {
    await expect(surface.locator('text').filter({ hasText: tabLabel }).first()).toBeVisible();
  }
  const pairTab = surface.getByRole('tab', { exact: true, name: 'Show LAN pairing Pair' });
  if ((await pairTab.count()) > 0) {
    await expect(pairTab).toBeVisible();
  } else {
    await expect(surface.locator('text').filter({ hasText: 'Policy target' }).first()).toBeVisible();
  }
  await expect(surface.locator('text').filter({ hasText: 'Local device' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'This parent portal' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'New child device' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Aarav laptop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Family tablet' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'AI / New child device' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'Select D001' })).toHaveCount(0);
  await expect(page.getByRole('button', { name: 'D004 is unsupported' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'UI check device 1' })).toHaveCount(0);

  await page.goto('/#/platforms-install');
  await expect(targetSelector).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Local Area Network' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Info' }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'Platforms / Parent desktop' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Platforms / Parent profile' })).toHaveCount(0);
}

export async function assertAssistantEntryAvailable(page: Page): Promise<void> {
  await page.goto('/#/overview');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(page.getByRole('button', { name: 'Open AI assistant' })).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'AI ASSISTANT' }).first()).toBeVisible();
  await page.getByRole('button', { name: 'Open AI assistant' }).click({ force: true });
  await expect(page).toHaveURL(/#\/assistant$/);
  await expect(page.getByRole('button', { name: 'Close parent assistant' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Hide action panel' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Send message to MIA' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Use voice input for MIA' })).toBeVisible();
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await page.getByRole('tab', { name: 'History' }).click({ force: true });
  await expect(page.getByRole('button', { name: /^Report history$/ })).toBeVisible();
  await expect(page.getByRole('button', { name: /^Rules history$/ })).toBeVisible();
  await page.getByRole('tab', { name: 'Quick Action' }).click({ force: true });
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await expect(page.getByRole('button', { name: /^Ask MIA: Give me the overall report$/ })).toBeVisible();
  await expect(page.getByRole('button', { name: /^Copy MIA message$/ }).first()).toBeVisible();
  await expect(surface.locator('text').filter({ hasText: 'AI assisted view' })).toHaveCount(0);
  await expect(surface.locator('text').filter({ hasText: 'Ask AI Assistant to update a setting' })).toHaveCount(0);
  await assistantQuickActionButton(page, assistantRulesAction.title).click({ force: true });
  await expectAssistantQuickActionChoice(page, assistantRulesAction.title, assistantRulesExplainChoice.label);
  await assistantQuickActionChoiceButton(page, assistantRulesAction.title, assistantRulesExplainChoice.label).click({
    force: true,
  });
  await expect(page).toHaveURL(/#\/assistant$/);
  await expect(page.getByRole('button', { name: /^Copy YOU message$/ }).first()).toBeVisible();
  await expect(page.getByRole('button', { name: /^Ask MIA: Change a rule$/ })).toBeVisible();
  await expect(page.getByRole('article', { name: `YOU: ${assistantRulesExplainChoice.label}` })).toBeVisible();
  await page.getByRole('button', { name: 'Hide action panel' }).click({ force: true });
  await expect(page.getByRole('button', { name: 'Show action panel' })).toBeVisible();
  await expect(assistantQuickActionButton(page, assistantRulesAction.title)).toHaveCount(0);
  await page.getByRole('button', { name: 'Show action panel' }).click({ force: true });
  await expectAssistantQuickAction(page, assistantRulesAction.title);
  await page.getByRole('button', { name: 'Close parent assistant' }).click({ force: true });
  await expect(page).toHaveURL(/#\/overview$/);
  await expect(page.getByRole('button', { name: 'Open AI assistant' })).toBeVisible();
}

export function assistantQuickActionButton(page: Page, actionTitle: string) {
  return page.getByRole('button', { exact: true, name: `Ask MIA about ${actionTitle}` });
}

export async function expectAssistantQuickAction(page: Page, actionTitle: string): Promise<void> {
  await expect(assistantQuickActionButton(page, actionTitle)).toBeVisible();
}

export function assistantQuickActionChoiceButton(page: Page, _actionTitle: string, choiceLabel: string) {
  return page.getByRole('button', { exact: true, name: `Ask MIA: ${choiceLabel}` });
}

export async function expectAssistantQuickActionChoice(
  page: Page,
  actionTitle: string,
  choiceLabel: string
): Promise<void> {
  await expect(assistantQuickActionChoiceButton(page, actionTitle, choiceLabel)).toBeVisible();
}

export async function assertSupportContactRoute(page: Page): Promise<void> {
  await page.goto('/#/diagnostics');
  const surface = page.locator('svg.parent-portal-svg-surface');
  await expect(page.getByRole('heading', { exact: true, name: 'Device diagnostics' })).toBeVisible();
  await expect(page.getByRole('button', { exact: true, name: 'Copy diagnostics' })).toBeVisible();
  await expectSurfaceTextToContain(surface, 'DIAGNOSTICS');
  await expectSurfaceTextToContain(
    surface,
    'Support messages are parent-authored and sent only when the parent chooses.'
  );
  await expectSurfaceTextToContain(surface, 'CURRENT AREA');
  await expectSurfaceTextToContain(surface, 'SUPPORT');
  await expectSurfaceTextToContain(surface, 'DATA CUSTODY');
  await expectSurfaceTextToContain(surface, 'LOCAL FIRST');
}

export async function assertFrameTunerRoute(page: Page): Promise<void> {
  await page.goto('/#/app-layout');
  await expect(page.getByRole('heading', { name: 'App layout' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Save JSON' })).toBeVisible();
  await expect(page.locator('.app-sidebar')).toHaveCount(0);
  await expect(page.locator('svg.portal-frame-backdrop-svg')).toHaveCount(0);
  await assertAppLayoutTopTabs(page);
  await assertMainAppLayoutHierarchy(page);
  await assertChatLayoutHierarchy(page);
}

export async function surfaceText(surface: ReturnType<Page['locator']>): Promise<string> {
  return (await surface.locator('text').allTextContents()).join(' ');
}

export async function expectSurfaceTextToContain(
  surface: ReturnType<Page['locator']>,
  expected: string
): Promise<void> {
  await expect.poll(() => surfaceText(surface), { timeout: routeSurfaceReadyTimeoutMs }).toContain(expected);
}

export async function expectSurfaceTextToMatch(surface: ReturnType<Page['locator']>, expected: RegExp): Promise<void> {
  await expect.poll(() => surfaceText(surface), { timeout: routeSurfaceReadyTimeoutMs }).toMatch(expected);
}

export async function closeParentPortalDetailIfOpen(page: Page): Promise<void> {
  const closeDetailButton = page.getByRole('button', { name: 'Close parent portal detail' });
  if ((await closeDetailButton.count()) === 0) {
    return;
  }
  await closeDetailButton.click({ force: true });
  await expect(closeDetailButton).toHaveCount(0, { timeout: routeSurfaceReadyTimeoutMs });
}

export async function assertBrowserPolicyDeviceTargets(
  page: Page,
  surface: ReturnType<Page['locator']>
): Promise<void> {
  const viewport = page.viewportSize();
  await page.setViewportSize({
    width: Math.max(viewport?.width ?? 1280, 1600),
    height: Math.max(viewport?.height ?? 720, 960),
  });
  try {
    await expect(page.getByText('Per Device').first()).toBeVisible();
    await expect(page.getByRole('button', { name: /^Select (?!LAN ).+/ }).first()).toBeVisible();
    await expect(surface.locator('text').filter({ hasText: /^LAN 192\.168\.2\.1$/ })).toHaveCount(0);
  } finally {
    if (viewport) {
      await page.setViewportSize(viewport);
    }
  }
}

export async function clickSidePanelButton(page: Page, name: string): Promise<void> {
  const button = page.getByRole('button', { exact: true, name });
  await expect(button).toBeVisible();
  await button.dispatchEvent('click');
}

export async function expandSidePanelGroup(page: Page, label: string): Promise<void> {
  const expandButton = page.getByRole('button', { name: `Expand ${label}` });
  if ((await expandButton.count()) > 0) {
    await expandButton.click({ force: true });
  }
  await expect(page.getByRole('button', { name: `Collapse ${label}` })).toBeVisible();
}

export async function assertMainAppLayoutHierarchy(page: Page): Promise<void> {
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Main App');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Side panel');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Sidepanel top');
  await expect(page.getByRole('tab', { name: 'Side panel' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Main panel' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Sidepanel top' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Sidepanel bottom' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Chrome' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Colors' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Content' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'Reset surface' })).toBeVisible();
  await page.getByRole('tab', { name: 'Content' }).click();
  await expect(page.getByRole('button', { name: 'Add foldout' })).toBeVisible();
  await expect(page.getByText('Sidepanel foldouts')).toBeVisible();
}

export async function assertAppLayoutTopTabs(page: Page): Promise<void> {
  await expect(page.getByRole('tab', { name: 'Main App' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Chat Interface' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Carousel' })).toHaveCount(0);
  await expect(page.getByRole('tab', { name: 'Golden card' })).toHaveCount(0);
  await expect(page.getByRole('tab', { name: 'Save and JSON' })).toHaveCount(0);
}

export async function assertChatLayoutHierarchy(page: Page): Promise<void> {
  await page.getByRole('tab', { name: 'Chat Interface' }).click();
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Chat Interface');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Side panel');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Sidepanel top');
  await expect(page.getByRole('tab', { name: 'Sidepanel top' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Sidepanel bottom' })).toBeVisible();
  await page.getByRole('tab', { name: 'Main panel' }).click();
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Chat Interface');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Main panel');
  await expect(page.locator('.portal-frame-tuner-hierarchy')).toContainText('Top choices');
  await expect(page.getByRole('tab', { name: 'Top choices' })).toBeVisible();
  await expect(page.getByRole('tab', { name: 'Main bottom' })).toBeVisible();
  await page.getByRole('tab', { name: 'Content' }).click();
  await expect(page.getByRole('tab', { name: 'Top choices' })).toHaveCount(2);
}
