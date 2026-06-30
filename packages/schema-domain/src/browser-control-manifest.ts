import { type Infer, Schema, withParser, NonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
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
  type BrowserControlFieldValue,
  type BrowserControlSchemaKnownWritesToPath,
} from '@ocentra-parent/schema-domain/browser-control-values';
import { ParentContractSchemaVersionSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  GeneratedBrowserControlManifestDefaults,
  browserControlConditionsAreMetGenerated,
  browserControlFieldDefaultMatchesOptionsGenerated,
  browserControlFieldIdsAreUniqueGenerated,
  browserControlSectionIdsAreUniqueGenerated,
  browserControlWritesToIsKnownGenerated,
} from './generated/browser-policy-control-catalog-helpers';

export const BrowserControlFieldOptionSchema = withParser(
  Schema.Struct({
    optionId: BrowserControlOptionIdSchema,
    label: NonEmptyStringSchema,
    value: BrowserControlFieldValueSchema,
    description: Schema.Union(NonEmptyStringSchema, Schema.Null),
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
  }).pipe(
    Schema.filter(
      (condition) =>
        browserControlWritesToIsKnownGenerated(condition.writesTo) ||
        'Expected browser-control conditions to use a known writesTo path'
    )
  )
);

export const BrowserControlAuthoringFieldBaseSchema = Schema.Struct({
  fieldId: BrowserControlFieldIdSchema,
  label: NonEmptyStringSchema,
  description: Schema.Union(NonEmptyStringSchema, Schema.Null),
  controlKind: BrowserControlKindSchema,
  writesTo: BrowserControlSchemaKnownWritesToPathSchema,
  defaultValue: BrowserControlFieldValueSchema,
  options: Schema.Array(BrowserControlFieldOptionSchema),
  visibleWhen: Schema.Array(BrowserControlConditionSchema),
  enabledWhen: Schema.Array(BrowserControlConditionSchema),
  required: Schema.Boolean,
});

export const BrowserControlAuthoringFieldSchema = withParser(
  BrowserControlAuthoringFieldBaseSchema.pipe(
    Schema.filter(
      (field) =>
        browserControlWritesToIsKnownGenerated(field.writesTo) ||
        'Expected browser-control authoring fields to use a known writesTo path'
    ),
    Schema.filter(
      (field) =>
        browserControlFieldDefaultMatchesOptionsGenerated(field) ||
        'Expected option-backed browser controls to include their default value in options'
    )
  )
);

export const BrowserControlAuthoringSectionBaseSchema = Schema.Struct({
  sectionId: BrowserControlSectionIdSchema,
  title: NonEmptyStringSchema,
  description: Schema.Union(NonEmptyStringSchema, Schema.Null),
  visibleWhen: Schema.Array(BrowserControlConditionSchema),
  fields: Schema.Array(BrowserControlAuthoringFieldSchema),
});

export const BrowserControlAuthoringSectionSchema = withParser(
  BrowserControlAuthoringSectionBaseSchema.pipe(
    Schema.filter(
      (section) =>
        browserControlFieldIdsAreUniqueGenerated(section.fields) ||
        'Expected browser-control authoring field ids to be unique'
    )
  )
);

export const BrowserControlAuthoringManifestBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  manifestId: BrowserControlManifestIdSchema,
  title: NonEmptyStringSchema,
  sections: Schema.Array(BrowserControlAuthoringSectionSchema),
});

export const BrowserControlAuthoringManifestSchema = withParser(
  BrowserControlAuthoringManifestBaseSchema.pipe(
    Schema.filter(
      (manifest) =>
        browserControlSectionIdsAreUniqueGenerated(manifest.sections) ||
        'Expected browser-control section ids to be unique'
    )
  )
);

export type BrowserControlFieldOption = Infer<typeof BrowserControlFieldOptionSchema>;
export type BrowserControlCondition = Infer<typeof BrowserControlConditionSchema>;
export type BrowserControlAuthoringField = Infer<typeof BrowserControlAuthoringFieldSchema>;
export type BrowserControlAuthoringSection = Infer<typeof BrowserControlAuthoringSectionSchema>;
export type BrowserControlAuthoringManifest = Infer<typeof BrowserControlAuthoringManifestSchema>;

export const BrowserControlManifestDefaults = {
  ManifestId: BrowserControlManifestIdSchema.parse(GeneratedBrowserControlManifestDefaults.ManifestId),
  Section: Object.fromEntries(
    Object.entries(GeneratedBrowserControlManifestDefaults.Section).map(([key, value]) => [
      key,
      BrowserControlSectionIdSchema.parse(value),
    ])
  ) as { readonly [K in keyof typeof GeneratedBrowserControlManifestDefaults.Section]: BrowserControlSectionId },
  Field: Object.fromEntries(
    Object.entries(GeneratedBrowserControlManifestDefaults.Field).map(([key, value]) => [
      key,
      BrowserControlFieldIdSchema.parse(value),
    ])
  ) as { readonly [K in keyof typeof GeneratedBrowserControlManifestDefaults.Field]: BrowserControlFieldId },
} as const;

export const decodeBrowserControlAuthoringManifest = Schema.decodeUnknownSync(BrowserControlAuthoringManifestSchema);

export function browserControlVisibleSectionIds(
  manifest: BrowserControlAuthoringManifest,
  values: Record<string, BrowserControlFieldValue>
): BrowserControlSectionId[] {
  return manifest.sections
    .filter((section) => browserControlConditionsAreMetGenerated(section.visibleWhen, values))
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
