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
  PortalOverviewCommands,
  PortalRouteDescriptors,
  PortalRouteGroup,
  PortalRoute,
  PortalRouteSchema,
  PortalRoutes,
  PortalSidebarRouteDescriptors,
  PortalTiming,
  PortalUnifiedChrome,
  decodePortalClipboardText,
  parentPortalRouteContext,
  type ParentPortalHashRoutePath,
  type ParentPortalNavSectionLabel,
} from '../src/contracts';

function routeFromHashPath(routePath: ParentPortalHashRoutePath): PortalRoute {
  return PortalRouteSchema.parse(routePath.slice(2));
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
  for (const topic of PARENT_PORTAL_GUIDE_TOPICS) {
    for (const note of [...topic.tips, ...topic.actions]) {
      if (!note.targetRoutePath) continue;
      const navMatches = PARENT_PORTAL_CONTENT.navItems.filter((item) => item.routePath === note.targetRoutePath);
      if (navMatches.length === 0) {
        const route = routeFromHashPath(note.targetRoutePath);
        const routeContext = PARENT_PORTAL_ROUTE_CONTEXT[route];
        expect(selectableTargetIds.has(routeContext?.selectedControlId ?? '')).toBe(true);
        continue;
      }
      expect(navMatches).toHaveLength(1);
      if (note.targetNavLabel) {
        expect(note.targetNavLabel).toBe(navMatches[0]?.label);
      }
    }
  }
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
      'report-settings',
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
    ]);
    expect(PortalRouteSchema.safeParse('commands').success).toBe(true);
    expect(PortalRouteSchema.safeParse('settings-rules').success).toBe(true);
    expect(PortalRouteSchema.safeParse('frame-tuner').success).toBe(true);
    expect(PortalRouteSchema.safeParse('billing').success).toBe(false);
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
    const manageRules = manageItems.find((item) => item.routePath === '#/rule-management');
    const manageSupport = manageItems.find((item) => item.routePath === '#/diagnostics');

    expect(guideRules?.label).toBe(PARENT_PORTAL_NAV_LABELS.RulesGuide);
    expect(manageRules?.label).toBe(PARENT_PORTAL_NAV_LABELS.RuleSet);
    expect(manageSupport?.label).toBe(PARENT_PORTAL_NAV_LABELS.Support);
    expect(manageSupport?.sectionLabel).toBe(PARENT_PORTAL_NAV_LABELS.Account);
    expect(guideAi?.routePath).toBe('#/ai-guide');
    expect(guideReports?.routePath).toBe('#/reports-guide');
    expect(parentPortalRouteContext(PortalRoute.Policy).pageMode).toBe('parentGuide');
    expect(parentPortalRouteContext(PortalRoute.RuleManagement).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.AiGuide).pageMode).toBe('parentGuide');
    expect(parentPortalRouteContext(PortalRoute.ReportsGuide).pageMode).toBe('parentGuide');
    expect(parentPortalRouteContext(PortalRoute.AiRuntime).pageMode).toBe('parentManage');
    expect(parentPortalRouteContext(PortalRoute.ReportSettings).pageMode).toBe('parentManage');
  });
});

describe('portal collapsed manage route contracts', () => {
  it('legacy device and activity routes resolve to the unified manage pages', () => {
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
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.PlatformsInstall)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.InstallUpdates)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Devices,
      selectedControlId: 'lan-pairing',
    });
    expect(parentPortalRouteContext(PortalRoute.ReportSettings)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.ScreenAnalysis)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.AppGameSessions)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.NetworkActivity)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
    expect(parentPortalRouteContext(PortalRoute.ReportCompiler)).toMatchObject({
      navLabel: PARENT_PORTAL_NAV_LABELS.Activity,
      selectedControlId: 'reports-settings',
    });
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
    const expectedManageRoutesBySection = new Map<ParentPortalNavSectionLabel, readonly ParentPortalHashRoutePath[]>([
      [PARENT_PORTAL_NAV_LABELS.Portal, ['#/settings-rules', '#/notifications', '#/notification-channels']],
      [
        PARENT_PORTAL_NAV_LABELS.Policies,
        ['#/browser-settings', '#/rule-management', '#/schedules', '#/approvals', '#/enforcement'],
      ],
      [PARENT_PORTAL_NAV_LABELS.DataPrivacy, ['#/drive-connections', '#/export-retention', '#/audit-history']],
      [PARENT_PORTAL_NAV_LABELS.AiMemory, ['#/ai-runtime', '#/api-providers', '#/memory-settings']],
      [PARENT_PORTAL_NAV_LABELS.Account, ['#/subscription', '#/diagnostics', '#/entitlements']],
    ]);

    for (const [sectionLabel, routePaths] of expectedManageRoutesBySection) {
      expect(
        PARENT_PORTAL_CONTENT.navItems
          .filter((item) => item.groupId === 'manage' && item.sectionLabel === sectionLabel)
          .map((item) => item.routePath)
      ).toEqual(routePaths);
    }

    expect(
      PARENT_PORTAL_CONTENT.navItems
        .filter((item) => item.groupId === 'manage' && item.label === PARENT_PORTAL_NAV_LABELS.Devices)
        .map((item) => ({
          icon: item.icon,
          routePath: item.routePath,
          sectionLabel: item.sectionLabel,
        }))
    ).toEqual([{ icon: 'lan', routePath: '#/lan-pairing', sectionLabel: undefined }]);
    expect(
      PARENT_PORTAL_CONTENT.navItems
        .filter((item) => item.groupId === 'manage' && item.sectionLabel === PARENT_PORTAL_NAV_LABELS.Devices)
        .map((item) => item.routePath)
    ).toEqual([]);
    expect(
      PARENT_PORTAL_CONTENT.navItems
        .filter((item) => item.groupId === 'manage' && item.label === PARENT_PORTAL_NAV_LABELS.Activity)
        .map((item) => ({
          icon: item.icon,
          routePath: item.routePath,
          sectionLabel: item.sectionLabel,
        }))
    ).toEqual([{ icon: 'activity', routePath: '#/activity', sectionLabel: undefined }]);
    expect(
      PARENT_PORTAL_CONTENT.navItems
        .filter((item) => item.groupId === 'manage' && item.sectionLabel === PARENT_PORTAL_NAV_LABELS.Activity)
        .map((item) => item.routePath)
    ).toEqual([]);
  });
});

describe('portal parent assistant contracts', () => {
  it('parent assistant quick actions: expose the typed assistant categories used by the chat side panel', () => {
    expect(PARENT_ASSISTANT_PORTAL_NEW_CHAT_ACTION?.quickActionId).toBe('new-chat');
    expect(PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS.map((action) => action.quickActionId)).toEqual([
      'report',
      'browser-state',
      'rules',
      'ai-setup',
      'drives',
      'support-api',
    ]);
    expect(PARENT_ASSISTANT_PORTAL_QUICK_ACTIONS[2]?.choices[1]?.nextActionKind).toBe('preview-rule-change');
  });
});

describe('portal command contracts', () => {
  it('PortalCommandButtons: maps each button to a typed command', () => {
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.health.check');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain('agent.health.reported');
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.activity.ingest.status.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain(
      'agent.activity.recent.summary.reported'
    );
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.browser.evidence.recent.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain(
      'agent.browser.evidence.recent.reported'
    );
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.activity.memory-graph.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain('agent.activity.memory-graph.reported');
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.browser.intervention.read-model.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain(
      'agent.browser.intervention.read-model.reported'
    );
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.browser.managed.bridge.poll');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain('agent.browser.managed.status.reported');
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.network.flow.read-model.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain(
      'agent.network.flow.read-model.reported'
    );
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.local-ai.runtime.status.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain(
      'agent.local-ai.runtime.status.reported'
    );
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.policy.preview.read-model.get');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain(
      'agent.policy.preview.read-model.reported'
    );
    expect(PortalOverviewCommands.map((button) => button.command)).toEqual([
      'agent.health.check',
      'agent.log.snapshot.get',
      'agent.activity.ingest.status.get',
      'agent.activity.recent.summary.get',
      'agent.browser.evidence.recent.get',
      'agent.activity.memory-graph.get',
      'agent.browser.intervention.read-model.get',
      'agent.network.flow.read-model.get',
      'agent.local-ai.runtime.status.get',
      'agent.policy.preview.read-model.get',
    ]);
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
    expect(PortalDetails.ParentRuleContextReferences).toBe('Parent rule context references');
    expect(PortalDetails.ParentRuleContextRefIds).toBe('Parent rule context ref IDs');
  });

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
    expect(PortalAssets.HeaderHomeIcon).toBe('/nav-overview.svg');
    expect(PortalAssets.HeaderLoginIcon).toBe('/header-login.svg');
    expect(PortalAssets.HeaderLogo).toBe('/ocentra-logo.svg');
    expect(PortalExternalLinks.Ocentra).toBe('https://ocentra.ca');
    expect(decodePortalClipboardText('copy payload')).toBe('copy payload');
  });
});
