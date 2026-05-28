import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BrowserControlCapabilityIdSchema,
  BrowserControlFieldIdSchema,
  BrowserControlManifestIdSchema,
  BrowserControlOptionIdSchema,
  BrowserControlSectionIdSchema,
  type BrowserControlFieldId,
  type BrowserControlSectionId,
} from './browser-control-identifiers';
import {
  BrowserControlCapabilityStateSchema,
  BrowserControlConditionKindSchema,
  BrowserControlDefaultPostureSchema,
  BrowserControlFieldValueSchema,
  BrowserControlKindSchema,
  BrowserControlSchemaKnownWritesToPathSchema,
  BrowserControlWritesToPath,
  type BrowserControlFieldValue,
  type BrowserControlSchemaKnownWritesToPath,
} from './browser-control-values';
import { ParentContractSchemaVersionSchema } from './reference-primitives';

const BrowserControlManifestTextSchema = Schema.String.pipe(Schema.minLength(1));

export const BrowserControlFieldOptionSchema = withParser(
  Schema.Struct({
    optionId: BrowserControlOptionIdSchema,
    label: BrowserControlManifestTextSchema,
    value: BrowserControlFieldValueSchema,
    description: Schema.Union(BrowserControlManifestTextSchema, Schema.Null),
  })
);

export const BrowserControlConditionSchema = withParser(
  Schema.Struct({
    kind: BrowserControlConditionKindSchema,
    writesTo: Schema.Union(BrowserControlSchemaKnownWritesToPathSchema, Schema.Null),
    expectedValue: Schema.Union(BrowserControlFieldValueSchema, Schema.Null),
    capabilityId: Schema.Union(BrowserControlCapabilityIdSchema, Schema.Null),
    capabilityState: Schema.Union(BrowserControlCapabilityStateSchema, Schema.Null),
    defaultPosture: Schema.Union(BrowserControlDefaultPostureSchema, Schema.Null),
  })
);

export const BrowserControlAuthoringFieldBaseSchema = Schema.Struct({
  fieldId: BrowserControlFieldIdSchema,
  label: BrowserControlManifestTextSchema,
  description: Schema.Union(BrowserControlManifestTextSchema, Schema.Null),
  controlKind: BrowserControlKindSchema,
  writesTo: BrowserControlSchemaKnownWritesToPathSchema,
  defaultValue: BrowserControlFieldValueSchema,
  options: Schema.Array(BrowserControlFieldOptionSchema),
  visibleWhen: Schema.Array(BrowserControlConditionSchema),
  enabledWhen: Schema.Array(BrowserControlConditionSchema),
  required: Schema.Boolean,
});

type BrowserControlAuthoringFieldCandidate = Infer<typeof BrowserControlAuthoringFieldBaseSchema>;

export const BrowserControlAuthoringFieldSchema = withParser(
  BrowserControlAuthoringFieldBaseSchema.pipe(
    Schema.filter(
      (field) =>
        browserControlFieldDefaultMatchesOptions(field) ||
        'Expected option-backed browser controls to include their default value in options'
    )
  )
);

export const BrowserControlAuthoringSectionBaseSchema = Schema.Struct({
  sectionId: BrowserControlSectionIdSchema,
  title: BrowserControlManifestTextSchema,
  description: Schema.Union(BrowserControlManifestTextSchema, Schema.Null),
  visibleWhen: Schema.Array(BrowserControlConditionSchema),
  fields: Schema.Array(BrowserControlAuthoringFieldSchema),
});

type BrowserControlAuthoringSectionCandidate = Infer<typeof BrowserControlAuthoringSectionBaseSchema>;

export const BrowserControlAuthoringSectionSchema = withParser(
  BrowserControlAuthoringSectionBaseSchema.pipe(
    Schema.filter(
      (section) =>
        browserControlFieldIdsAreUnique(section.fields) || 'Expected browser-control authoring field ids to be unique'
    )
  )
);

export const BrowserControlAuthoringManifestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  manifestId: BrowserControlManifestIdSchema,
  title: BrowserControlManifestTextSchema,
  sections: Schema.Array(BrowserControlAuthoringSectionSchema),
});

export const BrowserControlAuthoringManifestSchema = withParser(
  BrowserControlAuthoringManifestBaseSchema.pipe(
    Schema.filter(
      (manifest) =>
        browserControlSectionIdsAreUnique(manifest.sections) || 'Expected browser-control section ids to be unique'
    )
  )
);

export type BrowserControlFieldOption = Infer<typeof BrowserControlFieldOptionSchema>;
export type BrowserControlCondition = Infer<typeof BrowserControlConditionSchema>;
export type BrowserControlAuthoringField = Infer<typeof BrowserControlAuthoringFieldSchema>;
export type BrowserControlAuthoringSection = Infer<typeof BrowserControlAuthoringSectionSchema>;
export type BrowserControlAuthoringManifest = Infer<typeof BrowserControlAuthoringManifestSchema>;

export const BrowserControlManifestDefaults = {
  ManifestId: BrowserControlManifestIdSchema.parse('browser-control-authoring-v1'),
  Section: {
    Management: BrowserControlSectionIdSchema.parse('browser-management'),
    ManagedBrowser: BrowserControlSectionIdSchema.parse('managed-browser'),
    UnmanagedBrowser: BrowserControlSectionIdSchema.parse('unmanaged-browser'),
    UrlTabEvidence: BrowserControlSectionIdSchema.parse('url-tab-evidence'),
    WebRules: BrowserControlSectionIdSchema.parse('web-rules'),
    Budgets: BrowserControlSectionIdSchema.parse('budgets'),
    Downloads: BrowserControlSectionIdSchema.parse('downloads'),
    Approvals: BrowserControlSectionIdSchema.parse('approvals'),
    Reports: BrowserControlSectionIdSchema.parse('reports'),
    Audit: BrowserControlSectionIdSchema.parse('audit'),
  },
  Field: {
    Enabled: BrowserControlFieldIdSchema.parse('browser.enabled'),
    DefaultPosture: BrowserControlFieldIdSchema.parse('browser.defaultPosture'),
    ManagementMode: BrowserControlFieldIdSchema.parse('browser.managementMode'),
    ManagedBrowserMode: BrowserControlFieldIdSchema.parse('managedBrowser.mode'),
    ManagedBrowserAllowedFamilies: BrowserControlFieldIdSchema.parse('managedBrowser.allowedFamilies'),
    ManagedBrowserLaunchMode: BrowserControlFieldIdSchema.parse('managedBrowser.launchMode'),
    ManagedBrowserProfileMode: BrowserControlFieldIdSchema.parse('managedBrowser.profileMode'),
    ManagedBrowserBridgeRequirements: BrowserControlFieldIdSchema.parse('managedBrowser.bridgeRequirements'),
    ManagedBrowserIntegrationMechanisms: BrowserControlFieldIdSchema.parse('managedBrowser.integrationMechanisms'),
    UnmanagedBrowserMode: BrowserControlFieldIdSchema.parse('unmanagedBrowser.mode'),
    UnmanagedBrowserGraceSeconds: BrowserControlFieldIdSchema.parse('unmanagedBrowser.graceSeconds'),
    UnmanagedBrowserAllowRecoverLaunchUrl: BrowserControlFieldIdSchema.parse('unmanagedBrowser.allowRecoverLaunchUrl'),
    UnmanagedBrowserClassificationTargets: BrowserControlFieldIdSchema.parse('unmanagedBrowser.classificationTargets'),
    EvidenceUrlScope: BrowserControlFieldIdSchema.parse('evidence.urlScope'),
    RequiredProof: BrowserControlFieldIdSchema.parse('evidence.requiredProof'),
    WhenProofUnavailable: BrowserControlFieldIdSchema.parse('evidence.whenProofUnavailable'),
    EvidenceNeverCollect: BrowserControlFieldIdSchema.parse('evidence.neverCollect'),
    AllowedTargetTypes: BrowserControlFieldIdSchema.parse('rules.allowedTargetTypes'),
    AllowedActions: BrowserControlFieldIdSchema.parse('rules.allowedActions'),
    RuleItems: BrowserControlFieldIdSchema.parse('rules.items'),
    BudgetsEnabled: BrowserControlFieldIdSchema.parse('budgets.enabled'),
    DailyBudgetMinutes: BrowserControlFieldIdSchema.parse('budgets.defaultDailyMinutes'),
    BudgetCountingMode: BrowserControlFieldIdSchema.parse('budgets.countingMode'),
    DownloadMode: BrowserControlFieldIdSchema.parse('downloads.mode'),
    DownloadBlockedTypes: BrowserControlFieldIdSchema.parse('downloads.blockedTypes'),
    ApprovalRequiredFor: BrowserControlFieldIdSchema.parse('approvals.requiredFor'),
    ApprovalUnansweredDefault: BrowserControlFieldIdSchema.parse('approvals.unansweredDefault'),
    ReportVisibleFields: BrowserControlFieldIdSchema.parse('reports.visibleFields'),
    RetentionExactUrl: BrowserControlFieldIdSchema.parse('retention.exactUrl'),
    CustodyAllowedUses: BrowserControlFieldIdSchema.parse('custody.allowedUses'),
    AuditRequiredFields: BrowserControlFieldIdSchema.parse('audit.requiredFields'),
  },
} as const;

export const decodeBrowserControlAuthoringManifest = Schema.decodeUnknownSync(BrowserControlAuthoringManifestSchema);

export function browserControlVisibleSectionIds(
  manifest: BrowserControlAuthoringManifest,
  values: Record<string, BrowserControlFieldValue>
): BrowserControlSectionId[] {
  return manifest.sections
    .filter((section) => browserControlConditionsAreMet(section.visibleWhen, values))
    .map((section) => section.sectionId);
}

export function browserControlManifestAllowsField(
  manifest: BrowserControlAuthoringManifest,
  fieldId: BrowserControlFieldId
): boolean {
  return manifest.sections.some((section) => section.fields.some((field) => field.fieldId === fieldId));
}

export function browserControlManifestAllowsWritesTo(
  manifest: BrowserControlAuthoringManifest,
  writesTo: BrowserControlSchemaKnownWritesToPath
): boolean {
  return manifest.sections.some((section) => section.fields.some((field) => field.writesTo === writesTo));
}

function browserControlConditionsAreMet(
  conditions: ReadonlyArray<BrowserControlCondition>,
  values: Record<string, BrowserControlFieldValue>
): boolean {
  return conditions.every((condition) => browserControlConditionIsMet(condition, values));
}

function browserControlConditionIsMet(
  condition: BrowserControlCondition,
  values: Record<string, BrowserControlFieldValue>
): boolean {
  if (condition.kind === 'default-posture') {
    return values[BrowserControlWritesToPath.DefaultPosture] === condition.defaultPosture;
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

function browserControlFieldDefaultMatchesOptions(field: BrowserControlAuthoringFieldCandidate): boolean {
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

function browserControlFieldIdsAreUnique(fields: ReadonlyArray<BrowserControlAuthoringFieldCandidate>): boolean {
  return new Set(fields.map((field) => field.fieldId)).size === fields.length;
}

function browserControlSectionIdsAreUnique(sections: ReadonlyArray<BrowserControlAuthoringSectionCandidate>): boolean {
  return new Set(sections.map((section) => section.sectionId)).size === sections.length;
}
