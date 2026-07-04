import { AppControlCatalogSettingSeedsA } from './app-control-catalog-setting-seeds-a';
import { AppControlCatalogSettingSeedsB } from './app-control-catalog-setting-seeds-b';
import { AppControlCatalogSettingSeedsC } from './app-control-catalog-setting-seeds-c';
export type AppControlCatalogSettingSeed = readonly [
  sectionId: string,
  sectionTitle: string,
  policyLane: string,
  groupId: string,
  groupTitle: string,
  settingId: string,
  controlKind: string,
  uiQuestionText: string,
  writesTo: string,
  effectStatus: string,
  runtimeOwner: string,
  capabilityState: string,
  capabilityRequirement: string | null,
  proofRequirement: string | null,
  unsafeOrUnsupportedFallback: string | null,
  options: readonly AppControlCatalogOptionSeed[],
  defaultValue: AppControlCatalogDefaultValue,
];

export type AppControlGuideCatalogSettingSeed = readonly [
  sectionId: string,
  sectionTitle: string,
  sectionOrder: number,
  groupId: string,
  groupTitle: string,
  groupOrder: number,
  settingId: string,
  sourceOrder: number,
  sourceLine: number,
  sourceText: string,
];

export type AppControlCatalogOptionSeed =
  | string
  | {
      readonly value: string;
      readonly label: string;
      readonly meaning?: string;
    };

export type AppControlCatalogDefaultValue = string | number | boolean | readonly string[] | null;

export type AppControlCapabilitySeed = readonly [
  capabilityId: string,
  state: string,
  proof: string,
  source: string,
  affectsSettings: readonly string[],
];

export const AppControlCatalogSourceDocuments = [
  'docs/app-control-capability-guide.md',
  'docs/app-control-schema-proposal.md',
] as const;

export const AppControlTargetScopeSeeds = ['family', 'per-child', 'per-device', 'per-platform', 'per-app'] as const;

export const AppControlEffectModeSeeds = [
  'off',
  'observe',
  'dry-run',
  'warn',
  'notify',
  'ask',
  'limit',
  'block',
  'enforce',
  'audit-only',
] as const;

export const AppControlCatalogSettingSeeds: readonly AppControlCatalogSettingSeed[] = [
  ...AppControlCatalogSettingSeedsA,
  ...AppControlCatalogSettingSeedsB,
  ...AppControlCatalogSettingSeedsC,
];

export const AppControlCapabilitySeeds: readonly AppControlCapabilitySeed[] = [
  [
    'windows-app-inventory',
    'available',
    'runtime-read-model-required',
    'os-installed-apps-and-package-query',
    ['inventory.mode', 'inventory.sources', 'reports.visibleFields'],
  ],
  [
    'windows-process-observation',
    'available',
    'runtime-adapter-proof-required',
    'process-snapshot-and-process-events',
    ['evidence.runtimeSources', 'rules.allowedTargetTypes'],
  ],
  [
    'windows-foreground-window',
    'available',
    'runtime-adapter-proof-required',
    'foreground-window-observation',
    ['evidence.durationMode', 'budgets.enabled', 'budgets.defaultDailyMinutes'],
  ],
  [
    'windows-owned-process-terminate',
    'available',
    'service-proof-required-before-product-claim',
    'owned-process-termination-adapter',
    ['enforcement.allowedActions', 'budgets.whenExhausted'],
  ],
  [
    'windows-target-process-terminate',
    'manual-required',
    'real-host-adapter-proof-required',
    'target-process-termination-adapter',
    ['enforcement.allowedActions', 'rules.defaultUnknownRule'],
  ],
  [
    'windows-broad-app-blocking',
    'manual-required',
    'applocker-wdac-or-equivalent-proof-required',
    'application-control-policy',
    ['app.defaultPosture', 'enforcement.allowedActions'],
  ],
  [
    'android-package-lifecycle',
    'manual-required',
    'device-owner-profile-owner-or-mdm-proof-required',
    'device-policy-manager',
    ['lifecycle.mode', 'lifecycle.allowedOperations', 'enforcement.allowedActions'],
  ],
  [
    'ios-screen-time-shielding',
    'manual-required',
    'family-controls-managed-settings-device-activity-entitlement-proof-required',
    'screen-time-frameworks',
    ['enforcement.allowedActions', 'budgets.whenExhausted', 'rules.allowedTargetTypes'],
  ],
  [
    'macos-managed-app-control',
    'manual-required',
    'mdm-system-extension-or-approved-api-proof-required',
    'macos-managed-device-boundary',
    ['lifecycle.mode', 'enforcement.allowedActions'],
  ],
  [
    'linux-desktop-app-control',
    'manual-required',
    'target-distro-desktop-adapter-proof-required',
    'desktop-entry-package-process-policy',
    ['inventory.sources', 'enforcement.allowedActions'],
  ],
] as const;