import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ParentControlCapabilityName,
  ParentControlCapabilityNameSchema,
  ParentControlCapabilityStatus,
  ParentControlCapabilityStatusSchema,
  type ParentControlPlatform,
  ParentControlPlatformSchema,
} from './capabilities';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

const NonEmptyArtifactGateText = Schema.String.pipe(Schema.minLength(1));

export const V08OsAdapterManualArtifactGateReadModelIdSchema = NonEmptyArtifactGateText.pipe(
  Schema.brand('V08OsAdapterManualArtifactGateReadModelId')
);
export const V08OsAdapterManualArtifactGateEntryIdSchema = NonEmptyArtifactGateText.pipe(
  Schema.brand('V08OsAdapterManualArtifactGateEntryId')
);
export const V08OsAdapterManualArtifactGateReferenceSchema = NonEmptyArtifactGateText.pipe(
  Schema.brand('V08OsAdapterManualArtifactGateReference')
);
export const V08OsAdapterManualArtifactGateRequirementSchema = NonEmptyArtifactGateText.pipe(
  Schema.brand('V08OsAdapterManualArtifactGateRequirement')
);
export const V08OsAdapterManualArtifactGateClaimBoundarySchema = NonEmptyArtifactGateText.pipe(
  Schema.brand('V08OsAdapterManualArtifactGateClaimBoundary')
);
export const V08OsAdapterManualArtifactGateFallbackSchema = NonEmptyArtifactGateText.pipe(
  Schema.brand('V08OsAdapterManualArtifactGateFallback')
);

export const V08OsAdapterManualArtifactGateSurfaceSchema = withParser(
  Schema.Literal(
    'windows-broad-installed-app-identity',
    'windows-process-package-identity',
    'windows-owned-process-terminate',
    'windows-parent-cancel-override',
    'windows-network-domain-filter-apply-rollback',
    'windows-managed-browser-exact-url',
    'windows-unmanaged-exact-title-page-download',
    'windows-restart-recovery',
    'windows-audit-custody',
    'windows-service-permission',
    'windows-package-lifecycle',
    'linux-service-package-permission',
    'macos-service-package-permission',
    'android-usage-stats',
    'android-accessibility-service',
    'android-vpn-dns',
    'android-device-owner',
    'android-managed-profile',
    'android-package-lifecycle',
    'ios-family-controls',
    'ios-device-activity',
    'ios-screen-time',
    'ios-network-extension',
    'ios-background-execution-signing',
    'ios-testflight-device-install'
  )
);

export const V08OsAdapterManualArtifactGateEvidenceKindSchema = withParser(
  Schema.Literal(
    'app-identity',
    'process-identity',
    'parent-action',
    'network-domain-filter',
    'browser-evidence',
    'restart-recovery',
    'audit-custody',
    'service-permission',
    'package-lifecycle',
    'mobile-privilege',
    'mobile-distribution'
  )
);

export const V08OsAdapterManualArtifactGateDecisionSchema = withParser(
  Schema.Literal('requires-host-artifacts', 'requires-mobile-artifacts', 'unsupported-surface', 'adapter-unavailable')
);

export const V08OsAdapterManualArtifactGateOutcomeSchema = withParser(
  Schema.Literal('manual-required', 'not-claimed', 'unavailable')
);

const V08OsAdapterManualArtifactGateEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  gateEntryId: V08OsAdapterManualArtifactGateEntryIdSchema,
  surface: V08OsAdapterManualArtifactGateSurfaceSchema,
  platform: ParentControlPlatformSchema,
  capability: ParentControlCapabilityNameSchema,
  capabilityStatus: ParentControlCapabilityStatusSchema,
  evidenceKind: V08OsAdapterManualArtifactGateEvidenceKindSchema,
  gateDecision: V08OsAdapterManualArtifactGateDecisionSchema,
  gateOutcome: V08OsAdapterManualArtifactGateOutcomeSchema,
  requiredArtifacts: Schema.Array(V08OsAdapterManualArtifactGateRequirementSchema),
  linkedProofCommands: Schema.Array(V08OsAdapterManualArtifactGateReferenceSchema),
  linkedProofArtifacts: Schema.Array(V08OsAdapterManualArtifactGateReferenceSchema),
  hostCapabilityProbeRefs: Schema.Array(V08OsAdapterManualArtifactGateReferenceSchema),
  claimBoundary: V08OsAdapterManualArtifactGateClaimBoundarySchema,
  fallbackBehavior: V08OsAdapterManualArtifactGateFallbackSchema,
  productReadyBlockingClaimed: Schema.Boolean,
  broadInstalledAppBlockingClaimed: Schema.Boolean,
  networkDomainBlockingClaimed: Schema.Boolean,
  managedBrowserExactUrlClaimed: Schema.Boolean,
  unmanagedBrowserExactEvidenceClaimed: Schema.Boolean,
  unsupportedPlatformClaimed: Schema.Boolean,
  mobilePrivilegeClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V08OsAdapterManualArtifactGateEntryCandidate = Infer<typeof V08OsAdapterManualArtifactGateEntryBaseSchema>;

export const V08OsAdapterManualArtifactGateEntrySchema = withParser(
  V08OsAdapterManualArtifactGateEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        osAdapterManualArtifactGateEntryIsHonest(entry) ||
        'Expected V0.8 OS adapter manual artifact gates to preserve manual-required, not-claimed, and unavailable states without product-ready claim upgrades'
    )
  )
);

export const V08OsAdapterManualArtifactGateReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V08OsAdapterManualArtifactGateReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V08OsAdapterManualArtifactGateReferenceSchema),
    entries: Schema.Array(V08OsAdapterManualArtifactGateEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.gateEntryId)).size === readModel.entries.length ||
        'Expected V0.8 OS adapter manual artifact gate entry ids to be unique'
    )
  )
);

function osAdapterManualArtifactGateEntryIsHonest(entry: V08OsAdapterManualArtifactGateEntryCandidate): boolean {
  if (
    osAdapterManualArtifactGateEntryHasClaimUpgrade(entry) ||
    entry.requiredArtifacts.length === 0 ||
    !hostCapabilityProbeRefsMatchPlatform(entry)
  ) {
    return false;
  }

  switch (entry.gateOutcome) {
    case 'manual-required':
      return (
        entry.capabilityStatus === ParentControlCapabilityStatus.ManualRequired &&
        ['requires-host-artifacts', 'requires-mobile-artifacts'].includes(entry.gateDecision)
      );
    case 'not-claimed':
      return (
        entry.capabilityStatus === ParentControlCapabilityStatus.NotImplemented &&
        entry.gateDecision === 'unsupported-surface'
      );
    case 'unavailable':
      return (
        entry.capabilityStatus === ParentControlCapabilityStatus.Unavailable &&
        entry.gateDecision === 'adapter-unavailable'
      );
  }
}

function hostCapabilityProbeRefsMatchPlatform(entry: V08OsAdapterManualArtifactGateEntryCandidate): boolean {
  if (entry.platform === 'windows') {
    return sameRefs(entry.hostCapabilityProbeRefs, ['windows-host-local-probe-ref']);
  }
  if (entry.platform === 'linux') {
    return sameRefs(entry.hostCapabilityProbeRefs, ['linux-wsl-path-probe-ref', 'linux-docker-path-probe-ref']);
  }
  if (entry.platform === 'android') {
    return sameRefs(entry.hostCapabilityProbeRefs, ['android-adb-path-probe-ref', 'android-adb-sdk-probe-ref']);
  }
  return entry.hostCapabilityProbeRefs.length === 0;
}

function sameRefs(actual: readonly string[], expected: readonly string[]): boolean {
  return actual.length === expected.length && expected.every((ref) => actual.includes(ref));
}

function osAdapterManualArtifactGateEntryHasClaimUpgrade(entry: V08OsAdapterManualArtifactGateEntryCandidate): boolean {
  return [
    entry.productReadyBlockingClaimed,
    entry.broadInstalledAppBlockingClaimed,
    entry.networkDomainBlockingClaimed,
    entry.managedBrowserExactUrlClaimed,
    entry.unmanagedBrowserExactEvidenceClaimed,
    entry.unsupportedPlatformClaimed,
    entry.mobilePrivilegeClaimed,
  ].some(Boolean);
}

export type V08OsAdapterManualArtifactGateReadModelId = typeof V08OsAdapterManualArtifactGateReadModelIdSchema.Type;
export type V08OsAdapterManualArtifactGateEntryId = typeof V08OsAdapterManualArtifactGateEntryIdSchema.Type;
export type V08OsAdapterManualArtifactGateReference = typeof V08OsAdapterManualArtifactGateReferenceSchema.Type;
export type V08OsAdapterManualArtifactGateRequirement = typeof V08OsAdapterManualArtifactGateRequirementSchema.Type;
export type V08OsAdapterManualArtifactGateClaimBoundary = typeof V08OsAdapterManualArtifactGateClaimBoundarySchema.Type;
export type V08OsAdapterManualArtifactGateFallback = typeof V08OsAdapterManualArtifactGateFallbackSchema.Type;
export type V08OsAdapterManualArtifactGateSurface = Infer<typeof V08OsAdapterManualArtifactGateSurfaceSchema>;
export type V08OsAdapterManualArtifactGateEvidenceKind = Infer<typeof V08OsAdapterManualArtifactGateEvidenceKindSchema>;
export type V08OsAdapterManualArtifactGateDecision = Infer<typeof V08OsAdapterManualArtifactGateDecisionSchema>;
export type V08OsAdapterManualArtifactGateOutcome = Infer<typeof V08OsAdapterManualArtifactGateOutcomeSchema>;
export type V08OsAdapterManualArtifactGateEntry = Infer<typeof V08OsAdapterManualArtifactGateEntrySchema>;
export type V08OsAdapterManualArtifactGateReadModel = Infer<typeof V08OsAdapterManualArtifactGateReadModelSchema>;

type V08OsAdapterManualArtifactGateEntryInput = {
  gateEntryId: string;
  surface: V08OsAdapterManualArtifactGateSurface;
  platform: ParentControlPlatform;
  capability: typeof ParentControlCapabilityNameSchema.Type;
  capabilityStatus: typeof ParentControlCapabilityStatusSchema.Type;
  evidenceKind: V08OsAdapterManualArtifactGateEvidenceKind;
  gateDecision: V08OsAdapterManualArtifactGateDecision;
  gateOutcome: V08OsAdapterManualArtifactGateOutcome;
  requiredArtifacts: readonly string[];
  linkedProofCommands: readonly string[];
  linkedProofArtifacts: readonly string[];
  hostCapabilityProbeRefs: readonly string[];
  claimBoundary: string;
  fallbackBehavior: string;
};

export const V08OsAdapterManualArtifactGateSurface = {
  WindowsBroadInstalledAppIdentity: V08OsAdapterManualArtifactGateSurfaceSchema.parse(
    'windows-broad-installed-app-identity'
  ),
  WindowsProcessPackageIdentity: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-process-package-identity'),
  WindowsOwnedProcessTerminate: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-owned-process-terminate'),
  WindowsParentCancelOverride: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-parent-cancel-override'),
  WindowsNetworkDomainFilterApplyRollback: V08OsAdapterManualArtifactGateSurfaceSchema.parse(
    'windows-network-domain-filter-apply-rollback'
  ),
  WindowsManagedBrowserExactUrl: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-managed-browser-exact-url'),
  WindowsUnmanagedExactTitlePageDownload: V08OsAdapterManualArtifactGateSurfaceSchema.parse(
    'windows-unmanaged-exact-title-page-download'
  ),
  WindowsRestartRecovery: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-restart-recovery'),
  WindowsAuditCustody: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-audit-custody'),
  WindowsServicePermission: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-service-permission'),
  WindowsPackageLifecycle: V08OsAdapterManualArtifactGateSurfaceSchema.parse('windows-package-lifecycle'),
  LinuxServicePackagePermission: V08OsAdapterManualArtifactGateSurfaceSchema.parse('linux-service-package-permission'),
  MacosServicePackagePermission: V08OsAdapterManualArtifactGateSurfaceSchema.parse('macos-service-package-permission'),
  AndroidUsageStats: V08OsAdapterManualArtifactGateSurfaceSchema.parse('android-usage-stats'),
  AndroidAccessibilityService: V08OsAdapterManualArtifactGateSurfaceSchema.parse('android-accessibility-service'),
  AndroidVpnDns: V08OsAdapterManualArtifactGateSurfaceSchema.parse('android-vpn-dns'),
  AndroidDeviceOwner: V08OsAdapterManualArtifactGateSurfaceSchema.parse('android-device-owner'),
  AndroidManagedProfile: V08OsAdapterManualArtifactGateSurfaceSchema.parse('android-managed-profile'),
  AndroidPackageLifecycle: V08OsAdapterManualArtifactGateSurfaceSchema.parse('android-package-lifecycle'),
  IosFamilyControls: V08OsAdapterManualArtifactGateSurfaceSchema.parse('ios-family-controls'),
  IosDeviceActivity: V08OsAdapterManualArtifactGateSurfaceSchema.parse('ios-device-activity'),
  IosScreenTime: V08OsAdapterManualArtifactGateSurfaceSchema.parse('ios-screen-time'),
  IosNetworkExtension: V08OsAdapterManualArtifactGateSurfaceSchema.parse('ios-network-extension'),
  IosBackgroundExecutionSigning: V08OsAdapterManualArtifactGateSurfaceSchema.parse('ios-background-execution-signing'),
  IosTestflightDeviceInstall: V08OsAdapterManualArtifactGateSurfaceSchema.parse('ios-testflight-device-install'),
} as const;

const documentedAt = '2026-05-30T21:40:00.000Z';

export const V08OsAdapterManualArtifactGateReadModel = V08OsAdapterManualArtifactGateReadModelSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'v0-8-os-adapter-manual-artifact-gates',
  generatedAt: documentedAt,
  sourceReadModelIds: [
    'v0-8-broad-os-adapter-proof',
    'v0-8-browser-domain-adapter-proof',
    'v0-8-windows-adapter-artifact-gate',
    'v0-8-windows-adapter-artifact-ingestion-proof',
  ],
  entries: [
    hostGate(
      'v0-8-manual-gate-windows-broad-installed-app-identity',
      V08OsAdapterManualArtifactGateSurface.WindowsBroadInstalledAppIdentity,
      'windows',
      ParentControlCapabilityName.AppBlocking,
      'app-identity',
      ['OS-approved installed app identity', 'same-identity apply result', 'rollback result', 'audit custody'],
      'Broad installed-app blocking stays manual-required until app identity, apply, rollback, and audit custody artifacts exist.',
      'Return manual-required instead of product-ready blocking when any broad app artifact is missing.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-process-package-identity',
      V08OsAdapterManualArtifactGateSurface.WindowsProcessPackageIdentity,
      'windows',
      ParentControlCapabilityName.AppBlocking,
      'process-identity',
      ['process executable identity', 'package identity or installer identity', 'child-device host evidence'],
      'Process and package identity are required before process observations can support broad app claims.',
      'Keep process-only or package-only evidence scoped to diagnostics until same-identity proof exists.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-owned-process-terminate',
      V08OsAdapterManualArtifactGateSurface.WindowsOwnedProcessTerminate,
      'windows',
      ParentControlCapabilityName.OwnedProcessTerminate,
      'process-identity',
      ['pid and expected process name', 'terminate result', 'post-terminate observation', 'audit custody'],
      'Owned-process terminate proof remains pid/name scoped and does not become broad installed-app blocking.',
      'Reject missing pid/name or mismatched identity and keep broad app support manual-required.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-parent-cancel-override',
      V08OsAdapterManualArtifactGateSurface.WindowsParentCancelOverride,
      'windows',
      ParentControlCapabilityName.AppTimeLimit,
      'parent-action',
      ['parent cancel request', 'override reason', 'resulting timer state', 'audit custody'],
      'Parent cancel and override are app-timer boundaries and do not prove bypass-resistant OS blocking.',
      'Return manual-required for bypass resistance until parent action, timer transition, and custody artifacts exist.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-network-domain-filter',
      V08OsAdapterManualArtifactGateSurface.WindowsNetworkDomainFilterApplyRollback,
      'windows',
      ParentControlCapabilityName.NetworkDomainBlocking,
      'network-domain-filter',
      ['network filter apply result', 'domain match evidence', 'filter rollback result', 'audit custody'],
      'Network/domain filtering requires host filter apply plus rollback evidence and is not proved by domain observation.',
      'Return manual-required until DNS, VPN, firewall, or filter artifacts prove apply and rollback.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-managed-browser-exact-url',
      V08OsAdapterManualArtifactGateSurface.WindowsManagedBrowserExactUrl,
      'windows',
      ParentControlCapabilityName.ManagedBrowserControl,
      'browser-evidence',
      ['managed active-tab evidence', 'exact URL apply result', 'browser policy rollback', 'audit custody'],
      'Managed browser launch or policy state is not exact active-tab URL enforcement.',
      'Return manual-required until active-tab URL, apply, rollback, and custody artifacts exist.'
    ),
    notClaimedGate(
      'v0-8-manual-gate-windows-unmanaged-exact-evidence',
      V08OsAdapterManualArtifactGateSurface.WindowsUnmanagedExactTitlePageDownload,
      'windows',
      ParentControlCapabilityName.UnmanagedBrowserDetection,
      'browser-evidence',
      ['explicit browser integration', 'active tab evidence', 'title/page/download evidence'],
      'Unmanaged browser exact URL, title, page, download, HTTPS content, and intent evidence are not claimed.',
      'Do not infer exact browser content from process names, command lines, or network/domain observations.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-restart-recovery',
      V08OsAdapterManualArtifactGateSurface.WindowsRestartRecovery,
      'windows',
      ParentControlCapabilityName.AppTimeLimit,
      'restart-recovery',
      ['pre-restart persisted state', 'post-restart recovered state', 'host uptime evidence', 'audit custody'],
      'Restart recovery must be proved with before/after state and cannot upgrade app/domain/browser blocking.',
      'Report manual-required when persisted state, recovered state, or custody evidence is missing.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-audit-custody',
      V08OsAdapterManualArtifactGateSurface.WindowsAuditCustody,
      'windows',
      ParentControlCapabilityName.LocalStorage,
      'audit-custody',
      ['append-only audit event', 'source command id', 'artifact hash or path', 'retention/deletion state'],
      'Audit custody requires source command and artifact traceability before any manual review gate can upgrade.',
      'Return manual-required when audit identity, artifact hash, or retention/deletion state is missing.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-service-permission',
      V08OsAdapterManualArtifactGateSurface.WindowsServicePermission,
      'windows',
      ParentControlCapabilityName.HeadlessAgentService,
      'service-permission',
      ['service install state', 'permission state', 'start mode', 'operator consent evidence'],
      'Service permission and install state must be explicit before privileged adapter behavior can be reviewed.',
      'Return manual-required or unavailable when service permission is absent, unknown, or revoked.'
    ),
    hostGate(
      'v0-8-manual-gate-windows-package-lifecycle',
      V08OsAdapterManualArtifactGateSurface.WindowsPackageLifecycle,
      'windows',
      ParentControlCapabilityName.SignedAutoUpdate,
      'package-lifecycle',
      ['installer identity', 'version state', 'update channel', 'rollback or uninstall evidence'],
      'Package lifecycle proof is required before broad privileged adapter behavior can be product-ready.',
      'Keep update, rollback, and uninstall claims manual-required until package artifacts exist.'
    ),
    unavailableGate(
      'v0-8-manual-gate-linux-service-package-permission',
      V08OsAdapterManualArtifactGateSurface.LinuxServicePackagePermission,
      'linux',
      ParentControlCapabilityName.AppBlocking,
      'service-permission',
      ['Linux service manager proof', 'Linux package identity', 'permission prompt evidence', 'rollback evidence'],
      'Linux OS adapter artifact gates are unavailable in this proof and cannot inherit Windows artifacts.',
      'Report unavailable until Linux-specific service, package, permission, apply, rollback, and audit proof exists.'
    ),
    hostGate(
      'v0-8-manual-gate-macos-service-package-permission',
      V08OsAdapterManualArtifactGateSurface.MacosServicePackagePermission,
      'macos',
      ParentControlCapabilityName.AppBlocking,
      'service-permission',
      ['macOS permission grant', 'bundle identity', 'launch daemon or helper evidence', 'rollback evidence'],
      'macOS OS adapter artifact gates remain manual-required until platform permission and helper artifacts exist.',
      'Do not borrow Windows artifacts for macOS support; require macOS-specific permission and rollback proof.'
    ),
    mobileGate(
      'v0-8-manual-gate-android-usage-stats',
      V08OsAdapterManualArtifactGateSurface.AndroidUsageStats,
      'android',
      ParentControlCapabilityName.UsageStats,
      'mobile-privilege',
      ['UsageStats permission grant', 'package usage sample', 'foreground/background state', 'device artifact'],
      'Android UsageStats support is manual-required and needs emulator or physical-device artifacts.',
      'Keep Android usage claims manual-required until real permission and device evidence exists.'
    ),
    mobileGate(
      'v0-8-manual-gate-android-accessibility',
      V08OsAdapterManualArtifactGateSurface.AndroidAccessibilityService,
      'android',
      ParentControlCapabilityName.AccessibilityService,
      'mobile-privilege',
      ['Accessibility service declaration', 'user enablement state', 'event sample', 'device artifact'],
      'Android accessibility behavior must be separately approved and device-proved before support upgrades.',
      'Return manual-required until service enablement and event custody artifacts exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-android-vpn-dns',
      V08OsAdapterManualArtifactGateSurface.AndroidVpnDns,
      'android',
      ParentControlCapabilityName.VpnDnsFiltering,
      'mobile-privilege',
      ['VPN or DNS permission prompt', 'filter apply result', 'filter rollback result', 'device artifact'],
      'Android VPN/DNS filtering requires permission, apply, rollback, and real-device proof.',
      'Keep network/domain claims manual-required until Android VPN/DNS artifacts exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-android-device-owner',
      V08OsAdapterManualArtifactGateSurface.AndroidDeviceOwner,
      'android',
      ParentControlCapabilityName.DeviceOwnerPolicy,
      'mobile-privilege',
      ['device-owner provisioning', 'policy apply result', 'policy rollback result', 'device artifact'],
      'Android device-owner policy cannot be claimed without real provisioning and rollback artifacts.',
      'Return manual-required until device-owner setup and policy custody evidence exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-android-managed-profile',
      V08OsAdapterManualArtifactGateSurface.AndroidManagedProfile,
      'android',
      ParentControlCapabilityName.ManagedProfile,
      'mobile-privilege',
      ['managed-profile provisioning', 'profile policy result', 'profile rollback result', 'device artifact'],
      'Android managed-profile behavior is manual-required until profile provisioning artifacts exist.',
      'Keep managed-profile claims manual-required without profile setup, policy, rollback, and custody evidence.'
    ),
    mobileGate(
      'v0-8-manual-gate-android-package-lifecycle',
      V08OsAdapterManualArtifactGateSurface.AndroidPackageLifecycle,
      'android',
      ParentControlCapabilityName.PackageLifecycle,
      'package-lifecycle',
      ['package id', 'install or update result', 'foreground service state', 'device artifact'],
      'Android package lifecycle must be proved before child-device adapter behavior can be upgraded.',
      'Return manual-required until package install/update and foreground-service artifacts exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-ios-family-controls',
      V08OsAdapterManualArtifactGateSurface.IosFamilyControls,
      'ios',
      ParentControlCapabilityName.FamilyControlsEntitlement,
      'mobile-privilege',
      ['Family Controls entitlement', 'authorization state', 'selection apply result', 'device artifact'],
      'iOS Family Controls behavior requires entitlement, authorization, and device evidence.',
      'Keep iOS control manual-required until Apple entitlement and device artifacts exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-ios-device-activity',
      V08OsAdapterManualArtifactGateSurface.IosDeviceActivity,
      'ios',
      ParentControlCapabilityName.DeviceActivity,
      'mobile-privilege',
      ['DeviceActivity entitlement', 'monitor schedule result', 'event delivery result', 'device artifact'],
      'iOS DeviceActivity support is manual-required until entitlement and device schedule artifacts exist.',
      'Return manual-required without real DeviceActivity schedule and delivery proof.'
    ),
    mobileGate(
      'v0-8-manual-gate-ios-screen-time',
      V08OsAdapterManualArtifactGateSurface.IosScreenTime,
      'ios',
      ParentControlCapabilityName.ScreenTimeApi,
      'mobile-privilege',
      ['Screen Time authorization', 'shield apply result', 'shield rollback result', 'device artifact'],
      'iOS Screen Time behavior requires authorization, apply, rollback, and device proof.',
      'Keep Screen Time claims manual-required until real Apple API artifacts exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-ios-network-extension',
      V08OsAdapterManualArtifactGateSurface.IosNetworkExtension,
      'ios',
      ParentControlCapabilityName.NetworkExtension,
      'mobile-privilege',
      ['Network Extension entitlement', 'filter apply result', 'filter rollback result', 'device artifact'],
      'iOS Network Extension support is manual-required and cannot be inferred from desktop network proof.',
      'Return manual-required until entitlement, apply, rollback, and device artifacts exist.'
    ),
    mobileGate(
      'v0-8-manual-gate-ios-background-execution-signing',
      V08OsAdapterManualArtifactGateSurface.IosBackgroundExecutionSigning,
      'ios',
      ParentControlCapabilityName.SigningEntitlements,
      'mobile-distribution',
      ['background mode entitlement', 'signing team artifact', 'provisioning profile', 'device artifact'],
      'iOS background execution and signing remain manual-required until Apple signing artifacts exist.',
      'Keep background and signing claims manual-required without provisioning and device evidence.'
    ),
    mobileGate(
      'v0-8-manual-gate-ios-testflight-device-install',
      V08OsAdapterManualArtifactGateSurface.IosTestflightDeviceInstall,
      'ios',
      ParentControlCapabilityName.TestflightDistribution,
      'mobile-distribution',
      ['TestFlight build', 'install result', 'entitlement verification', 'device artifact'],
      'iOS TestFlight or device install proof is required before mobile package readiness can upgrade.',
      'Return manual-required until TestFlight/device install and entitlement artifacts exist.'
    ),
  ],
});

function hostGate(
  gateEntryId: string,
  surface: V08OsAdapterManualArtifactGateSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08OsAdapterManualArtifactGateEvidenceKind,
  requiredArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08OsAdapterManualArtifactGateEntry {
  return gateEntry({
    gateEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind,
    gateDecision: 'requires-host-artifacts',
    gateOutcome: 'manual-required',
    requiredArtifacts,
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    hostCapabilityProbeRefs: hostCapabilityProbeRefsForPlatform(platform),
    claimBoundary,
    fallbackBehavior,
  });
}

function mobileGate(
  gateEntryId: string,
  surface: V08OsAdapterManualArtifactGateSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08OsAdapterManualArtifactGateEvidenceKind,
  requiredArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08OsAdapterManualArtifactGateEntry {
  return gateEntry({
    gateEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.ManualRequired,
    evidenceKind,
    gateDecision: 'requires-mobile-artifacts',
    gateOutcome: 'manual-required',
    requiredArtifacts,
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    hostCapabilityProbeRefs: hostCapabilityProbeRefsForPlatform(platform),
    claimBoundary,
    fallbackBehavior,
  });
}

function notClaimedGate(
  gateEntryId: string,
  surface: V08OsAdapterManualArtifactGateSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08OsAdapterManualArtifactGateEvidenceKind,
  requiredArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08OsAdapterManualArtifactGateEntry {
  return gateEntry({
    gateEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.NotImplemented,
    evidenceKind,
    gateDecision: 'unsupported-surface',
    gateOutcome: 'not-claimed',
    requiredArtifacts,
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    hostCapabilityProbeRefs: hostCapabilityProbeRefsForPlatform(platform),
    claimBoundary,
    fallbackBehavior,
  });
}

function unavailableGate(
  gateEntryId: string,
  surface: V08OsAdapterManualArtifactGateSurface,
  platform: ParentControlPlatform,
  capability: typeof ParentControlCapabilityNameSchema.Type,
  evidenceKind: V08OsAdapterManualArtifactGateEvidenceKind,
  requiredArtifacts: readonly string[],
  claimBoundary: string,
  fallbackBehavior: string
): V08OsAdapterManualArtifactGateEntry {
  return gateEntry({
    gateEntryId,
    surface,
    platform,
    capability,
    capabilityStatus: ParentControlCapabilityStatus.Unavailable,
    evidenceKind,
    gateDecision: 'adapter-unavailable',
    gateOutcome: 'unavailable',
    requiredArtifacts,
    linkedProofCommands: [],
    linkedProofArtifacts: [],
    hostCapabilityProbeRefs: hostCapabilityProbeRefsForPlatform(platform),
    claimBoundary,
    fallbackBehavior,
  });
}

function gateEntry(input: V08OsAdapterManualArtifactGateEntryInput): V08OsAdapterManualArtifactGateEntry {
  return V08OsAdapterManualArtifactGateEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    gateEntryId: input.gateEntryId,
    surface: input.surface,
    platform: input.platform,
    capability: input.capability,
    capabilityStatus: input.capabilityStatus,
    evidenceKind: input.evidenceKind,
    gateDecision: input.gateDecision,
    gateOutcome: input.gateOutcome,
    requiredArtifacts: [...input.requiredArtifacts],
    linkedProofCommands: [...input.linkedProofCommands],
    linkedProofArtifacts: [...input.linkedProofArtifacts],
    hostCapabilityProbeRefs: [...input.hostCapabilityProbeRefs],
    claimBoundary: input.claimBoundary,
    fallbackBehavior: input.fallbackBehavior,
    productReadyBlockingClaimed: false,
    broadInstalledAppBlockingClaimed: false,
    networkDomainBlockingClaimed: false,
    managedBrowserExactUrlClaimed: false,
    unmanagedBrowserExactEvidenceClaimed: false,
    unsupportedPlatformClaimed: false,
    mobilePrivilegeClaimed: false,
    lastCheckedAt: documentedAt,
  });
}

function hostCapabilityProbeRefsForPlatform(platform: ParentControlPlatform): readonly string[] {
  if (platform === 'windows') {
    return ['windows-host-local-probe-ref'];
  }
  if (platform === 'linux') {
    return ['linux-wsl-path-probe-ref', 'linux-docker-path-probe-ref'];
  }
  if (platform === 'android') {
    return ['android-adb-path-probe-ref', 'android-adb-sdk-probe-ref'];
  }
  return [];
}

export const decodeV08OsAdapterManualArtifactGateEntry = Schema.decodeUnknownSync(
  V08OsAdapterManualArtifactGateEntrySchema
);
export const decodeV08OsAdapterManualArtifactGateReadModel = Schema.decodeUnknownSync(
  V08OsAdapterManualArtifactGateReadModelSchema
);
