export type AppGamePlatformActionRuleInput =
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

export type AppGamePlatformAuthorityTierRuleInput =
  | 'observe-only'
  | 'user-approved-helper'
  | 'accessibility-assisted'
  | 'managed-profile'
  | 'device-owner'
  | 'mdm-enrolled'
  | 'supervised-device'
  | 'system-extension'
  | 'root-or-admin-service'
  | 'kiosk-or-single-app'
  | 'manual-required'
  | 'not-claimed';

export type AppGamePlatformProofKindRuleInput =
  | 'contract-proof'
  | 'manual-host-proof'
  | 'rollback-proof'
  | 'windows-applocker-proof'
  | 'windows-app-control-proof'
  | 'device-owner-proof'
  | 'profile-owner-proof'
  | 'family-controls-authorization'
  | 'managed-settings-shield-proof'
  | 'mdm-profile-proof'
  | 'endpoint-security-proof'
  | 'system-extension-proof'
  | 'linux-mechanism-proof'
  | 'linux-distro-proof'
  | 'linux-session-proof'
  | 'accessibility-permission-proof'
  | 'usage-stats-proof';

export interface AppGamePlatformAuthorityRowRuleInput {
  readonly platform: 'windows' | 'linux' | 'macos' | 'android' | 'ios';
  readonly action: AppGamePlatformActionRuleInput;
  readonly authorityTier: AppGamePlatformAuthorityTierRuleInput;
  readonly capabilityState: 'supported' | 'unavailable' | 'degraded' | 'dry-run' | 'observe-only' | 'manual-required';
  readonly canExecuteAdapter: boolean;
  readonly supportedModes: readonly unknown[];
  readonly proofReferences: readonly { readonly proofKind: AppGamePlatformProofKindRuleInput }[];
  readonly parentVisibleLimitation: unknown;
  readonly linuxMechanism: unknown | null;
  readonly linuxDistro: unknown | null;
  readonly linuxSession: unknown | null;
}
