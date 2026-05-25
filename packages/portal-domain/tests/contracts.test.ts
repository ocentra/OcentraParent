import { describe, expect, it } from 'vitest';
import {
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
  PortalRouteSchema,
  PortalRoutes,
  PortalSidebarRouteDescriptors,
  PortalTiming,
  PortalUnifiedChrome,
  decodePortalClipboardText,
} from '../src/contracts';

describe('portal route contracts', () => {
  it('PortalRouteSchema: accepts only declared dev routes', () => {
    expect(PortalRoutes).toEqual([
      'overview',
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
      'ai-runtime',
      'api-providers',
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
      'frame-tuner',
      'commands',
      'events',
    ]);
    expect(PortalRouteSchema.safeParse('commands').success).toBe(true);
    expect(PortalRouteSchema.safeParse('settings-rules').success).toBe(true);
    expect(PortalRouteSchema.safeParse('billing').success).toBe(false);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.group)).toContain(PortalRouteGroup.Monitor);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Start here');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Activity');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Private by design');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Notifications');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Connect your drives');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Settings');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Frame tuner');
    expect(PortalSidebarRouteDescriptors.map((descriptor) => descriptor.label)).not.toContain('Frame tuner');
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
