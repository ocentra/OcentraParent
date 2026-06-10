import { describe, expect, it } from 'vitest';
import {
  PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION,
  PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS,
  PARENT_PORTAL_CONTENT,
  PARENT_PORTAL_GUIDE_TOPICS,
  PARENT_PORTAL_NAV_LABELS,
  PARENT_PORTAL_ROUTE_CONTEXT,
  PortalClipboard,
  PortalCommandButtons,
  PortalConnectionState,
  PortalDetails,
  PortalDiagnostics,
  PortalDom,
  PortalAssets,
  PortalExternalLinks,
  PortalFrameTuner,
  PortalActivitySurfaceDefaultRequestPayload,
  PortalOverviewCommands,
  PortalAiRuntimeRoutes,
  PortalRouteDescriptors,
  PortalRouteGroup,
  PortalAppGameParentSurfaceRoutes,
  PortalBrowserParentSurfaceRoutes,
  PortalNetworkEvidenceDrawerRoutes,
  PortalRoute,
  PortalRouteSchema,
  PortalRoutes,
  PortalScreenSettingsRoutes,
  PortalScreenSummaryRoutes,
  PortalSidebarRouteDescriptors,
  PortalTiming,
  PortalTrackingStatusRoutes,
  PortalUnifiedChrome,
  decodePortalClipboardText,
  isPortalAiRuntimeRoute,
  isPortalAppGameParentSurfaceRoute,
  isPortalBrowserParentSurfaceRoute,
  isPortalNetworkEvidenceDrawerRoute,
  isPortalScreenSettingsRoute,
  isPortalScreenSummaryRoute,
  isPortalTrackingStatusRoute,
  parentPortalRouteContext,
  type ParentPortalHashRoutePath,
  type ParentPortalNavItem,
  type ParentPortalNavLabel,
  type ParentPortalNavSectionLabel,
} from '../src/contracts';

function routeFromHashPath(routePath: ParentPortalHashRoutePath): PortalRoute {
  const [routeId = PortalDom.EmptyHashRoute] = routePath
    .slice(PortalDom.HashPrefix.length)
    .split(PortalDom.HashQuerySeparator);
  return PortalRouteSchema.parse(routeId);
}

function selectableParentPortalTargetIds(): ReadonlySet<string> {
  const controls = [...PARENT_PORTAL_CONTENT.controlAreas, ...PARENT_PORTAL_CONTENT.quickControls];
  return new Set([...controls.map((control) => control.id), ...PARENT_PORTAL_GUIDE_TOPICS.map((topic) => topic.id)]);
}

function expectNavRouteLabelsToMatchContexts(): void {
  for (const item of PARENT_PORTAL_CONTENT.navItems.filter((entry) => entry.routePath)) {
    const routePath = item.routePath;
    expect(routePath).toBeDefined();
    const route = routeFromHashPath(routePath);
    const routeContext = PARENT_PORTAL_ROUTE_CONTEXT[route];
    if (routePath.includes(PortalDom.HashQuerySeparator)) {
      expect(routePath).toBe(
        `${PortalDom.HashPrefix}${PortalRoute.FrameTuner}${PortalDom.HashQuerySeparator}${PortalDom.BackgroundDevToolHashFlag}`
      );
      expect(item.label).toBe(PARENT_PORTAL_NAV_LABELS.Background);
      continue;
    }
    expect(routeContext?.navLabel).toBe(item.label);
  }
}

function expectRouteContextsToTargetSelectableControls(selectableTargetIds: ReadonlySet<string>): void {
  for (const [route, routeContext] of Object.entries(PARENT_PORTAL_ROUTE_CONTEXT) as Array<
    [PortalRoute, NonNullable<(typeof PARENT_PORTAL_ROUTE_CONTEXT)[PortalRoute]>]
  >) {
    const navMatches = PARENT_PORTAL_CONTENT.navItems.filter((item) => item.routePath === `#/${route}`);
    if (navMatches.length > 0) {
      expect(navMatches.map((item) => item.label)).toEqual([routeContext.navLabel]);
    }
    expect(selectableTargetIds.has(routeContext.selectedControlId)).toBe(true);
  }
}

function expectGuideTargetsToResolve(selectableTargetIds: ReadonlySet<string>): void {
  const declaredNavLabels = new Set(Object.values(PARENT_PORTAL_NAV_LABELS));

  for (const topic of PARENT_PORTAL_GUIDE_TOPICS) {
    for (const note of [...topic.tips, ...topic.actions]) {
      if (!note.targetRoutePath) continue;
      const navMatches = PARENT_PORTAL_CONTENT.navItems.filter((item) => item.routePath === note.targetRoutePath);
      if (note.targetNavLabel) {
        expect(declaredNavLabels.has(note.targetNavLabel)).toBe(true);
      }
      if (navMatches.length === 0) {
        const route = routeFromHashPath(note.targetRoutePath);
        const routeContext = PARENT_PORTAL_ROUTE_CONTEXT[route];
        expect(selectableTargetIds.has(routeContext?.selectedControlId ?? '')).toBe(true);
        continue;
      }
      expect(navMatches).toHaveLength(1);
    }
  }
}

function manageNavItems(): readonly ParentPortalNavItem[] {
  return PARENT_PORTAL_CONTENT.navItems.filter((item) => item.groupId === 'manage');
}

function summarizeManageItem(item: ParentPortalNavItem) {
  return {
    icon: item.icon,
    routePath: item.routePath,
    sectionLabel: item.sectionLabel,
  };
}

function expectManageItemOrder(): void {
  expect(manageNavItems().map((item) => item.routePath)).toEqual([
    '#/settings-rules',
    '#/devices',
    '#/activity',
    '#/browser-settings',
    '#/policy-apps',
    '#/policy-games',
    '#/policy-screen',
    '#/policy-network',
    '#/policy-tracking',
    '#/policy-remote-screen',
    '#/drive-connections',
    '#/ai-runtime',
    '#/subscription',
  ]);
}

function expectManageSectionRoutes(
  sectionLabel: ParentPortalNavSectionLabel,
  routePaths: readonly ParentPortalHashRoutePath[]
): void {
  expect(
    manageNavItems()
      .filter((item) => item.sectionLabel === sectionLabel)
      .map((item) => item.routePath)
  ).toEqual(routePaths);
}

function expectManageStandaloneItem(
  label: ParentPortalNavLabel,
  expected: ReturnType<typeof summarizeManageItem>
): void {
  expect(
    manageNavItems()
      .filter((item) => item.label === label)
      .map((item) => summarizeManageItem(item))
  ).toEqual([expected]);
}

function expectNoManageSectionChildren(sectionLabel: ParentPortalNavSectionLabel): void {
  expect(
    manageNavItems()
      .filter((item) => item.sectionLabel === sectionLabel)
      .map((item) => item.routePath)
  ).toEqual([]);
}

function expectManageAccountAndControlBuckets(): void {
  expectManageStandaloneItem(PARENT_PORTAL_NAV_LABELS.Portal, {
    icon: 'portal',
    routePath: '#/settings-rules',
    sectionLabel: undefined,
  });
  expectManageStandaloneItem(PARENT_PORTAL_NAV_LABELS.DataPrivacy, {
    icon: 'drives',
    routePath: '#/drive-connections',
    sectionLabel: undefined,
  });
  expectManageStandaloneItem(PARENT_PORTAL_NAV_LABELS.AiMemory, {
    icon: 'ai-setup',
    routePath: '#/ai-runtime',
    sectionLabel: undefined,
  });
  expectManageStandaloneItem(PARENT_PORTAL_NAV_LABELS.Account, {
    icon: 'account',
    routePath: '#/subscription',
    sectionLabel: undefined,
  });
}

function expectManageCollapsedSections(): void {
  expectNoManageSectionChildren(PARENT_PORTAL_NAV_LABELS.Portal);
  expectNoManageSectionChildren(PARENT_PORTAL_NAV_LABELS.DataPrivacy);
  expectNoManageSectionChildren(PARENT_PORTAL_NAV_LABELS.AiMemory);
  expectNoManageSectionChildren(PARENT_PORTAL_NAV_LABELS.Account);
  expectManageStandaloneItem(PARENT_PORTAL_NAV_LABELS.Devices, {
    icon: 'lan',
    routePath: '#/devices',
    sectionLabel: undefined,
  });
  expectNoManageSectionChildren(PARENT_PORTAL_NAV_LABELS.Devices);
  expectManageStandaloneItem(PARENT_PORTAL_NAV_LABELS.Activity, {
    icon: 'activity',
    routePath: '#/activity',
    sectionLabel: undefined,
  });
  expectNoManageSectionChildren(PARENT_PORTAL_NAV_LABELS.Activity);
}

describe('portal route schema contracts', () => {
  it('PortalRouteSchema: accepts only declared dev routes', () => {
    expect(PortalRoutes).toEqual([
      'overview',
      'assistant',
      'start',
      'activity',
      'browser',
      'browser-settings',
      'policy',
      'policy-apps',
      'policy-games',
      'policy-screen',
      'policy-network',
      'policy-tracking',
      'policy-remote-screen',
      'rule-management',
      'schedules',
      'approvals',
      'enforcement',
      'privacy-design',
      'memory',
      'memory-settings',
      'ai-guide',
      'ai-runtime',
      'api-providers',
      'reports-guide',
      'screen-analysis',
      'app-game-sessions',
      'network-activity',
      'devices',
      'lan-pairing',
      'capability-status',
      'notifications',
      'notification-channels',
      'drive-connections',
      'export-retention',
      'remote-access',
      'report-compiler',
      'audit-history',
      'subscription',
      'entitlements',
      'platforms-install',
      'install-updates',
      'diagnostics',
      'settings-rules',
      'app-layout',
      'commands',
      'events',
      'logs',
    ]);
    expect(PortalRouteSchema.safeParse('commands').success).toBe(true);
    expect(PortalRouteSchema.safeParse('settings-rules').success).toBe(true);
    expect(PortalRouteSchema.safeParse('policy-apps').success).toBe(true);
    expect(PortalRouteSchema.safeParse('policy-games').success).toBe(true);
    expect(PortalRouteSchema.safeParse('policy-screen').success).toBe(true);
    expect(PortalRouteSchema.safeParse('policy-network').success).toBe(true);
    expect(PortalRouteSchema.safeParse('policy-tracking').success).toBe(true);
    expect(PortalRouteSchema.safeParse('frame-tuner').success).toBe(false);
    expect(PortalRouteSchema.safeParse('billing').success).toBe(false);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.route)).toContain(PortalRoute.PolicyApps);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.route)).toContain(PortalRoute.PolicyGames);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.route)).toContain(PortalRoute.PolicyScreen);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.route)).toContain(PortalRoute.PolicyNetwork);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.route)).toContain(PortalRoute.PolicyTracking);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.group)).toContain(PortalRouteGroup.Monitor);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Start here');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Activity');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Private by design');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Notifications');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Connect your drives');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Settings');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('App layout');
    expect(PortalSidebarRouteDescriptors.map((descriptor) => descriptor.label)).not.toContain('App layout');
  });
});

describe('portal guide route contracts', () => {
  it('guide side-panel routes: keep guide entries on guide pages until the user chooses a manage action', () => {
    const guideItems = PARENT_PORTAL_CONTENT.navItems.filter((item) => item.groupId === 'guide');
    const manageItems = PARENT_PORTAL_CONTENT.navItems.filter((item) => item.groupId === 'manage');
    const guideRules = guideItems.find((item) => item.routePath === '#/policy');
    const guideAi = guideItems.find((item) => item.label === PARENT_PORTAL_NAV_LABELS.Ai);
    const guideReports = guideItems.find((item) => item.label === PARENT_PORTAL_NAV_LABELS.ReportsGuide);
    const policyRoutes = manageItems
      .filter((item) => item.sectionLabel === PARENT_PORTAL_NAV_LABELS.Policies)
      .map((item) => item.routePath);

    expect(guideRules?.label).toBe(PARENT_PORTAL_NAV_LABELS.RulesGuide);
    expect(manageItems.find((item) => item.label === PARENT_PORTAL_NAV_LABELS.Portal)?.routePath).toBe(
      '#/settings-rules'
    );
    expect(manageItems.find((item) => item.label === PARENT_PORTAL_NAV_LABELS.Account)?.routePath).toBe(
      '#/subscription'
    );
    expect(manageItems.find((item) => item.label === PARENT_PORTAL_NAV_LABELS.DataPrivacy)?.routePath).toBe(
      '#/drive-connections'
    );
    expect(manageItems.find((item) => item.label === PARENT_PORTAL_NAV_LABELS.AiMemory)?.routePath).toBe(
      '#/ai-runtime'
    );
    expect(policyRoutes).toEqual([
      '#/browser-settings',
      '#/policy-apps',
      '#/policy-games',
      '#/policy-screen',
      '#/policy-network',
      '#/policy-tracking',
      '#/policy-remote-screen',
    ]);
    expect(manageItems.some((item) => item.routePath === '#/rule-management')).toBe(false);
    expect(manageItems.some((item) => item.routePath === '#/schedules')).toBe(false);
    expect(manageItems.some((item) => item.routePath === '#/approvals')).toBe(false);
    expect(manageItems.some((item) => item.routePath === '#/enforcement')).toBe(false);
    expect(guideAi?.routePath).toBe('#/ai-guide');
    expect(guideReports?.routePath).toBe('#/reports-guide');
    expect(parentPortalRouteContext(PortalRoute.Policy).pageMode).toBe('parentGuide');
    expect(parentPortalRouteContext(PortalRoute.RuleManagement).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.PolicyApps).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.PolicyGames).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.PolicyScreen).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.PolicyNetwork).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.PolicyTracking).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.AiGuide).pageMode).toBe('parentGuide');
    expect(parentPortalRouteContext(PortalRoute.ReportsGuide).pageMode).toBe('parentGuide');
    expect(parentPortalRouteContext(PortalRoute.AiRuntime).pageMode).toBe('parentManage');
  });
});

describe('portal collapsed manage route contracts', () => {
  it('collapsed device and activity routes resolve to the unified manage pages', () => {
    expect(parentPortalRouteContext(PortalRoute.Devices)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.LanPairing)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.CapabilityStatus)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.RemoteAccess)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.DataPrivacy,
      selectedControlId: 'remote-access',
    });
    expect(parentPortalRouteContext(PortalRoute.PlatformsInstall)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.InstallUpdates)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.ScreenAnalysis)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.AppGameSessions)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'app-game-sessions',
    });
    expect(parentPortalRouteContext(PortalRoute.NetworkActivity)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.ReportCompiler)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.RuleManagement)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.RuleSet,
      selectedControlId: 'rules-management',
    });
    expect(parentPortalRouteContext(PortalRoute.Schedules)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Schedules,
      selectedControlId: 'schedules-budgets',
    });
    expect(parentPortalRouteContext(PortalRoute.Approvals)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Approvals,
      selectedControlId: 'approvals',
    });
    expect(parentPortalRouteContext(PortalRoute.Enforcement)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Enforce,
      selectedControlId: 'enforcement-readiness',
    });
  });
});

describe('portal product route panel contracts', () => {
  it('keeps network evidence drawer routes canonical and product-route scoped', () => {
    expect(PortalNetworkEvidenceDrawerRoutes).toEqual([PortalRoute.Activity, PortalRoute.NetworkActivity]);
    expect(isPortalNetworkEvidenceDrawerRoute(PortalRoute.Activity)).toBe(true);
    expect(isPortalNetworkEvidenceDrawerRoute(PortalRoute.NetworkActivity)).toBe(true);
    expect(isPortalNetworkEvidenceDrawerRoute(PortalRoute.Commands)).toBe(false);
    expect(isPortalNetworkEvidenceDrawerRoute(PortalRoute.Overview)).toBe(false);
  });

  it('keeps product route panel bindings owned by portal-domain', () => {
    expect(PortalAppGameParentSurfaceRoutes).toEqual([PortalRoute.AppGameSessions]);
    expect(PortalAiRuntimeRoutes).toEqual([PortalRoute.AiRuntime]);
    expect(PortalBrowserParentSurfaceRoutes).toEqual([PortalRoute.Browser]);
    expect(PortalScreenSettingsRoutes).toEqual([PortalRoute.SettingsRules]);
    expect(PortalScreenSummaryRoutes).toEqual([PortalRoute.ScreenAnalysis]);
    expect(PortalTrackingStatusRoutes).toEqual([PortalRoute.PolicyTracking]);
    expect(isPortalAiRuntimeRoute(PortalRoute.AiRuntime)).toBe(true);
    expect(isPortalAppGameParentSurfaceRoute(PortalRoute.AppGameSessions)).toBe(true);
    expect(isPortalBrowserParentSurfaceRoute(PortalRoute.Browser)).toBe(true);
    expect(isPortalScreenSettingsRoute(PortalRoute.SettingsRules)).toBe(true);
    expect(isPortalScreenSummaryRoute(PortalRoute.ScreenAnalysis)).toBe(true);
    expect(isPortalTrackingStatusRoute(PortalRoute.PolicyTracking)).toBe(true);
    expect(isPortalAiRuntimeRoute(PortalRoute.Browser)).toBe(false);
    expect(isPortalAppGameParentSurfaceRoute(PortalRoute.Browser)).toBe(false);
    expect(isPortalBrowserParentSurfaceRoute(PortalRoute.NetworkActivity)).toBe(false);
    expect(isPortalScreenSettingsRoute(PortalRoute.PolicyTracking)).toBe(false);
    expect(isPortalScreenSummaryRoute(PortalRoute.Activity)).toBe(false);
    expect(isPortalTrackingStatusRoute(PortalRoute.Activity)).toBe(false);
  });
});

describe('portal nav matrix contracts', () => {
  it('parent portal nav matrix: route labels, guide actions, and selected controls stay route-addressed', () => {
    const selectableTargetIds = selectableParentPortalTargetIds();
    const navItemsWithRoutes = PARENT_PORTAL_CONTENT.navItems.filter((item) => item.routePath);
    const routePaths = navItemsWithRoutes.map((item) => item.routePath);

    expect(new Set(routePaths).size).toBe(routePaths.length);
    expectNavRouteLabelsToMatchContexts();
    expectRouteContextsToTargetSelectableControls(selectableTargetIds);
    expectGuideTargetsToResolve(selectableTargetIds);
  });
});

describe('portal manage section contracts', () => {
  it('parent portal manage sections: keep side-panel items in the agreed account and control buckets', () => {
    expectManageItemOrder();
    expectManageSectionRoutes(PARENT_PORTAL_NAV_LABELS.Policies, [
      '#/browser-settings',
      '#/policy-apps',
      '#/policy-games',
      '#/policy-screen',
      '#/policy-network',
      '#/policy-tracking',
      '#/policy-remote-screen',
    ]);
    expectManageAccountAndControlBuckets();
    expectManageCollapsedSections();
  });
});

describe('portal parent assistant contracts', () => {
  it('parent assistant quick actions: expose the typed assistant categories used by the chat side panel', () => {
    expect(PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION?.quickActionId).toBe('new-chat');
    expect(PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS.map((action) => action.quickActionId)).toEqual([
      'overview',
      'start',
      'report',
      'browser-state',
      'rules',
      'memory',
      'ai-setup',
      'private',
      'devices',
      'alerts',
      'drives',
      'support-api',
    ]);
    expect(
      PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS.find((action) => action.quickActionId === 'rules')?.choices[1]
        ?.nextActionKind
    ).toBe('preview-rule-change');
  });
});

const EXPECTED_PORTAL_COMMAND_BUTTONS = [
  ['agent.health.check', 'agent.health.reported'],
  ['agent.activity.ingest.status.get', 'agent.activity.recent.summary.reported'],
  ['agent.browser.evidence.recent.get', 'agent.browser.evidence.recent.reported'],
  ['agent.activity.memory-graph.get', 'agent.activity.memory-graph.reported'],
  ['agent.browser.intervention.read-model.get', 'agent.browser.intervention.read-model.reported'],
  ['agent.browser.managed.bridge.poll', 'agent.browser.managed.status.reported'],
  ['agent.network.flow.read-model.get', 'agent.network.flow.read-model.reported'],
  ['agent.network.runtime.event-chain.stream.get', 'agent.network.runtime.event-chain.stream.reported'],
  ['agent.network.remote-delivery.status.get', 'agent.network.remote-delivery.status.reported'],
  ['agent.network.live-capture.status.get', 'agent.network.live-capture.status.reported'],
  ['agent.network.linux-nftables-lab.status.get', 'agent.network.linux-nftables-lab.status.reported'],
  ['agent.network.windows-firewall-lab.status.get', 'agent.network.windows-firewall-lab.status.reported'],
  ['agent.network.windows-wfp-gate.status.get', 'agent.network.windows-wfp-gate.status.reported'],
  ['agent.activity.tracking.read-model.get', 'agent.activity.tracking.read-model.reported'],
  ['agent.local-ai.runtime.status.get', 'agent.local-ai.runtime.status.reported'],
  ['agent.policy.preview.read-model.get', 'agent.policy.preview.read-model.reported'],
] as const;

const EXPECTED_PORTAL_OVERVIEW_COMMANDS = [
  'agent.health.check',
  'agent.log.snapshot.get',
  'agent.network.flow.read-model.get',
  'agent.lan-pairing.status.get',
  'agent.activity.ingest.status.get',
  'agent.activity.recent.summary.get',
  'agent.browser.evidence.recent.get',
  'agent.browser.managed.bridge.poll',
  'agent.browser.inventory.read-model.get',
  'agent.activity.memory-graph.get',
  'agent.activity.report.history.list',
  'agent.activity.screen.read-model.get',
  'agent.activity.app-use.read-model.get',
  'agent.activity.browser.read-model.get',
  'agent.activity.games.read-model.get',
  'agent.activity.app-game.notification-readiness.read-model.get',
  'agent.activity.network.read-model.get',
  'agent.browser.intervention.read-model.get',
  'agent.network.runtime.event-chain.stream.get',
  'agent.network.remote-delivery.status.get',
  'agent.network.live-capture.status.get',
  'agent.network.linux-nftables-lab.status.get',
  'agent.network.windows-firewall-lab.status.get',
  'agent.network.windows-wfp-gate.status.get',
  'agent.activity.tracking.read-model.get',
  'agent.local-ai.runtime.status.get',
  'agent.policy.preview.read-model.get',
] as const;

describe('portal command contracts', () => {
  it('PortalCommandButtons: maps each button to a typed command', () => {
    const commands = PortalCommandButtons.map((button) => button.command);
    const resultEvents = PortalCommandButtons.map((button) => button.resultEvent);
    for (const [command, resultEvent] of EXPECTED_PORTAL_COMMAND_BUTTONS) {
      expect(commands).toContain(command);
      expect(resultEvents).toContain(resultEvent);
    }
  });

  it('PortalOverviewCommands: keeps overview commands in expected order', () => {
    expect(PortalOverviewCommands.map((button) => button.command)).toEqual(EXPECTED_PORTAL_OVERVIEW_COMMANDS);
  });

  it('PortalActivitySurfaceDefaultRequestPayload: wires screen read-model commands to default request payload', () => {
    expect(
      PortalOverviewCommands.find((button) => button.command === 'agent.activity.screen.read-model.get')?.payload
    ).toEqual(PortalActivitySurfaceDefaultRequestPayload);
    expect(
      PortalCommandButtons.find((button) => button.command === 'agent.activity.screen.read-model.get')?.payload
    ).toEqual(PortalActivitySurfaceDefaultRequestPayload);
    expect(PortalActivitySurfaceDefaultRequestPayload).toMatchObject({
      scopeKind: 'family',
      familyId: 'family-local',
      rangeStart: expect.stringMatching(/^\d{4}-\d{2}-\d{2}T/u),
      rangeEnd: expect.stringMatching(/^\d{4}-\d{2}-\d{2}T/u),
      requestedAt: expect.stringMatching(/^\d{4}-\d{2}-\d{2}T/u),
    });
  });
});

describe('portal shared constants', () => {
  it('PortalConnectionState: exposes core shared tokens', () => {
    expect(PortalRoutes).toContain('overview');
    expect(PortalConnectionState.Connected).toBe('connected');
    expect(PortalTiming.CopyFeedbackMs).toBeGreaterThan(0);
    expect(PortalDiagnostics.SchemaVersion).toBe(1);
    expect(PortalDiagnostics.Field.AgentUrl).toBe('agentUrl');
    expect(PortalClipboard.CommandCopy).toBe('copy');
    expect(PortalDetails.PrivacyMode).toBe('Privacy mode');
    expect(PortalDetails.AdapterBoundary).toBe('Adapter boundary');
    expect(PortalDetails.ExecutionState).toBe('Execution state');
    expect(PortalDetails.ProviderSource).toBe('Provider source');
    expect(PortalDetails.DecisionSource).toBe('Decision source');
    expect(PortalDetails.ManagedSessionIntervention).toBe('Managed session intervention');
    expect(PortalDetails.UnmanagedBrowserEnforcement).toBe('Unmanaged browser enforcement');
    expect(PortalDetails.UnmanagedFallbackAction).toBe('Unmanaged fallback action');
    expect(PortalDetails.ParentRuleContextReferences).toBe('Parent rule context references');
    expect(PortalDetails.ParentRuleContextRefIds).toBe('Parent rule context ref IDs');
  });

  it('PortalDetails: exposes browser intervention proof labels', () => {
    expect(PortalDetails.BrowserBoundary).toBe('Browser boundary');
    expect(PortalDetails.ExactUrlClaim).toBe('Exact URL claim');
    expect(PortalDetails.UnmanagedDetection).toBe('Unmanaged detection');
    expect(PortalDetails.InterventionActionId).toBe('Intervention action ID');
    expect(PortalDetails.InterventionAuditId).toBe('Intervention audit ID');
    expect(PortalDetails.InterventionChildDelivery).toBe('Child delivery');
  });
});

describe('portal chrome constants', () => {
  it('PortalDom: exposes product surface class and event tokens', () => {
    expect(PortalDom.Tags.Image).toBe('img');
    expect(PortalDom.Classes.ControlCardGoldenArt).toBe('control-card-golden-art');
    expect(PortalDom.Classes.ControlCarouselFrame).toBe('control-carousel-frame');
    expect(PortalDom.Classes.ControlCarouselRail).toBe('control-carousel-rail');
    expect(PortalDom.Classes.ProductStatusCard).toBe('product-status-card');
    expect(PortalDom.Classes.ProductStatusCardBadge).toBe('product-status-card-badge');
    expect(PortalDom.Classes.ProductStatusCardBody).toBe('product-status-card-body');
    expect(PortalDom.Classes.ProductStatusCardMedia).toBe('product-status-card-media');
    expect(PortalDom.Classes.ProductStatusCardMeta).toBe('product-status-card-meta');
    expect(PortalDom.Classes.ProductStatusCardMetaValue).toBe('product-status-card-meta-value');
    expect(PortalDom.Attributes.AriaHidden).toBe('aria-hidden');
    expect(PortalDom.Events.Storage).toBe('storage');
    expect(PortalDom.Tags.TextArea).toBe('textarea');
    expect(PortalFrameTuner.GoldenFrame.StorageKey).toBe('ocentra-foreign-frame-config');
    expect(PortalFrameTuner.GoldenFrame.Channel).toBe('ocentra-foreign-frame-channel');
  });

  it('PortalUnifiedChrome: exposes outline header/footer tokens', () => {
    expect(PortalUnifiedChrome.Tags.Footer).toBe('footer');
    expect(PortalUnifiedChrome.Classes.Shell).toBe('portal-unified-shell');
    expect(PortalUnifiedChrome.Classes.ShellWork).toBe('portal-shell-work');
    expect(PortalUnifiedChrome.Classes.OutlineHeader).toBe('portal-outline-header');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderAction).toBe('portal-outline-header__action');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderActionIcon).toBe('portal-outline-header__action-icon');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderActionIconImage).toBe('portal-outline-header__action-icon-image');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderActionLabel).toBe('portal-outline-header__action-label');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderBrand).toBe('portal-outline-header__brand');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderBrandLogo).toBe('portal-outline-header__brand-logo');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderBrandLogoMount).toBe('portal-outline-header__brand-logo-mount');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderConnector).toBe('portal-outline-header__connector');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderConnectorBox).toBe('portal-outline-header__connector-box');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderConnectorSvg).toBe('portal-outline-header__connector-svg');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderFrame).toBe('portal-outline-header__frame');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderFrameLine).toBe('portal-outline-header__frame-line');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderFrameOuter).toBe('portal-outline-header__frame-outer');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderFrameSegmentGroup).toBe(
      'portal-outline-header__frame-segment-group'
    );
    expect(PortalUnifiedChrome.Classes.OutlineHeaderFrameSvg).toBe('portal-outline-header__frame-svg');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderBrandPart).toBe('portal-outline-header__brand-part');
    expect(PortalUnifiedChrome.Classes.OutlineHeaderBrandPartMuted).toBe('portal-outline-header__brand-part-muted');
    expect(PortalUnifiedChrome.Alt.DecorativeImage).toBe('');
    expect(PortalUnifiedChrome.Attributes.ShellHeaderExtension).toBe('data-oc-shell-header-extension');
    expect(PortalUnifiedChrome.Svg.FrameKeyOuter).toBe('outerFrame');
    expect(PortalUnifiedChrome.Svg.FrameColorCyan).toBe('#2fddff');
    expect(PortalUnifiedChrome.Svg.FrameLineVariant.Line).toBe('line');
    expect(PortalUnifiedChrome.Svg.HeaderConnectorViewBox).toBe('0 0 100 44');
    expect(PortalUnifiedChrome.Svg.PointerEventsNone).toBe('none');
    expect(PortalUnifiedChrome.Version.App).toBe('0.1.1');
  });

  it('PortalAssets: exposes auth assets and external links', () => {
    expect(PortalAssets.HeaderHomeIcon).toBe('/images/home.png');
    expect(PortalAssets.HeaderLoginIcon).toBe('/images/login.png');
    expect(PortalAssets.HeaderLogo).toBe('/ocentra-logo.svg');
    expect(PortalExternalLinks.Ocentra).toBe('https://ocentra.ca');
    expect(decodePortalClipboardText('copy payload')).toBe('copy payload');
  });
});
