import { describe, expect, it } from 'vitest';
import {
  type TamperUninstallArtifactStatusReadModel,
  TamperUninstallArtifactStatusReadModelSchema,
} from '../../src/tamper-uninstall-artifact-status';

describe('tamper uninstall artifact status schema contracts', () => {
  acceptsParentAuthorizedRemovalAndRevocationStates();
  rejectsChildSelfAuthorizationUpgrade();
  rejectsMissingParentAuthorizationOrRevocationAuditRefs();
  rejectsRevokedTrustOrTeardownCleanupRegression();
});

function acceptsParentAuthorizedRemovalAndRevocationStates(): void {
  it('TamperUninstallArtifactStatusReadModelSchema: accepts parent-authorized removal states with revocation and teardown proof refs', () => {
    const parsed = TamperUninstallArtifactStatusReadModelSchema.parse(validReadModel());

    expect(parsed.removalFlow).toEqual({
      childSelfAuthorizationState: 'child-self-authorize-forbidden',
      parentAuthorizationState: 'required-where-platform-allows',
      revokedTrustState: 'inactive-until-parent-reauthorizes',
      revocationAuditState: 'audit-trail-required',
      teardownState: 'authority-ends-cleanly-when-removal-is-proved',
      residualStateVisibility: 'reported-until-cleanup-proof',
      parentAuthorizationRefs: ['parent-authorized-uninstall-request-ref'],
      revocationAuditRefs: ['trust-revocation-audit-trail-ref'],
      teardownProofRefs: ['child-authority-teardown-proof-ref'],
      cleanupProofRefs: ['residual-state-cleanup-review-ref'],
      boundary:
        'Child trust removal cannot self-authorize; parent authorization is required where the platform allows uninstall control, revoked trust becomes inactive until parent reauthorization, audit trail and cleanup proof stay attached, and residual state remains visible until teardown proof completes.',
    });
    expect(parsed.entries.map((entry) => entry.surface)).toEqual([
      'windows-service-stop',
      'windows-package-uninstall',
      'linux-service-package',
      'macos-launchd-package',
      'android-package-removed',
      'android-device-owner-managed-profile',
      'ios-family-controls-device-activity',
      'admin-removal-flow',
    ]);
  });
}

function rejectsChildSelfAuthorizationUpgrade(): void {
  it('TamperUninstallArtifactStatusReadModelSchema: rejects child self-authorizing trust removal', () => {
    const model = validReadModel();

    expect(
      TamperUninstallArtifactStatusReadModelSchema.safeParse({
        ...model,
        removalFlow: {
          ...model.removalFlow,
          childSelfAuthorizationState: 'child-self-authorize-allowed',
        },
      }).success
    ).toBe(false);
  });
}

function rejectsMissingParentAuthorizationOrRevocationAuditRefs(): void {
  it('TamperUninstallArtifactStatusReadModelSchema: rejects missing parent authorization or revocation audit refs', () => {
    const model = validReadModel();

    expect(
      TamperUninstallArtifactStatusReadModelSchema.safeParse({
        ...model,
        removalFlow: {
          ...model.removalFlow,
          parentAuthorizationRefs: [],
        },
      }).success
    ).toBe(false);

    expect(
      TamperUninstallArtifactStatusReadModelSchema.safeParse({
        ...model,
        removalFlow: {
          ...model.removalFlow,
          revocationAuditRefs: [],
        },
      }).success
    ).toBe(false);
  });
}

function rejectsRevokedTrustOrTeardownCleanupRegression(): void {
  it('TamperUninstallArtifactStatusReadModelSchema: rejects revoked trust staying active or missing teardown/cleanup proof', () => {
    const model = validReadModel();

    expect(
      TamperUninstallArtifactStatusReadModelSchema.safeParse({
        ...model,
        removalFlow: {
          ...model.removalFlow,
          revokedTrustState: 'still-active-after-revocation',
        },
      }).success
    ).toBe(false);

    expect(
      TamperUninstallArtifactStatusReadModelSchema.safeParse({
        ...model,
        removalFlow: {
          ...model.removalFlow,
          teardownProofRefs: [],
        },
      }).success
    ).toBe(false);

    expect(
      TamperUninstallArtifactStatusReadModelSchema.safeParse({
        ...model,
        removalFlow: {
          ...model.removalFlow,
          cleanupProofRefs: [],
        },
      }).success
    ).toBe(false);
  });
}

function validReadModel(): TamperUninstallArtifactStatusReadModel {
  return TamperUninstallArtifactStatusReadModelSchema.parse({
    schemaVersion: 'v0.6',
    readModelId: 'tamper-uninstall-artifact-status-proof',
    generatedAt: '2026-06-03T10:15:06.243Z',
    sourceReadModelIds: [
      'tamper-integrity-audit-contract-proof',
      'v0-8-integrity-alert-status-bridge',
      'v0-8-os-adapter-manual-artifact-gates',
    ],
    removalFlow: {
      childSelfAuthorizationState: 'child-self-authorize-forbidden',
      parentAuthorizationState: 'required-where-platform-allows',
      revokedTrustState: 'inactive-until-parent-reauthorizes',
      revocationAuditState: 'audit-trail-required',
      teardownState: 'authority-ends-cleanly-when-removal-is-proved',
      residualStateVisibility: 'reported-until-cleanup-proof',
      parentAuthorizationRefs: ['parent-authorized-uninstall-request-ref'],
      revocationAuditRefs: ['trust-revocation-audit-trail-ref'],
      teardownProofRefs: ['child-authority-teardown-proof-ref'],
      cleanupProofRefs: ['residual-state-cleanup-review-ref'],
      boundary:
        'Child trust removal cannot self-authorize; parent authorization is required where the platform allows uninstall control, revoked trust becomes inactive until parent reauthorization, audit trail and cleanup proof stay attached, and residual state remains visible until teardown proof completes.',
    },
    entries: entries(),
  });
}

function entries() {
  return [
    hostEntry('windows-service-stop', 'windows', 'headless-agent-service', [
      'Windows service manager stopped-state artifact',
      'operator consent or admin action ref',
      'post-stop heartbeat/status ref',
    ]),
    hostEntry('windows-package-uninstall', 'windows', 'package-lifecycle', [
      'Windows installer uninstall or rollback artifact',
      'package identity and version ref',
      'post-uninstall agent absence ref',
    ]),
    hostEntry('linux-service-package', 'linux', 'headless-agent-service', [
      'Linux service manager stop/remove artifact',
      'Linux package identity ref',
      'post-remove heartbeat/status ref',
    ]),
    hostEntry('macos-launchd-package', 'macos', 'headless-agent-service', [
      'macOS launchd/helper stop or removal artifact',
      'bundle/package identity ref',
      'post-remove heartbeat/status ref',
    ]),
    deviceEntry('android-package-removed', 'android', 'package-lifecycle', [
      'Android package removed broadcast or package manager artifact',
      'foreground service stopped artifact',
      'device install identity ref',
    ]),
    deviceEntry('android-device-owner-managed-profile', 'android', 'device-owner-policy', [
      'Android device-owner or managed-profile enrollment artifact',
      'policy removal/admin action artifact',
      'post-removal package lifecycle ref',
    ]),
    deviceEntry('ios-family-controls-device-activity', 'ios', 'family-controls-entitlement', [
      'iOS Family Controls authorization artifact',
      'DeviceActivity monitor status artifact',
      'device install or entitlement ref',
    ]),
    {
      schemaVersion: 'v0.6',
      statusEntryId: 'tamper-uninstall-admin-removal-flow',
      surface: 'admin-removal-flow',
      platform: 'windows',
      capability: 'package-lifecycle',
      capabilityStatus: 'manual-required',
      artifactState: 'documented-admin-removal',
      parentVisibleStatus: 'admin-removal-documented',
      custodyState: 'documented-flow-only',
      requiredArtifacts: ['documented parent/admin removal path', 'support/admin drill-in ref'],
      sourceProofRefs: sourceProofRefs(),
      auditRefs: ['tamper-integrity-audit-ref'],
      integrityRefs: ['integrity-alert-status-bridge-ref'],
      adminRemovalFlowRefs: ['documented-parent-admin-removal-flow-ref'],
      boundary:
        'Admin removal is documented as a parent/admin flow and is not blocked by this proof or upgraded to anti-removal behavior.',
      uninstallDetectionClaimed: false,
      tamperResistanceClaimed: false,
      stealthPersistenceClaimed: false,
      privilegeEscalationClaimed: false,
      adminRemovalBlockingClaimed: false,
      providerDeliveryClaimed: false,
      rawChildDataIncluded: false,
      lastCheckedAt: '2026-06-03T10:15:06.243Z',
    },
  ];
}

function hostEntry(surface: string, platform: string, capability: string, requiredArtifacts: readonly string[]) {
  return entry(surface, platform, capability, 'manual-required', 'artifact-needed', 'manual-review-required', requiredArtifacts);
}

function deviceEntry(surface: string, platform: string, capability: string, requiredArtifacts: readonly string[]) {
  return entry(
    surface,
    platform,
    capability,
    'device-proof-required',
    'device-proof-needed',
    'not-collected',
    requiredArtifacts
  );
}

function entry(
  surface: string,
  platform: string,
  capability: string,
  artifactState: string,
  parentVisibleStatus: string,
  custodyState: string,
  requiredArtifacts: readonly string[]
) {
  return {
    schemaVersion: 'v0.6',
    statusEntryId: `tamper-uninstall-${surface}`,
    surface,
    platform,
    capability,
    capabilityStatus: 'manual-required',
    artifactState,
    parentVisibleStatus,
    custodyState,
    requiredArtifacts: [...requiredArtifacts],
    sourceProofRefs: sourceProofRefs(),
    auditRefs: ['tamper-integrity-audit-ref'],
    integrityRefs: ['integrity-alert-status-bridge-ref'],
    adminRemovalFlowRefs: [],
    boundary:
      platform === 'android' || platform === 'ios'
        ? 'Mobile uninstall artifact status requires real emulator or physical-device evidence before any detection claim can upgrade.'
        : 'Desktop uninstall or stopped-service artifact status remains manual-required until platform-specific service/package evidence is attached.',
    uninstallDetectionClaimed: false,
    tamperResistanceClaimed: false,
    stealthPersistenceClaimed: false,
    privilegeEscalationClaimed: false,
    adminRemovalBlockingClaimed: false,
    providerDeliveryClaimed: false,
    rawChildDataIncluded: false,
    lastCheckedAt: '2026-06-03T10:15:06.243Z',
  };
}

function sourceProofRefs() {
  return [
    'tamper-integrity-audit-contract-proof',
    'v0-8-integrity-alert-status-bridge',
    'v0-8-os-adapter-manual-artifact-gates',
  ];
}
