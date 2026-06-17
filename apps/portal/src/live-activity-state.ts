import { AgentProtocolDefaults, type AgentEventEnvelope } from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  resolveLiveActivityState as resolvePortalDomainLiveActivityState,
  type PortalBrowserRuntimeEventChainEntry,
  type PortalBrowserRuntimeEventChainStream,
  type PortalLiveActivityState,
  type PortalNetworkRuntimeEventChainStream,
} from '@ocentra-parent/portal-domain/live-activity-state';

type PortalLanAddDeviceReadModel = NonNullable<PortalLiveActivityState['lanAddDeviceReadModel']>;

export function resolveLiveActivityState(events: readonly AgentEventEnvelope[]): PortalLiveActivityState {
  const state = resolvePortalDomainLiveActivityState(events);
  if (state.lanAddDeviceReadModel !== null) {
    return state;
  }

  const lanAddDeviceReadModel = parseLanAddDeviceReadModel(
    state.lanPairingStatusEvent?.payload?.[AgentProtocolDefaults.Field.LanAddDeviceReadModel]
  );

  return lanAddDeviceReadModel === null
    ? state
    : {
        ...state,
        lanAddDeviceReadModel,
      };
}

function parseLanAddDeviceReadModel(value: unknown): PortalLanAddDeviceReadModel | null {
  const parsedValue = typeof value === 'string' ? safeJsonParse(value) : value;
  return isPortalLanAddDeviceReadModel(parsedValue) ? parsedValue : null;
}

function safeJsonParse(value: string): unknown {
  try {
    return JSON.parse(value);
  } catch {
    return null;
  }
}

function isPortalLanAddDeviceReadModel(value: unknown): value is PortalLanAddDeviceReadModel {
  if (!isRecord(value)) {
    return false;
  }

  return (
    typeof value.addDeviceState === 'string' &&
    Array.isArray(value.canonicalHouseholdDevices) &&
    Array.isArray(value.discoveredDevices) &&
    Array.isArray(value.honestNonClaims) &&
    isRecord(value.scanSummary) &&
    typeof value.scanSummary.scannedDeviceCount === 'number' &&
    typeof value.scanSummary.agentDeviceCount === 'number' &&
    isRecord(value.selectedDeviceReadiness) &&
    typeof value.selectedDeviceReadiness.reachability === 'string' &&
    typeof value.selectedDeviceReadiness.readyForControl === 'boolean'
  );
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

export {
  type PortalBrowserRuntimeEventChainEntry,
  type PortalBrowserRuntimeEventChainStream,
  type PortalLiveActivityState,
  type PortalNetworkRuntimeEventChainStream,
};
