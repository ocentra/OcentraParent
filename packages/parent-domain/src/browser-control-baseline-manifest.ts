import {
  BrowserControlAuthoringFieldSchema,
  BrowserControlAuthoringManifestSchema,
  BrowserControlConditionSchema,
  BrowserControlFieldOptionSchema,
  BrowserControlManifestDefaults,
  type BrowserControlCondition,
  type BrowserControlFieldOption,
} from './browser-control-manifest';
import type { BrowserControlFieldId } from './browser-control-identifiers';
import {
  BrowserControlWritesToPath,
  type BrowserControlFieldValue,
  type BrowserControlKind,
  type BrowserControlSchemaKnownWritesToPath,
} from './browser-control-values';

const enabled = equals(BrowserControlWritesToPath.Enabled, true);

export const BaselineBrowserControlAuthoringManifest = BrowserControlAuthoringManifestSchema.parse({
  schemaVersion: 'v0.6',
  manifestId: BrowserControlManifestDefaults.ManifestId,
  title: 'Browser controls',
  sections: [
    {
      sectionId: BrowserControlManifestDefaults.Section.Management,
      title: 'Browser management',
      description: 'Top-level browser policy switch and default posture.',
      visibleWhen: [],
      fields: [
        booleanField(
          BrowserControlManifestDefaults.Field.Enabled,
          BrowserControlWritesToPath.Enabled,
          'Enable browser management?',
          false,
          []
        ),
        selectField(
          BrowserControlManifestDefaults.Field.DefaultPosture,
          BrowserControlWritesToPath.DefaultPosture,
          'What should happen to browser activity?',
          'observe',
          ['allow', 'observe', 'warn', 'ask', 'limit', 'block'],
          [enabled]
        ),
        selectField(
          BrowserControlManifestDefaults.Field.ManagementMode,
          BrowserControlWritesToPath.ManagementMode,
          'How should browser management run on this device?',
          'local-child-agent',
          ['local-child-agent', 'lan-live', 'authoring-only', 'unavailable'],
          [enabled]
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.ManagedBrowser,
      title: 'Managed browser',
      description: 'Configure the browser path that can support exact URL, tab, download, and request-level rules.',
      visibleWhen: [enabled],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserMode,
          BrowserControlWritesToPath.ManagedBrowserMode,
          'How should managed browser be used?',
          'available-for-exact-rules',
          ['disabled', 'available-for-exact-rules', 'required-for-exact-rules', 'required-for-all-browsing'],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserAllowedFamilies,
          BrowserControlWritesToPath.ManagedBrowserAllowedFamilies,
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
        selectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserLaunchMode,
          BrowserControlWritesToPath.ManagedBrowserLaunchMode,
          'How should allowed browsing launch?',
          'ocentra-launcher',
          ['manual', 'ocentra-launcher', 'default-browser-route', 'managed-shell', 'admin-provisioned'],
          []
        ),
        selectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserProfileMode,
          BrowserControlWritesToPath.ManagedBrowserProfileMode,
          'How should the managed profile behave?',
          'persistent-managed-profile',
          ['persistent-managed-profile', 'clear-on-schedule', 'clear-on-session-end', 'ephemeral'],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserBridgeRequirements,
          BrowserControlWritesToPath.ManagedBrowserBridgeRequirements,
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
        multiSelectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserIntegrationMechanisms,
          BrowserControlWritesToPath.ManagedBrowserIntegrationMechanisms,
          'Which managed browser integrations may be used?',
          ['chromium-cdp', 'managed-extension-native-host', 'browser-policy'],
          ['chromium-cdp', 'webdriver-bidi', 'managed-extension-native-host', 'browser-policy', 'owned-webview'],
          []
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.UnmanagedBrowser,
      title: 'Unmanaged browser',
      description: 'Choose what happens when browser-like activity is outside the managed boundary.',
      visibleWhen: [enabled],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.UnmanagedBrowserMode,
          BrowserControlWritesToPath.UnmanagedBrowserMode,
          'What should happen to unmanaged browsers?',
          'monitor',
          ['allow', 'monitor', 'warn', 'ask', 'relaunch-managed', 'block'],
          []
        ),
        numberField(
          BrowserControlManifestDefaults.Field.UnmanagedBrowserGraceSeconds,
          BrowserControlWritesToPath.UnmanagedBrowserGraceSeconds,
          'How long should the child get before unmanaged browser action applies?',
          0,
          [includes(BrowserControlWritesToPath.UnmanagedBrowserMode, 'warn')]
        ),
        booleanField(
          BrowserControlManifestDefaults.Field.UnmanagedBrowserAllowRecoverLaunchUrl,
          BrowserControlWritesToPath.UnmanagedBrowserAllowRecoverLaunchUrl,
          'If a launch URL is visible, should it reopen in managed browser?',
          true,
          [equals(BrowserControlWritesToPath.UnmanagedBrowserMode, 'relaunch-managed')]
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.UnmanagedBrowserClassificationTargets,
          BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
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
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.UrlTabEvidence,
      title: 'URL and tab evidence',
      description: 'Choose what exact browser state may be collected and used.',
      visibleWhen: [enabled, notEquals(BrowserControlWritesToPath.DefaultPosture, 'block')],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.EvidenceUrlScope,
          BrowserControlWritesToPath.EvidenceUrlScope,
          'What URL detail may rules use?',
          'domain-origin-title',
          ['none', 'domain-only', 'domain-origin-title', 'full-url-without-query', 'full-url-with-query'],
          []
        ),
        selectField(
          BrowserControlManifestDefaults.Field.RequiredProof,
          BrowserControlWritesToPath.RequiredProof,
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
        selectField(
          BrowserControlManifestDefaults.Field.WhenProofUnavailable,
          BrowserControlWritesToPath.WhenProofUnavailable,
          'What if browser proof is unavailable?',
          'ask',
          ['allow', 'observe', 'warn', 'ask', 'block-until-ready', 'mark-unavailable'],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.EvidenceNeverCollect,
          BrowserControlWritesToPath.EvidenceNeverCollect,
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
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.WebRules,
      title: 'Web rules',
      description: 'Rules for URLs, domains, categories, search, video, browser sessions, and browser processes.',
      visibleWhen: [enabled, notEquals(BrowserControlWritesToPath.DefaultPosture, 'allow')],
      fields: [
        multiSelectField(
          BrowserControlManifestDefaults.Field.AllowedTargetTypes,
          BrowserControlWritesToPath.AllowedTargetTypes,
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
          ],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.AllowedActions,
          BrowserControlWritesToPath.AllowedActions,
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
            'relaunch-managed',
          ],
          []
        ),
        field(
          'rule-list',
          BrowserControlManifestDefaults.Field.RuleItems,
          BrowserControlWritesToPath.RuleItems,
          'Rules',
          [],
          [],
          []
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.Budgets,
      title: 'Budgets',
      description: 'Browser time budgets and counting mode.',
      visibleWhen: [enabled, equals(BrowserControlWritesToPath.DefaultPosture, 'limit')],
      fields: [
        booleanField(
          BrowserControlManifestDefaults.Field.BudgetsEnabled,
          BrowserControlWritesToPath.BudgetsEnabled,
          'Enable browser budgets?',
          true,
          []
        ),
        numberField(
          BrowserControlManifestDefaults.Field.DailyBudgetMinutes,
          BrowserControlWritesToPath.DailyBudgetMinutes,
          'Default daily browser minutes',
          60,
          [equals(BrowserControlWritesToPath.BudgetsEnabled, true)]
        ),
        selectField(
          BrowserControlManifestDefaults.Field.BudgetCountingMode,
          BrowserControlWritesToPath.BudgetCountingMode,
          'How should browser time count?',
          'foreground-browser-time',
          [
            'foreground-browser-time',
            'managed-active-tab-time',
            'managed-session-time',
            'all-browser-process-time',
            'unmanaged-as-unknown-web-time',
          ],
          [equals(BrowserControlWritesToPath.BudgetsEnabled, true)]
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.Downloads,
      title: 'Downloads',
      description: 'Download monitoring and risky file handling.',
      visibleWhen: [enabled],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.DownloadMode,
          BrowserControlWritesToPath.DownloadMode,
          'How should downloads be handled?',
          'observe',
          ['off', 'observe', 'warn', 'ask', 'block-risky', 'block-all'],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.DownloadBlockedTypes,
          BrowserControlWritesToPath.DownloadBlockedTypes,
          'Which downloads are risky?',
          ['executable', 'script', 'unknown'],
          ['executable', 'script', 'archive', 'media', 'unknown', 'large-file', 'browser-danger'],
          [notEquals(BrowserControlWritesToPath.DownloadMode, 'off')]
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.Approvals,
      title: 'Approvals',
      description: 'Parent approval triggers and unanswered request behavior.',
      visibleWhen: [enabled],
      fields: [
        multiSelectField(
          BrowserControlManifestDefaults.Field.ApprovalRequiredFor,
          BrowserControlWritesToPath.ApprovalRequiredFor,
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
        selectField(
          BrowserControlManifestDefaults.Field.ApprovalUnansweredDefault,
          BrowserControlWritesToPath.ApprovalUnansweredDefault,
          'What if parent does not answer?',
          'deny',
          ['deny', 'allow-temporarily', 'continue-observe-only', 'keep-waiting'],
          []
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.Reports,
      title: 'Reports',
      description: 'Parent-visible report fields, retention, and custody.',
      visibleWhen: [enabled],
      fields: [
        multiSelectField(
          BrowserControlManifestDefaults.Field.ReportVisibleFields,
          BrowserControlWritesToPath.ReportVisibleFields,
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
        field(
          'retention',
          BrowserControlManifestDefaults.Field.RetentionExactUrl,
          BrowserControlWritesToPath.RetentionExactUrl,
          'How long can exact URL evidence be retained?',
          '7-days',
          ['fresh-only', '24-hours', '7-days', '30-days', 'until-reset', 'delete-expired'],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.CustodyAllowedUses,
          BrowserControlWritesToPath.CustodyAllowedUses,
          'Where may browser evidence be used?',
          ['child-local', 'lan-live', 'parent-cache', 'parent-report'],
          ['child-local', 'lan-live', 'parent-cache', 'parent-export', 'parent-report', 'unavailable'],
          []
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.Audit,
      title: 'Audit',
      description: 'Required audit fields for strict browser-control actions.',
      visibleWhen: [enabled],
      fields: [
        multiSelectField(
          BrowserControlManifestDefaults.Field.AuditRequiredFields,
          BrowserControlWritesToPath.AuditRequiredFields,
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
      ],
    },
  ],
});

function option(optionId: string, label: string, value: string): BrowserControlFieldOption {
  return BrowserControlFieldOptionSchema.parse({ optionId, label, value, description: null });
}

function options(fieldId: BrowserControlFieldId, values: ReadonlyArray<string>): BrowserControlFieldOption[] {
  return values.map((value) => option(`${fieldId}.${value}`, value, value));
}

function condition(
  kind: 'equals' | 'notEquals' | 'includes',
  writesTo: BrowserControlSchemaKnownWritesToPath,
  expectedValue: BrowserControlFieldValue
) {
  return BrowserControlConditionSchema.parse({
    kind,
    writesTo,
    expectedValue,
    capabilityId: null,
    capabilityState: null,
    defaultPosture: null,
  });
}

function equals(writesTo: BrowserControlSchemaKnownWritesToPath, expectedValue: BrowserControlFieldValue) {
  return condition('equals', writesTo, expectedValue);
}

function notEquals(writesTo: BrowserControlSchemaKnownWritesToPath, expectedValue: BrowserControlFieldValue) {
  return condition('notEquals', writesTo, expectedValue);
}

function includes(writesTo: BrowserControlSchemaKnownWritesToPath, expectedValue: string) {
  return condition('includes', writesTo, expectedValue);
}

function field(
  controlKind: BrowserControlKind,
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: BrowserControlFieldValue,
  optionValues: ReadonlyArray<string>,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return BrowserControlAuthoringFieldSchema.parse({
    fieldId,
    label,
    description: null,
    controlKind,
    writesTo,
    defaultValue,
    options: options(fieldId, optionValues),
    visibleWhen,
    enabledWhen: [],
    required: true,
  });
}

function booleanField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: boolean,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return field('boolean', fieldId, writesTo, label, defaultValue, [], visibleWhen);
}

function numberField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: number,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return field('number', fieldId, writesTo, label, defaultValue, [], visibleWhen);
}

function selectField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: string,
  optionValues: ReadonlyArray<string>,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return field('single-choice', fieldId, writesTo, label, defaultValue, optionValues, visibleWhen);
}

function multiSelectField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: string[],
  optionValues: ReadonlyArray<string>,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return field('multi-choice', fieldId, writesTo, label, defaultValue, optionValues, visibleWhen);
}
