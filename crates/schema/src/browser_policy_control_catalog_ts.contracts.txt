/* generated from crates/schema/src/browser_policy_control_catalog_ts.rs */

export const BrowserPolicyControlCatalogContractRuntime = {
  SchemaVersion: 'v0.6',
} as const;

export const GeneratedBrowserGamePolicyTargetKindValues = [
  'browser-game-url',
  'game-portal',
  'cloud-gaming-session',
  'game-account-signup',
  'game-login',
  'game-purchase',
  'unblocked-game-site',
  'ugc-multiplayer-game',
  'educational-game',
  'unknown-game',
  'manual-required',
] as const;

export const GeneratedBrowserGamePolicyActionCandidateValues = [
  'allow-candidate',
  'warn-candidate',
  'parent-review-candidate',
  'block-candidate',
  'time-limit-candidate',
  'manual-review-candidate',
  'unknown-candidate',
] as const;

export const GeneratedBrowserGamePolicyReasonCodeValues = [
  'parent-rule-match',
  'browser-game-risk-high',
  'educational-benefit-present',
  'cloud-gaming-risk',
  'purchase-risk',
  'account-required-risk',
  'ugc-chat-risk',
  'unblocked-game-site-risk',
  'low-confidence',
  'manual-required',
  'missing-game-evidence',
  'degraded-analysis',
  'schedule-context',
  'unknown-evidence',
  'mobile-capability-manual-required',
] as const;

export const GeneratedBrowserGamePolicyCompilerModeValues = [
  'contract-only',
  'manual-required',
  'unavailable',
] as const;

export const GeneratedBrowserGamePolicyConfidenceValues = ['high', 'medium', 'low', 'unknown'] as const;

export const GeneratedBrowserControlManagedBrowserFamilyValues = [
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
] as const;

export const GeneratedBrowserControlManagedBrowserLaunchModeValues = [
  'manual',
  'ocentra-launcher',
  'default-browser-route',
  'managed-shell',
  'admin-provisioned',
] as const;

export const GeneratedBrowserControlManagedBrowserProfileModeValues = [
  'persistent-managed-profile',
  'clear-on-schedule',
  'clear-on-session-end',
  'ephemeral',
] as const;

export const GeneratedBrowserControlManagedBrowserBridgeRequirementValues = [
  'owned-profile',
  'loopback-only',
  'random-port',
  'reject-default-profile',
  'reject-unmanaged-profile',
  'redacted-refs',
  'close-on-session-end',
  'degrade-safely',
] as const;

export const GeneratedBrowserControlManagedBrowserIntegrationMechanismValues = [
  'chromium-cdp',
  'webdriver-bidi',
  'managed-extension-native-host',
  'browser-policy',
  'owned-webview',
] as const;

export const GeneratedBrowserControlManagedPolicyWriterControlValues = [
  'disable-incognito',
  'disable-guest-browsing',
  'disable-profile-adding',
  'limit-history-deletion',
  'force-safe-search',
  'force-restricted-mode',
  'url-allow-list',
  'url-block-list',
] as const;

export const GeneratedBrowserControlManagedPolicyWriterFallbackValues = [
  'observe-only',
  'manual-required',
  'degraded',
  'unsupported',
  'not-claimed',
] as const;

export const GeneratedBrowserControlUnmanagedBrowserClassificationTargetValues = [
  'known-browser',
  'portable-browser',
  'renamed-browser',
  'browser-like-process',
  'embedded-webview',
  'private-or-tor',
  'unknown',
] as const;

export const GeneratedBrowserControlEvidenceUrlScopeValues = [
  'none',
  'domain-only',
  'domain-origin-title',
  'full-url-without-query',
  'full-url-with-query',
] as const;

export const GeneratedBrowserControlEvidenceNeverCollectValues = [
  'page-body',
  'chat-content',
  'screenshots',
  'keystrokes',
  'form-values',
  'secrets',
  'decrypted-https-payload',
  'raw-protocol-dumps',
] as const;

export const GeneratedBrowserControlRuleActionValues = [
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
] as const;

export const GeneratedBrowserControlBrowserGamePolicyModeValues = [
  'allow',
  'observe',
  'warn',
  'parent-review',
  'limit',
  'block',
  'manual-required',
] as const;

export const GeneratedBrowserControlBrowserGameApprovalModeValues = [
  'allow',
  'parent-review',
  'block',
  'manual-required',
] as const;

export const GeneratedBrowserControlBudgetCountingModeValues = [
  'foreground-browser-time',
  'managed-active-tab-time',
  'managed-session-time',
  'all-browser-process-time',
  'unmanaged-as-unknown-web-time',
] as const;

export const GeneratedBrowserControlDownloadBlockedTypeValues = [
  'executable',
  'script',
  'archive',
  'media',
  'unknown',
  'large-file',
  'browser-danger',
] as const;

export const GeneratedBrowserControlApprovalRequiredForValues = [
  'blocked-site',
  'new-domain',
  'unknown-category',
  'unmanaged-browser',
  'download',
  'time-extension',
  'managed-setup',
  'new-browser-install',
] as const;

export const GeneratedBrowserControlApprovalUnansweredDefaultValues = [
  'deny',
  'allow-temporarily',
  'continue-observe-only',
  'keep-waiting',
] as const;

export const GeneratedBrowserControlReportVisibleFieldValues = [
  'managed-status',
  'recent-url',
  'recent-domain-title',
  'unmanaged-use',
  'policy-decisions',
  'block-results',
  'time-budget',
  'download-events',
  'source-capability',
] as const;

export const GeneratedBrowserControlRetentionExactUrlValues = [
  'fresh-only',
  '24-hours',
  '7-days',
  '30-days',
  'until-reset',
  'delete-expired',
] as const;

export const GeneratedBrowserControlCustodyAllowedUseValues = [
  'child-local',
  'lan-live',
  'parent-cache',
  'parent-export',
  'parent-report',
  'unavailable',
] as const;

export const GeneratedBrowserControlAuditRequiredFieldValues = [
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
] as const;

export const GeneratedBrowserControlKindValues = [
  'boolean',
  'single-choice',
  'multi-choice',
  'number',
  'duration',
  'schedule',
  'rule-list',
  'target-list',
  'retention',
  'action-list',
  'read-only-status',
  'toggle',
  'single-select',
  'multi-select',
  'readonly-status',
] as const;

export const GeneratedBrowserControlConditionKindValues = [
  'equals',
  'notEquals',
  'not-equals',
  'includes',
  'notIncludes',
  'not-includes',
  'all',
  'any',
  'capabilityAvailable',
  'capability-state',
  'default-posture',
  'platformIn',
  'proofAtLeast',
] as const;

export const GeneratedBrowserControlDefaultPostureValues = [
  'observe',
  'allow',
  'warn',
  'ask',
  'limit',
  'parent-review',
  'block',
] as const;

export const GeneratedBrowserControlExecutionModeValues = ['observe', 'dry-run', 'warn-ask', 'enforce'] as const;

export const GeneratedBrowserControlManagementModeValues = [
  'disabled',
  'observe-only',
  'managed-browser',
  'network-assisted',
  'local-child-agent',
  'lan-live',
  'authoring-only',
  'unavailable',
] as const;

export const GeneratedBrowserControlManagedBrowserModeValues = [
  'disabled',
  'not-required',
  'preferred',
  'available-for-exact-rules',
  'required-for-exact-rules',
  'required-for-all-browsing',
] as const;

export const GeneratedBrowserControlUnmanagedBrowserModeValues = [
  'report-only',
  'observe-only',
  'network-domain-only',
  'manual-review',
  'allow',
  'allowed-unmanaged-exception',
  'monitor',
  'warn-child',
  'warn',
  'parent-review',
  'ask',
  'terminate-process',
  'relaunch-managed',
  'os-block-configured',
  'os-block-manual-required',
  'block',
] as const;

export const GeneratedBrowserControlUrlTargetTypeValues = [
  'domain',
  'url-prefix',
  'exact-url',
  'domain-origin',
  'site-category',
  'search-terms',
  'video-channel',
  'browser-session',
  'browser-process',
  'capability-state',
  'download',
  'social-platform',
  'social-route-kind',
  'social-account-creation',
  'social-unknown-account',
  'social-secondary-account',
  'social-feed',
  'social-short-video-feed',
  'social-messaging',
  'social-upload-post',
  'social-livestream',
  'unknown-social-site',
  'browser-game',
  'browser-game-platform',
  'browser-game-portal',
  'browser-game-url',
  'educational-game',
  'cloud-gaming',
  'webgl-canvas-game',
  'multiplayer-ugc-game',
  'game-chat',
  'game-account',
  'game-purchase',
  'game-loot-box',
  'unknown-game',
  'unblocked-game-site',
] as const;

export const GeneratedBrowserControlEvidenceProofLevelValues = [
  'none',
  'process-running',
  'foreground-window',
  'network-domain',
  'managed-active-tab',
  'managed-tab-list',
  'fresh-managed-tab-list',
  'fresh-managed-active-tab',
  'classifier-category',
  'url-shape-metadata',
  'social-route-evidence',
  'browser-game-runtime-signal',
  'browser-policy-writer',
  'adapter-action',
] as const;

export const GeneratedBrowserControlProofFallbackValues = [
  'downgrade-to-domain',
  'parent-review',
  'block-until-proof',
  'observe-only',
  'allow',
  'observe',
  'warn',
  'ask',
  'block-until-ready',
  'mark-unavailable',
] as const;

export const GeneratedBrowserControlDownloadStateValues = [
  'not-configured',
  'allow',
  'observe',
  'warn',
  'ask',
  'parent-review',
  'block',
  'block-risky',
  'block-all',
  'off',
] as const;

export const GeneratedBrowserControlApprovalStateValues = [
  'not-required',
  'required',
  'pending',
  'approved',
  'denied',
] as const;

export const GeneratedBrowserControlReportStateValues = ['disabled', 'daily', 'weekly', 'on-demand'] as const;

export const GeneratedBrowserControlAuditStateValues = [
  'disabled',
  'local-only',
  'parent-visible',
  'retained',
] as const;

export const GeneratedBrowserControlRetentionStateValues = [
  'none',
  'seven-days',
  'thirty-days',
  'fresh-only',
  '24-hours',
  '7-days',
  '30-days',
  'until-reset',
  'delete-expired',
] as const;

export const GeneratedBrowserControlCapabilityStateValues = [
  'supported',
  'unsupported',
  'degraded',
  'unavailable',
  'unknown',
  'ready',
  'manual-required',
] as const;

export const GeneratedBrowserControlRejectionReasonValues = [
  'invalid-request',
  'unknown-writes-to',
  'unknown-field',
  'invalid-enum-value',
  'missing-budget-or-fallback',
  'missing-managed-proof-or-fallback',
  'capability-unavailable',
  'storage-unavailable',
  'stale-revision',
  'scaffold-unavailable',
  'revision-not-found',
] as const;

export const GeneratedBrowserControlPatchOperationValues = ['replace'] as const;
export const GeneratedBrowserControlUpdateKindValues = ['get', 'preview', 'patch', 'replace', 'rollback'] as const;
export const GeneratedBrowserControlUpdateStatusValues = ['accepted', 'rejected'] as const;

export const GeneratedBrowserControlWritesToPath = {
  Enabled: '/browserPolicy/enabled',
  ExecutionMode: '/browserPolicy/executionMode',
  DefaultPosture: '/browserPolicy/defaultPosture',
  ManagementMode: '/browserPolicy/managementMode',
  DiscoveryScanInstalledBrowsers: '/browserPolicy/discovery/scanInstalledBrowsers',
  DiscoveryScanRunningBrowsers: '/browserPolicy/discovery/scanRunningBrowsers',
  DiscoveryDetectUnmanagedBrowsers: '/browserPolicy/discovery/detectUnmanagedBrowsers',
  ManagedBrowserMode: '/browserPolicy/managedBrowser/mode',
  ManagedBrowserAllowedFamilies: '/browserPolicy/managedBrowser/allowedFamilies',
  ManagedBrowserLaunchMode: '/browserPolicy/managedBrowser/launchMode',
  ManagedBrowserProfileMode: '/browserPolicy/managedBrowser/profileMode',
  ManagedBrowserBridgeRequirements: '/browserPolicy/managedBrowser/bridgeRequirements',
  ManagedBrowserIntegrationMechanisms: '/browserPolicy/managedBrowser/integrationMechanisms',
  ManagedBrowserPolicyWriterControls: '/browserPolicy/managedBrowser/policyWriterControls',
  ManagedBrowserPolicyWriterFallback: '/browserPolicy/managedBrowser/policyWriterFallback',
  UnmanagedBrowserMode: '/browserPolicy/unmanagedBrowser/mode',
  UnmanagedBrowserGraceSeconds: '/browserPolicy/unmanagedBrowser/graceSeconds',
  UnmanagedBrowserAllowRecoverLaunchUrl: '/browserPolicy/unmanagedBrowser/allowRecoverLaunchUrl',
  UnmanagedBrowserClassificationTargets: '/browserPolicy/unmanagedBrowser/classificationTargets',
  EvidenceUrlScope: '/browserPolicy/evidence/urlScope',
  RequiredProof: '/browserPolicy/evidence/requiredProof',
  ProofFallback: '/browserPolicy/evidence/proofFallback',
  WhenProofUnavailable: '/browserPolicy/evidence/whenProofUnavailable',
  EvidenceNeverCollect: '/browserPolicy/evidence/neverCollect',
  AllowedTargetTypes: '/browserPolicy/rules/allowedTargetTypes',
  AllowedActions: '/browserPolicy/rules/allowedActions',
  RuleItems: '/browserPolicy/rules/items',
  UrlAllowList: '/browserPolicy/rules/urlAllowList',
  UrlBlockList: '/browserPolicy/rules/urlBlockList',
  BudgetsEnabled: '/browserPolicy/budgets/enabled',
  DailyBudgetMinutes: '/browserPolicy/budgets/defaultDailyMinutes',
  BudgetCountingMode: '/browserPolicy/budgets/countingMode',
  BrowserGameEducationalMode: '/browserPolicy/browserGames/educationalGameMode',
  BrowserGameUnknownMode: '/browserPolicy/browserGames/unknownGameMode',
  BrowserGameCloudGamingApproval: '/browserPolicy/browserGames/cloudGamingApproval',
  BrowserGamePurchaseAccountApproval: '/browserPolicy/browserGames/purchaseAccountApproval',
  BrowserGameUnblockedPortalMode: '/browserPolicy/browserGames/unblockedPortalMode',
  BrowserGameWebglCanvasMode: '/browserPolicy/browserGames/webglCanvasMode',
  BrowserGameDailyBudgetMinutes: '/browserPolicy/browserGames/defaultDailyMinutes',
  DownloadMode: '/browserPolicy/downloads/mode',
  DownloadBlockedTypes: '/browserPolicy/downloads/blockedTypes',
  DownloadState: '/browserPolicy/downloads/state',
  ApprovalRequiredFor: '/browserPolicy/approvals/requiredFor',
  ApprovalUnansweredDefault: '/browserPolicy/approvals/unansweredDefault',
  ApprovalState: '/browserPolicy/approvals/state',
  ReportVisibleFields: '/browserPolicy/reports/visibleFields',
  ReportState: '/browserPolicy/reports/state',
  RetentionExactUrl: '/browserPolicy/retention/exactUrl',
  RetentionState: '/browserPolicy/retention/state',
  CustodyAllowedUses: '/browserPolicy/custody/allowedUses',
  AuditRequiredFields: '/browserPolicy/audit/requiredFields',
  AuditState: '/browserPolicy/audit/state',
} as const;

export const GeneratedBrowserControlFullCatalogControlKindValues = [
  'toggle',
  'single-choice',
  'multi-choice',
  'number',
  'duration',
  'schedule',
  'rule-list',
  'target-list',
  'retention',
  'action-list',
  'read-only-status',
] as const;

export const GeneratedBrowserControlFullCatalogEffectStatusValues = [
  'already-represented',
  'needs-effect-wiring',
  'represented-by-existing-policy-shape',
  'manual-required',
  'unavailable',
  'future-gap',
  'degraded',
  'permission-required',
  'permission-limited',
  'proof-required',
] as const;

export const GeneratedBrowserControlFullCatalogRuntimeOwnerValues = [
  'portal-only',
  'rust-parent-runtime',
  'agent-protocol',
  'rust-service',
  'child-agent',
  'os-adapter',
  'manual-proof',
  'parent-owned-storage',
  'local-ai-runtime',
] as const;

export const GeneratedBrowserControlFullCatalogCapabilityStateValues = [
  'available',
  'disabled',
  'unsupported',
  'permission-required',
  'permission-limited',
  'protected',
  'degraded',
  'manual-required',
  'future-gap',
  'unavailable',
] as const;

export const GeneratedBrowserControlFullCatalogSidePanelCategoryValues = ['browser'] as const;
export const GeneratedBrowserControlFullCatalogUiTabValues = [
  'rules',
  'schedule',
  'approvals',
  'enforcement',
  'audit',
  'evidence',
  'setup',
  'reports',
  'platform',
  'data',
  'ai',
] as const;

export const GeneratedBrowserControlFullCatalogCardKindValues = [
  'single-choice-compact',
  'single-choice-many',
  'multi-choice-normal',
  'multi-choice-many',
  'toggle',
  'schedule-card',
  'rule-list-card',
  'target-list-card',
  'retention-card',
  'status-card',
] as const;

export const GeneratedBrowserControlFullCatalogSelectionModeValues = ['single', 'multi'] as const;
export const GeneratedBrowserControlFullCatalogSectionKindValues = [
  'setting-section',
  'rule-dimension-section',
  'candidate-mvp-section',
  'planning-gap-section',
] as const;
