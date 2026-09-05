import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { ParentRoute } from '../../generated/parent-ui-bridge';
import { ScreenSettingsRoutePanel, shouldRenderScreenSettingsRoute } from '../../src/ScreenSettingsRoutePanel';
import type { PortalRenderActions } from '../../src/portal-actions';

const actions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

describe('screen settings route panel', () => {
  it('belongs only to the Screen policy route', () => {
    expect([
      shouldRenderScreenSettingsRoute(ParentRoute.PolicyScreen),
      shouldRenderScreenSettingsRoute(ParentRoute.SettingsRules),
      shouldRenderScreenSettingsRoute(ParentRoute.ScreenAnalysis),
      shouldRenderScreenSettingsRoute(ParentRoute.Overview),
    ]).toEqual([true, false, false, false]);
  });

  it('renders real parent-owned screen setting intents without inventing a service response', () => {
    const markup = renderToStaticMarkup(
      createElement(ScreenSettingsRoutePanel, {
        actions,
        commandEnabled: false,
        serviceResponseSnapshot: null,
      })
    );

    expect(markup).toContain('aria-label="Screen analysis settings"');
    expect(markup).toContain('data-ocentra-screen-settings-connection="unavailable"');
    expect(markup).toContain('Keep screen analysis disabled');
    expect(markup).toContain('Enable observe-only summaries');
    expect(markup.match(/<button[^>]*disabled=""[^>]*>/gu)).toHaveLength(4);
    expect(markup).toContain('service command unavailable while disconnected');
    expect(markup).toContain('Retry status');
    expect(markup).toContain('Raw image retained');
    expect(markup).toContain('Raw image retention');
    expect(markup).not.toContain('Save selected screen setting');
    expect(markup).not.toContain('Refresh persisted screen setting');
    expect(markup).not.toContain('<dt>REQUEST ID</dt>');
    expect(markup).not.toContain('<dt>VERSION</dt>');
    expect(markup).not.toContain('<dt>EVENT ID</dt>');
    expect(markup).not.toContain('<dt>REASON</dt>');
    expect(markup).not.toContain('service accepted persisted setting');
    expect(markup).not.toContain('screen-optional-visibility-capability-status-proof');
    expect(markup).not.toContain('Catalog settings');
    expect(markup).not.toContain('docs/screen-evidence-analysis');
  });

  it('enables parent setting choices only when the service command boundary is connected', () => {
    const markup = renderToStaticMarkup(
      createElement(ScreenSettingsRoutePanel, {
        actions,
        commandEnabled: true,
        serviceResponseSnapshot: null,
      })
    );

    expect(markup.match(/<button[^>]*aria-pressed=/gu)).toHaveLength(4);
    expect(markup).not.toContain('disabled=""');
    expect(markup).toContain('Save selected screen setting');
    expect(markup).toContain('Refresh persisted screen setting');
  });
});
