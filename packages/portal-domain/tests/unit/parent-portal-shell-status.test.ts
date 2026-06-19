import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  type AgentEventEnvelope,
  type AgentEventName,
  type AgentProtocolLogFields,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolveParentPortalShellStatus } from '../../src/parent-portal-shell-status';
import { PARENT_PORTAL_SERVICE_STATE } from '../../src/parent-portal-service-state';
import { PortalRoute } from '../../src/routes';

describe('parent portal shell status', () => {
  it('keeps protocol routes live-local without inventing household proof', () => {
    const status = resolveParentPortalShellStatus({
      route: PortalRoute.Commands,
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [],
    });

    expect(status.dataSourceLabel).toBe('live local');
    expect(status.routeCapabilityState).toBe('available');
    expect(status.globalConnectionState).toBe('online');
    expect(status.cards.find((card) => card.id === 'parent-access')?.value).toBe('proof-missing');
  });

  it('reports browser route capability from real browser inventory service events', () => {
    const status = resolveParentPortalShellStatus({
      route: PortalRoute.Browser,
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.BrowserInventoryReadModelReported, {
          [AgentProtocolDefaults.Field.Returned]: 1,
          [AgentProtocolDefaults.Field.CapabilityStatus]: 'managed-target-list',
          browserInventoryRowId: 'browser-inventory-row-1',
          runningState: 'running-managed',
          exactUrlCapability: 'managed-target-list-only',
          activeTabCapability: 'target-list-only',
          unmanagedFallbackCapability: 'report-only',
        }),
      ],
    });

    expect(status.dataSourceLabel).toBe('live local');
    expect(status.routeCapabilityState).toBe('available');
    expect(status.globalConnectionState).toBe('online');
  });

  it('shows no-household and no-selected-device when LAN proof exists without household assignment', () => {
    const status = resolveParentPortalShellStatus({
      route: PortalRoute.Devices,
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAuthenticationState]: 'paired',
          [AgentProtocolDefaults.Field.LanParentAuthority]: 'active-controller',
          [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 0,
          [AgentProtocolDefaults.Field.LanPendingPairingCount]: 0,
        }),
      ],
    });

    expect(status.parentAccessState).toBe('active-controller');
    expect(status.dataSourceLabel).toBe('LAN');
    expect(status.globalConnectionState).toBe('manual-required');
    expect(status.cards.find((card) => card.id === 'household')?.value).toBe('no household configured');
    expect(status.cards.find((card) => card.id === 'child-device')?.value).toBe('no selected child device');
  });

  it('renders unauthenticated state when LAN authentication is explicitly unauthenticated', () => {
    const status = resolveParentPortalShellStatus({
      route: PortalRoute.Devices,
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAuthenticationState]: 'unauthenticated',
        }),
      ],
    });

    expect(status.parentAccessState).toBe('unauthenticated');
    expect(status.routeCapabilityState).toBe('permission-missing');
    expect(status.globalConnectionState).toBe('unauthenticated');
  });

  it('keeps stale selected-device readiness visible with redacted device ids', () => {
    const status = resolveParentPortalShellStatus({
      route: PortalRoute.Devices,
      connectionState: PARENT_PORTAL_SERVICE_STATE.Connection.Connected,
      events: [
        payloadEvent(AgentEvent.LanPairingStatusReported, {
          [AgentProtocolDefaults.Field.LanAuthenticationState]: 'paired',
          [AgentProtocolDefaults.Field.LanParentAuthority]: 'active-controller',
          [AgentProtocolDefaults.Field.LanTrustedDeviceCount]: 1,
          [AgentProtocolDefaults.Field.LanSelectedChildDeviceId]: 'child-android-device-001',
          [AgentProtocolDefaults.Field.LanSelectedDeviceReachability]: 'stale',
        }),
      ],
    });

    expect(status.dataSourceLabel).toBe('LAN');
    expect(status.globalConnectionState).toBe('stale');
    expect(status.cards.find((card) => card.id === 'child-device')?.value).toBe('chil...-001');
  });
});

function payloadEvent(
  event: AgentEventName,
  payload: AgentProtocolLogFields,
  sentAt = '2026-06-18T04:00:00Z'
): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: 1,
    eventId: `evt-${event}`,
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
