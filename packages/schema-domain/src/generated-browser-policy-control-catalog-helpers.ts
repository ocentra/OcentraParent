/* generated from crates/schema/src/browser_policy_control_catalog_ts.rs */

import {
  GeneratedBrowserControlFullCatalogCapabilityStateValues,
  GeneratedBrowserControlFullCatalogCardKindValues,
  GeneratedBrowserControlFullCatalogControlKindValues,
  GeneratedBrowserControlFullCatalogEffectStatusValues,
  GeneratedBrowserControlFullCatalogRuntimeOwnerValues,
  GeneratedBrowserControlWritesToPath,
} from './generated-browser-policy-control-catalog-contracts';

type BrowserControlFieldValueGenerated = string | number | boolean | readonly string[] | null;

type BrowserControlConditionGenerated = {
  readonly kind: string;
  readonly writesTo: string | null;
  readonly expectedValue: BrowserControlFieldValueGenerated | null;
  readonly defaultPosture: string | null;
};

type BrowserControlFieldOptionGenerated = {
  readonly optionId: string;
  readonly label: string;
  readonly value: BrowserControlFieldValueGenerated;
};

type BrowserControlFieldGenerated = {
  readonly fieldId: string;
  readonly controlKind: string;
  readonly writesTo?: string;
  readonly defaultValue: BrowserControlFieldValueGenerated;
  readonly options: readonly BrowserControlFieldOptionGenerated[];
};

type BrowserControlSectionGenerated = {
  readonly sectionId: string;
};

type BrowserControlManifestSectionGenerated = BrowserControlSectionGenerated & {
  readonly visibleWhen: readonly BrowserControlConditionGenerated[];
  readonly fields: readonly BrowserControlManifestFieldGenerated[];
};

type BrowserControlManifestGenerated = {
  readonly sections: readonly BrowserControlManifestSectionGenerated[];
};

type BrowserControlManifestFieldGenerated = {
  readonly fieldId: string;
  readonly writesTo: string;
};

type BrowserControlRuleGenerated = {
  readonly targetType: string | null;
  readonly targetValue: string | null;
  readonly target: { readonly kind: string } | null;
};

type BrowserControlPolicyGenerated = {
  readonly defaultPosture: string;
  readonly fallbackPosture: string | null;
  readonly managedBrowser: { readonly mode: string };
  readonly evidence: {
    readonly requiredProof: string;
    readonly proofFallback: string | null;
    readonly whenProofUnavailable: string;
  };
  readonly budgets: {
    readonly enabled: boolean;
    readonly defaultDailyMinutes: number | null;
  };
  readonly rules: {
    readonly allowedTargetTypes: readonly string[];
    readonly items: readonly BrowserControlRuleGenerated[];
    readonly entries: readonly BrowserControlRuleGenerated[];
  };
  readonly browserGames: {
    readonly educationalGameMode: string;
    readonly unknownGameMode: string;
    readonly unblockedPortalMode: string;
    readonly webglCanvasMode: string;
    readonly defaultDailyMinutes: number | null;
  };
};

type BrowserControlEffectivePolicyGenerated = {
  readonly defaultPosture: string;
  readonly fallbackPosture: string | null;
  readonly budgets: {
    readonly enabled: boolean;
    readonly defaultDailyMinutes: number | null;
  };
};

type BrowserControlFullCatalogOptionGenerated = {
  readonly optionId: string;
  readonly label: string;
  readonly value: string;
  readonly originalSourceText: string;
  readonly meaning: string | null;
  readonly defaultSelected: boolean;
};

type BrowserControlFullCatalogRuleGenerated = {
  readonly ruleId: string;
  readonly description: string;
};

type BrowserGamePolicyCompilerInputGenerated = {
  readonly targetKind: string;
  readonly analysisRefs: readonly string[];
  readonly parentRuleRefs: readonly string[];
  readonly compilerMode: string;
  readonly rawGamePayloadIncluded: boolean;
  readonly rawModelTextIncluded: boolean;
  readonly activityDomainObjectIncluded: boolean;
  readonly finalDecisionClaimedByInput: boolean;
  readonly runtimeGateClaimedByInput: boolean;
  readonly uiClaimedByInput: boolean;
  readonly enforcementClaimedByInput: boolean;
  readonly nativeGameControlClaimed: boolean;
  readonly cloudFrameAnalysisClaimed: boolean;
};

type BrowserGamePolicyDecisionCandidateGenerated = {
  readonly actionCandidate: string;
  readonly reasonCodes: readonly string[];
  readonly compilerMode: string;
  readonly analysisRefs: readonly string[];
  readonly fallbackUsed: boolean;
  readonly parentApprovalRequired: boolean;
  readonly finalPolicyDecisionClaimed: boolean;
  readonly runtimeGateExecutedClaimed: boolean;
  readonly uiRenderedClaimed: boolean;
  readonly enforcementClaimed: boolean;
  readonly nativeGameControlClaimed: boolean;
  readonly cloudFrameAnalysisClaimed: boolean;
  readonly rawGamePayloadStored: boolean;
  readonly rawModelTextUsed: boolean;
};

const BrowserGamePolicyAllowReasonsGenerated = ['educational-benefit-present', 'parent-rule-match'] as const;
const BrowserGamePolicyUnknownFallbackReasonsGenerated = [
  'missing-game-evidence',
  'degraded-analysis',
  'low-confidence',
  'unknown-evidence',
] as const;

const BrowserControlKnownWritesToPathsGenerated = new Set<string>(Object.values(GeneratedBrowserControlWritesToPath));

export const GeneratedBrowserControlManifestDefaults = {
  ManifestId: 'browser-control-authoring-v1',
  Section: {
    Management: 'browser-management',
    BrowserDiscovery: 'browser-discovery',
    ManagedBrowser: 'managed-browser',
    UnmanagedBrowser: 'unmanaged-browser',
    UrlTabEvidence: 'url-tab-evidence',
    WebRules: 'web-rules',
    Budgets: 'budgets',
    BrowserGames: 'browser-games',
    Downloads: 'downloads',
    Approvals: 'approvals',
    Reports: 'reports',
    Audit: 'audit',
  },
  Field: {
    Enabled: 'browser.enabled',
    ExecutionMode: 'browser.executionMode',
    DefaultPosture: 'browser.defaultPosture',
    ManagementMode: 'browser.managementMode',
    DiscoveryScanInstalledBrowsers: 'discovery.scanInstalledBrowsers',
    DiscoveryScanRunningBrowsers: 'discovery.scanRunningBrowsers',
    DiscoveryDetectUnmanagedBrowsers: 'discovery.detectUnmanagedBrowsers',
    ManagedBrowserMode: 'managedBrowser.mode',
    ManagedBrowserAllowedFamilies: 'managedBrowser.allowedFamilies',
    ManagedBrowserLaunchMode: 'managedBrowser.launchMode',
    ManagedBrowserProfileMode: 'managedBrowser.profileMode',
    ManagedBrowserBridgeRequirements: 'managedBrowser.bridgeRequirements',
    ManagedBrowserIntegrationMechanisms: 'managedBrowser.integrationMechanisms',
    ManagedBrowserPolicyWriterControls: 'managedBrowser.policyWriterControls',
    ManagedBrowserPolicyWriterFallback: 'managedBrowser.policyWriterFallback',
    UnmanagedBrowserMode: 'unmanagedBrowser.mode',
    UnmanagedBrowserGraceSeconds: 'unmanagedBrowser.graceSeconds',
    UnmanagedBrowserAllowRecoverLaunchUrl: 'unmanagedBrowser.allowRecoverLaunchUrl',
    UnmanagedBrowserClassificationTargets: 'unmanagedBrowser.classificationTargets',
    EvidenceUrlScope: 'evidence.urlScope',
    RequiredProof: 'evidence.requiredProof',
    WhenProofUnavailable: 'evidence.whenProofUnavailable',
    EvidenceNeverCollect: 'evidence.neverCollect',
    AllowedTargetTypes: 'rules.allowedTargetTypes',
    AllowedActions: 'rules.allowedActions',
    RuleItems: 'rules.items',
    UrlAllowList: 'rules.urlAllowList',
    UrlBlockList: 'rules.urlBlockList',
    BudgetsEnabled: 'budgets.enabled',
    DailyBudgetMinutes: 'budgets.defaultDailyMinutes',
    BudgetCountingMode: 'budgets.countingMode',
    BrowserGameEducationalMode: 'browserGames.educationalGameMode',
    BrowserGameUnknownMode: 'browserGames.unknownGameMode',
    BrowserGameCloudGamingApproval: 'browserGames.cloudGamingApproval',
    BrowserGamePurchaseAccountApproval: 'browserGames.purchaseAccountApproval',
    BrowserGameUnblockedPortalMode: 'browserGames.unblockedPortalMode',
    BrowserGameWebglCanvasMode: 'browserGames.webglCanvasMode',
    BrowserGameDailyBudgetMinutes: 'browserGames.defaultDailyMinutes',
    DownloadMode: 'downloads.mode',
    DownloadBlockedTypes: 'downloads.blockedTypes',
    ApprovalRequiredFor: 'approvals.requiredFor',
    ApprovalUnansweredDefault: 'approvals.unansweredDefault',
    ReportVisibleFields: 'reports.visibleFields',
    RetentionExactUrl: 'retention.exactUrl',
    CustodyAllowedUses: 'custody.allowedUses',
    AuditRequiredFields: 'audit.requiredFields',
  },
} as const;
export const GeneratedBrowserControlFullCatalogSourceDocument = 'docs/browser-policy-settings-catalog.md';
export const GeneratedBrowserControlFullCatalogSourceDocuments = [
  GeneratedBrowserControlFullCatalogSourceDocument,
  'docs/browser-control-schema-proposal.md',
  'docs/managed-unmanaged-browser.md',
  'docs/browser-control-coverage-matrix.md',
] as const;
export const GeneratedBrowserControlFullCatalogSidePanelCategory = 'browser' as const;
export const GeneratedBrowserControlFullCatalogTargetScopeOptions = optionsGenerated([
  'Family',
  'Per Child',
  'Per Device',
  'Per Platform',
  'Per Browser',
  'Per Network',
]);
export const GeneratedBrowserControlFullCatalogEffectModeOptions = optionsGenerated([
  'Off',
  'Observe',
  'Dry Run',
  'Warn',
  'Notify',
  'Ask',
  'Limit',
  'Block',
  'Enforce',
  'Audit Only',
]);
export const GeneratedBrowserControlFullCatalogTabOrder = [
  'enforcement',
  'rules',
  'schedule',
  'approvals',
  'evidence',
  'reports',
  'data',
  'audit',
  'ai',
  'setup',
  'platform',
] as const;
export const GeneratedBrowserControlFullCatalogTabTitles = {
  enforcement: 'Enforcement',
  rules: 'Rules',
  schedule: 'Schedule',
  approvals: 'Approvals',
  evidence: 'Evidence',
  reports: 'Reports',
  data: 'Data',
  audit: 'Audit',
  ai: 'AI',
  setup: 'Setup',
  platform: 'Platform',
} as const;

const enabledManifestConditionGenerated = equalsManifestConditionGenerated(
  GeneratedBrowserControlWritesToPath.Enabled,
  true
);

export const GeneratedBaselineBrowserControlAuthoringManifest = {
  schemaVersion: 'v0.6',
  manifestId: GeneratedBrowserControlManifestDefaults.ManifestId,
  title: 'Browser controls',
  sections: [
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.Management,
      'Browser management',
      'Top-level browser policy switch and default posture.',
      [],
      [
        manifestBooleanFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.Enabled,
          GeneratedBrowserControlWritesToPath.Enabled,
          'Enable browser management?',
          false,
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ExecutionMode,
          GeneratedBrowserControlWritesToPath.ExecutionMode,
          'How should browser-control decisions run?',
          'observe',
          ['observe', 'dry-run', 'warn-ask', 'enforce'],
          [enabledManifestConditionGenerated]
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DefaultPosture,
          GeneratedBrowserControlWritesToPath.DefaultPosture,
          'What should happen to browser activity?',
          'observe',
          ['allow', 'observe', 'warn', 'ask', 'limit', 'block'],
          [enabledManifestConditionGenerated]
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagementMode,
          GeneratedBrowserControlWritesToPath.ManagementMode,
          'How should browser management run on this device?',
          'local-child-agent',
          ['local-child-agent', 'lan-live', 'authoring-only', 'unavailable'],
          [enabledManifestConditionGenerated]
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.BrowserDiscovery,
      'Browser discovery',
      'Detect installed, running, and unmanaged browser activity before enforcement claims are made.',
      [enabledManifestConditionGenerated],
      [
        manifestBooleanFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DiscoveryScanInstalledBrowsers,
          GeneratedBrowserControlWritesToPath.DiscoveryScanInstalledBrowsers,
          'Scan installed browsers?',
          false,
          []
        ),
        manifestBooleanFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DiscoveryScanRunningBrowsers,
          GeneratedBrowserControlWritesToPath.DiscoveryScanRunningBrowsers,
          'Scan running browsers?',
          true,
          []
        ),
        manifestBooleanFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DiscoveryDetectUnmanagedBrowsers,
          GeneratedBrowserControlWritesToPath.DiscoveryDetectUnmanagedBrowsers,
          'Detect unmanaged browser use?',
          true,
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.ManagedBrowser,
      'Managed browser',
      'Configure the browser path that can support exact URL, tab, download, and request-level rules.',
      [enabledManifestConditionGenerated],
      [
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserMode,
          GeneratedBrowserControlWritesToPath.ManagedBrowserMode,
          'How should managed browser be used?',
          'available-for-exact-rules',
          ['disabled', 'available-for-exact-rules', 'required-for-exact-rules', 'required-for-all-browsing'],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserAllowedFamilies,
          GeneratedBrowserControlWritesToPath.ManagedBrowserAllowedFamilies,
          'Which managed browsers are allowed?',
          ['edge-stable', 'chrome-stable', 'chrome-for-testing'],
          [
            'edge-stable',
            'edge-beta',
            'edge-dev',
            'chrome-stable',
            'chrome-beta',
            'chrome-dev',
            'chrome-for-testing',
            'brave',
            'firefox',
            'safari-webkit',
            'owned-webview',
          ],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserLaunchMode,
          GeneratedBrowserControlWritesToPath.ManagedBrowserLaunchMode,
          'How should allowed browsing launch?',
          'ocentra-launcher',
          ['manual', 'ocentra-launcher', 'default-browser-route', 'managed-shell', 'admin-provisioned'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserProfileMode,
          GeneratedBrowserControlWritesToPath.ManagedBrowserProfileMode,
          'How should the managed profile behave?',
          'persistent-managed-profile',
          ['persistent-managed-profile', 'clear-on-schedule', 'clear-on-session-end', 'ephemeral'],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserBridgeRequirements,
          GeneratedBrowserControlWritesToPath.ManagedBrowserBridgeRequirements,
          'Which bridge security rules are required?',
          [
            'owned-profile',
            'loopback-only',
            'random-port',
            'reject-default-profile',
            'reject-unmanaged-profile',
            'redacted-refs',
            'close-on-session-end',
            'degrade-safely',
          ],
          [
            'owned-profile',
            'loopback-only',
            'random-port',
            'reject-default-profile',
            'reject-unmanaged-profile',
            'redacted-refs',
            'close-on-session-end',
            'degrade-safely',
          ],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserIntegrationMechanisms,
          GeneratedBrowserControlWritesToPath.ManagedBrowserIntegrationMechanisms,
          'Which managed browser integrations may be used?',
          ['chromium-cdp', 'managed-extension-native-host', 'browser-policy'],
          ['chromium-cdp', 'webdriver-bidi', 'managed-extension-native-host', 'browser-policy', 'owned-webview'],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserPolicyWriterControls,
          GeneratedBrowserControlWritesToPath.ManagedBrowserPolicyWriterControls,
          'Which managed Chrome/Edge policy-writer inputs may be authored?',
          [
            'disable-incognito',
            'disable-guest-browsing',
            'disable-profile-adding',
            'limit-history-deletion',
            'force-safe-search',
            'force-restricted-mode',
          ],
          [
            'disable-incognito',
            'disable-guest-browsing',
            'disable-profile-adding',
            'limit-history-deletion',
            'force-safe-search',
            'force-restricted-mode',
            'url-allow-list',
            'url-block-list',
          ],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ManagedBrowserPolicyWriterFallback,
          GeneratedBrowserControlWritesToPath.ManagedBrowserPolicyWriterFallback,
          'What if managed browser policy writing is unsupported?',
          'manual-required',
          ['observe-only', 'manual-required', 'degraded', 'unsupported', 'not-claimed'],
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.UnmanagedBrowser,
      'Unmanaged browser',
      'Choose what happens when browser-like activity is outside the managed boundary.',
      [enabledManifestConditionGenerated],
      [
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.UnmanagedBrowserMode,
          GeneratedBrowserControlWritesToPath.UnmanagedBrowserMode,
          'What should happen to unmanaged browsers?',
          'report-only',
          [
            'report-only',
            'allowed-unmanaged-exception',
            'warn-child',
            'parent-review',
            'terminate-process',
            'relaunch-managed',
            'os-block-configured',
            'os-block-manual-required',
          ],
          []
        ),
        manifestNumberFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.UnmanagedBrowserGraceSeconds,
          GeneratedBrowserControlWritesToPath.UnmanagedBrowserGraceSeconds,
          'How long should the child get before unmanaged browser action applies?',
          0,
          [includesManifestConditionGenerated(GeneratedBrowserControlWritesToPath.UnmanagedBrowserMode, 'warn')]
        ),
        manifestBooleanFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.UnmanagedBrowserAllowRecoverLaunchUrl,
          GeneratedBrowserControlWritesToPath.UnmanagedBrowserAllowRecoverLaunchUrl,
          'If a launch URL is visible, should it reopen in managed browser?',
          true,
          [
            equalsManifestConditionGenerated(
              GeneratedBrowserControlWritesToPath.UnmanagedBrowserMode,
              'relaunch-managed'
            ),
          ]
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.UnmanagedBrowserClassificationTargets,
          GeneratedBrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
          'Which unmanaged browser types should be detected?',
          ['known-browser', 'portable-browser', 'renamed-browser', 'browser-like-process', 'private-or-tor'],
          [
            'known-browser',
            'portable-browser',
            'renamed-browser',
            'browser-like-process',
            'embedded-webview',
            'private-or-tor',
            'unknown',
          ],
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.UrlTabEvidence,
      'URL and tab evidence',
      'Choose what exact browser state may be collected and used.',
      [
        enabledManifestConditionGenerated,
        notEqualsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.DefaultPosture, 'block'),
      ],
      [
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.EvidenceUrlScope,
          GeneratedBrowserControlWritesToPath.EvidenceUrlScope,
          'What URL detail may rules use?',
          'domain-origin-title',
          ['none', 'domain-only', 'domain-origin-title', 'full-url-without-query', 'full-url-with-query'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.RequiredProof,
          GeneratedBrowserControlWritesToPath.RequiredProof,
          'What proof is enough for exact browser rules?',
          'fresh-managed-active-tab',
          [
            'process-running',
            'foreground-window',
            'network-domain',
            'managed-tab-list',
            'fresh-managed-tab-list',
            'fresh-managed-active-tab',
          ],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.WhenProofUnavailable,
          GeneratedBrowserControlWritesToPath.WhenProofUnavailable,
          'What if browser proof is unavailable?',
          'ask',
          ['allow', 'observe', 'warn', 'ask', 'block-until-ready', 'mark-unavailable'],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.EvidenceNeverCollect,
          GeneratedBrowserControlWritesToPath.EvidenceNeverCollect,
          'What must browser rules never collect?',
          [
            'page-body',
            'chat-content',
            'screenshots',
            'keystrokes',
            'form-values',
            'secrets',
            'decrypted-https-payload',
            'raw-protocol-dumps',
          ],
          [
            'page-body',
            'chat-content',
            'screenshots',
            'keystrokes',
            'form-values',
            'secrets',
            'decrypted-https-payload',
            'raw-protocol-dumps',
          ],
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.WebRules,
      'Web rules',
      'Rules for URLs, domains, categories, search, video, browser sessions, and browser processes.',
      [
        enabledManifestConditionGenerated,
        notEqualsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.DefaultPosture, 'allow'),
      ],
      [
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.AllowedTargetTypes,
          GeneratedBrowserControlWritesToPath.AllowedTargetTypes,
          'What browser targets should rules match?',
          ['exact-url', 'domain-origin', 'site-category', 'browser-session', 'browser-process', 'capability-state'],
          [
            'exact-url',
            'domain-origin',
            'site-category',
            'search-terms',
            'video-channel',
            'browser-session',
            'browser-process',
            'capability-state',
            'download',
            'browser-game-portal',
            'cloud-gaming',
            'webgl-canvas-game',
            'game-account',
            'game-purchase',
          ],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.AllowedActions,
          GeneratedBrowserControlWritesToPath.AllowedActions,
          'What actions may browser rules use?',
          ['allow', 'warn', 'ask', 'limit', 'block'],
          [
            'allow',
            'monitor',
            'warn',
            'ask',
            'limit',
            'block',
            'redirect',
            'close-tab',
            'close-browser',
            'terminate-process',
            'relaunch-managed',
          ],
          []
        ),
        manifestFieldGenerated(
          'rule-list',
          GeneratedBrowserControlManifestDefaults.Field.RuleItems,
          GeneratedBrowserControlWritesToPath.RuleItems,
          'Rules',
          [],
          [],
          []
        ),
        manifestFieldGenerated(
          'target-list',
          GeneratedBrowserControlManifestDefaults.Field.UrlAllowList,
          GeneratedBrowserControlWritesToPath.UrlAllowList,
          'Allowed URL or domain list',
          [],
          [],
          []
        ),
        manifestFieldGenerated(
          'target-list',
          GeneratedBrowserControlManifestDefaults.Field.UrlBlockList,
          GeneratedBrowserControlWritesToPath.UrlBlockList,
          'Blocked URL or domain list',
          [],
          [],
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.Budgets,
      'Budgets',
      'Browser time budgets and counting mode.',
      [
        enabledManifestConditionGenerated,
        equalsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.DefaultPosture, 'limit'),
      ],
      [
        manifestBooleanFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BudgetsEnabled,
          GeneratedBrowserControlWritesToPath.BudgetsEnabled,
          'Enable browser budgets?',
          true,
          []
        ),
        manifestNumberFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DailyBudgetMinutes,
          GeneratedBrowserControlWritesToPath.DailyBudgetMinutes,
          'Default daily browser minutes',
          60,
          [equalsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.BudgetsEnabled, true)]
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BudgetCountingMode,
          GeneratedBrowserControlWritesToPath.BudgetCountingMode,
          'How should browser time count?',
          'foreground-browser-time',
          [
            'foreground-browser-time',
            'managed-active-tab-time',
            'managed-session-time',
            'all-browser-process-time',
            'unmanaged-as-unknown-web-time',
          ],
          [equalsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.BudgetsEnabled, true)]
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.BrowserGames,
      'Browser games and cloud gaming',
      'Author educational game, unknown game, cloud gaming, account, purchase, portal, and canvas rules.',
      [
        enabledManifestConditionGenerated,
        notEqualsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.DefaultPosture, 'allow'),
      ],
      [
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGameEducationalMode,
          GeneratedBrowserControlWritesToPath.BrowserGameEducationalMode,
          'What should happen to educational browser games?',
          'allow',
          ['allow', 'observe', 'warn', 'parent-review', 'limit', 'block', 'manual-required'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGameUnknownMode,
          GeneratedBrowserControlWritesToPath.BrowserGameUnknownMode,
          'What should happen to unknown browser games?',
          'parent-review',
          ['allow', 'observe', 'warn', 'parent-review', 'limit', 'block', 'manual-required'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGameCloudGamingApproval,
          GeneratedBrowserControlWritesToPath.BrowserGameCloudGamingApproval,
          'How should cloud gaming be approved?',
          'parent-review',
          ['allow', 'parent-review', 'block', 'manual-required'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGamePurchaseAccountApproval,
          GeneratedBrowserControlWritesToPath.BrowserGamePurchaseAccountApproval,
          'How should game purchases and accounts be approved?',
          'parent-review',
          ['allow', 'parent-review', 'block', 'manual-required'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGameUnblockedPortalMode,
          GeneratedBrowserControlWritesToPath.BrowserGameUnblockedPortalMode,
          'What should happen to unblocked game portals?',
          'warn',
          ['allow', 'observe', 'warn', 'parent-review', 'limit', 'block', 'manual-required'],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGameWebglCanvasMode,
          GeneratedBrowserControlWritesToPath.BrowserGameWebglCanvasMode,
          'What should happen to WebGL or canvas games?',
          'observe',
          ['allow', 'observe', 'warn', 'parent-review', 'limit', 'block', 'manual-required'],
          []
        ),
        manifestNumberFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.BrowserGameDailyBudgetMinutes,
          GeneratedBrowserControlWritesToPath.BrowserGameDailyBudgetMinutes,
          'Default daily browser-game minutes',
          30,
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.Downloads,
      'Downloads',
      'Download monitoring and risky file handling.',
      [enabledManifestConditionGenerated],
      [
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DownloadMode,
          GeneratedBrowserControlWritesToPath.DownloadMode,
          'How should downloads be handled?',
          'observe',
          ['off', 'observe', 'warn', 'ask', 'block-risky', 'block-all'],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.DownloadBlockedTypes,
          GeneratedBrowserControlWritesToPath.DownloadBlockedTypes,
          'Which downloads are risky?',
          ['executable', 'script', 'unknown'],
          ['executable', 'script', 'archive', 'media', 'unknown', 'large-file', 'browser-danger'],
          [notEqualsManifestConditionGenerated(GeneratedBrowserControlWritesToPath.DownloadMode, 'off')]
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.Approvals,
      'Approvals',
      'Parent approval triggers and unanswered request behavior.',
      [enabledManifestConditionGenerated],
      [
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ApprovalRequiredFor,
          GeneratedBrowserControlWritesToPath.ApprovalRequiredFor,
          'What requires parent approval?',
          ['blocked-site', 'new-domain', 'unmanaged-browser', 'download', 'time-extension'],
          [
            'blocked-site',
            'new-domain',
            'unknown-category',
            'unmanaged-browser',
            'download',
            'time-extension',
            'managed-setup',
            'new-browser-install',
          ],
          []
        ),
        manifestSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ApprovalUnansweredDefault,
          GeneratedBrowserControlWritesToPath.ApprovalUnansweredDefault,
          'What if parent does not answer?',
          'deny',
          ['deny', 'allow-temporarily', 'continue-observe-only', 'keep-waiting'],
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.Reports,
      'Reports',
      'Parent-visible report fields, retention, and custody.',
      [enabledManifestConditionGenerated],
      [
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.ReportVisibleFields,
          GeneratedBrowserControlWritesToPath.ReportVisibleFields,
          'Which report fields may parents see?',
          [
            'managed-status',
            'recent-domain-title',
            'unmanaged-use',
            'policy-decisions',
            'block-results',
            'time-budget',
            'source-capability',
          ],
          [
            'managed-status',
            'recent-url',
            'recent-domain-title',
            'unmanaged-use',
            'policy-decisions',
            'block-results',
            'time-budget',
            'download-events',
            'source-capability',
          ],
          []
        ),
        manifestFieldGenerated(
          'retention',
          GeneratedBrowserControlManifestDefaults.Field.RetentionExactUrl,
          GeneratedBrowserControlWritesToPath.RetentionExactUrl,
          'How long can exact URL evidence be retained?',
          '7-days',
          ['fresh-only', '24-hours', '7-days', '30-days', 'until-reset', 'delete-expired'],
          []
        ),
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.CustodyAllowedUses,
          GeneratedBrowserControlWritesToPath.CustodyAllowedUses,
          'Where may browser evidence be used?',
          ['child-local', 'lan-live', 'parent-cache', 'parent-report'],
          ['child-local', 'lan-live', 'parent-cache', 'parent-export', 'parent-report', 'unavailable'],
          []
        ),
      ]
    ),
    manifestSectionGenerated(
      GeneratedBrowserControlManifestDefaults.Section.Audit,
      'Audit',
      'Required audit fields for strict browser-control actions.',
      [enabledManifestConditionGenerated],
      [
        manifestMultiSelectFieldGenerated(
          GeneratedBrowserControlManifestDefaults.Field.AuditRequiredFields,
          GeneratedBrowserControlWritesToPath.AuditRequiredFields,
          'Which audit fields are required?',
          [
            'policy-decision',
            'evidence-ref',
            'adapter-result',
            'timer-state',
            'parent-override',
            'rollback',
            'policy-version',
          ],
          [
            'policy-decision',
            'evidence-ref',
            'ai-ref',
            'adapter-result',
            'timer-state',
            'parent-override',
            'rollback',
            'policy-version',
            'capability-state',
            'custody-label',
          ],
          []
        ),
      ]
    ),
  ],
} as const;
export function browserGamePolicyCompilerInputIsConsistentGenerated(
  value: BrowserGamePolicyCompilerInputGenerated
): boolean {
  if (
    value.rawGamePayloadIncluded ||
    value.rawModelTextIncluded ||
    value.activityDomainObjectIncluded ||
    value.finalDecisionClaimedByInput ||
    value.runtimeGateClaimedByInput ||
    value.uiClaimedByInput ||
    value.enforcementClaimedByInput ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed
  ) {
    return false;
  }

  if (value.compilerMode === 'contract-only') {
    return value.analysisRefs.length > 0 && value.parentRuleRefs.length > 0 && value.targetKind !== 'manual-required';
  }

  return value.analysisRefs.length === 0 || value.targetKind === 'manual-required';
}

export function browserGamePolicyDecisionCandidateIsConsistentGenerated(
  value: BrowserGamePolicyDecisionCandidateGenerated
): boolean {
  if (
    value.finalPolicyDecisionClaimed ||
    value.runtimeGateExecutedClaimed ||
    value.uiRenderedClaimed ||
    value.enforcementClaimed ||
    value.nativeGameControlClaimed ||
    value.cloudFrameAnalysisClaimed ||
    value.rawGamePayloadStored ||
    value.rawModelTextUsed
  ) {
    return false;
  }

  switch (value.actionCandidate) {
    case 'unknown-candidate':
      return (
        value.fallbackUsed &&
        BrowserGamePolicyUnknownFallbackReasonsGenerated.some((reason) => value.reasonCodes.includes(reason))
      );
    case 'manual-review-candidate':
      return value.fallbackUsed && value.reasonCodes.includes('manual-required');
    case 'parent-review-candidate':
      return value.parentApprovalRequired && value.reasonCodes.includes('parent-rule-match');
    case 'allow-candidate':
      return BrowserGamePolicyAllowReasonsGenerated.some((reason) => value.reasonCodes.includes(reason));
    case 'time-limit-candidate':
      return value.reasonCodes.includes('schedule-context') && value.reasonCodes.includes('parent-rule-match');
    case 'warn-candidate':
    case 'block-candidate':
      return value.compilerMode === 'contract-only' && value.analysisRefs.length > 0;
    default:
      return false;
  }
}

export function browserControlWritesToIsKnownGenerated(writesTo: string | null): boolean {
  return writesTo === null || BrowserControlKnownWritesToPathsGenerated.has(String(writesTo));
}

export function browserControlConditionsAreMetGenerated(
  conditions: readonly BrowserControlConditionGenerated[],
  values: Record<string, BrowserControlFieldValueGenerated>
): boolean {
  return conditions.every((condition) => browserControlConditionIsMetGenerated(condition, values));
}

export function browserControlManifestVisibleSectionIdsGenerated(
  manifest: BrowserControlManifestGenerated,
  values: Record<string, BrowserControlFieldValueGenerated>
): string[] {
  return manifest.sections
    .filter((section) => browserControlConditionsAreMetGenerated(section.visibleWhen, values))
    .map((section) => section.sectionId);
}

export function browserControlManifestAllowsFieldGenerated(
  manifest: BrowserControlManifestGenerated,
  fieldId: string
): boolean {
  return manifest.sections.some((section) => section.fields.some((field) => field.fieldId === fieldId));
}

export function browserControlManifestAllowsWritesToGenerated(
  manifest: BrowserControlManifestGenerated,
  writesTo: string
): boolean {
  return manifest.sections.some((section) => section.fields.some((field) => field.writesTo === writesTo));
}
export function browserControlFieldDefaultMatchesOptionsGenerated(field: BrowserControlFieldGenerated): boolean {
  if (
    field.controlKind === 'toggle' ||
    field.controlKind === 'boolean' ||
    field.controlKind === 'number' ||
    field.controlKind === 'duration' ||
    field.controlKind === 'schedule' ||
    field.controlKind === 'rule-list' ||
    field.controlKind === 'target-list' ||
    field.controlKind === 'action-list' ||
    field.controlKind === 'readonly-status' ||
    field.controlKind === 'read-only-status'
  ) {
    return true;
  }

  if (Array.isArray(field.defaultValue)) {
    return field.defaultValue.every((value) => field.options.some((optionItem) => optionItem.value === value));
  }

  return field.options.some((optionItem) => optionItem.value === field.defaultValue);
}

export function browserControlFieldIdsAreUniqueGenerated(fields: readonly BrowserControlFieldGenerated[]): boolean {
  return new Set(fields.map((field) => field.fieldId)).size === fields.length;
}

export function browserControlSectionIdsAreUniqueGenerated(
  sections: readonly BrowserControlSectionGenerated[]
): boolean {
  return new Set(sections.map((section) => section.sectionId)).size === sections.length;
}

export function browserControlLimitPostureIsConsistentGenerated(policy: BrowserControlPolicyGenerated): boolean {
  return (
    policy.defaultPosture !== 'limit' ||
    (policy.budgets.enabled && policy.budgets.defaultDailyMinutes !== null) ||
    policy.fallbackPosture !== null
  );
}

export function browserControlEffectiveLimitPostureIsConsistentGenerated(
  policy: BrowserControlEffectivePolicyGenerated
): boolean {
  return (
    policy.defaultPosture !== 'limit' ||
    (policy.budgets.enabled && policy.budgets.defaultDailyMinutes !== null) ||
    policy.fallbackPosture !== null
  );
}

export function browserControlRuleTargetIsSpecifiedGenerated(rule: BrowserControlRuleGenerated): boolean {
  return (rule.targetType !== null && rule.targetValue !== null) || rule.target !== null;
}
export function browserControlExactUrlPolicyIsHonestGenerated(policy: BrowserControlPolicyGenerated): boolean {
  const authoredRules = [...policy.rules.items, ...policy.rules.entries];
  if (!policy.rules.allowedTargetTypes.includes('exact-url') && !authoredRules.some(ruleUsesExactUrlTargetGenerated)) {
    return true;
  }
  if (policy.evidence.proofFallback !== null) {
    return true;
  }
  if (policy.evidence.whenProofUnavailable !== 'mark-unavailable') {
    return true;
  }
  return (
    (policy.managedBrowser.mode === 'required-for-exact-rules' ||
      policy.managedBrowser.mode === 'required-for-all-browsing') &&
    policy.evidence.requiredProof === 'fresh-managed-active-tab'
  );
}

export function browserControlBrowserGameLimitIsConsistentGenerated(policy: BrowserControlPolicyGenerated): boolean {
  const gameLimitSelected =
    policy.browserGames.educationalGameMode === 'limit' ||
    policy.browserGames.unknownGameMode === 'limit' ||
    policy.browserGames.unblockedPortalMode === 'limit' ||
    policy.browserGames.webglCanvasMode === 'limit';

  return !gameLimitSelected || policy.browserGames.defaultDailyMinutes !== null || policy.fallbackPosture !== null;
}
export function defaultBrowserGamesGenerated() {
  return {
    educationalGameMode: 'allow' as const,
    unknownGameMode: 'parent-review' as const,
    cloudGamingApproval: 'parent-review' as const,
    purchaseAccountApproval: 'parent-review' as const,
    unblockedPortalMode: 'warn' as const,
    webglCanvasMode: 'observe' as const,
    defaultDailyMinutes: 30,
  };
}

export function defaultChildFacingGenerated() {
  return {
    showWarnText: false,
    showBlockReason: false,
    showAskParentState: false,
    showTimeLeft: false,
    showUseManagedBrowserAction: false,
    hideParentDiagnostics: false,
  };
}

export function defaultDiscoveryGenerated() {
  return {
    scanInstalledBrowsers: false,
    scanRunningBrowsers: true,
    detectUnmanagedBrowsers: true,
  };
}

export function defaultPortalAiGenerated() {
  return {
    allowSummaries: false,
    allowPolicyExplanation: false,
    allowRuleSuggestions: false,
    allowEvidenceRefs: false,
    allowRawContent: false,
    requiresManualReview: false,
    fallbackWhenUnavailable: null,
  };
}

export function defaultPlatformCapabilityGenerated() {
  return {
    enabled: false,
    state: null,
    allowedAdapters: [],
    manualRequiredAdapters: [],
    authoringOnly: false,
    mayRunCapture: false,
    mayConnectToBrowserBridge: false,
  };
}

export function defaultPlatformsGenerated() {
  return {
    windows: defaultPlatformCapabilityGenerated(),
    macos: defaultPlatformCapabilityGenerated(),
    linux: defaultPlatformCapabilityGenerated(),
    android: defaultPlatformCapabilityGenerated(),
    ios: defaultPlatformCapabilityGenerated(),
    webPortal: defaultPlatformCapabilityGenerated(),
  };
}

export function defaultFallbacksGenerated() {
  return {
    managedProfileMissing: null,
    bridgeMissing: null,
    extensionDisabled: null,
    nativeHostMissing: null,
    unsupportedBrowser: null,
    staleEvidence: null,
    networkAdapterUnavailable: null,
    processControlUnavailable: null,
    enforcementFailure: null,
    childDeviceOffline: null,
    platformUnsupported: null,
  };
}
export function optionsFromSourceTextGenerated(sourceText: string): BrowserControlFullCatalogOptionGenerated[] {
  const explicit = explicitOptionLabelsGenerated(sourceText);
  if (explicit.length > 0) {
    return optionsGenerated(explicit);
  }
  return optionsGenerated(['Enabled', 'Disabled']);
}

export function selectionModeForGenerated(
  sourceText: string,
  settingOptions: readonly BrowserControlFullCatalogOptionGenerated[]
): 'single' | 'multi' {
  if (settingOptions.length <= 2 && settingOptions[0]?.value === 'enabled') {
    return 'single';
  }
  const prefix = sourceText.split(':')[0]?.toLowerCase() ?? sourceText.toLowerCase();
  return /targets|actions|approvals|reports|proof|custody|audit|budgets|choose covered browsers/u.test(prefix)
    ? 'multi'
    : 'single';
}

export function cardKindForGenerated(
  selectionMode: 'single' | 'multi',
  settingOptions: readonly BrowserControlFullCatalogOptionGenerated[]
): (typeof GeneratedBrowserControlFullCatalogCardKindValues)[number] {
  if (settingOptions.length <= 2 && settingOptions[0]?.value === 'enabled') {
    return 'toggle';
  }
  if (selectionMode === 'multi') {
    return settingOptions.length > 4 ? 'multi-choice-many' : 'multi-choice-normal';
  }
  return settingOptions.length > 4 ? 'single-choice-many' : 'single-choice-compact';
}

export function controlKindForGenerated(
  sourceText: string,
  selectionMode: 'single' | 'multi',
  settingOptions: readonly BrowserControlFullCatalogOptionGenerated[]
): (typeof GeneratedBrowserControlFullCatalogControlKindValues)[number] {
  const prefix = sourceText.split(':')[0]?.toLowerCase() ?? sourceText.toLowerCase();
  const matchedRule = BrowserControlFullCatalogControlKindRulesGenerated.find((ruleCandidate) =>
    ruleCandidate.pattern.test(prefix)
  );
  if (
    matchedRule !== undefined &&
    (matchedRule.resolve(sourceText) !== 'read-only-status' || settingOptions.length <= 2)
  ) {
    return matchedRule.resolve(sourceText);
  }
  if (settingOptions.length <= 2 && settingOptions[0]?.value === 'enabled') {
    return 'toggle';
  }
  return selectionMode === 'multi' ? 'multi-choice' : 'single-choice';
}

export function layoutHintsForGenerated(
  selectionMode: 'single' | 'multi',
  settingOptions: readonly BrowserControlFullCatalogOptionGenerated[]
) {
  const manyOptions = settingOptions.length > 4;
  return {
    preferredColumnSpan: manyOptions ? 2 : 1,
    collapsible: manyOptions || selectionMode === 'multi',
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(settingOptions.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && selectionMode === 'multi',
    showSelectedCount: selectionMode === 'multi',
  };
}

export function questionFromSourceTextGenerated(sourceText: string): string {
  const trimmed = sourceText.replace(/\.$/u, '');
  const colonIndex = trimmed.indexOf(':');
  if (colonIndex !== -1) {
    return `Choose ${lowerFirstGenerated(trimmed.slice(0, colonIndex))}.`;
  }
  if (
    /^(enable|disable|allow|require|scan|detect|notify|ask|auto-classify|re-scan|show|hide|keep|redact|collect)/iu.test(
      trimmed
    )
  ) {
    return `${trimmed}?`;
  }
  return `Use ${lowerFirstGenerated(trimmed)}?`;
}

export function helperTextForGenerated(sectionTitle: string, sourceText: string): string {
  if (proofRequirementForGenerated(sectionTitle, sourceText) !== null) {
    return 'Exact browser evidence must stay proof-gated; Portal renders intent while runtime proves capability.';
  }
  if (effectStatusForSectionGenerated(sectionTitle, sourceText) !== 'needs-effect-wiring') {
    return 'Render this with its capability state and fallback rather than claiming unsupported enforcement.';
  }
  return 'Portal renders authored intent; child runtime owns persistence, compile, evaluation, and audit.';
}
export function effectStatusForSectionGenerated(
  sectionTitle: string,
  sourceText: string
): (typeof GeneratedBrowserControlFullCatalogEffectStatusValues)[number] {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Gaps To Decide Before UI Contracts/u.test(sectionTitle)) {
    return 'future-gap';
  }
  if (/Never-Collect|Portal Display|Child-Facing|Report|Audit|Custody|Retention|Data Minimization/u.test(searchable)) {
    return 'already-represented';
  }
  if (
    /Exact URL|active tab|page title|download source|browser evidence|required proof|proof requirement/iu.test(
      searchable
    )
  ) {
    return 'proof-required';
  }
  if (/Private|Tor|permission|protected browser/iu.test(searchable)) {
    return 'permission-required';
  }
  if (/Network|Capability Failure|Fallback|Degradation|Unmanaged Browser Recovery|degraded/iu.test(searchable)) {
    return 'degraded';
  }
  if (/Platform|Setup|Provisioning|Managed Browser Operation|Notifications|manual/iu.test(searchable)) {
    return 'manual-required';
  }
  return 'needs-effect-wiring';
}
export function runtimeOwnerForSectionGenerated(
  sectionTitle: string,
  sourceText: string
): (typeof GeneratedBrowserControlFullCatalogRuntimeOwnerValues)[number] {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Portal Display|Child-Facing|Report/iu.test(searchable)) {
    return 'portal-only';
  }
  if (/Audit|Custody|Retention|Never-Collect|Data Minimization/iu.test(searchable)) {
    return 'parent-owned-storage';
  }
  if (/AI/iu.test(searchable)) {
    return 'local-ai-runtime';
  }
  if (
    /Platform|Setup|Provisioning|Managed Browser Operation|Private|Tor|Network|Capability Failure/iu.test(searchable)
  ) {
    return 'os-adapter';
  }
  if (/manual|permission/iu.test(searchable)) {
    return 'manual-proof';
  }
  if (/policy value|protocol|patch|replace|rollback/iu.test(searchable)) {
    return 'agent-protocol';
  }
  return 'child-agent';
}
export function capabilityStateForSectionGenerated(
  sectionTitle: string,
  sourceText: string
): (typeof GeneratedBrowserControlFullCatalogCapabilityStateValues)[number] {
  const status = effectStatusForSectionGenerated(sectionTitle, sourceText);
  if (status === 'future-gap') {
    return 'future-gap';
  }
  if (status === 'permission-required') {
    return 'permission-required';
  }
  if (status === 'degraded') {
    return 'degraded';
  }
  if (status === 'manual-required') {
    return 'manual-required';
  }
  if (status === 'proof-required') {
    return 'protected';
  }
  return 'available';
}
export function capabilityRequirementForGenerated(sectionTitle: string, sourceText: string): string {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Exact URL|active tab|page title|download source/iu.test(searchable)) {
    return 'managed-browser-or-explicit-browser-integration';
  }
  if (/Network/iu.test(searchable)) {
    return 'network-metadata-observation-only';
  }
  if (/Private|Tor|permission/iu.test(searchable)) {
    return 'explicit-permission-and-platform-proof';
  }
  if (/AI/iu.test(searchable)) {
    return 'local-ai-runtime-with-parent-enabled-analysis';
  }
  if (/Audit|Retention|Custody|Never-Collect/iu.test(searchable)) {
    return 'parent-owned-local-storage-and-redaction';
  }
  return 'browser-control-capability-registry';
}
export function proofRequirementForGenerated(sectionTitle: string, sourceText: string): string | null {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Exact URL|active tab|page title|download source/iu.test(searchable)) {
    return 'managed-browser-or-explicit-browser-integration';
  }
  if (/browser evidence|required proof|proof requirement/iu.test(searchable)) {
    return 'schema-valid-evidence-ref-with-runtime-custody';
  }
  if (/Network/iu.test(searchable)) {
    return 'network-evidence-must-not-be-treated-as-exact-url-or-tab-content';
  }
  if (/process|window|foreground/iu.test(searchable)) {
    return 'process-or-window-evidence-only-with-no-url-claim';
  }
  if (/AI/iu.test(searchable)) {
    return 'local-analysis-summary-and-evidence-refs-without-raw-browser-data-upload';
  }
  return null;
}
export function fallbackForGenerated(sectionTitle: string, sourceText: string): string {
  const status = effectStatusForSectionGenerated(sectionTitle, sourceText);
  if (status === 'future-gap') {
    return 'Expose as future gap or planning-only control; do not compile into enforcement.';
  }
  if (status === 'manual-required') {
    return 'Disable or degrade until manual setup/proof confirms the required browser capability.';
  }
  if (status === 'permission-required') {
    return 'Disable strict behavior until permission exists; keep observe/audit-only alternatives available.';
  }
  if (status === 'degraded') {
    return 'Show degraded capability and compile only the observable subset without exact URL/tab claims.';
  }
  if (status === 'proof-required') {
    return 'Require explicit proof before enforcement; otherwise compile observe or manual-required behavior.';
  }
  return 'Keep as authored intent until runtime wiring proves the exact effect key.';
}
export function visibilityConditionsForGenerated(): BrowserControlFullCatalogRuleGenerated[] {
  return [ruleGenerated('Visible when the Browser side-panel category is selected.')];
}

export function enabledConditionsForGenerated(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogRuleGenerated[] {
  return [
    ruleGenerated('A family, child, or device target must be selected before writing policy intent.'),
    ruleGenerated(
      `Capability state must allow ${effectStatusForSectionGenerated(sectionTitle, sourceText)} presentation.`
    ),
  ];
}
export function validationRulesForGenerated(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogRuleGenerated[] {
  const proof = proofRequirementForGenerated(sectionTitle, sourceText);
  const rules = [
    ruleGenerated('Selected option ids must belong to this setting acceptedOptions list.'),
    ruleGenerated(
      'Portal writes only authored intent; child runtime owns compile, persistence, evaluation, and audit.'
    ),
  ];
  if (proof !== null) {
    rules.push(ruleGenerated(`Enforcement requires proof: ${proof}.`));
  }
  return rules;
}

export function sectionKindForTitleGenerated(
  sectionTitle: string
): 'setting-section' | 'rule-dimension-section' | 'candidate-mvp-section' | 'planning-gap-section' {
  if (sectionTitle === 'Global Rule Dimensions') {
    return 'rule-dimension-section';
  }
  if (sectionTitle === 'Candidate MVP Setting Set') {
    return 'candidate-mvp-section';
  }
  if (sectionTitle === 'Gaps To Decide Before UI Contracts') {
    return 'planning-gap-section';
  }
  return 'setting-section';
}
export function uiTabForSectionGenerated(sectionTitle: string): string {
  if (/Rule|Search|Video|Conflict/u.test(sectionTitle)) {
    return 'rules';
  }
  if (/Schedule|Time Budget/u.test(sectionTitle)) {
    return 'schedule';
  }
  if (/Approval|Override|Notifications/u.test(sectionTitle)) {
    return 'approvals';
  }
  if (/Evidence|Never-Collect/u.test(sectionTitle)) {
    return 'evidence';
  }
  if (/Report|Portal Display|Child-Facing/u.test(sectionTitle)) {
    return 'reports';
  }
  if (/Custody|Retention/u.test(sectionTitle)) {
    return 'data';
  }
  if (/Audit/u.test(sectionTitle)) {
    return 'audit';
  }
  if (/AI/u.test(sectionTitle)) {
    return 'ai';
  }
  if (/Platform/u.test(sectionTitle)) {
    return 'platform';
  }
  if (/Setup|Provisioning/u.test(sectionTitle)) {
    return 'setup';
  }
  return 'enforcement';
}
interface ControlKindRuleGenerated {
  readonly pattern: RegExp;
  readonly resolve: (sourceText: string) => (typeof GeneratedBrowserControlFullCatalogControlKindValues)[number];
}

const BrowserControlFullCatalogControlKindRulesGenerated: readonly ControlKindRuleGenerated[] = [
  { pattern: /schedule|time window|bedtime|school hours/u, resolve: () => 'schedule' },
  { pattern: /budget|limit|minutes|seconds|retention days/u, resolve: numericOrDurationControlGenerated },
  { pattern: /retention|custody|delete|redact/u, resolve: () => 'retention' },
  { pattern: /rule items|target list|allowlist|blocklist/u, resolve: () => 'rule-list' },
  { pattern: /actions|approval actions|notification actions/u, resolve: () => 'action-list' },
  { pattern: /status|state|capability|proof/u, resolve: () => 'read-only-status' },
];

function browserControlConditionIsMetGenerated(
  condition: BrowserControlConditionGenerated,
  values: Record<string, BrowserControlFieldValueGenerated>
): boolean {
  if (condition.kind === 'default-posture') {
    return values[GeneratedBrowserControlWritesToPath.DefaultPosture] === condition.defaultPosture;
  }
  if (condition.writesTo === null) {
    return false;
  }
  const actual = values[condition.writesTo];
  if (condition.kind === 'equals') {
    return actual === condition.expectedValue;
  }
  if (condition.kind === 'not-equals' || condition.kind === 'notEquals') {
    return actual !== condition.expectedValue;
  }
  if (Array.isArray(actual) && typeof condition.expectedValue === 'string') {
    return condition.kind === 'includes'
      ? actual.includes(condition.expectedValue)
      : !actual.includes(condition.expectedValue);
  }
  return false;
}
function explicitOptionLabelsGenerated(sourceText: string): string[] {
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  const suffix = sourceText
    .slice(colonIndex + 1)
    .replace(/warn\/ask/giu, 'warn, ask')
    .replace(/\.$/u, '');
  return uniqueGenerated(
    suffix
      .split(/,|;|\bor\b/u)
      .map((part) => cleanOptionLabelGenerated(part))
      .filter((part) => part.length > 0)
  );
}

function numericOrDurationControlGenerated(
  sourceText: string
): (typeof GeneratedBrowserControlFullCatalogControlKindValues)[number] {
  return /minutes|seconds|days/u.test(sourceText.toLowerCase()) ? 'number' : 'duration';
}

function optionsGenerated(labels: readonly string[]): BrowserControlFullCatalogOptionGenerated[] {
  return labels.map((label) => {
    const value = slugGenerated(label);
    return {
      optionId: `browser-catalog-option-${value}`,
      label,
      value,
      originalSourceText: label,
      meaning: null,
      defaultSelected: false,
    };
  });
}

function ruleGenerated(description: string): BrowserControlFullCatalogRuleGenerated {
  return {
    ruleId: `browser-catalog-rule-${slugGenerated(description)}`,
    description,
  };
}

function ruleUsesExactUrlTargetGenerated(rule: BrowserControlRuleGenerated): boolean {
  return rule.targetType === 'exact-url' || rule.target?.kind === 'exact-url';
}

function manifestSectionGenerated(
  sectionId: string,
  title: string,
  description: string,
  visibleWhen: readonly BrowserControlConditionGenerated[],
  fields: readonly ReturnType<typeof manifestFieldGenerated>[]
) {
  return {
    sectionId,
    title,
    description,
    visibleWhen,
    fields,
  };
}

function manifestOptionGenerated(optionId: string, label: string, value: string) {
  return {
    optionId,
    label,
    value,
    description: null,
  };
}

function manifestOptionsGenerated(fieldId: string, values: readonly string[]) {
  return values.map((value) => manifestOptionGenerated(`${fieldId}.${value}`, value, value));
}

function manifestConditionGenerated(
  kind: 'equals' | 'notEquals' | 'includes',
  writesTo: string,
  expectedValue: BrowserControlFieldValueGenerated
) {
  return {
    kind,
    writesTo,
    expectedValue,
    capabilityId: null,
    capabilityState: null,
    defaultPosture: null,
  };
}

function equalsManifestConditionGenerated(writesTo: string, expectedValue: BrowserControlFieldValueGenerated) {
  return manifestConditionGenerated('equals', writesTo, expectedValue);
}

function notEqualsManifestConditionGenerated(writesTo: string, expectedValue: BrowserControlFieldValueGenerated) {
  return manifestConditionGenerated('notEquals', writesTo, expectedValue);
}

function includesManifestConditionGenerated(writesTo: string, expectedValue: string) {
  return manifestConditionGenerated('includes', writesTo, expectedValue);
}

function manifestFieldGenerated(
  controlKind: string,
  fieldId: string,
  writesTo: string,
  label: string,
  defaultValue: BrowserControlFieldValueGenerated,
  optionValues: readonly string[],
  visibleWhen: readonly BrowserControlConditionGenerated[]
) {
  return {
    fieldId,
    label,
    description: null,
    controlKind,
    writesTo,
    defaultValue,
    options: manifestOptionsGenerated(fieldId, optionValues),
    visibleWhen,
    enabledWhen: [],
    required: true,
  };
}

function manifestBooleanFieldGenerated(
  fieldId: string,
  writesTo: string,
  label: string,
  defaultValue: boolean,
  visibleWhen: readonly BrowserControlConditionGenerated[]
) {
  return manifestFieldGenerated('boolean', fieldId, writesTo, label, defaultValue, [], visibleWhen);
}

function manifestNumberFieldGenerated(
  fieldId: string,
  writesTo: string,
  label: string,
  defaultValue: number,
  visibleWhen: readonly BrowserControlConditionGenerated[]
) {
  return manifestFieldGenerated('number', fieldId, writesTo, label, defaultValue, [], visibleWhen);
}

function manifestSelectFieldGenerated(
  fieldId: string,
  writesTo: string,
  label: string,
  defaultValue: string,
  optionValues: readonly string[],
  visibleWhen: readonly BrowserControlConditionGenerated[]
) {
  return manifestFieldGenerated('single-choice', fieldId, writesTo, label, defaultValue, optionValues, visibleWhen);
}

function manifestMultiSelectFieldGenerated(
  fieldId: string,
  writesTo: string,
  label: string,
  defaultValue: readonly string[],
  optionValues: readonly string[],
  visibleWhen: readonly BrowserControlConditionGenerated[]
) {
  return manifestFieldGenerated('multi-choice', fieldId, writesTo, label, defaultValue, optionValues, visibleWhen);
}

function cleanOptionLabelGenerated(value: string): string {
  return titleizeGenerated(value.trim().replace(/\.$/u, '').replace(/\s+/gu, ' '));
}

function titleizeGenerated(value: string): string {
  return value
    .split(/[\s-]+/u)
    .filter((part) => part.length > 0)
    .map((part) => `${part.charAt(0).toUpperCase()}${part.slice(1)}`)
    .join(' ');
}

function slugGenerated(value: string): string {
  const normalized = value
    .toLowerCase()
    .replace(/&/gu, ' and ')
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .replace(/-{2,}/gu, '-');
  return normalized.length > 0 ? normalized : 'option';
}

function lowerFirstGenerated(value: string): string {
  return `${value.charAt(0).toLowerCase()}${value.slice(1)}`;
}

function uniqueGenerated(values: readonly string[]): string[] {
  return [...new Set(values)];
}
