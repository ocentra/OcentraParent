import { expect, it } from 'vitest';
import {
  V08EnforcementProductControlSpineEntrySchema,
  V08EnforcementProductControlSpineReadModel,
  V08EnforcementProductControlSpineReadModelSchema,
  V08EnforcementProductControlSurface,
} from '@ocentra-parent/schema-domain/v0-8-enforcement-product-control-spine';

it('captures product-control actions without broad claim upgrades', () => {
  const readModel = V08EnforcementProductControlSpineReadModelSchema.parse(V08EnforcementProductControlSpineReadModel);
  const claimCounts = countBy(readModel.entries.map((entry) => entry.productClaimState));
  const devicePolicyCounts = countBy(readModel.entries.map((entry) => entry.devicePolicyState));

  expect(readModel.entries).toHaveLength(15);
  expect(claimCounts).toEqual({
    'implemented-boundary': 6,
    'degraded-boundary': 1,
    'dry-run-only': 1,
    'manual-required': 6,
    'not-claimed': 1,
  });
  expect(devicePolicyCounts).toEqual({
    'control-capable': 5,
    'report-only': 2,
    'preview-only': 1,
    'manual-required': 6,
    'not-claimed': 1,
  });
  expect(readModel.entries.every((entry) => !entry.broadAppBlockingClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.networkDomainBlockingClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.managedExactUrlBlockingClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.unmanagedExactUrlClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.tamperResistanceClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.notificationDeliveryClaimed)).toBe(true);
});

it('keeps parent-visible action states exact for control-capable surfaces', () => {
  const ownedProcess = entryFor(V08EnforcementProductControlSurface.WindowsOwnedProcessTimeLimit);
  const appTimer = entryFor(V08EnforcementProductControlSurface.WindowsAppTimeLimitLifecycle);
  const managedBrowser = entryFor(V08EnforcementProductControlSurface.WindowsManagedBrowserSessionIntervention);
  const policyPreview = entryFor(V08EnforcementProductControlSurface.WindowsPolicyDryRunPreview);

  expect(ownedProcess).toMatchObject({
    capability: 'owned-process-terminate',
    productClaimState: 'implemented-boundary',
    devicePolicyState: 'control-capable',
    parentVisibleActions: ['observe', 'time-limit', 'block-scoped-process'],
  });
  expect(appTimer.parentVisibleActions).toEqual(['observe', 'time-limit', 'ask-parent']);
  expect(managedBrowser.parentVisibleActions).toEqual(['observe', 'warn', 'time-limit']);
  expect(policyPreview).toMatchObject({
    productClaimState: 'dry-run-only',
    adapterExecutionState: 'returns-dry-run-preview',
    devicePolicyState: 'preview-only',
    parentVisibleActions: ['dry-run-preview', 'ask-parent'],
  });
});

it('keeps manual and not-claimed surfaces report-only', () => {
  const broadApp = entryFor(V08EnforcementProductControlSurface.WindowsBroadAppBlocking);
  const networkDomain = entryFor(V08EnforcementProductControlSurface.WindowsNetworkDomainBlocking);
  const managedExactUrl = entryFor(V08EnforcementProductControlSurface.WindowsManagedExactUrlControl);
  const unmanagedExactUrl = entryFor(V08EnforcementProductControlSurface.WindowsUnmanagedExactUrlNotClaimed);
  const tamper = entryFor(V08EnforcementProductControlSurface.WindowsTamperUninstallAlerts);

  expect(broadApp).toMatchObject({
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
    devicePolicyState: 'manual-required',
    parentVisibleActions: ['report-only'],
  });
  expect(networkDomain.claimBoundary).toContain('not proved by network observation metadata');
  expect(managedExactUrl.manualProofRequirements).toContain('exact URL apply result');
  expect(unmanagedExactUrl).toMatchObject({
    productClaimState: 'not-claimed',
    adapterExecutionState: 'not-invoked',
    devicePolicyState: 'not-claimed',
  });
  expect(tamper.claimBoundary).toContain('do not imply stealth');
});

it('rejects direct action drift for manual and dry-run surfaces', () => {
  const networkDomain = entryFor(V08EnforcementProductControlSurface.WindowsNetworkDomainBlocking);
  const policyPreview = entryFor(V08EnforcementProductControlSurface.WindowsPolicyDryRunPreview);
  const unmanagedExactUrl = entryFor(V08EnforcementProductControlSurface.WindowsUnmanagedExactUrlNotClaimed);

  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...networkDomain,
      entryId: 'invalid-network-domain-block-action-drift',
      parentVisibleActions: ['observe', 'block-scoped-process'],
    })
  ).toThrow();
  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...policyPreview,
      entryId: 'invalid-policy-preview-execution-drift',
      productClaimState: 'implemented-boundary',
      adapterExecutionState: 'executes-real-service',
      devicePolicyState: 'control-capable',
    })
  ).toThrow();
  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...unmanagedExactUrl,
      entryId: 'invalid-unmanaged-exact-url-claim-drift',
      productClaimState: 'manual-required',
      adapterExecutionState: 'returns-manual-required',
      devicePolicyState: 'manual-required',
    })
  ).toThrow();
});

it('rejects broad, exact URL, tamper, and notification claim upgrades', () => {
  const broadApp = entryFor(V08EnforcementProductControlSurface.WindowsBroadAppBlocking);
  const managedExactUrl = entryFor(V08EnforcementProductControlSurface.WindowsManagedExactUrlControl);
  const permissionLoss = entryFor(V08EnforcementProductControlSurface.WindowsPermissionLossAlerts);
  const tamper = entryFor(V08EnforcementProductControlSurface.WindowsTamperUninstallAlerts);

  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...broadApp,
      entryId: 'invalid-broad-app-claim-upgrade',
      broadAppBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...managedExactUrl,
      entryId: 'invalid-managed-exact-url-claim-upgrade',
      managedExactUrlBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...permissionLoss,
      entryId: 'invalid-notification-delivery-claim-upgrade',
      notificationDeliveryClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08EnforcementProductControlSpineEntrySchema.parse({
      ...tamper,
      entryId: 'invalid-tamper-resistance-claim-upgrade',
      tamperResistanceClaimed: true,
    })
  ).toThrow();
});

function entryFor(surface: string) {
  const entry = V08EnforcementProductControlSpineReadModel.entries.find((candidate) => candidate.surface === surface);
  if (entry === undefined) {
    throw new Error(`Missing V0.8 enforcement product-control entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
