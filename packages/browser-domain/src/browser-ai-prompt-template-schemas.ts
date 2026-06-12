import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from '@ocentra-parent/evidence-domain/primitives';
import { BrowserAiPromptTemplateSchema } from './browser-ai-analysis-schemas';
import {
  BrowserAiModelRuntimeRefSchema,
  BrowserAiPromptTemplateVersionSchema,
  BrowserAiRequestedTaskSchema,
  BrowserPolicyVersionRefSchema,
} from './browser-ai-analysis-values';
import {
  BrowserAiPromptTemplateChangeReasonSchema,
  BrowserAiPromptTemplateChangeRefSchema,
  BrowserAiPromptTemplateHashRefSchema,
  BrowserAiPromptTemplateRegistryIdSchema,
  BrowserAiPromptTemplateSelectionDegradedStateSchema,
  BrowserAiPromptTemplateSelectionStateSchema,
  type BrowserAiPromptTemplateStatus,
  BrowserAiPromptTemplateStatusSchema,
} from './browser-ai-prompt-template-values';

const OptionalPromptTemplateVersionSchema = Schema.Union(BrowserAiPromptTemplateVersionSchema, Schema.Null);
const OptionalPromptTimestampSchema = Schema.Union(ActivityTimestampSchema, Schema.Null);
const PromptTemplateEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one prompt template audit evidence id')
);
const PromptTemplateChangeReasonsSchema = Schema.Array(BrowserAiPromptTemplateChangeReasonSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one prompt template change reason')
);
const PromptTemplateModelRefsSchema = Schema.Array(BrowserAiModelRuntimeRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one prompt template model runtime ref')
);
const PromptTemplatePolicyRefsSchema = Schema.Array(BrowserPolicyVersionRefSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one prompt template policy version ref')
);
const PromptTemplateDegradedStatesSchema = Schema.Array(BrowserAiPromptTemplateSelectionDegradedStateSchema);

export const BrowserAiPromptTemplateVersioningSchemaVersion = 1;

const BrowserAiPromptTemplateVersionRecordBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiPromptTemplateVersioningSchemaVersion),
  template: BrowserAiPromptTemplateSchema,
  status: BrowserAiPromptTemplateStatusSchema,
  promptHashRef: BrowserAiPromptTemplateHashRefSchema,
  changeRef: BrowserAiPromptTemplateChangeRefSchema,
  versionedAt: ActivityTimestampSchema,
  validFrom: ActivityTimestampSchema,
  validUntil: OptionalPromptTimestampSchema,
  previousPromptTemplateVersion: OptionalPromptTemplateVersionSchema,
  supersededByPromptTemplateVersion: OptionalPromptTemplateVersionSchema,
  changeReasons: PromptTemplateChangeReasonsSchema,
  compatibleModelRuntimeRefs: PromptTemplateModelRefsSchema,
  policyVersionRefs: PromptTemplatePolicyRefsSchema,
  auditEvidenceIds: PromptTemplateEvidenceIdsSchema,
  invalidatesMemory: Schema.Boolean,
  inputFieldRefsChanged: Schema.Boolean,
});
export const BrowserAiPromptTemplateVersionRecordSchema = withParser(
  BrowserAiPromptTemplateVersionRecordBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiPromptTemplateVersionRecordIsConsistent(value) ||
        'Expected prompt template version record to be lifecycle-valid and memory-invalidation aware'
    )
  )
);

const PromptTemplateVersionRecordsSchema = Schema.Array(BrowserAiPromptTemplateVersionRecordSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one prompt template version record')
);
const BrowserAiPromptTemplateRegistryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiPromptTemplateVersioningSchemaVersion),
  registryId: BrowserAiPromptTemplateRegistryIdSchema,
  publishedAt: ActivityTimestampSchema,
  versions: PromptTemplateVersionRecordsSchema,
});
export const BrowserAiPromptTemplateRegistrySchema = withParser(
  BrowserAiPromptTemplateRegistryBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiPromptTemplateRegistryIsConsistent(value) ||
        'Expected prompt template registry to have one active version per task and model runtime'
    )
  )
);

const BrowserAiPromptTemplateSelectionBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(BrowserAiPromptTemplateVersioningSchemaVersion),
  selectedAt: ActivityTimestampSchema,
  requestedTask: BrowserAiRequestedTaskSchema,
  modelRuntimeRef: BrowserAiModelRuntimeRefSchema,
  policyVersionRef: BrowserPolicyVersionRefSchema,
  selectionState: BrowserAiPromptTemplateSelectionStateSchema,
  selectedPromptTemplate: Schema.Union(BrowserAiPromptTemplateSchema, Schema.Null),
  degradedStates: PromptTemplateDegradedStatesSchema,
  auditEvidenceIds: PromptTemplateEvidenceIdsSchema,
  promptChangedInvalidatesMemory: Schema.Boolean,
});
export const BrowserAiPromptTemplateSelectionSchema = withParser(
  BrowserAiPromptTemplateSelectionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        browserAiPromptTemplateSelectionIsConsistent(value) ||
        'Expected prompt template selection to choose an active version or fail closed'
    )
  )
);

const BrowserAiPromptTemplateSelectionRequestSchema = withParser(
  Schema.Struct({
    registry: BrowserAiPromptTemplateRegistrySchema,
    requestedTask: BrowserAiRequestedTaskSchema,
    modelRuntimeRef: BrowserAiModelRuntimeRefSchema,
    policyVersionRef: BrowserPolicyVersionRefSchema,
    selectedAt: ActivityTimestampSchema,
    auditEvidenceIds: PromptTemplateEvidenceIdsSchema,
  })
);

export const decodeBrowserAiPromptTemplateVersionRecord = Schema.decodeUnknownSync(
  BrowserAiPromptTemplateVersionRecordSchema
);
export const decodeBrowserAiPromptTemplateRegistry = Schema.decodeUnknownSync(BrowserAiPromptTemplateRegistrySchema);
export const decodeBrowserAiPromptTemplateSelection = Schema.decodeUnknownSync(BrowserAiPromptTemplateSelectionSchema);

export function selectBrowserAiPromptTemplate(
  request: Infer<typeof BrowserAiPromptTemplateSelectionRequestSchema>
): BrowserAiPromptTemplateSelection {
  const parsed = BrowserAiPromptTemplateSelectionRequestSchema.parse(request);
  const activeByTask = parsed.registry.versions.filter(
    (version) => version.status === 'active' && version.template.requestedTask === parsed.requestedTask
  );
  const activeByModel = activeByTask.filter((version) =>
    version.compatibleModelRuntimeRefs.includes(parsed.modelRuntimeRef)
  );
  const activeByPolicy = activeByModel.filter((version) => version.policyVersionRefs.includes(parsed.policyVersionRef));
  const selected = activeByPolicy.length === 1 ? activeByPolicy[0] : null;

  return BrowserAiPromptTemplateSelectionSchema.parse({
    schemaVersion: BrowserAiPromptTemplateVersioningSchemaVersion,
    selectedAt: parsed.selectedAt,
    requestedTask: parsed.requestedTask,
    modelRuntimeRef: parsed.modelRuntimeRef,
    policyVersionRef: parsed.policyVersionRef,
    selectionState: selected === null ? 'manual-required' : 'selected',
    selectedPromptTemplate: selected?.template ?? null,
    degradedStates: selected === null ? promptTemplateSelectionDegradedStates(activeByTask, activeByModel) : [],
    auditEvidenceIds: parsed.auditEvidenceIds,
    promptChangedInvalidatesMemory: selected?.invalidatesMemory ?? false,
  });
}

export type BrowserAiPromptTemplateVersionRecord = Infer<typeof BrowserAiPromptTemplateVersionRecordSchema>;
export type BrowserAiPromptTemplateRegistry = Infer<typeof BrowserAiPromptTemplateRegistrySchema>;
export type BrowserAiPromptTemplateSelection = Infer<typeof BrowserAiPromptTemplateSelectionSchema>;

function browserAiPromptTemplateVersionRecordIsConsistent(
  value: Infer<typeof BrowserAiPromptTemplateVersionRecordBaseSchema>
) {
  if (!promptTemplateChangeIsMemoryAware(value)) {
    return false;
  }
  if (value.status === 'active') {
    return value.validUntil === null && value.supersededByPromptTemplateVersion === null;
  }
  return promptTemplateInactiveLifecycleIsConsistent(
    value.status,
    value.validUntil,
    value.supersededByPromptTemplateVersion
  );
}

function browserAiPromptTemplateRegistryIsConsistent(value: Infer<typeof BrowserAiPromptTemplateRegistryBaseSchema>) {
  const activeTaskModelKeys = new Set<string>();
  for (const version of value.versions) {
    if (version.status !== 'active') {
      continue;
    }
    for (const modelRuntimeRef of version.compatibleModelRuntimeRefs) {
      const key = `${version.template.requestedTask}:${modelRuntimeRef}`;
      if (activeTaskModelKeys.has(key)) {
        return false;
      }
      activeTaskModelKeys.add(key);
    }
  }
  return true;
}

function browserAiPromptTemplateSelectionIsConsistent(value: Infer<typeof BrowserAiPromptTemplateSelectionBaseSchema>) {
  if (value.selectionState === 'selected') {
    return value.selectedPromptTemplate !== null && value.degradedStates.length === 0;
  }
  return (
    value.selectedPromptTemplate === null && value.degradedStates.length > 0 && !value.promptChangedInvalidatesMemory
  );
}

function promptTemplateChangeIsMemoryAware(value: Infer<typeof BrowserAiPromptTemplateVersionRecordBaseSchema>) {
  if (value.previousPromptTemplateVersion !== null && !value.invalidatesMemory) {
    return false;
  }
  if (value.inputFieldRefsChanged) {
    return value.invalidatesMemory && value.changeReasons.includes('input-field-change');
  }
  return true;
}

function promptTemplateInactiveLifecycleIsConsistent(
  status: BrowserAiPromptTemplateStatus,
  validUntil: Infer<typeof OptionalPromptTimestampSchema>,
  supersededByPromptTemplateVersion: Infer<typeof OptionalPromptTemplateVersionSchema>
) {
  if (status === 'draft') {
    return validUntil === null && supersededByPromptTemplateVersion === null;
  }
  return validUntil !== null && supersededByPromptTemplateVersion !== null;
}

function promptTemplateSelectionDegradedStates(
  activeByTask: BrowserAiPromptTemplateVersionRecord[],
  activeByModel: BrowserAiPromptTemplateVersionRecord[]
) {
  if (activeByTask.length === 0) {
    return ['template-missing'] as const;
  }
  if (activeByModel.length === 0) {
    return ['model-unsupported'] as const;
  }
  return ['policy-version-unsupported'] as const;
}
