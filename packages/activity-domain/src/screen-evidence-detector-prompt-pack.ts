import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceIdSchema, ActivityTimestampSchema } from './primitives';
import {
  ScreenCategoryCandidateSchema,
  ScreenOcrTextSnippetSchema,
  ScreenRiskSignalCandidateSchema,
} from './screen-evidence-result';
import { ScreenEvidenceConfidenceSchema, ScreenEvidenceSummaryTextSchema } from './screen-evidence-primitives';
import {
  ScreenRedactionNoteSchema,
  ScreenRiskSignalSchema,
  ScreenUncertaintyReasonSchema,
  ScreenVisibleCategorySchema,
} from './screen-evidence-states';
import {
  ScreenDetectorForbiddenOutputFieldSchema,
  ScreenDetectorIdSchema,
  ScreenDetectorInputFieldSchema,
  ScreenDetectorOutputFieldSchema,
  ScreenDetectorPromptHashRefSchema,
  ScreenDetectorPromptPackDegradedStateSchema,
  ScreenDetectorPromptPackIdSchema,
  ScreenDetectorPromptPackSchemaVersion,
  ScreenDetectorPromptPackStatusSchema,
  ScreenDetectorPromptPackVersionSchema,
  ScreenDetectorRequiredIds,
} from './screen-evidence-detector-prompt-pack-values';

export * from './screen-evidence-detector-prompt-pack-values';

const RequiredFalse = Schema.Literal(false);
const RequiredTrue = Schema.Literal(true);
const RequiredEvidenceIdsSchema = Schema.Array(ActivityEvidenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one detector prompt source evidence id')
);
const RequiredInputFieldsSchema = Schema.Array(ScreenDetectorInputFieldSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected detector prompt input fields')
);
const RequiredOutputFieldsSchema = Schema.Array(ScreenDetectorOutputFieldSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected detector prompt output fields')
);
const RequiredForbiddenFieldsSchema = Schema.Array(ScreenDetectorForbiddenOutputFieldSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected detector prompt forbidden output fields')
);

const ScreenDetectorPromptDefinitionBaseSchema = Schema.Struct({
  detectorId: ScreenDetectorIdSchema,
  promptPackId: ScreenDetectorPromptPackIdSchema,
  promptPackVersion: ScreenDetectorPromptPackVersionSchema,
  promptHashRef: ScreenDetectorPromptHashRefSchema,
  targetCategories: Schema.Array(ScreenVisibleCategorySchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected detector target categories')
  ),
  targetRiskSignals: Schema.Array(ScreenRiskSignalSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected detector target risk signals')
  ),
  allowedInputFields: RequiredInputFieldsSchema,
  outputFields: RequiredOutputFieldsSchema,
  forbiddenOutputFields: RequiredForbiddenFieldsSchema,
  rawPromptTextIncluded: RequiredFalse,
  openEndedDescriptionAllowed: RequiredFalse,
  fullOcrTextAllowed: RequiredFalse,
  privateMessageTextAllowed: RequiredFalse,
  personalNamesAllowed: RequiredFalse,
  credentialTextAllowed: RequiredFalse,
  rawScreenshotRefAllowed: RequiredFalse,
  childSafetyOnly: RequiredTrue,
});
export const ScreenDetectorPromptDefinitionSchema = withParser(
  ScreenDetectorPromptDefinitionBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenDetectorPromptDefinitionIsConsistent(value) ||
        'Expected detector prompt to be schema-bound, detector-specific, and privacy-negative'
    )
  )
);

const ScreenDetectorPromptDefinitionsSchema = Schema.Array(ScreenDetectorPromptDefinitionSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected at least one screen detector prompt definition')
);
const ScreenDetectorPromptPackBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenDetectorPromptPackSchemaVersion),
  promptPackId: ScreenDetectorPromptPackIdSchema,
  promptPackVersion: ScreenDetectorPromptPackVersionSchema,
  publishedAt: ActivityTimestampSchema,
  status: ScreenDetectorPromptPackStatusSchema,
  detectors: ScreenDetectorPromptDefinitionsSchema,
  degradedStates: Schema.Array(ScreenDetectorPromptPackDegradedStateSchema),
  auditEvidenceIds: RequiredEvidenceIdsSchema,
});
export const ScreenDetectorPromptPackSchema = withParser(
  ScreenDetectorPromptPackBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenDetectorPromptPackIsConsistent(value) ||
        'Expected active detector prompt pack to include each required detector exactly once'
    )
  )
);

const ScreenDetectorPromptOutputBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(ScreenDetectorPromptPackSchemaVersion),
  detectorId: ScreenDetectorIdSchema,
  promptPackVersion: ScreenDetectorPromptPackVersionSchema,
  analyzedAt: ActivityTimestampSchema,
  sourceEvidenceIds: RequiredEvidenceIdsSchema,
  primaryCategory: ScreenVisibleCategorySchema,
  categoryCandidates: Schema.Array(ScreenCategoryCandidateSchema).pipe(
    Schema.filter((value) => value.length > 0 || 'Expected detector category candidates')
  ),
  riskSignals: Schema.Array(ScreenRiskSignalCandidateSchema),
  ocrSnippets: Schema.Array(ScreenOcrTextSnippetSchema),
  confidence: ScreenEvidenceConfidenceSchema,
  uncertaintyReasons: Schema.Array(ScreenUncertaintyReasonSchema),
  redactionNotes: Schema.Array(ScreenRedactionNoteSchema),
  childSafeSummary: Schema.Union(ScreenEvidenceSummaryTextSchema, Schema.Null),
  privateMessageTextIncluded: RequiredFalse,
  personalNamesIncluded: RequiredFalse,
  credentialTextIncluded: RequiredFalse,
  fullOcrTextIncluded: RequiredFalse,
  rawScreenshotRefIncluded: RequiredFalse,
  finalPolicyActionClaimed: RequiredFalse,
  enforcementActionClaimed: RequiredFalse,
});
export const ScreenDetectorPromptOutputSchema = withParser(
  ScreenDetectorPromptOutputBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenDetectorPromptOutputIsConsistent(value) ||
        'Expected detector output to expose uncertainty for low-confidence or unknown results'
    )
  )
);

export const decodeScreenDetectorPromptPack = Schema.decodeUnknownSync(ScreenDetectorPromptPackSchema);
export const decodeScreenDetectorPromptOutput = Schema.decodeUnknownSync(ScreenDetectorPromptOutputSchema);

export type ScreenDetectorPromptDefinition = Infer<typeof ScreenDetectorPromptDefinitionSchema>;
export type ScreenDetectorPromptPack = Infer<typeof ScreenDetectorPromptPackSchema>;
export type ScreenDetectorPromptOutput = Infer<typeof ScreenDetectorPromptOutputSchema>;

function screenDetectorPromptDefinitionIsConsistent(value: Infer<typeof ScreenDetectorPromptDefinitionBaseSchema>) {
  return (
    containsAllRequiredOutputFields(value.outputFields) &&
    containsAllRequiredForbiddenFields(value.forbiddenOutputFields) &&
    value.allowedInputFields.includes('sourceEvidenceRefs')
  );
}

function screenDetectorPromptPackIsConsistent(value: Infer<typeof ScreenDetectorPromptPackBaseSchema>) {
  const detectorIds = value.detectors.map((detector) => detector.detectorId);
  const uniqueIds = new Set(detectorIds);
  if (uniqueIds.size !== detectorIds.length) {
    return false;
  }
  if (value.status === 'active') {
    return (
      ScreenDetectorRequiredIds.every((detectorId) => uniqueIds.has(detectorId)) && value.degradedStates.length === 0
    );
  }
  return value.degradedStates.length > 0;
}

function screenDetectorPromptOutputIsConsistent(value: Infer<typeof ScreenDetectorPromptOutputBaseSchema>) {
  if (value.primaryCategory === 'unknown' || value.confidence < 0.5) {
    return value.uncertaintyReasons.length > 0;
  }
  return true;
}

function containsAllRequiredOutputFields(fields: ReadonlyArray<string>) {
  return ['detectorId', 'categoryCandidates', 'riskSignals', 'confidence', 'uncertaintyReasons', 'evidenceRefs'].every(
    (field) => fields.includes(field)
  );
}

function containsAllRequiredForbiddenFields(fields: ReadonlyArray<string>) {
  return ['privateMessageText', 'personName', 'credentialText', 'fullOcrText', 'rawScreenshotRef'].every((field) =>
    fields.includes(field)
  );
}
