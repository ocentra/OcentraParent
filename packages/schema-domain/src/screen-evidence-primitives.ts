import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from './effect';

export const ScreenEvidenceSchemaVersion = 1;
const BoundedSummaryText = NonEmptyStringSchema.pipe(
  Schema.filter((value) => value.length <= 1000 || 'Expected screen summary text within 1000 characters')
);
const BoundedSnippetText = NonEmptyStringSchema.pipe(
  Schema.filter((value) => value.length <= 240 || 'Expected OCR snippet text within 240 characters')
);
const NonNegativeInteger = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const ScreenEvidenceQueueJobIdSchema = brandedNonEmptyStringSchema('ScreenEvidenceQueueJobId');
export const ScreenEvidenceResultIdSchema = brandedNonEmptyStringSchema('ScreenEvidenceResultId');
export const ScreenEvidenceParentSettingRefSchema = brandedNonEmptyStringSchema('ScreenEvidenceParentSettingRef');
export const ScreenEvidenceLocalUserRefSchema = brandedNonEmptyStringSchema('ScreenEvidenceLocalUserRef');
export const ScreenEvidenceAdapterIdSchema = brandedNonEmptyStringSchema('ScreenEvidenceAdapterId');
export const ScreenEvidenceEncryptedImageRefSchema = brandedNonEmptyStringSchema('ScreenEvidenceEncryptedImageRef');
export const ScreenEvidenceImageDigestSchema = brandedNonEmptyStringSchema('ScreenEvidenceImageDigest');
export const ScreenEvidenceModelRuntimeRefSchema = brandedNonEmptyStringSchema('ScreenEvidenceModelRuntimeRef');
export const ScreenEvidenceModelIdSchema = brandedNonEmptyStringSchema('ScreenEvidenceModelId');
export const ScreenEvidenceTemplateVersionSchema = brandedNonEmptyStringSchema('ScreenEvidenceTemplateVersion');
export const ScreenEvidenceDeletionProofRefSchema = brandedNonEmptyStringSchema('ScreenEvidenceDeletionProofRef');
export const ScreenEvidenceRemoteApprovalRefSchema = brandedNonEmptyStringSchema('ScreenEvidenceRemoteApprovalRef');
export const ScreenEvidenceReasonSchema = BoundedSummaryText.pipe(Schema.brand('ScreenEvidenceReason'));
export const ScreenEvidenceSummaryTextSchema = BoundedSummaryText.pipe(Schema.brand('ScreenEvidenceSummaryText'));
export const ScreenEvidenceOcrSnippetTextSchema = BoundedSnippetText.pipe(Schema.brand('ScreenEvidenceOcrSnippetText'));
export const ScreenEvidenceCountSchema = withParser(NonNegativeInteger);
export const ScreenEvidenceConfidenceSchema = withParser(Schema.Number.pipe(Schema.between(0, 1)));
export const ScreenEvidenceSettingVersionSchema = withParser(
  Schema.Number.pipe(Schema.int(), Schema.between(1, 1000000))
);
export const ScreenEvidenceCadenceSecondsSchema = withParser(
  Schema.Number.pipe(Schema.int(), Schema.between(60, 3600))
);
export const ScreenEvidenceTtlSecondsSchema = withParser(Schema.Number.pipe(Schema.int(), Schema.between(60, 1800)));
export const ScreenEvidenceRetryCountSchema = withParser(Schema.Number.pipe(Schema.int(), Schema.between(0, 5)));
export const ScreenEvidenceSnippetLimitSchema = withParser(Schema.Number.pipe(Schema.int(), Schema.between(0, 20)));

export type ScreenEvidenceQueueJobId = Infer<typeof ScreenEvidenceQueueJobIdSchema>;
export type ScreenEvidenceResultId = Infer<typeof ScreenEvidenceResultIdSchema>;
export type ScreenEvidenceParentSettingRef = Infer<typeof ScreenEvidenceParentSettingRefSchema>;
export type ScreenEvidenceImageDigest = Infer<typeof ScreenEvidenceImageDigestSchema>;
export type ScreenEvidenceRemoteApprovalRef = Infer<typeof ScreenEvidenceRemoteApprovalRefSchema>;
