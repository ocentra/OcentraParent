import { describe, expect, it } from 'vitest';
import {
  PortalClipboard,
  PortalCommandButtons,
  PortalConnectionState,
  PortalAuthChrome,
  PortalDetails,
  PortalDiagnostics,
  PortalDom,
  PortalAssets,
  PortalExternalLinks,
  PortalOverviewCommands,
  PortalRouteDescriptors,
  PortalRouteGroup,
  PortalRouteSchema,
  PortalRoutes,
  PortalTiming,
  PortalUnifiedChrome,
  decodePortalClipboardText,
} from '../src/contracts';

describe('portal route contracts', () => {
  it('PortalRouteSchema: accepts only declared dev routes', () => {
    expect(PortalRoutes).toEqual([
      'overview',
      'activity',
      'browser',
      'policy',
      'privacy-design',
      'memory',
      'ai-runtime',
      'devices',
      'notifications',
      'drive-connections',
      'diagnostics',
      'settings-rules',
      'commands',
      'events',
    ]);
    expect(PortalRouteSchema.safeParse('commands').success).toBe(true);
    expect(PortalRouteSchema.safeParse('settings-rules').success).toBe(true);
    expect(PortalRouteSchema.safeParse('billing').success).toBe(false);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.group)).toContain(PortalRouteGroup.Monitor);
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Activity');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Private by design');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Notifications');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Connect your drives');
    expect(PortalRouteDescriptors.map((descriptor) => descriptor.label)).toContain('Settings');
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
  it('PortalConnectionState: exposes connected state token', () => {
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
    expect(PortalDom.Tags.TextArea).toBe('textarea');
    expect(PortalUnifiedChrome.Tags.Footer).toBe('footer');
    expect(PortalUnifiedChrome.Classes.Header).toBe('ocentra-game-header');
    expect(PortalAuthChrome.Classes.Dialog).toBe('portal-auth-dialog');
    expect(PortalAuthChrome.Assets.Google).toBe('/ocentra-game-assets/auth/google.png');
    expect(PortalAuthChrome.Modes.SignIn).toBe('signin');
    expect(PortalAssets.HeaderLogo).toBe('/ocentra-game-assets/commons/OcentraLogo.svg');
    expect(PortalExternalLinks.Ocentra).toBe('https://ocentra.ca');
    expect(decodePortalClipboardText('copy payload')).toBe('copy payload');
  });
});
