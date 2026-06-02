import { describe, expect, it } from 'vitest';
import {
  ParentDesktopReleaseSupportReadModelSchema,
  type ParentDesktopReleaseSupportDiagnosticField,
  type ParentDesktopReleaseSupportOperation,
  type ParentDesktopReleaseSupportSigningSurface,
} from '../src/parent-desktop-release-support';
import { RuntimeReadModel } from './parent-desktop-release-support-fixtures';

describe('parent desktop release support proof contracts', () => {
  registerAcceptedStateTest();
  registerObserverAuthorityGuardrailTest();
  registerMobileBridgeGuardrailTest();
  registerReleaseClaimGuardrailTests();
  registerSupportAndRunbookGuardrailTests();
});

function registerAcceptedStateTest(): void {
  it('accepts release support state with observer-only authority and manual production gaps', () => {
    const parsed = ParentDesktopReleaseSupportReadModelSchema.parse(RuntimeReadModel);

    expect(parsed.observerAuthority.map((entry) => entry.operation)).toEqual([
      'read-service-state',
      'read-route-state',
      'write-policy',
      'approve-request',
      'take-controller',
    ]);
    expect(parsed.mobileBridgeBoundary.childAndroidAgentState).toBe('manual-required');
    expect(parsed.updateStates.find((entry) => entry.channel === 'unsigned-preview')?.rollbackState).toBe(
      'rollback-unavailable'
    );
    expect(parsed.signingStoreStates.map((entry) => entry.surface)).toEqual([
      'windows-code-signing',
      'macos-notarization',
      'google-play',
      'testflight',
      'app-store',
    ]);
    expect(parsed.platformCapabilityMatrix.map((entry) => entry.target)).toEqual([
      'parent-desktop',
      'parent-mobile',
      'child-desktop',
      'child-android',
      'child-ios',
      'relay',
      'signing',
      'store',
      'support',
    ]);
  });
}

function registerObserverAuthorityGuardrailTest(): void {
  it('rejects observer write, approval, or controller operations presented as completed', () => {
    for (const operation of ['write-policy', 'approve-request', 'take-controller'] as const) {
      const claim = withObserverOperation(operation, { result: 'completed', rejectionReason: null });

      expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(claim).success).toBe(false);
    }
  });
}

function registerMobileBridgeGuardrailTest(): void {
  it('rejects parent mobile bridge state that implies child mobile agent parity', () => {
    const androidParityClaim = withMobileBoundary({ childAndroidAgentState: 'implemented' });
    const iosParityClaim = withMobileBoundary({ childIosAgentState: 'implemented' });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(androidParityClaim).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(iosParityClaim).success).toBe(false);
  });
}

function registerReleaseClaimGuardrailTests(): void {
  it('rejects unsigned preview rollback and production package readiness without promotion', () => {
    const rollbackClaim = withUpdateChannel('unsigned-preview', { rollbackState: 'rollback-available' });
    const productionClaim = withUpdateChannel('production', { packageState: 'ci-artifact-present' });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(rollbackClaim).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(productionClaim).success).toBe(false);
  });

  it('rejects signing, notarization, store, or TestFlight surfaces marked proved without credentials', () => {
    for (const surface of [
      'windows-code-signing',
      'macos-notarization',
      'google-play',
      'testflight',
      'app-store',
    ] as const) {
      const claim = withSigningSurface(surface, { state: 'implemented', credentialState: 'implemented' });

      expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(claim).success).toBe(false);
    }
  });

  it('rejects ready package claims when GitHub artifact proof is pending or absent', () => {
    const readyWithoutArtifacts = {
      ...RuntimeReadModel,
      ciArtifactProof: {
        ...RuntimeReadModel.ciArtifactProof,
        packageReadinessClaim: 'ready',
        runStatus: 'success',
        artifactState: 'manual-required',
      },
    };

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(readyWithoutArtifacts).success).toBe(false);
  });
}

function registerSupportAndRunbookGuardrailTests(): void {
  it('rejects support diagnostics that omit required fields or leak unredacted sensitive values', () => {
    const missingCommit = {
      ...RuntimeReadModel,
      supportDiagnostics: {
        ...RuntimeReadModel.supportDiagnostics,
        entries: RuntimeReadModel.supportDiagnostics.entries.filter((entry) => entry.field !== 'commit'),
      },
    };
    const leakedToken = withDiagnostic('service', { value: 'token=abc123', redactionState: 'safe' });

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingCommit).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(leakedToken).success).toBe(false);
  });

  it('rejects incomplete platform matrices and manual runbooks', () => {
    const missingRelayRow = {
      ...RuntimeReadModel,
      platformCapabilityMatrix: RuntimeReadModel.platformCapabilityMatrix.filter((entry) => entry.target !== 'relay'),
    };
    const missingRunbookGap = {
      ...RuntimeReadModel,
      manualRunbook: RuntimeReadModel.manualRunbook.map((entry) =>
        entry.target === 'support' ? { ...entry, knownGaps: [] } : entry
      ),
    };

    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingRelayRow).success).toBe(false);
    expect(ParentDesktopReleaseSupportReadModelSchema.safeParse(missingRunbookGap).success).toBe(false);
  });
}

function withMobileBoundary(patch: object) {
  return { ...RuntimeReadModel, mobileBridgeBoundary: { ...RuntimeReadModel.mobileBridgeBoundary, ...patch } };
}

function withObserverOperation(operation: ParentDesktopReleaseSupportOperation, patch: object) {
  return {
    ...RuntimeReadModel,
    observerAuthority: RuntimeReadModel.observerAuthority.map((entry) =>
      entry.operation === operation ? { ...entry, ...patch } : entry
    ),
  };
}

function withUpdateChannel(
  channel: 'scaffold' | 'unsigned-preview' | 'signature-required' | 'production',
  patch: object
) {
  return {
    ...RuntimeReadModel,
    updateStates: RuntimeReadModel.updateStates.map((entry) =>
      entry.channel === channel ? { ...entry, ...patch } : entry
    ),
  };
}

function withSigningSurface(surface: ParentDesktopReleaseSupportSigningSurface, patch: object) {
  return {
    ...RuntimeReadModel,
    signingStoreStates: RuntimeReadModel.signingStoreStates.map((entry) =>
      entry.surface === surface ? { ...entry, ...patch } : entry
    ),
  };
}

function withDiagnostic(field: ParentDesktopReleaseSupportDiagnosticField, patch: object) {
  return {
    ...RuntimeReadModel,
    supportDiagnostics: {
      ...RuntimeReadModel.supportDiagnostics,
      entries: RuntimeReadModel.supportDiagnostics.entries.map((entry) =>
        entry.field === field ? { ...entry, ...patch } : entry
      ),
    },
  };
}
