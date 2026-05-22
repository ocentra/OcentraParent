import { describe, expect, it } from 'vitest';
import {
  PortalClipboard,
  PortalCommandButtons,
  PortalConnectionState,
  PortalDetails,
  PortalDiagnostics,
  PortalDom,
  PortalOverviewCommands,
  PortalRouteSchema,
  PortalRoutes,
  PortalTiming,
  decodePortalClipboardText,
} from '../src/contracts';

describe('portal domain contracts', () => {
  it('PortalRouteSchema: accepts only declared dev routes', () => {
    expect(PortalRouteSchema.safeParse('commands').success).toBe(true);
    expect(PortalRouteSchema.safeParse('settings').success).toBe(false);
  });

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
      'agent.network.flow.read-model.get',
      'agent.local-ai.runtime.status.get',
      'agent.policy.preview.read-model.get',
    ]);
  });

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
    expect(PortalDetails.ParentRuleContextReferences).toBe('Parent rule context references');
    expect(PortalDetails.ParentRuleContextRefIds).toBe('Parent rule context ref IDs');
    expect(PortalDom.Tags.TextArea).toBe('textarea');
    expect(decodePortalClipboardText('copy payload')).toBe('copy payload');
  });
});
