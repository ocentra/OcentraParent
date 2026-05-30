import { describe, expect, it } from 'vitest';
import {
  V08OsAdapterManualArtifactGateEntrySchema,
  V08OsAdapterManualArtifactGateReadModel,
  V08OsAdapterManualArtifactGateReadModelSchema,
  V08OsAdapterManualArtifactGateSurface,
} from '../src/v0-8-os-adapter-manual-artifact-gates';

describe('V0.8 OS adapter manual artifact gates', () => {
  capturesManualArtifactGatesAcrossPlatforms();
  keepsWindowsHostGatesManual();
  recordsAndroidPrivilegedCapabilityGates();
  recordsIosEntitlementDistributionGates();
  rejectsClaimUpgrades();
});

function capturesManualArtifactGatesAcrossPlatforms() {
  it('captures manual artifact gates across desktop and mobile platforms without product-ready claims', () => {
    const readModel = V08OsAdapterManualArtifactGateReadModelSchema.parse(V08OsAdapterManualArtifactGateReadModel);
    const platformCounts = countBy(readModel.entries.map((entry) => entry.platform));
    const outcomeCounts = countBy(readModel.entries.map((entry) => entry.gateOutcome));

    expect(readModel.entries).toHaveLength(25);
    expect(platformCounts).toEqual({
      windows: 11,
      linux: 1,
      macos: 1,
      android: 6,
      ios: 6,
    });
    expect(outcomeCounts).toEqual({
      'manual-required': 23,
      'not-claimed': 1,
      unavailable: 1,
    });
    expect(new Set(readModel.entries.map((entry) => entry.gateEntryId)).size).toBe(readModel.entries.length);
    expect(readModel.entries.every((entry) => entry.requiredArtifacts.length > 0)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.productReadyBlockingClaimed)).toBe(true);
  });
}

function keepsWindowsHostGatesManual() {
  it('keeps Windows app, network, browser, restart, audit, permission, and package gates manual', () => {
    const broadApp = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsBroadInstalledAppIdentity);
    const network = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsNetworkDomainFilterApplyRollback);
    const managedUrl = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsManagedBrowserExactUrl);
    const unmanagedExact = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsUnmanagedExactTitlePageDownload);
    const restart = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsRestartRecovery);
    const audit = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsAuditCustody);
    const service = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsServicePermission);
    const lifecycle = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsPackageLifecycle);

    expect(broadApp.requiredArtifacts).toContain('OS-approved installed app identity');
    expect(network.requiredArtifacts).toContain('network filter apply result');
    expect(managedUrl.requiredArtifacts).toContain('managed active-tab evidence');
    expect(unmanagedExact).toMatchObject({
      capabilityStatus: 'not-implemented',
      gateOutcome: 'not-claimed',
      unmanagedBrowserExactEvidenceClaimed: false,
    });
    expect(restart.requiredArtifacts).toContain('post-restart recovered state');
    expect(audit.requiredArtifacts).toContain('artifact hash or path');
    expect(service.requiredArtifacts).toContain('operator consent evidence');
    expect(lifecycle.requiredArtifacts).toContain('rollback or uninstall evidence');
  });
}

function recordsAndroidPrivilegedCapabilityGates() {
  it('records Android privileged capability gates separately', () => {
    const usageStats = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidUsageStats);
    const accessibility = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidAccessibilityService);
    const vpnDns = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidVpnDns);
    const deviceOwner = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidDeviceOwner);
    const managedProfile = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidManagedProfile);
    const lifecycle = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidPackageLifecycle);

    expect(usageStats.requiredArtifacts).toContain('UsageStats permission grant');
    expect(accessibility.requiredArtifacts).toContain('user enablement state');
    expect(vpnDns.requiredArtifacts).toContain('filter rollback result');
    expect(deviceOwner.requiredArtifacts).toContain('device-owner provisioning');
    expect(managedProfile.requiredArtifacts).toContain('managed-profile provisioning');
    expect(lifecycle.requiredArtifacts).toContain('foreground service state');
    expect(
      [usageStats, accessibility, vpnDns, deviceOwner, managedProfile, lifecycle].map((entry) => entry.gateDecision)
    ).toEqual([
      'requires-mobile-artifacts',
      'requires-mobile-artifacts',
      'requires-mobile-artifacts',
      'requires-mobile-artifacts',
      'requires-mobile-artifacts',
      'requires-mobile-artifacts',
    ]);
  });
}

function recordsIosEntitlementDistributionGates() {
  it('records iOS entitlement, background, signing, and TestFlight gates separately', () => {
    const familyControls = entryFor(V08OsAdapterManualArtifactGateSurface.IosFamilyControls);
    const deviceActivity = entryFor(V08OsAdapterManualArtifactGateSurface.IosDeviceActivity);
    const screenTime = entryFor(V08OsAdapterManualArtifactGateSurface.IosScreenTime);
    const networkExtension = entryFor(V08OsAdapterManualArtifactGateSurface.IosNetworkExtension);
    const signing = entryFor(V08OsAdapterManualArtifactGateSurface.IosBackgroundExecutionSigning);
    const testflight = entryFor(V08OsAdapterManualArtifactGateSurface.IosTestflightDeviceInstall);

    expect(familyControls.requiredArtifacts).toContain('Family Controls entitlement');
    expect(deviceActivity.requiredArtifacts).toContain('monitor schedule result');
    expect(screenTime.requiredArtifacts).toContain('shield rollback result');
    expect(networkExtension.requiredArtifacts).toContain('Network Extension entitlement');
    expect(signing.requiredArtifacts).toContain('provisioning profile');
    expect(testflight.requiredArtifacts).toContain('TestFlight build');
    expect(
      [familyControls, deviceActivity, screenTime, networkExtension, signing, testflight].map(
        (entry) => entry.mobilePrivilegeClaimed
      )
    ).toEqual([false, false, false, false, false, false]);
  });
}

function rejectsClaimUpgrades() {
  it('rejects product-ready, broad app, network, exact browser, unsupported platform, and mobile privilege upgrades', () => {
    const broadApp = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsBroadInstalledAppIdentity);
    const network = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsNetworkDomainFilterApplyRollback);
    const managedUrl = entryFor(V08OsAdapterManualArtifactGateSurface.WindowsManagedBrowserExactUrl);
    const linux = entryFor(V08OsAdapterManualArtifactGateSurface.LinuxServicePackagePermission);
    const android = entryFor(V08OsAdapterManualArtifactGateSurface.AndroidDeviceOwner);

    expect(() =>
      V08OsAdapterManualArtifactGateEntrySchema.parse({
        ...broadApp,
        gateEntryId: 'invalid-product-ready-upgrade',
        productReadyBlockingClaimed: true,
        broadInstalledAppBlockingClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08OsAdapterManualArtifactGateEntrySchema.parse({
        ...network,
        gateEntryId: 'invalid-network-upgrade',
        networkDomainBlockingClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08OsAdapterManualArtifactGateEntrySchema.parse({
        ...managedUrl,
        gateEntryId: 'invalid-managed-url-upgrade',
        managedBrowserExactUrlClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08OsAdapterManualArtifactGateEntrySchema.parse({
        ...linux,
        gateEntryId: 'invalid-unsupported-platform-upgrade',
        unsupportedPlatformClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08OsAdapterManualArtifactGateEntrySchema.parse({
        ...android,
        gateEntryId: 'invalid-mobile-privilege-upgrade',
        mobilePrivilegeClaimed: true,
      })
    ).toThrow();
  });
}

function entryFor(surface: string) {
  const entry = V08OsAdapterManualArtifactGateReadModel.entries.find((candidate) => candidate.surface === surface);
  if (entry === undefined) {
    throw new Error(`Missing V0.8 OS adapter manual artifact gate entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
