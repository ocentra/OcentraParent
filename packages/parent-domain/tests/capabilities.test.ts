import { describe, expect, it } from 'vitest';
import {
  type ParentControlCapability,
  type ParentControlCapabilityName as ParentControlCapabilityNameType,
  type ParentControlPlatform,
  ParentControlCapabilityName,
  ParentControlCapabilityStatus,
  ParentControlPlatformCapabilities,
  ParentControlPlatformCapabilitySchema,
} from '../src/capabilities';

describe('parent control platform capabilities', () => {
  it('ParentControlPlatformCapabilities: keeps platform claims explicit and schema-valid', () => {
    const parsed = ParentControlPlatformCapabilities.map((entry) => ParentControlPlatformCapabilitySchema.parse(entry));

    expect(parsed.map((entry) => entry.platform)).toEqual(['windows', 'linux', 'macos', 'android', 'ios']);
  });

  it('ParentControlPlatformCapabilities: does not claim mobile policy or store distribution support yet', () => {
    const mobileCapabilities = ParentControlPlatformCapabilities.filter((entry) =>
      ['android', 'ios'].includes(entry.platform)
    ).flatMap((entry) => entry.capabilities);

    expect(capabilityStatus(mobileCapabilities, ParentControlCapabilityName.DeviceOwnerPolicy)).toEqual(
      ParentControlCapabilityStatus.ManualRequired
    );
    expect(capabilityStatus(mobileCapabilities, ParentControlCapabilityName.FamilyControlsEntitlement)).toEqual(
      ParentControlCapabilityStatus.ManualRequired
    );
    expect(capabilityStatus(mobileCapabilities, ParentControlCapabilityName.StoreDistribution)).toEqual(
      ParentControlCapabilityStatus.Planned
    );
  });

  it('ParentControlPlatformCapabilities: splits Android child capability proof states', () => {
    const androidCapabilities = capabilitiesForPlatform('android');

    expectCapabilityStatuses(androidCapabilities, [
      [ParentControlCapabilityName.ForegroundMobileService, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.LocalStorage, ParentControlCapabilityStatus.Scaffold],
      [ParentControlCapabilityName.TypedProtocolBridge, ParentControlCapabilityStatus.Scaffold],
      [ParentControlCapabilityName.UsageStats, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.AccessibilityService, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.VpnDnsFiltering, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.ManagedProfile, ParentControlCapabilityStatus.ManualRequired],
    ]);
  });

  it('ParentControlPlatformCapabilities: splits iOS entitlement and device proof states', () => {
    const iosCapabilities = capabilitiesForPlatform('ios');

    expectCapabilityStatuses(iosCapabilities, [
      [ParentControlCapabilityName.ForegroundMobileService, ParentControlCapabilityStatus.Unavailable],
      [ParentControlCapabilityName.DeviceActivity, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.ScreenTimeApi, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.NetworkExtension, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.Notifications, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.BackgroundExecution, ParentControlCapabilityStatus.ManualRequired],
      [ParentControlCapabilityName.TestflightDistribution, ParentControlCapabilityStatus.ManualRequired],
    ]);
  });
});

function capabilitiesForPlatform(platform: ParentControlPlatform) {
  return ParentControlPlatformCapabilities.find((entry) => entry.platform === platform)?.capabilities ?? [];
}

function capabilityStatus(
  capabilities: ReadonlyArray<ParentControlCapability>,
  capabilityName: ParentControlCapabilityNameType
) {
  return capabilities.find((capability) => capability.capability === capabilityName)?.status;
}

function expectCapabilityStatuses(
  capabilities: ReadonlyArray<ParentControlCapability>,
  expected: ReadonlyArray<
    readonly [
      ParentControlCapabilityNameType,
      (typeof ParentControlCapabilityStatus)[keyof typeof ParentControlCapabilityStatus],
    ]
  >
) {
  for (const [capabilityName, status] of expected) {
    expect(capabilityStatus(capabilities, capabilityName)).toEqual(status);
  }
}
