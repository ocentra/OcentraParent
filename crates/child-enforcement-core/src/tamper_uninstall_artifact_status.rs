const TAMPER_UNINSTALL_ARTIFACT_STATUS_TYPESCRIPT: &str = r#"/* generated from crates/child-enforcement-core/src/tamper_uninstall_artifact_status.rs */

import { ParentControlCapabilityName, ParentControlCapabilityStatus } from '@ocentra-parent/schema-domain/capabilities';
import type {
  ParentControlCapabilityNameSchema,
  ParentControlCapabilityStatusSchema,
  ParentControlPlatformSchema,
} from '@ocentra-parent/schema-domain/capabilities';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  TamperUninstallArtifactStatusEntrySchema,
  TamperUninstallArtifactStatusReadModelSchema,
  TamperUninstallRemovalFlowSummarySchema,
  type TamperUninstallArtifactCustody,
  type TamperUninstallArtifactState,
  type TamperUninstallRemovalFlowSummary,
  type TamperUninstallArtifactStatusEntry,
  type TamperUninstallArtifactSurface,
  type TamperUninstallParentVisibleStatus,
} from '@ocentra-parent/schema-domain/tamper-uninstall-artifact-status';

type TamperUninstallArtifactStatusEntryInput = {
  statusEntryId: string;
  surface: TamperUninstallArtifactSurface;
  platform: typeof ParentControlPlatformSchema.Type;
  capability: typeof ParentControlCapabilityNameSchema.Type;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  artifactState: TamperUninstallArtifactState;
  parentVisibleStatus: TamperUninstallParentVisibleStatus;
  custodyState: TamperUninstallArtifactCustody;
  requiredArtifacts: readonly string[];
  boundary: string;
  adminRemovalFlowRefs?: readonly string[];
};

const generatedAt = '2026-06-03T10:15:06.243Z';
const SourceReadModelIds = [
  'tamper-integrity-audit-contract-proof',
  'v0-8-integrity-alert-status-bridge',
  'v0-8-os-adapter-manual-artifact-gates',
] as const;

export const TamperUninstallArtifactStatusReadModel = TamperUninstallArtifactStatusReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'tamper-uninstall-artifact-status-proof',
  generatedAt,
  sourceReadModelIds: SourceReadModelIds,
  removalFlow: removalFlowSummary(),
  entries: [
    hostEntry('windows-service-stop', ParentControlCapabilityName.HeadlessAgentService, [
      'Windows service manager stopped-state artifact',
      'operator consent or admin action ref',
      'post-stop heartbeat/status ref',
    ]),
    hostEntry('windows-package-uninstall', ParentControlCapabilityName.PackageLifecycle, [
      'Windows installer uninstall or rollback artifact',
      'package identity and version ref',
      'post-uninstall agent absence ref',
    ]),
    hostEntry('linux-service-package', ParentControlCapabilityName.HeadlessAgentService, [
      'Linux service manager stop/remove artifact',
      'Linux package identity ref',
      'post-remove heartbeat/status ref',
    ]),
    hostEntry('macos-launchd-package', ParentControlCapabilityName.HeadlessAgentService, [
      'macOS launchd/helper stop or removal artifact',
      'bundle/package identity ref',
      'post-remove heartbeat/status ref',
    ]),
    deviceEntry('android-package-removed', 'android', ParentControlCapabilityName.PackageLifecycle, [
      'Android package removed broadcast or package manager artifact',
      'foreground service stopped artifact',
      'device install identity ref',
    ]),
    deviceEntry('android-device-owner-managed-profile', 'android', ParentControlCapabilityName.DeviceOwnerPolicy, [
      'Android device-owner or managed-profile enrollment artifact',
      'policy removal/admin action artifact',
      'post-removal package lifecycle ref',
    ]),
    deviceEntry('ios-family-controls-device-activity', 'ios', ParentControlCapabilityName.FamilyControlsEntitlement, [
      'iOS Family Controls authorization artifact',
      'DeviceActivity monitor status artifact',
      'device install or entitlement ref',
    ]),
    entry({
      statusEntryId: 'tamper-uninstall-admin-removal-flow',
      surface: 'admin-removal-flow',
      platform: 'windows',
      capability: ParentControlCapabilityName.PackageLifecycle,
      capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
      artifactState: 'documented-admin-removal',
      parentVisibleStatus: 'admin-removal-documented',
      custodyState: 'documented-flow-only',
      requiredArtifacts: ['documented parent/admin removal path', 'support/admin drill-in ref'],
      adminRemovalFlowRefs: ['documented-parent-admin-removal-flow-ref'],
      boundary:
        'Admin removal is documented as a parent/admin flow and is not blocked by this proof or upgraded to anti-removal behavior.',
    }),
  ],
});

function removalFlowSummary(): TamperUninstallRemovalFlowSummary {
  return TamperUninstallRemovalFlowSummarySchema.parse({
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
}

function hostEntry(
  surface: Exclude<TamperUninstallArtifactSurface, 'admin-removal-flow'>,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  requiredArtifacts: readonly string[]
): TamperUninstallArtifactStatusEntry {
  return entry({
    statusEntryId: `tamper-uninstall-${surface}`,
    surface,
    platform: surface.startsWith('linux') ? 'linux' : surface.startsWith('macos') ? 'macos' : 'windows',
    capability,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    artifactState: 'manual-required',
    parentVisibleStatus: 'artifact-needed',
    custodyState: 'manual-review-required',
    requiredArtifacts,
    boundary:
      'Desktop uninstall or stopped-service artifact status remains manual-required until platform-specific service/package evidence is attached.',
  });
}

function deviceEntry(
  surface: Exclude<TamperUninstallArtifactSurface, 'admin-removal-flow'>,
  platform: 'android' | 'ios',
  capability: typeof ParentControlCapabilityNameSchema.Type,
  requiredArtifacts: readonly string[]
): TamperUninstallArtifactStatusEntry {
  return entry({
    statusEntryId: `tamper-uninstall-${surface}`,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    artifactState: 'device-proof-required',
    parentVisibleStatus: 'device-proof-needed',
    custodyState: 'not-collected',
    requiredArtifacts,
    boundary:
      'Mobile uninstall artifact status requires real emulator or physical-device evidence before any detection claim can upgrade.',
  });
}

function entry(input: TamperUninstallArtifactStatusEntryInput): TamperUninstallArtifactStatusEntry {
  return TamperUninstallArtifactStatusEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    sourceProofRefs: [...SourceReadModelIds],
    auditRefs: ['tamper-integrity-audit-ref'],
    integrityRefs: ['integrity-alert-status-bridge-ref'],
    adminRemovalFlowRefs: [...(input.adminRemovalFlowRefs ?? [])],
    uninstallDetectionClaimed: false,
    tamperResistanceClaimed: false,
    stealthPersistenceClaimed: false,
    privilegeEscalationClaimed: false,
    adminRemovalBlockingClaimed: false,
    providerDeliveryClaimed: false,
    rawChildDataIncluded: false,
    lastCheckedAt: generatedAt,
    ...input,
    requiredArtifacts: [...input.requiredArtifacts],
  });
}
"#;

pub fn tamper_uninstall_artifact_status_typescript() -> String {
    TAMPER_UNINSTALL_ARTIFACT_STATUS_TYPESCRIPT.to_string()
}
