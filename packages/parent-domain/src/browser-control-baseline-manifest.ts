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
  type BrowserControlSchemaKnownWritesToPath,
} from './browser-control-values';

export const BaselineBrowserControlAuthoringManifest = BrowserControlAuthoringManifestSchema.parse({
  schemaVersion: 'v0.6',
  manifestId: BrowserControlManifestDefaults.ManifestId,
  title: 'Browser controls',
  sections: [
    {
      sectionId: BrowserControlManifestDefaults.Section.Management,
      title: 'Browser management',
      description: 'Core switch and management posture for the child browser policy.',
      visibleWhen: [],
      fields: [
        toggleField(
          BrowserControlManifestDefaults.Field.Enabled,
          BrowserControlWritesToPath.Enabled,
          'Enable browser controls',
          true
        ),
        selectField(
          BrowserControlManifestDefaults.Field.ManagementMode,
          BrowserControlWritesToPath.ManagementMode,
          'Management mode',
          'observe-only',
          [
            option('management-mode-observe-only', 'Observe only', 'observe-only'),
            option('management-mode-managed-browser', 'Managed browser', 'managed-browser'),
            option('management-mode-network-assisted', 'Network assisted', 'network-assisted'),
          ],
          [equals(BrowserControlWritesToPath.Enabled, true)]
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.DefaultPosture,
      title: 'Default posture',
      description: 'Decision taken when no specific browser rule matches.',
      visibleWhen: [equals(BrowserControlWritesToPath.Enabled, true)],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.DefaultPosture,
          BrowserControlWritesToPath.DefaultPosture,
          'Default posture',
          'limit',
          [
            option('default-posture-observe', 'Observe', 'observe'),
            option('default-posture-allow', 'Allow', 'allow'),
            option('default-posture-limit', 'Limit', 'limit'),
            option('default-posture-ask-parent', 'Ask parent', 'ask-parent'),
            option('default-posture-block', 'Block', 'block'),
          ],
          []
        ),
        numberField(
          BrowserControlManifestDefaults.Field.DailyBudgetMinutes,
          BrowserControlWritesToPath.DailyBudgetMinutes,
          'Daily browser budget',
          60,
          [defaultPosture('limit')]
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.ExactUrlRules,
      title: 'URL rule evidence',
      description: 'Proof needed before exact URL rules can be authored or enforced.',
      visibleWhen: [
        equals(BrowserControlWritesToPath.Enabled, true),
        equals(BrowserControlWritesToPath.ManagementMode, 'managed-browser'),
      ],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.ManagedBrowserMode,
          BrowserControlWritesToPath.ManagedBrowserMode,
          'Managed browser mode',
          'required-for-exact-rules',
          [
            option('managed-not-required', 'Not required', 'not-required'),
            option('managed-preferred', 'Preferred', 'preferred'),
            option('managed-required-exact', 'Required for exact rules', 'required-for-exact-rules'),
          ],
          []
        ),
        selectField(
          BrowserControlManifestDefaults.Field.RequiredProof,
          BrowserControlWritesToPath.RequiredProof,
          'Required proof',
          'fresh-managed-active-tab',
          [
            option('proof-network-domain', 'Network domain', 'network-domain'),
            option('proof-managed-active-tab', 'Managed active tab', 'managed-active-tab'),
            option('proof-fresh-managed-active-tab', 'Fresh managed active tab', 'fresh-managed-active-tab'),
          ],
          []
        ),
        selectField(
          BrowserControlManifestDefaults.Field.ProofFallback,
          BrowserControlWritesToPath.ProofFallback,
          'Proof fallback',
          'downgrade-to-domain',
          [
            option('fallback-downgrade-domain', 'Downgrade to domain', 'downgrade-to-domain'),
            option('fallback-ask-parent', 'Ask parent', 'ask-parent'),
            option('fallback-block-until-proof', 'Block until proof', 'block-until-proof'),
            option('fallback-observe-only', 'Observe only', 'observe-only'),
          ],
          []
        ),
        multiSelectField(
          BrowserControlManifestDefaults.Field.AllowedTargetTypes,
          BrowserControlWritesToPath.AllowedTargetTypes,
          'Rule target types',
          ['domain', 'url-prefix', 'exact-url'],
          [
            option('target-domain', 'Domain', 'domain'),
            option('target-url-prefix', 'URL prefix', 'url-prefix'),
            option('target-exact-url', 'Exact URL', 'exact-url'),
          ],
          []
        ),
      ],
    },
    {
      sectionId: BrowserControlManifestDefaults.Section.Reporting,
      title: 'Reporting and audit',
      description: 'Local reporting and audit retention controls.',
      visibleWhen: [equals(BrowserControlWritesToPath.Enabled, true)],
      fields: [
        selectField(
          BrowserControlManifestDefaults.Field.ReportState,
          BrowserControlWritesToPath.ReportState,
          'Reports',
          'weekly',
          [
            option('reports-disabled', 'Disabled', 'disabled'),
            option('reports-daily', 'Daily', 'daily'),
            option('reports-weekly', 'Weekly', 'weekly'),
            option('reports-on-demand', 'On demand', 'on-demand'),
          ],
          []
        ),
        selectField(
          BrowserControlManifestDefaults.Field.AuditState,
          BrowserControlWritesToPath.AuditState,
          'Audit trail',
          'local-only',
          [
            option('audit-disabled', 'Disabled', 'disabled'),
            option('audit-local-only', 'Local only', 'local-only'),
            option('audit-parent-visible', 'Parent visible', 'parent-visible'),
            option('audit-retained', 'Retained', 'retained'),
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

function equals(writesTo: BrowserControlSchemaKnownWritesToPath, expectedValue: BrowserControlFieldValue) {
  return BrowserControlConditionSchema.parse({
    kind: 'equals',
    writesTo,
    expectedValue,
    capabilityId: null,
    capabilityState: null,
    defaultPosture: null,
  });
}

function defaultPosture(defaultPostureValue: string) {
  return BrowserControlConditionSchema.parse({
    kind: 'default-posture',
    writesTo: null,
    expectedValue: null,
    capabilityId: null,
    capabilityState: null,
    defaultPosture: defaultPostureValue,
  });
}

function toggleField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: boolean
) {
  return BrowserControlAuthoringFieldSchema.parse({
    fieldId,
    label,
    description: null,
    controlKind: 'toggle',
    writesTo,
    defaultValue,
    options: [],
    visibleWhen: [],
    enabledWhen: [],
    required: true,
  });
}

function numberField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: number,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return BrowserControlAuthoringFieldSchema.parse({
    fieldId,
    label,
    description: null,
    controlKind: 'number',
    writesTo,
    defaultValue,
    options: [],
    visibleWhen,
    enabledWhen: [],
    required: true,
  });
}

function selectField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: string,
  options: ReadonlyArray<BrowserControlFieldOption>,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return BrowserControlAuthoringFieldSchema.parse({
    fieldId,
    label,
    description: null,
    controlKind: 'single-select',
    writesTo,
    defaultValue,
    options,
    visibleWhen,
    enabledWhen: [],
    required: true,
  });
}

function multiSelectField(
  fieldId: BrowserControlFieldId,
  writesTo: BrowserControlSchemaKnownWritesToPath,
  label: string,
  defaultValue: string[],
  options: ReadonlyArray<BrowserControlFieldOption>,
  visibleWhen: ReadonlyArray<BrowserControlCondition>
) {
  return BrowserControlAuthoringFieldSchema.parse({
    fieldId,
    label,
    description: null,
    controlKind: 'multi-select',
    writesTo,
    defaultValue,
    options,
    visibleWhen,
    enabledWhen: [],
    required: true,
  });
}
