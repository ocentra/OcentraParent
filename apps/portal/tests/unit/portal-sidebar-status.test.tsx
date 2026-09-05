import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { decodeDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { ParentRoute, ParentRouteGroup } from '../../generated/parent-ui-bridge';
import { portalRouteUsesProductShell } from '../../src/portal-app-behavior';
import { portalRouteDescriptorsForGroup, SidebarStatus } from '../../src/PortalSidebar';
import type { PortalRenderActions } from '../../src/portal-actions';
import { createPortalRuntimeState } from '../../src/portal-state';

const actions: PortalRenderActions = {
  reconnect() {},
  selectCommandResult() {},
  async sendCommand() {
    return null;
  },
};

describe('portal sidebar status', () => {
  it('shows one retry action and exposes the latest host result as live feedback', () => {
    const state = createPortalRuntimeState();
    state.lastHostMessage = decodeDisplayText('The local service owner is unavailable.');

    const markup = renderToStaticMarkup(createElement(SidebarStatus, { actions, state }));

    expect(markup).toContain('role="status">Not connected</div>');
    expect(markup).toContain('>Retry status</button>');
    expect(markup).toContain('aria-live="polite"');
    expect(markup).toContain('The local service owner is unavailable.');
    expect(markup.match(/<button/g)).toHaveLength(1);
    expect(markup).not.toContain('>Reconnect<');
  });

  it('routes developer pages through the protocol shell and populates every sidebar group', () => {
    expect(portalRouteUsesProductShell(ParentRoute.Overview)).toBe(true);
    expect(portalRouteUsesProductShell(ParentRoute.Commands)).toBe(false);
    expect(portalRouteUsesProductShell(ParentRoute.Events)).toBe(false);
    expect(portalRouteUsesProductShell(ParentRoute.Logs)).toBe(false);
    expect(portalRouteUsesProductShell(ParentRoute.AppLayout)).toBe(false);
    expect(portalRouteUsesProductShell(ParentRoute.FrameTuner)).toBe(false);

    expect(portalRouteDescriptorsForGroup(ParentRouteGroup.Monitor).map(({ route }) => route)).toContain(
      ParentRoute.Overview
    );
    expect(portalRouteDescriptorsForGroup(ParentRouteGroup.Guide).map(({ route }) => route)).toContain(
      ParentRoute.Policy
    );
    expect(portalRouteDescriptorsForGroup(ParentRouteGroup.Operate).map(({ route }) => route)).toContain(
      ParentRoute.Devices
    );
    expect(portalRouteDescriptorsForGroup(ParentRouteGroup.DevTools).map(({ route }) => route)).toEqual([
      ParentRoute.ProofPanels,
      ParentRoute.Commands,
      ParentRoute.Events,
      ParentRoute.Logs,
    ]);
  });
});
