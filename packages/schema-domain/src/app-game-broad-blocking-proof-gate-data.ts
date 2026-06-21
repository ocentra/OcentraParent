import {
  type AppGamePlatformAction,
  type AppGamePlatformAuthorityTier,
  type AppGamePlatformSetupState,
} from './app-game-control-platform-authority';
import type { EnforcementCapabilityStateSchema, EnforcementModeSchema } from '@ocentra-parent/schema-domain/enforcement';
import {
  type AppGameBroadBlockingAdapterDispatchState,
  type AppGameBroadBlockingAuditState,
  type AppGameBroadBlockingGate,
  type AppGameBroadBlockingGateOutcome,
  AppGameBroadBlockingGateMatrixSchema,
  AppGameBroadBlockingGateSchema,
  type AppGameBroadBlockingProofKind,
  type AppGameBroadBlockingRollbackState,
} from './app-game-broad-blocking-proof-gates';
import {
  type ParentPlatform,
  ParentContractSchemaVersion,
  ParentPlatform as ParentPlatformValue,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

type AppGameBroadBlockingGateInput = Omit<
  AppGameBroadBlockingGateDraft,
  | 'outcomeState'
  | 'adapterDispatchState'
  | 'capabilityState'
  | 'supportedModes'
  | 'canCallAdapter'
  | 'rollbackState'
  | 'auditState'
  | 'proofReferences'
  | 'broadBlockingClaimed'
> & {
  sourceGateRefs?: readonly string[];
};

type AppGameBroadBlockingGateDraft = {
  gateId: string;
  platform: ParentPlatform;
  action: AppGamePlatformAction;
  outcomeState: AppGameBroadBlockingGateOutcome;
  adapterDispatchState: AppGameBroadBlockingAdapterDispatchState;
  authorityTier: AppGamePlatformAuthorityTier;
  setupState: AppGamePlatformSetupState;
  capabilityState: typeof EnforcementCapabilityStateSchema.Type;
  supportedModes: ReadonlyArray<typeof EnforcementModeSchema.Type>;
  canCallAdapter: boolean;
  rollbackState: AppGameBroadBlockingRollbackState;
  auditState: AppGameBroadBlockingAuditState;
  parentVisibleReason: string;
  requiredProofKinds: ReadonlyArray<AppGameBroadBlockingProofKind>;
  proofReferences: ReadonlyArray<{
    proofKind: AppGameBroadBlockingProofKind;
    artifactRef: string;
  }>;
  sourceGateRefs?: readonly string[];
  broadBlockingClaimed: boolean;
};

const GeneratedAt = '2026-06-03T10:15:00.000Z';

export const AppGameBroadBlockingGateMatrix = AppGameBroadBlockingGateMatrixSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  matrixId: 'app-game-broad-blocking-proof-gates',
  generatedAt: GeneratedAt,
  gates: [
    manualGate({
      gateId: 'windows-block-launch-applocker-app-control-manual-required',
      platform: ParentPlatformValue.Windows,
      action: 'block-launch',
      authorityTier: 'manual-required',
      setupState: 'manual-required',
      parentVisibleReason:
        'Block launch stays manual-required until AppLocker or App Control apply, rollback, audit, and system-app allowlist proof exists.',
      requiredProofKinds: [
        'setup-proof',
        'authority-tier-proof',
        'rollback-proof',
        'audit-state-proof',
        'windows-applocker-proof',
        'windows-app-control-proof',
        'windows-system-app-allowlist-proof',
      ],
    }),
    manualGate({
      gateId: 'windows-applocker-audit-is-not-enforce-proof',
      platform: ParentPlatformValue.Windows,
      action: 'block-launch',
      authorityTier: 'manual-required',
      setupState: 'manual-required',
      parentVisibleReason:
        'AppLocker audit evidence is not AppLocker enforce proof; blocking stays manual-required until enforce apply, rollback, and audit custody proof exists.',
      requiredProofKinds: [
        'setup-proof',
        'authority-tier-proof',
        'rollback-proof',
        'audit-state-proof',
        'windows-applocker-audit-proof',
        'windows-applocker-proof',
      ],
    }),
    manualGate({
      gateId: 'macos-hard-block-endpoint-mdm-manual-required',
      platform: ParentPlatformValue.Macos,
      action: 'block-launch',
      authorityTier: 'manual-required',
      setupState: 'system-extension-required',
      parentVisibleReason:
        'macOS hard blocking needs MDM, Endpoint Security, or System Extension setup plus rollback and audit proof.',
      requiredProofKinds: [
        'setup-proof',
        'authority-tier-proof',
        'rollback-proof',
        'audit-state-proof',
        'macos-mdm-profile-proof',
        'macos-endpoint-security-proof',
        'macos-system-extension-proof',
      ],
    }),
    unavailableGate({
      gateId: 'linux-hard-block-mechanism-unavailable',
      platform: ParentPlatformValue.Linux,
      action: 'block-launch',
      authorityTier: 'manual-required',
      setupState: 'admin-or-root-required',
      parentVisibleReason:
        'Linux blocking is unavailable without a named mechanism, distro, session, rollback, and audit proof.',
      requiredProofKinds: [
        'setup-proof',
        'authority-tier-proof',
        'rollback-proof',
        'audit-state-proof',
        'linux-mechanism-proof',
        'linux-distro-proof',
        'linux-session-proof',
      ],
    }),
    manualGate({
      gateId: 'android-normal-mode-hide-suspend-manual-required',
      platform: ParentPlatformValue.Android,
      action: 'suspend-app',
      authorityTier: 'manual-required',
      setupState: 'device-owner-required',
      parentVisibleReason:
        'Android normal mode cannot hide or suspend packages; Device Owner or Profile Owner proof is required before adapter dispatch.',
      requiredProofKinds: [
        'setup-proof',
        'authority-tier-proof',
        'rollback-proof',
        'audit-state-proof',
        'android-device-owner-proof',
        'android-profile-owner-proof',
      ],
    }),
    manualGate({
      gateId: 'ios-managedsettings-shield-manual-required',
      platform: ParentPlatformValue.Ios,
      action: 'shield-app',
      authorityTier: 'manual-required',
      setupState: 'supervision-required',
      parentVisibleReason:
        'iOS app shielding needs FamilyControls authorization and ManagedSettings proof; process scanning or killing is not claimed.',
      requiredProofKinds: [
        'setup-proof',
        'authority-tier-proof',
        'rollback-proof',
        'audit-state-proof',
        'ios-family-controls-proof',
        'ios-managed-settings-proof',
      ],
    }),
    notClaimedGate({
      gateId: 'ios-process-kill-not-claimed',
      platform: ParentPlatformValue.Ios,
      action: 'terminate-process',
      authorityTier: 'not-claimed',
      setupState: 'not-claimed',
      parentVisibleReason:
        'iOS process enumeration and process killing are not claimed; iOS control must use Screen Time, ManagedSettings, MDM, or App Lock proof.',
      requiredProofKinds: ['ios-family-controls-proof', 'ios-managed-settings-proof', 'ios-supervised-mdm-proof'],
    }),
  ],
});

function manualGate(input: AppGameBroadBlockingGateInput): AppGameBroadBlockingGate {
  return gate({
    outcomeState: 'manual-required',
    adapterDispatchState: 'blocked-before-adapter',
    capabilityState: 'manual-required',
    rollbackState: 'rollback-required',
    auditState: 'audit-required',
    canCallAdapter: false,
    supportedModes: [],
    proofReferences: [],
    broadBlockingClaimed: false,
    ...input,
  });
}

function unavailableGate(input: AppGameBroadBlockingGateInput): AppGameBroadBlockingGate {
  return gate({
    outcomeState: 'unavailable',
    adapterDispatchState: 'adapter-unavailable',
    capabilityState: 'unavailable',
    rollbackState: 'rollback-required',
    auditState: 'audit-required',
    canCallAdapter: false,
    supportedModes: [],
    proofReferences: [],
    broadBlockingClaimed: false,
    ...input,
  });
}

function notClaimedGate(input: AppGameBroadBlockingGateInput): AppGameBroadBlockingGate {
  return gate({
    outcomeState: 'not-claimed',
    adapterDispatchState: 'not-dispatched',
    capabilityState: 'manual-required',
    rollbackState: 'not-applicable',
    auditState: 'not-applicable',
    canCallAdapter: false,
    supportedModes: [],
    proofReferences: [],
    broadBlockingClaimed: false,
    ...input,
  });
}

function gate(input: AppGameBroadBlockingGateDraft): AppGameBroadBlockingGate {
  return AppGameBroadBlockingGateSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    sourceGateRefs: [
      'v0-8-broad-os-adapter-proof',
      'v0-8-broad-os-adapter-runtime-proof',
      'v0-8-os-adapter-manual-artifact-gates',
      ...(input.sourceGateRefs ?? []),
    ],
    lastCheckedAt: GeneratedAt,
    ...input,
  });
}
