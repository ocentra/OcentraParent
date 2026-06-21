import { describe, expect, it } from 'vitest';
import { AgentProtocolDefaults } from '../../src/agent-protocol-defaults';
import { AgentProtocolSchemaVersion } from '../../src/event-primitives';

describe('agent protocol defaults', () => {
  it('exposes the core protocol transport defaults', () => {
    expect(AgentProtocolDefaults.SchemaVersion).toBe(AgentProtocolSchemaVersion);
    expect(AgentProtocolDefaults.WebSocketUrl).toBe('ws://127.0.0.1:4477/api/dev/ws');
    expect(AgentProtocolDefaults.MessageIdPrefix).toBe('cmd-');
    expect(AgentProtocolDefaults.Peer.PortalDev.role).toBe('portal');
    expect(AgentProtocolDefaults.Target.LocalNetworkWindowsAgent.route).toBe(
      'local-network'
    );
  });

  it('preserves shared LAN, policy, and network value catalogs', () => {
    expect(AgentProtocolDefaults.PairingState.Paired).toBe('paired');
    expect(AgentProtocolDefaults.LanHouseholdActionKind.Trust).toBe('trust');
    expect(AgentProtocolDefaults.RouteSecurity.LocalNetwork.requiresPairing).toBe(
      true
    );
    expect(AgentProtocolDefaults.Field.ParentAssistantThreadId).toBe(
      'assistantThreadId'
    );
    expect(AgentProtocolDefaults.PolicyPreview.Action.ManualReview).toBe(
      'manual-review'
    );
    expect(
      AgentProtocolDefaults.NetworkRemoteDeliveryStatus.CrossProcessReplayCursorRef
    ).toBe('network.remote-delivery.cross-process-replay-cursor.10r');
    expect(AgentProtocolDefaults.NetworkLinuxNftablesLabStatus.TableName).toBe(
      'ocentra_parent_lab_row42a'
    );
  });
});
