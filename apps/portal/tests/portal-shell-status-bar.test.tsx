import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import {
  decodeAgentWebSocketUrl,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalConnectionState, PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { PortalShellStatusBar } from '../src/PortalShellStatusBar';
import { createPortalRuntimeState } from '../src/portal-state';

describe('portal shell status bar', () => {
  it('renders unauthenticated and no-household shell states from service-backed LAN events', () => {
    const unauthenticatedMarkup = renderShellStatusBar(
      PortalRoute.Devices,
      [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAuthenticationState]: 'unauthenticated',
        }),
      ]
    );
    expect(unauthenticatedMarkup).toContain('unauthenticated');
    expect(unauthenticatedMarkup).toContain('permission-missing');

    const noHouseholdMarkup = renderShellStatusBar(
      PortalRoute.Devices,
      [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAuthenticationState]: 'paired',
          [AgentProtocolDefaults.Field.LanParentAuthority]: 'active-controller',
          [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 0,
          [AgentProtocolDefaults.Field.LanPendingPairingCount]: 0,
        }),
      ]
    );
    expect(noHouseholdMarkup).toContain('no household configured');
    expect(noHouseholdMarkup).toContain('no selected child device');
  });

  it('replaces the previously selected device instead of leaking stale shell text', () => {
    const firstMarkup = renderShellStatusBar(
      PortalRoute.Devices,
      [selectedDeviceEvent('child-android-device-001', 'stale')]
    );
    expect(firstMarkup).toContain('chil...-001');
    expect(firstMarkup).toContain('stale');

    const secondMarkup = renderShellStatusBar(
      PortalRoute.Devices,
      [selectedDeviceEvent('child-ipad-device-002', 'online')]
    );
    expect(secondMarkup).toContain('chil...-002');
    expect(secondMarkup).toContain('online');
    expect(secondMarkup).not.toContain('chil...-001');
  });

  it('drops stale LAN-selected-device text when the route switches to a local protocol surface', () => {
    const devicesMarkup = renderShellStatusBar(
      PortalRoute.Devices,
      [selectedDeviceEvent('child-android-device-001', 'stale')]
    );
    expect(devicesMarkup).toContain('chil...-001');

    const commandsMarkup = renderShellStatusBar(PortalRoute.Commands, []);
    expect(commandsMarkup).not.toContain('chil...-001');
    expect(commandsMarkup).toContain('live local');
    expect(commandsMarkup).toContain('proof-missing');
  });

  it('keeps selected-device shell context visible on product routes that still act on child devices', () => {
    const browserSettingsMarkup = renderShellStatusBar(
      PortalRoute.BrowserSettings,
      [selectedDeviceEvent('child-android-device-001', 'online')]
    );
    expect(browserSettingsMarkup).toContain('chil...-001');
    expect(browserSettingsMarkup).toContain('reachability: online');
    expect(browserSettingsMarkup).not.toContain('no selected child device');

    const ruleManagementMarkup = renderShellStatusBar(
      PortalRoute.RuleManagement,
      [selectedDeviceEvent('child-android-device-001', 'online')]
    );
    expect(ruleManagementMarkup).toContain('chil...-001');
    expect(ruleManagementMarkup).toContain('reachability: online');
    expect(ruleManagementMarkup).not.toContain('no selected child device');
  });
});

function renderShellStatusBar(route: PortalRoute, events: readonly AgentEventEnvelope[]): string {
  const baseState = createPortalRuntimeState(decodeAgentWebSocketUrl('ws://127.0.0.1:4489/api/dev/ws'));
  const state = {
    ...baseState,
    connectionState: PortalConnectionState.Connected,
    events: [...events],
  };
  return renderToStaticMarkup(createElement(PortalShellStatusBar, { route, state }));
}

function selectedDeviceEvent(selectedChildDeviceId: string, reachability: string): AgentEventEnvelope {
  return payloadEvent(AgentEvent.LanPairingStatusReported, {
    [AgentProtocolDefaults.Field.LanAuthenticationState]: 'paired',
    [AgentProtocolDefaults.Field.LanParentAuthority]: 'active-controller',
    [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 1,
    [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: selectedChildDeviceId,
    [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: reachability,
  });
}

function payloadEvent(
  event: AgentEventName,
  payload: AgentProtocolLogFields,
  sentAt = '2026-06-18T05:30:00Z'
): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${event}-${sentAt}`,
    correlationId: `cmd-${event}`,
    sentAt,
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: AgentProtocolDefaults.Peer.PortalDev,
    event,
    severity: 'info',
    payload,
    snapshot: null,
  });
}
