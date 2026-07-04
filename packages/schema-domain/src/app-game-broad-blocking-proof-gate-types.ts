export type AppGameBroadBlockingPlatformRuleInput = 'windows' | 'linux' | 'macos' | 'android' | 'ios';

export type AppGameBroadBlockingActionRuleInput =
  | 'inventory'
  | 'runtime'
  | 'foreground'
  | 'warn'
  | 'ask-parent'
  | 'time-limit'
  | 'terminate-process'
  | 'hide-app'
  | 'suspend-app'
  | 'shield-app'
  | 'block-launch'
  | 'enforce-allowlist';

export type AppGameBroadBlockingProofKindRuleInput =
  | 'setup-proof'
  | 'authority-tier-proof'
  | 'rollback-proof'
  | 'audit-state-proof'
  | 'windows-applocker-proof'
  | 'windows-applocker-audit-proof'
  | 'windows-app-control-proof'
  | 'windows-system-app-allowlist-proof'
  | 'macos-mdm-profile-proof'
  | 'macos-endpoint-security-proof'
  | 'macos-system-extension-proof'
  | 'linux-mechanism-proof'
  | 'linux-distro-proof'
  | 'linux-session-proof'
  | 'android-device-owner-proof'
  | 'android-profile-owner-proof'
  | 'ios-family-controls-proof'
  | 'ios-managed-settings-proof'
  | 'ios-supervised-mdm-proof';

export interface AppGameBroadBlockingGateRuleInput {
  readonly platform: AppGameBroadBlockingPlatformRuleInput;
  readonly action: AppGameBroadBlockingActionRuleInput;
  readonly outcomeState: 'manual-required' | 'unavailable' | 'not-claimed' | 'supported';
  readonly adapterDispatchState:
    | 'blocked-before-adapter'
    | 'adapter-unavailable'
    | 'not-dispatched'
    | 'dispatch-eligible';
  readonly supportedModes: readonly unknown[];
  readonly canCallAdapter: boolean;
  readonly rollbackState: 'rollback-required' | 'rollback-proof-attached' | 'not-applicable';
  readonly auditState: 'audit-required' | 'audit-proof-attached' | 'not-applicable';
  readonly parentVisibleReason: unknown;
  readonly requiredProofKinds: readonly AppGameBroadBlockingProofKindRuleInput[];
  readonly proofReferences: readonly { readonly proofKind: AppGameBroadBlockingProofKindRuleInput }[];
  readonly broadBlockingClaimed: boolean;
}
