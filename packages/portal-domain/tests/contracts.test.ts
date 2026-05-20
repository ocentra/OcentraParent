import { describe, expect, it } from 'vitest';
import {
  PortalClipboard,
  PortalCommandButtons,
  PortalConnectionState,
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
    expect(PortalOverviewCommands.map((button) => button.command)).toEqual([
      'agent.health.check',
      'agent.log.snapshot.get',
      'agent.activity.ingest.status.get',
      'agent.activity.recent.summary.get',
    ]);
  });

  it('PortalConnectionState: exposes connected state token', () => {
    expect(PortalRoutes).toContain('overview');
    expect(PortalConnectionState.Connected).toBe('connected');
    expect(PortalTiming.CopyFeedbackMs).toBeGreaterThan(0);
    expect(PortalDiagnostics.SchemaVersion).toBe(1);
    expect(PortalDiagnostics.Field.AgentUrl).toBe('agentUrl');
    expect(PortalClipboard.CommandCopy).toBe('copy');
    expect(PortalDom.Tags.TextArea).toBe('textarea');
    expect(decodePortalClipboardText('copy payload')).toBe('copy payload');
  });
});
