import { expect, it } from 'vitest';
import { AgentCommand, AgentProtocolDefaults } from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalCommandButtons, PortalOverviewCommands } from '@ocentra-parent/portal-domain/commands';
import { isPortalDirectEnforcementActionCommand, resolvePortalCommandTarget } from '../src/transport';

it('isPortalDirectEnforcementActionCommand: rejects portal-side enforcement action commands', () => {
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementExecute)).toBe(true);
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementTimerRecover)).toBe(true);
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementTimerExpire)).toBe(true);
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementOverrideCancel)).toBe(true);
});

it('isPortalDirectEnforcementActionCommand: allows portal read-model and proof commands', () => {
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementProductControlSpineGet)).toBe(false);
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementPolicyDispatchGet)).toBe(false);
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementBroadAdapterProofGet)).toBe(false);
  expect(isPortalDirectEnforcementActionCommand(AgentCommand.EnforcementSupportedAdapterRuntimeProofGet)).toBe(false);
});

it('portal command inventory: contains no direct enforcement action command', () => {
  const commandInventory = [
    ...PortalOverviewCommands.map((command) => command.command),
    ...PortalCommandButtons.map((command) => command.command),
  ];

  expect(commandInventory.filter(isPortalDirectEnforcementActionCommand)).toStrictEqual([]);
});

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

it('resolvePortalCommandTarget: sends canonical household decisions over the local-network route', () => {
  const target = resolvePortalCommandTarget(
    AgentProtocolDefaults.Target.LocalhostWindowsAgent,
    AgentCommand.LanPairingAddDeviceRequest,
    {
      [AgentProtocolDefaults.Field.LanCanonicalDeviceId]: 'lan-physical-mac-54271e97c331',
      [AgentProtocolDefaults.Field.LanHouseholdActionKind]: 'rename',
    }
  );

  expect(target).toMatchObject({
    deviceId: 'local-dev-agent',
    platform: 'windows',
    route: 'local-network',
  });
});
