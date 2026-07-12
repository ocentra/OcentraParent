/* generated from crates/schema/src/browser_generated_values_ts.rs */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import {
  GeneratedBrowserControlManifestDefaults,
  browserControlManifestAllowsFieldGenerated,
  browserControlManifestAllowsWritesToGenerated,
  browserControlManifestVisibleSectionIdsGenerated,
} from './generated-browser-policy-control-catalog-helpers';
import {
  BrowserControlManifestIdSchema,
  BrowserControlSectionIdSchema,
  BrowserControlFieldIdSchema,
} from './generated-browser-control-identifiers';
import {
  BrowserControlFieldValueSchema,
  BrowserControlSchemaKnownWritesToPathSchema,
  type BrowserControlFieldValue,
} from './browser-control-values';

export const BrowserControlFieldOptionSchema = withParser(
  Schema.Struct({
    optionId: brandedNonEmptyStringSchema('BrowserControlOptionId'),
    label: brandedNonEmptyStringSchema('BrowserControlOptionLabel'),
    value: BrowserControlFieldValueSchema,
    description: Schema.Union(brandedNonEmptyStringSchema('BrowserControlOptionDescription'), Schema.Null),
  })
);

export const BrowserControlConditionSchema = withParser(
  Schema.Struct({
    kind: brandedNonEmptyStringSchema('BrowserControlConditionKind'),
    writesTo: Schema.Union(BrowserControlSchemaKnownWritesToPathSchema, Schema.Null),
    expectedValue: Schema.Union(BrowserControlFieldValueSchema, Schema.Null),
    capabilityId: Schema.Union(brandedNonEmptyStringSchema('BrowserControlCapabilityId'), Schema.Null),
    capabilityState: Schema.Union(brandedNonEmptyStringSchema('BrowserControlCapabilityState'), Schema.Null),
    defaultPosture: Schema.Union(brandedNonEmptyStringSchema('BrowserControlDefaultPosture'), Schema.Null),
  })
);

const BrowserControlAuthoringFieldBaseSchema = Schema.Struct({
  fieldId: BrowserControlFieldIdSchema,
  label: brandedNonEmptyStringSchema('BrowserControlFieldLabel'),
  description: Schema.Union(brandedNonEmptyStringSchema('BrowserControlFieldDescription'), Schema.Null),
  controlKind: brandedNonEmptyStringSchema('BrowserControlKind'),
  writesTo: BrowserControlSchemaKnownWritesToPathSchema,
  defaultValue: BrowserControlFieldValueSchema,
  options: Schema.Array(BrowserControlFieldOptionSchema),
  visibleWhen: Schema.Array(BrowserControlConditionSchema),
  enabledWhen: Schema.Array(BrowserControlConditionSchema),
  required: Schema.Boolean,
});

export const BrowserControlAuthoringFieldSchema = withParser(BrowserControlAuthoringFieldBaseSchema);

const BrowserControlAuthoringSectionBaseSchema = Schema.Struct({
  sectionId: BrowserControlSectionIdSchema,
  title: brandedNonEmptyStringSchema('BrowserControlSectionTitle'),
  description: Schema.Union(brandedNonEmptyStringSchema('BrowserControlSectionDescription'), Schema.Null),
  visibleWhen: Schema.Array(BrowserControlConditionSchema),
  fields: Schema.Array(BrowserControlAuthoringFieldSchema),
});

export const BrowserControlAuthoringSectionSchema = withParser(BrowserControlAuthoringSectionBaseSchema);

const BrowserControlAuthoringManifestBaseSchema = Schema.Struct({
  schemaVersion: brandedNonEmptyStringSchema('ParentContractSchemaVersion'),
  manifestId: BrowserControlManifestIdSchema,
  title: brandedNonEmptyStringSchema('BrowserControlManifestTitle'),
  sections: Schema.Array(BrowserControlAuthoringSectionSchema),
});

export const BrowserControlAuthoringManifestSchema = withParser(BrowserControlAuthoringManifestBaseSchema);

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
  ) as {
    readonly [K in keyof typeof GeneratedBrowserControlManifestDefaults.Section]: typeof BrowserControlSectionIdSchema.Type;
  },
  Field: Object.fromEntries(
    Object.entries(GeneratedBrowserControlManifestDefaults.Field).map(([key, value]) => [
      key,
      BrowserControlFieldIdSchema.parse(value),
    ])
  ) as {
    readonly [K in keyof typeof GeneratedBrowserControlManifestDefaults.Field]: typeof BrowserControlFieldIdSchema.Type;
  },
} as const;

export const decodeBrowserControlAuthoringManifest = Schema.decodeUnknownSync(BrowserControlAuthoringManifestSchema);

export function browserControlVisibleSectionIds(
  manifest: BrowserControlAuthoringManifest,
  values: Record<string, BrowserControlFieldValue>
): (typeof BrowserControlSectionIdSchema.Type)[] {
  return browserControlManifestVisibleSectionIdsGenerated(
    manifest,
    values
  ) as (typeof BrowserControlSectionIdSchema.Type)[];
}

export function browserControlManifestAllowsField(
  manifest: BrowserControlAuthoringManifest,
  fieldId: typeof BrowserControlFieldIdSchema.Type
): boolean {
  return browserControlManifestAllowsFieldGenerated(manifest, fieldId);
}

export function browserControlManifestAllowsWritesTo(
  manifest: BrowserControlAuthoringManifest,
  writesTo: typeof BrowserControlSchemaKnownWritesToPathSchema.Type
): boolean {
  return browserControlManifestAllowsWritesToGenerated(manifest, writesTo);
}
