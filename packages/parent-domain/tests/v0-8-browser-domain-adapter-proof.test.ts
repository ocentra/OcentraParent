import { describe, expect, it } from 'vitest';
import {
  V08BrowserDomainAdapterProofEntrySchema,
  V08BrowserDomainAdapterProofReadModel,
  V08BrowserDomainAdapterProofReadModelSchema,
  V08BrowserDomainAdapterProofSurface,
} from '../src/v0-8-browser-domain-adapter-proof';

describe('V0.8 browser domain adapter proof', () => {
  capturesBrowserAndDomainAdapterStates();
  separatesManagedBrowserFromExactUrl();
  recordsUnmanagedBrowserBoundaries();
  keepsNetworkAndUnsupportedTargetsBounded();
  recordsAuditRestartAndRollbackVisibility();
  rejectsSurfaceStateDrift();
  rejectsClaimUpgrades();
});

function capturesBrowserAndDomainAdapterStates() {
  it('captures browser and domain adapter states without claim upgrades', () => {
    const readModel = V08BrowserDomainAdapterProofReadModelSchema.parse(V08BrowserDomainAdapterProofReadModel);
    const claimCounts = countBy(readModel.entries.map((entry) => entry.productClaimState));
    const platformCounts = countBy(readModel.entries.map((entry) => entry.platform));

    expect(readModel.entries).toHaveLength(14);
    expect(claimCounts).toEqual({
      'implemented-boundary': 5,
      'degraded-boundary': 1,
      'manual-required': 4,
      unavailable: 3,
      'not-claimed': 1,
    });
    expect(platformCounts).toEqual({
      windows: 10,
      linux: 1,
      macos: 1,
      android: 1,
      ios: 1,
    });
    expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
    expect(
      readModel.entries.every(
        (entry) =>
          !entry.managedExactUrlClaimed &&
          !entry.unmanagedExactUrlClaimed &&
          !entry.networkDomainBlockingClaimed &&
          !entry.broadBrowserControlClaimed &&
          !entry.unsupportedOsClaimed
      )
    ).toBe(true);
  });
}

function separatesManagedBrowserFromExactUrl() {
  it('separates managed browser intervention from exact URL enforcement', () => {
    const managed = entryFor(V08BrowserDomainAdapterProofSurface.WindowsManagedBrowserInterventionState);
    const exactUrl = entryFor(V08BrowserDomainAdapterProofSurface.WindowsManagedBrowserExactUrlManual);

    expect(managed).toMatchObject({
      platform: 'windows',
      capability: 'managed-browser-control',
      productClaimState: 'implemented-boundary',
      adapterExecutionState: 'executes-real-service',
      managedExactUrlClaimed: false,
    });
    expect(managed.claimBoundary).toContain('managed-session boundary');
    expect(exactUrl).toMatchObject({
      productClaimState: 'manual-required',
      adapterExecutionState: 'returns-manual-required',
      managedExactUrlClaimed: false,
    });
    expect(exactUrl.manualProofRequirements).toContain('exact URL apply result');
  });
}

function recordsUnmanagedBrowserBoundaries() {
  it('records unmanaged browser terminate and warn boundaries without URL certainty', () => {
    const terminate = entryFor(V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserTerminateBoundary);
    const warn = entryFor(V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserWarnNoop);
    const exactEvidence = entryFor(V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserExactEvidenceNotClaimed);

    expect(terminate.claimBoundary).toContain('process-only');
    expect(warn).toMatchObject({
      productClaimState: 'degraded-boundary',
      adapterExecutionState: 'returns-degraded-noop',
    });
    expect(warn.manualProofRequirements).toContain('parent-visible warning delivery proof');
    expect(exactEvidence).toMatchObject({
      productClaimState: 'not-claimed',
      unmanagedExactUrlClaimed: false,
    });
    expect(exactEvidence.claimBoundary).toContain('remain not-claimed');
  });
}

function keepsNetworkAndUnsupportedTargetsBounded() {
  it('keeps network domain blocking and unsupported targets manual or unavailable', () => {
    const networkManual = entryFor(V08BrowserDomainAdapterProofSurface.WindowsNetworkDomainFilterManual);
    const networkUnavailable = entryFor(V08BrowserDomainAdapterProofSurface.WindowsNetworkDomainAdapterUnavailable);
    const linux = entryFor(V08BrowserDomainAdapterProofSurface.LinuxBrowserDomainAdapterUnavailable);
    const macos = entryFor(V08BrowserDomainAdapterProofSurface.MacosBrowserDomainAdapterUnavailable);
    const android = entryFor(V08BrowserDomainAdapterProofSurface.AndroidBrowserDomainAdapterManual);
    const ios = entryFor(V08BrowserDomainAdapterProofSurface.IosBrowserDomainAdapterManual);

    expect(networkManual).toMatchObject({
      capability: 'network-domain-blocking',
      productClaimState: 'manual-required',
      networkDomainBlockingClaimed: false,
    });
    expect(networkUnavailable).toMatchObject({
      productClaimState: 'unavailable',
      adapterExecutionState: 'returns-unavailable',
    });
    expect(linux.claimBoundary).toContain('cannot inherit Windows');
    expect(macos.productClaimState).toBe('unavailable');
    expect(android.manualProofRequirements).toContain('Android VPN or DNS filtering proof');
    expect(ios.manualProofRequirements).toContain('Network Extension entitlement proof');
  });
}

function recordsAuditRestartAndRollbackVisibility() {
  it('records audit restart and rollback visibility as bounded service proof', () => {
    const audit = entryFor(V08BrowserDomainAdapterProofSurface.WindowsAuditVisibilityBoundary);
    const restart = entryFor(V08BrowserDomainAdapterProofSurface.WindowsRestartRecoveryVisibilityBoundary);
    const rollback = entryFor(V08BrowserDomainAdapterProofSurface.WindowsBrowserPolicyRollbackVisibility);

    expect(audit.claimBoundary).toContain('journal and browser policy event seams');
    expect(restart.claimBoundary).toContain('app time-limit state recovery');
    expect(rollback.linkedProofCommands).toContain(
      'cargo test -p ocentra-parent-agent-service browser_policy_rollback_restores_earlier_persisted_revision'
    );
    expect(rollback.claimBoundary).toContain('stored policy revision rollback only');
  });
}

function rejectsSurfaceStateDrift() {
  it('rejects state drift for exact URL, unmanaged exact evidence, and network surfaces', () => {
    const exactUrl = entryFor(V08BrowserDomainAdapterProofSurface.WindowsManagedBrowserExactUrlManual);
    const unmanagedExact = entryFor(V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserExactEvidenceNotClaimed);
    const networkManual = entryFor(V08BrowserDomainAdapterProofSurface.WindowsNetworkDomainFilterManual);

    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...exactUrl,
        proofEntryId: 'invalid-managed-exact-url-state-upgrade',
        capabilityStatus: 'implemented',
        productClaimState: 'implemented-boundary',
        adapterExecutionState: 'executes-real-service',
        linkedProofCommands: ['node scripts/test/managed-browser-intervention-proof.mjs'],
        linkedProofArtifacts: ['test-results/managed-browser-intervention-proof/proof.json'],
        manualProofRequirements: [],
      })
    ).toThrow();
    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...unmanagedExact,
        proofEntryId: 'invalid-unmanaged-exact-evidence-state-upgrade',
        capabilityStatus: 'supported',
        productClaimState: 'degraded-boundary',
        adapterExecutionState: 'returns-degraded-noop',
        linkedProofCommands: ['node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs'],
        linkedProofArtifacts: ['unmanaged browser warning no-op service event'],
      })
    ).toThrow();
    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...networkManual,
        proofEntryId: 'invalid-network-domain-capability-drift',
        capability: 'managed-browser-control',
      })
    ).toThrow();
  });
}

function rejectsClaimUpgrades() {
  it('rejects exact URL, domain blocking, broad browser, and unsupported OS claim upgrades', () => {
    const managed = entryFor(V08BrowserDomainAdapterProofSurface.WindowsManagedBrowserExactUrlManual);
    const unmanaged = entryFor(V08BrowserDomainAdapterProofSurface.WindowsUnmanagedBrowserExactEvidenceNotClaimed);
    const network = entryFor(V08BrowserDomainAdapterProofSurface.WindowsNetworkDomainFilterManual);
    const linux = entryFor(V08BrowserDomainAdapterProofSurface.LinuxBrowserDomainAdapterUnavailable);

    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...managed,
        proofEntryId: 'invalid-managed-exact-url-upgrade',
        managedExactUrlClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...unmanaged,
        proofEntryId: 'invalid-unmanaged-exact-url-upgrade',
        unmanagedExactUrlClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...network,
        proofEntryId: 'invalid-network-domain-upgrade',
        networkDomainBlockingClaimed: true,
      })
    ).toThrow();
    expect(() =>
      V08BrowserDomainAdapterProofEntrySchema.parse({
        ...linux,
        proofEntryId: 'invalid-unsupported-os-upgrade',
        unsupportedOsClaimed: true,
      })
    ).toThrow();
  });
}

function entryFor(surface: string) {
  const entry = V08BrowserDomainAdapterProofReadModel.entries.find((candidate) => candidate.surface === surface);
  if (entry === undefined) {
    throw new Error(`Missing V0.8 browser/domain adapter proof entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
