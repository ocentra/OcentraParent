import { describe, expect, it } from 'vitest';
import { PortalCommandButtons, PortalConnectionState, PortalRouteSchema, PortalRoutes } from '../src/contracts';

describe('portal domain contracts', () => {
  it('PortalRouteSchema: accepts only declared dev routes', () => {
    expect(PortalRouteSchema.safeParse('commands').success).toBe(true);
    expect(PortalRouteSchema.safeParse('settings').success).toBe(false);
  });

  it('PortalCommandButtons: maps each button to a typed command', () => {
    expect(PortalCommandButtons.map((button) => button.command)).toContain('agent.health.check');
    expect(PortalCommandButtons.map((button) => button.resultEvent)).toContain('agent.health.reported');
  });

  it('PortalConnectionState: exposes connected state token', () => {
    expect(PortalRoutes).toContain('overview');
    expect(PortalConnectionState.Connected).toBe('connected');
  });
});
