import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { SetupFirstRunRoutePanel, shouldRenderSetupFirstRunRoute } from '../src/SetupFirstRunRoutePanel';

describe('setup first-run portal route panel', () => {
  it('attaches only to the start route', () => {
    expect(shouldRenderSetupFirstRunRoute(PortalRoute.Start)).toBe(true);
    expect(shouldRenderSetupFirstRunRoute(PortalRoute.Overview)).toBe(false);
    expect(shouldRenderSetupFirstRunRoute(PortalRoute.Devices)).toBe(false);
  });

  it('renders the first-run setup state machine, non-ready states, and owner handoffs', () => {
    const markup = renderToStaticMarkup(createElement(SetupFirstRunRoutePanel));

    expect(markup).toContain('First-run family setup');
    expect(markup).toContain('State machine summary');
    expect(markup).toContain('welcome-screen');
    expect(markup).toContain('data-custody-status-screen');
    expect(markup).toContain('manual-required-screen');
    expect(markup).toContain('setup-blocked-screen');
    expect(markup).toContain('setup-complete-screen');
    expect(markup).toContain('physical-household-lan');
    expect(markup).toContain('parent-owned-storage');
    expect(markup).toContain('productionReady');
    expect(markup).toContain('account-identity-family-plan');
    expect(markup).toContain('parent-desktop-runtime-package-plan');
  });
});
