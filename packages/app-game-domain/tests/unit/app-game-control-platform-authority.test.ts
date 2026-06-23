import { describe, expect, it } from 'vitest';
import {
  AppGamePlatformAction,
  AppGamePlatformAuthorityMatrixSchema,
  AppGamePlatformAuthorityRowSchema,
  AppGamePlatformAuthorityTier,
} from '@ocentra-parent/schema-domain/app-game-control-platform-authority';
import { EnforcementCapabilityState, EnforcementMode } from '@ocentra-parent/schema-domain/enforcement';
import { ParentContractSchemaVersion, ParentPlatform } from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-03T03:05:00Z';

const Proof = {
  rollback: { proofKind: 'rollback-proof', artifactRef: 'proof/rollback.md' },
  deviceOwner: { proofKind: 'device-owner-proof', artifactRef: 'proof/android-device-owner.md' },
  profileOwner: { proofKind: 'profile-owner-proof', artifactRef: 'proof/android-profile-owner.md' },
  familyControls: { proofKind: 'family-controls-authorization', artifactRef: 'proof/ios-family-controls.md' },
  managedSettings: { proofKind: 'managed-settings-shield-proof', artifactRef: 'proof/ios-managed-settings.md' },
  endpointSecurity: { proofKind: 'endpoint-security-proof', artifactRef: 'proof/macos-endpoint-security.md' },
  linuxMechanism: { proofKind: 'linux-mechanism-proof', artifactRef: 'proof/linux-cgroup.md' },
  linuxDistro: { proofKind: 'linux-distro-proof', artifactRef: 'proof/linux-distro.md' },
  linuxSession: { proofKind: 'linux-session-proof', artifactRef: 'proof/linux-session.md' },
  mdmProfile: { proofKind: 'mdm-profile-proof', artifactRef: 'proof/macos-mdm-profile.md' },
} as const;

const SupportedAndroidHideRow = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  rowId: 'android-hide-row',
  platform: ParentPlatform.Android,
  action: AppGamePlatformAction.HideApp,
  authorityTier: AppGamePlatformAuthorityTier.DeviceOwner,
  setupState: 'device-owner-required',
  proofState: 'runtime-proof-attached',
  capabilityState: EnforcementCapabilityState.Supported,
  parentVisibleState: 'managed-device-required',
  parentVisibleLimitation: 'Requires Android Device Owner provisioning and rollback proof.',
  canExecuteAdapter: true,
  supportedModes: [EnforcementMode.BlockProcess],
  proofReferences: [Proof.deviceOwner, Proof.rollback],
  proofNeededToClaim: ['device-owner-proof', 'rollback-proof'],
  linuxMechanism: null,
  linuxDistro: null,
  linuxSession: null,
  lastCheckedAt: Timestamp,
} as const;

const ManualWindowsBlockRow = {
  ...SupportedAndroidHideRow,
  rowId: 'windows-broad-block-row',
  platform: ParentPlatform.Windows,
  action: AppGamePlatformAction.BlockLaunch,
  authorityTier: AppGamePlatformAuthorityTier.ManualRequired,
  setupState: 'manual-required',
  proofState: 'manual-required',
  capabilityState: EnforcementCapabilityState.ManualRequired,
  parentVisibleState: 'manual-required',
  parentVisibleLimitation: 'Broad installed-app blocking needs AppLocker or App Control proof before execution.',
  canExecuteAdapter: false,
  supportedModes: [],
  proofReferences: [],
  proofNeededToClaim: ['windows-applocker-proof', 'windows-app-control-proof', 'rollback-proof'],
} as const;

const expectRowAcceptance = (row: unknown) => {
  expect(AppGamePlatformAuthorityRowSchema.safeParse(row).success).toBe(true);
};

const expectRowRejection = (row: unknown) => {
  expect(AppGamePlatformAuthorityRowSchema.safeParse(row).success).toBe(false);
};

const assertAcceptsProofedRowsInUniqueMatrix = () => {
  expectRowAcceptance(SupportedAndroidHideRow);
  expectRowAcceptance(ManualWindowsBlockRow);
  expect(
    AppGamePlatformAuthorityMatrixSchema.safeParse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      matrixId: 'app-game-platform-authority-matrix',
      rows: [SupportedAndroidHideRow, ManualWindowsBlockRow],
      generatedAt: Timestamp,
    }).success
  ).toBe(true);
};

const assertRejectsObserveOnlyExecution = () => {
  expectRowRejection({
    ...ManualWindowsBlockRow,
    rowId: 'observe-only-block-row',
    authorityTier: AppGamePlatformAuthorityTier.ObserveOnly,
    setupState: 'not-required',
    proofState: 'fixture-proof',
    capabilityState: EnforcementCapabilityState.Supported,
    parentVisibleState: 'observe-only',
    canExecuteAdapter: true,
    supportedModes: [EnforcementMode.BlockProcess],
    proofReferences: [Proof.rollback],
  });
};

const assertRejectsManualOrNotClaimedExecution = () => {
  expectRowRejection({
    ...ManualWindowsBlockRow,
    capabilityState: EnforcementCapabilityState.Supported,
    canExecuteAdapter: true,
    supportedModes: [EnforcementMode.BlockProcess],
  });
  expectRowRejection({
    ...ManualWindowsBlockRow,
    rowId: 'not-claimed-block-row',
    authorityTier: AppGamePlatformAuthorityTier.NotClaimed,
    proofState: 'not-claimed',
    capabilityState: EnforcementCapabilityState.Supported,
    parentVisibleState: 'not-claimed',
    canExecuteAdapter: true,
    supportedModes: [EnforcementMode.BlockProcess],
  });
};

const assertRequiresAndroidOwnerProof = () => {
  expectRowAcceptance({
    ...ManualWindowsBlockRow,
    rowId: 'android-normal-hide-row',
    platform: ParentPlatform.Android,
    action: AppGamePlatformAction.HideApp,
    parentVisibleLimitation: 'Android normal mode cannot hide packages without Device Owner or Profile Owner proof.',
    proofNeededToClaim: ['device-owner-proof', 'profile-owner-proof', 'rollback-proof'],
  });
  expectRowAcceptance({
    ...ManualWindowsBlockRow,
    rowId: 'android-normal-suspend-row',
    platform: ParentPlatform.Android,
    action: AppGamePlatformAction.SuspendApp,
    parentVisibleLimitation: 'Android normal mode cannot suspend packages without Device Owner or Profile Owner proof.',
    proofNeededToClaim: ['device-owner-proof', 'profile-owner-proof', 'rollback-proof'],
  });
  expectRowRejection({
    ...SupportedAndroidHideRow,
    proofReferences: [Proof.rollback],
  });
  expectRowAcceptance({
    ...SupportedAndroidHideRow,
    rowId: 'android-suspend-row',
    action: AppGamePlatformAction.SuspendApp,
    authorityTier: AppGamePlatformAuthorityTier.ManagedProfile,
    setupState: 'managed-profile-required',
    proofReferences: [Proof.profileOwner, Proof.rollback],
    proofNeededToClaim: ['profile-owner-proof', 'rollback-proof'],
  });
};

const assertRequiresIosShieldProof = () => {
  const iosShieldRow = {
    ...SupportedAndroidHideRow,
    rowId: 'ios-shield-row',
    platform: ParentPlatform.Ios,
    action: AppGamePlatformAction.ShieldApp,
    authorityTier: AppGamePlatformAuthorityTier.UserApprovedHelper,
    setupState: 'permission-required',
    parentVisibleState: 'permission-required',
    parentVisibleLimitation: 'Requires FamilyControls authorization and ManagedSettings shield proof.',
    proofReferences: [Proof.familyControls, Proof.managedSettings, Proof.rollback],
    proofNeededToClaim: ['family-controls-authorization', 'managed-settings-shield-proof', 'rollback-proof'],
  } as const;

  expectRowAcceptance(iosShieldRow);
  expectRowRejection({ ...iosShieldRow, proofReferences: [Proof.managedSettings, Proof.rollback] });
};

const assertRejectsIosProcessKillClaims = () => {
  expectRowAcceptance({
    ...ManualWindowsBlockRow,
    rowId: 'ios-process-kill-not-claimed-row',
    platform: ParentPlatform.Ios,
    action: AppGamePlatformAction.TerminateProcess,
    authorityTier: AppGamePlatformAuthorityTier.ManualRequired,
    setupState: 'manual-required',
    proofState: 'manual-required',
    capabilityState: EnforcementCapabilityState.ManualRequired,
    parentVisibleState: 'not-claimed',
    parentVisibleLimitation: 'iOS process scanning and process killing are not claimed for native app/game control.',
    proofNeededToClaim: ['mdm-profile-proof', 'rollback-proof'],
  });
  expectRowRejection({
    ...SupportedAndroidHideRow,
    rowId: 'ios-process-kill-claimed-row',
    platform: ParentPlatform.Ios,
    action: AppGamePlatformAction.TerminateProcess,
    authorityTier: AppGamePlatformAuthorityTier.SupervisedDevice,
    setupState: 'supervision-required',
    parentVisibleState: 'supervised-device-required',
    parentVisibleLimitation: 'iOS process killing cannot be claimed by generic rollback proof.',
    proofReferences: [Proof.mdmProfile, Proof.rollback],
    proofNeededToClaim: ['mdm-profile-proof', 'rollback-proof'],
  });
};

const assertRequiresMacosHardBlockProof = () => {
  const macosBlockRow = {
    ...ManualWindowsBlockRow,
    rowId: 'macos-block-row',
    platform: ParentPlatform.Macos,
    authorityTier: AppGamePlatformAuthorityTier.SystemExtension,
    setupState: 'system-extension-required',
    proofState: 'runtime-proof-attached',
    capabilityState: EnforcementCapabilityState.Supported,
    parentVisibleState: 'system-extension-required',
    parentVisibleLimitation: 'Requires Endpoint Security or MDM hard-block proof plus rollback proof.',
    canExecuteAdapter: true,
    supportedModes: [EnforcementMode.BlockProcess],
    proofReferences: [Proof.endpointSecurity, Proof.rollback],
    proofNeededToClaim: ['endpoint-security-proof', 'rollback-proof'],
  } as const;

  expectRowAcceptance(macosBlockRow);
  expectRowRejection({ ...macosBlockRow, authorityTier: AppGamePlatformAuthorityTier.UserApprovedHelper });
  expectRowRejection({ ...macosBlockRow, proofReferences: [Proof.rollback] });
};

const assertRequiresLinuxBlockProof = () => {
  const linuxBlockRow = {
    ...ManualWindowsBlockRow,
    rowId: 'linux-block-row',
    platform: ParentPlatform.Linux,
    authorityTier: AppGamePlatformAuthorityTier.RootOrAdminService,
    setupState: 'admin-or-root-required',
    proofState: 'runtime-proof-attached',
    capabilityState: EnforcementCapabilityState.Supported,
    parentVisibleState: 'admin-or-root-required',
    parentVisibleLimitation: 'Requires distro, desktop session, and cgroup/systemd proof.',
    canExecuteAdapter: true,
    supportedModes: [EnforcementMode.BlockProcess],
    proofReferences: [Proof.linuxMechanism, Proof.linuxDistro, Proof.linuxSession, Proof.rollback],
    proofNeededToClaim: ['linux-mechanism-proof', 'linux-distro-proof', 'linux-session-proof', 'rollback-proof'],
    linuxMechanism: 'systemd-cgroup-scope',
    linuxDistro: 'ubuntu-24.04',
    linuxSession: 'wayland-gnome',
  } as const;

  expectRowAcceptance(linuxBlockRow);
  expectRowRejection({ ...linuxBlockRow, linuxSession: null });
};

const assertRejectsBareUnsupportedCopyAndDuplicateRows = () => {
  expectRowRejection({ ...ManualWindowsBlockRow, parentVisibleLimitation: 'Unsupported' });
  expect(
    AppGamePlatformAuthorityMatrixSchema.safeParse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      matrixId: 'duplicate-app-game-platform-authority-matrix',
      rows: [ManualWindowsBlockRow, { ...ManualWindowsBlockRow, rowId: 'duplicate-windows-block-row' }],
      generatedAt: Timestamp,
    }).success
  ).toBe(false);
};

describe('app/game platform authority matrix contracts', () => {
  it(
    'accepts proofed hard-control and manual-required rows in one unique matrix',
    assertAcceptsProofedRowsInUniqueMatrix
  );
  it('rejects observe-only rows that try to execute hard-control adapters', assertRejectsObserveOnlyExecution);
  it(
    'rejects manual-required and not-claimed rows that try to execute adapters',
    assertRejectsManualOrNotClaimedExecution
  );
  it(
    'requires Android hide and suspend rows to carry Device Owner or Profile Owner proof',
    assertRequiresAndroidOwnerProof
  );
  it('requires iOS shield rows to carry FamilyControls and ManagedSettings proof', assertRequiresIosShieldProof);
  it('rejects iOS process scanning and killing claims', assertRejectsIosProcessKillClaims);
  it(
    'requires macOS hard block rows to carry MDM or Endpoint/System Extension proof',
    assertRequiresMacosHardBlockProof
  );
  it('requires Linux hard block rows to name mechanism, distro, and session proof', assertRequiresLinuxBlockProof);
  it(
    'rejects bare unsupported copy and duplicate platform action rows',
    assertRejectsBareUnsupportedCopyAndDuplicateRows
  );
});
