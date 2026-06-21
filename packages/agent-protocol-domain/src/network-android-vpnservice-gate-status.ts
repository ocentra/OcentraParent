import {
  AgentNetworkAndroidVpnServiceGateStatusSchema,
  type AgentNetworkAndroidVpnServiceGateStatus,
} from '@ocentra-parent/schema-domain/agent-network-android-vpnservice-status';
import { AgentEvent, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';
import { AgentProtocolDefaults } from './defaults';

export type AgentNetworkAndroidVpnServiceGateStatusParseResult =
  | {
      readonly ok: true;
      readonly status: AgentNetworkAndroidVpnServiceGateStatus;
    }
  | {
      readonly ok: false;
      readonly reason:
        | 'wrong-event'
        | 'missing-android-vpn-service-gate-status'
        | 'invalid-android-vpn-service-gate-status-json'
        | 'invalid-android-vpn-service-gate-status';
    };

export function parseAgentNetworkAndroidVpnServiceGateStatusEvent(
  event: AgentEventEnvelope
): AgentNetworkAndroidVpnServiceGateStatusParseResult {
  if (event.event !== AgentEvent.NetworkAndroidVpnServiceGateStatusReported) {
    return { ok: false, reason: 'wrong-event' };
  }

  const raw = event.payload[AgentProtocolDefaults.Field.NetworkAndroidVpnServiceGateStatus];
  if (!isAgentProtocolLogText(raw)) {
    return { ok: false, reason: 'missing-android-vpn-service-gate-status' };
  }

  let value: unknown;
  try {
    value = JSON.parse(raw) as unknown;
  } catch {
    return { ok: false, reason: 'invalid-android-vpn-service-gate-status-json' };
  }

  const parsed = AgentNetworkAndroidVpnServiceGateStatusSchema.safeParse(value);
  if (!parsed.success) {
    return { ok: false, reason: 'invalid-android-vpn-service-gate-status' };
  }

  return { ok: true, status: parsed.data };
}
