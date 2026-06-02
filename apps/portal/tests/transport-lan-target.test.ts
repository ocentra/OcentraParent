import { expect, it } from 'vitest';
import { AgentCommand, AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
import { resolvePortalCommandTarget } from '../src/transport';

it('resolvePortalCommandTarget: routes LAN commands to the selected local-network child device', () => {
  const target = resolvePortalCommandTarget(
    AgentProtocolDefaults.Target.LocalhostWindowsAgent,
    AgentCommand.LanPairingAddDeviceRequest,
    {
      [AgentProtocolDefaults.Field.LanChildDeviceId]: 'child-device-1',
    }
  );

  expect(target).toMatchObject({
    deviceId: 'child-device-1',
    platform: 'windows',
    route: 'local-network',
  });
});

it('resolvePortalCommandTarget: keeps non-LAN commands on the existing target', () => {
  expect(
    resolvePortalCommandTarget(AgentProtocolDefaults.Target.LocalhostWindowsAgent, AgentCommand.HealthCheck, {})
  ).toBe(AgentProtocolDefaults.Target.LocalhostWindowsAgent);
});

it('resolvePortalCommandTarget: keeps status refreshes on the service target until a child is selected', () => {
  expect(
    resolvePortalCommandTarget(AgentProtocolDefaults.Target.LocalhostWindowsAgent, AgentCommand.LanPairingStatusGet, {})
  ).toStrictEqual(AgentProtocolDefaults.Target.LocalhostWindowsAgent);
});

it('resolvePortalCommandTarget: reads LAN status through the service route when the portal is on a LAN URL', () => {
  expect(
    resolvePortalCommandTarget(
      AgentProtocolDefaults.Target.LocalNetworkWindowsAgent,
      AgentCommand.LanPairingStatusGet,
      {}
    )
  ).toMatchObject({
    deviceId: 'local-dev-agent',
    platform: 'windows',
    route: 'localhost',
  });
});

it('resolvePortalCommandTarget: sends browser discovery scans over the local-network route', () => {
  const target = resolvePortalCommandTarget(
    AgentProtocolDefaults.Target.LocalhostWindowsAgent,
    AgentCommand.LanPairingBrowserDiscoveryScan,
    {}
  );

  expect(target).toMatchObject({
    deviceId: 'local-dev-agent',
    platform: 'windows',
    route: 'local-network',
  });
});
