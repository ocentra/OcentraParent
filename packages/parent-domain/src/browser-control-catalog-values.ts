import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const BrowserControlManagedBrowserFamilySchema = withParser(
  Schema.Literal(
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
    'owned-webview'
  )
);

export const BrowserControlManagedBrowserLaunchModeSchema = withParser(
  Schema.Literal('manual', 'ocentra-launcher', 'default-browser-route', 'managed-shell', 'admin-provisioned')
);

export const BrowserControlManagedBrowserProfileModeSchema = withParser(
  Schema.Literal('persistent-managed-profile', 'clear-on-schedule', 'clear-on-session-end', 'ephemeral')
);

export const BrowserControlManagedBrowserBridgeRequirementSchema = withParser(
  Schema.Literal(
    'owned-profile',
    'loopback-only',
    'random-port',
    'reject-default-profile',
    'reject-unmanaged-profile',
    'redacted-refs',
    'close-on-session-end',
    'degrade-safely'
  )
);

export const BrowserControlManagedBrowserIntegrationMechanismSchema = withParser(
  Schema.Literal('chromium-cdp', 'webdriver-bidi', 'managed-extension-native-host', 'browser-policy', 'owned-webview')
);

export const BrowserControlManagedPolicyWriterControlSchema = withParser(
  Schema.Literal(
    'disable-incognito',
    'disable-guest-browsing',
    'disable-profile-adding',
    'limit-history-deletion',
    'force-safe-search',
    'force-restricted-mode',
    'url-allow-list',
    'url-block-list'
  )
);

export const BrowserControlManagedPolicyWriterFallbackSchema = withParser(
  Schema.Literal('observe-only', 'manual-required', 'degraded', 'unsupported', 'not-claimed')
);

export const BrowserControlUnmanagedBrowserClassificationTargetSchema = withParser(
  Schema.Literal(
    'known-browser',
    'portable-browser',
    'renamed-browser',
    'browser-like-process',
    'embedded-webview',
    'private-or-tor',
    'unknown'
  )
);

export const BrowserControlEvidenceUrlScopeSchema = withParser(
  Schema.Literal('none', 'domain-only', 'domain-origin-title', 'full-url-without-query', 'full-url-with-query')
);

export const BrowserControlEvidenceNeverCollectSchema = withParser(
  Schema.Literal(
    'page-body',
    'chat-content',
    'screenshots',
    'keystrokes',
    'form-values',
    'secrets',
    'decrypted-https-payload',
    'raw-protocol-dumps'
  )
);

export const BrowserControlRuleActionSchema = withParser(
  Schema.Literal(
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
    'relaunch-managed'
  )
);

export const BrowserControlBrowserGamePolicyModeSchema = withParser(
  Schema.Literal('allow', 'observe', 'warn', 'ask-parent', 'limit', 'block', 'manual-required')
);

export const BrowserControlBrowserGameApprovalModeSchema = withParser(
  Schema.Literal('allow', 'ask-parent', 'block', 'manual-required')
);

export const BrowserControlBudgetCountingModeSchema = withParser(
  Schema.Literal(
    'foreground-browser-time',
    'managed-active-tab-time',
    'managed-session-time',
    'all-browser-process-time',
    'unmanaged-as-unknown-web-time'
  )
);

export const BrowserControlDownloadBlockedTypeSchema = withParser(
  Schema.Literal('executable', 'script', 'archive', 'media', 'unknown', 'large-file', 'browser-danger')
);

export const BrowserControlApprovalRequiredForSchema = withParser(
  Schema.Literal(
    'blocked-site',
    'new-domain',
    'unknown-category',
    'unmanaged-browser',
    'download',
    'time-extension',
    'managed-setup',
    'new-browser-install'
  )
);

export const BrowserControlApprovalUnansweredDefaultSchema = withParser(
  Schema.Literal('deny', 'allow-temporarily', 'continue-observe-only', 'keep-waiting')
);

export const BrowserControlReportVisibleFieldSchema = withParser(
  Schema.Literal(
    'managed-status',
    'recent-url',
    'recent-domain-title',
    'unmanaged-use',
    'policy-decisions',
    'block-results',
    'time-budget',
    'download-events',
    'source-capability'
  )
);

export const BrowserControlRetentionExactUrlSchema = withParser(
  Schema.Literal('fresh-only', '24-hours', '7-days', '30-days', 'until-reset', 'delete-expired')
);

export const BrowserControlCustodyAllowedUseSchema = withParser(
  Schema.Literal('child-local', 'lan-live', 'parent-cache', 'parent-export', 'parent-report', 'unavailable')
);

export const BrowserControlAuditRequiredFieldSchema = withParser(
  Schema.Literal(
    'policy-decision',
    'evidence-ref',
    'ai-ref',
    'adapter-result',
    'timer-state',
    'parent-override',
    'rollback',
    'policy-version',
    'capability-state',
    'custody-label'
  )
);

export type BrowserControlManagedBrowserFamily = Infer<typeof BrowserControlManagedBrowserFamilySchema>;
export type BrowserControlManagedBrowserLaunchMode = Infer<typeof BrowserControlManagedBrowserLaunchModeSchema>;
export type BrowserControlManagedBrowserProfileMode = Infer<typeof BrowserControlManagedBrowserProfileModeSchema>;
export type BrowserControlManagedBrowserBridgeRequirement = Infer<
  typeof BrowserControlManagedBrowserBridgeRequirementSchema
>;
export type BrowserControlManagedBrowserIntegrationMechanism = Infer<
  typeof BrowserControlManagedBrowserIntegrationMechanismSchema
>;
export type BrowserControlManagedPolicyWriterControl = Infer<typeof BrowserControlManagedPolicyWriterControlSchema>;
export type BrowserControlManagedPolicyWriterFallback = Infer<typeof BrowserControlManagedPolicyWriterFallbackSchema>;
export type BrowserControlUnmanagedBrowserClassificationTarget = Infer<
  typeof BrowserControlUnmanagedBrowserClassificationTargetSchema
>;
export type BrowserControlEvidenceUrlScope = Infer<typeof BrowserControlEvidenceUrlScopeSchema>;
export type BrowserControlEvidenceNeverCollect = Infer<typeof BrowserControlEvidenceNeverCollectSchema>;
export type BrowserControlRuleAction = Infer<typeof BrowserControlRuleActionSchema>;
export type BrowserControlBrowserGamePolicyMode = Infer<typeof BrowserControlBrowserGamePolicyModeSchema>;
export type BrowserControlBrowserGameApprovalMode = Infer<typeof BrowserControlBrowserGameApprovalModeSchema>;
export type BrowserControlBudgetCountingMode = Infer<typeof BrowserControlBudgetCountingModeSchema>;
export type BrowserControlDownloadBlockedType = Infer<typeof BrowserControlDownloadBlockedTypeSchema>;
export type BrowserControlApprovalRequiredFor = Infer<typeof BrowserControlApprovalRequiredForSchema>;
export type BrowserControlApprovalUnansweredDefault = Infer<typeof BrowserControlApprovalUnansweredDefaultSchema>;
export type BrowserControlReportVisibleField = Infer<typeof BrowserControlReportVisibleFieldSchema>;
export type BrowserControlRetentionExactUrl = Infer<typeof BrowserControlRetentionExactUrlSchema>;
export type BrowserControlCustodyAllowedUse = Infer<typeof BrowserControlCustodyAllowedUseSchema>;
export type BrowserControlAuditRequiredField = Infer<typeof BrowserControlAuditRequiredFieldSchema>;
