import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const BrowserControlSchemaKnownWritesToPathSchema = withParser(
  NonEmptyStringSchema.pipe(Schema.brand('BrowserControlSchemaKnownWritesToPath'))
);

export const BrowserControlFieldValueSchema = withParser(
  Schema.Union(NonEmptyStringSchema, Schema.Number, Schema.Boolean, Schema.Array(NonEmptyStringSchema), Schema.Null)
);

export const BrowserControlKindSchema = withParser(
  Schema.Literal(
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
    'readonly-status'
  )
);
export const BrowserControlConditionKindSchema = withParser(
  Schema.Literal(
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
    'proofAtLeast'
  )
);
export const BrowserControlDefaultPostureSchema = withParser(
  Schema.Literal('observe', 'allow', 'warn', 'ask', 'limit', 'parent-review', 'block')
);
export const BrowserControlExecutionModeSchema = withParser(
  Schema.Literal('observe', 'dry-run', 'warn-ask', 'enforce')
);
export const BrowserControlManagementModeSchema = withParser(
  Schema.Literal(
    'disabled',
    'observe-only',
    'managed-browser',
    'network-assisted',
    'local-child-agent',
    'lan-live',
    'authoring-only',
    'unavailable'
  )
);
export const BrowserControlManagedBrowserModeSchema = withParser(
  Schema.Literal(
    'disabled',
    'not-required',
    'preferred',
    'available-for-exact-rules',
    'required-for-exact-rules',
    'required-for-all-browsing'
  )
);
export const BrowserControlUnmanagedBrowserModeSchema = withParser(
  Schema.Literal(
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
    'block'
  )
);
export const BrowserControlUrlTargetTypeSchema = withParser(
  Schema.Literal(
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
    'unblocked-game-site'
  )
);
export const BrowserControlEvidenceProofLevelSchema = withParser(
  Schema.Literal(
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
    'adapter-action'
  )
);
export const BrowserControlProofFallbackSchema = withParser(
  Schema.Literal(
    'downgrade-to-domain',
    'parent-review',
    'block-until-proof',
    'observe-only',
    'allow',
    'observe',
    'warn',
    'ask',
    'block-until-ready',
    'mark-unavailable'
  )
);
export const BrowserControlDownloadStateSchema = withParser(
  Schema.Literal(
    'not-configured',
    'allow',
    'observe',
    'warn',
    'ask',
    'parent-review',
    'block',
    'block-risky',
    'block-all',
    'off'
  )
);
export const BrowserControlApprovalStateSchema = withParser(
  Schema.Literal('not-required', 'required', 'pending', 'approved', 'denied')
);
export const BrowserControlReportStateSchema = withParser(Schema.Literal('disabled', 'daily', 'weekly', 'on-demand'));
export const BrowserControlAuditStateSchema = withParser(
  Schema.Literal('disabled', 'local-only', 'parent-visible', 'retained')
);
export const BrowserControlRetentionStateSchema = withParser(
  Schema.Literal(
    'none',
    'seven-days',
    'thirty-days',
    'fresh-only',
    '24-hours',
    '7-days',
    '30-days',
    'until-reset',
    'delete-expired'
  )
);
export const BrowserControlCapabilityStateSchema = withParser(
  Schema.Literal('supported', 'unsupported', 'degraded', 'unavailable', 'unknown', 'ready', 'manual-required')
);
export const BrowserControlRejectionReasonSchema = withParser(
  Schema.Literal(
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
    'revision-not-found'
  )
);
export const BrowserControlPatchOperationSchema = withParser(Schema.Literal('replace'));
export const BrowserControlUpdateKindSchema = withParser(
  Schema.Literal('get', 'preview', 'patch', 'replace', 'rollback')
);
export const BrowserControlUpdateStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));

export type BrowserControlSchemaKnownWritesToPath = typeof BrowserControlSchemaKnownWritesToPathSchema.Type;
export type BrowserControlFieldValue = Infer<typeof BrowserControlFieldValueSchema>;
export type BrowserControlKind = Infer<typeof BrowserControlKindSchema>;
export type BrowserControlConditionKind = Infer<typeof BrowserControlConditionKindSchema>;
export type BrowserControlDefaultPosture = Infer<typeof BrowserControlDefaultPostureSchema>;
export type BrowserControlExecutionMode = Infer<typeof BrowserControlExecutionModeSchema>;
export type BrowserControlManagementMode = Infer<typeof BrowserControlManagementModeSchema>;
export type BrowserControlManagedBrowserMode = Infer<typeof BrowserControlManagedBrowserModeSchema>;
export type BrowserControlUnmanagedBrowserMode = Infer<typeof BrowserControlUnmanagedBrowserModeSchema>;
export type BrowserControlUrlTargetType = Infer<typeof BrowserControlUrlTargetTypeSchema>;
export type BrowserControlEvidenceProofLevel = Infer<typeof BrowserControlEvidenceProofLevelSchema>;
export type BrowserControlProofFallback = Infer<typeof BrowserControlProofFallbackSchema>;
export type BrowserControlDownloadState = Infer<typeof BrowserControlDownloadStateSchema>;
export type BrowserControlApprovalState = Infer<typeof BrowserControlApprovalStateSchema>;
export type BrowserControlReportState = Infer<typeof BrowserControlReportStateSchema>;
export type BrowserControlAuditState = Infer<typeof BrowserControlAuditStateSchema>;
export type BrowserControlRetentionState = Infer<typeof BrowserControlRetentionStateSchema>;
export type BrowserControlCapabilityState = Infer<typeof BrowserControlCapabilityStateSchema>;
export type BrowserControlRejectionReason = Infer<typeof BrowserControlRejectionReasonSchema>;
export type BrowserControlPatchOperation = Infer<typeof BrowserControlPatchOperationSchema>;
export type BrowserControlUpdateKind = Infer<typeof BrowserControlUpdateKindSchema>;
export type BrowserControlUpdateStatus = Infer<typeof BrowserControlUpdateStatusSchema>;

export const BrowserControlWritesToPath = {
  Enabled: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/enabled'),
  ExecutionMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/executionMode'),
  DefaultPosture: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/defaultPosture'),
  ManagementMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/managementMode'),
  DiscoveryScanInstalledBrowsers: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/discovery/scanInstalledBrowsers'
  ),
  DiscoveryScanRunningBrowsers: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/discovery/scanRunningBrowsers'
  ),
  DiscoveryDetectUnmanagedBrowsers: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/discovery/detectUnmanagedBrowsers'
  ),
  ManagedBrowserMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/managedBrowser/mode'),
  ManagedBrowserAllowedFamilies: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/allowedFamilies'
  ),
  ManagedBrowserLaunchMode: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/launchMode'
  ),
  ManagedBrowserProfileMode: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/profileMode'
  ),
  ManagedBrowserBridgeRequirements: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/bridgeRequirements'
  ),
  ManagedBrowserIntegrationMechanisms: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/integrationMechanisms'
  ),
  ManagedBrowserPolicyWriterControls: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/policyWriterControls'
  ),
  ManagedBrowserPolicyWriterFallback: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/managedBrowser/policyWriterFallback'
  ),
  UnmanagedBrowserMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/unmanagedBrowser/mode'),
  UnmanagedBrowserGraceSeconds: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/unmanagedBrowser/graceSeconds'
  ),
  UnmanagedBrowserAllowRecoverLaunchUrl: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/unmanagedBrowser/allowRecoverLaunchUrl'
  ),
  UnmanagedBrowserClassificationTargets: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/unmanagedBrowser/classificationTargets'
  ),
  EvidenceUrlScope: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/evidence/urlScope'),
  RequiredProof: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/evidence/requiredProof'),
  ProofFallback: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/evidence/proofFallback'),
  WhenProofUnavailable: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/evidence/whenProofUnavailable'
  ),
  EvidenceNeverCollect: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/evidence/neverCollect'),
  AllowedTargetTypes: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/rules/allowedTargetTypes'),
  AllowedActions: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/rules/allowedActions'),
  RuleItems: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/rules/items'),
  UrlAllowList: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/rules/urlAllowList'),
  UrlBlockList: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/rules/urlBlockList'),
  BudgetsEnabled: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/budgets/enabled'),
  DailyBudgetMinutes: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/budgets/defaultDailyMinutes'),
  BudgetCountingMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/budgets/countingMode'),
  BrowserGameEducationalMode: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/educationalGameMode'
  ),
  BrowserGameUnknownMode: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/unknownGameMode'
  ),
  BrowserGameCloudGamingApproval: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/cloudGamingApproval'
  ),
  BrowserGamePurchaseAccountApproval: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/purchaseAccountApproval'
  ),
  BrowserGameUnblockedPortalMode: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/unblockedPortalMode'
  ),
  BrowserGameWebglCanvasMode: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/webglCanvasMode'
  ),
  BrowserGameDailyBudgetMinutes: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/browserGames/defaultDailyMinutes'
  ),
  DownloadMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/downloads/mode'),
  DownloadBlockedTypes: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/downloads/blockedTypes'),
  DownloadState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/downloads/state'),
  ApprovalRequiredFor: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/approvals/requiredFor'),
  ApprovalUnansweredDefault: BrowserControlSchemaKnownWritesToPathSchema.parse(
    '/browserPolicy/approvals/unansweredDefault'
  ),
  ApprovalState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/approvals/state'),
  ReportVisibleFields: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/reports/visibleFields'),
  ReportState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/reports/state'),
  RetentionExactUrl: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/retention/exactUrl'),
  RetentionState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/retention/state'),
  CustodyAllowedUses: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/custody/allowedUses'),
  AuditRequiredFields: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/audit/requiredFields'),
  AuditState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/audit/state'),
} as const;
