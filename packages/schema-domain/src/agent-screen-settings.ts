import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import { ScreenAnalysisParentSettingSchema } from './screen-evidence-settings';

export const ScreenSettingsSchemaVersion = 1 as const;

export const ScreenSettingsUpdateKindSchema = withParser(Schema.Literal('get', 'replace'));
export const ScreenSettingsUpdateStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));
export const ScreenSettingsRejectionReasonSchema = withParser(
  Schema.Literal(
    'storage-unavailable',
    'invalid-setting',
    'stale-revision',
    'raw-retention-forbidden',
    'disabled-setting-inconsistent',
    'policy-mode-inconsistent',
    'strict-mode-inconsistent',
    'trigger-mode-inconsistent',
    'ocr-mode-inconsistent'
  )
);

export const ScreenSettingsGetRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenSettingsSchemaVersion),
    requestId: NonEmptyStringSchema,
    kind: Schema.Literal('get'),
  })
);

export const ScreenSettingsReplaceRequestSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenSettingsSchemaVersion),
    requestId: NonEmptyStringSchema,
    kind: Schema.Literal('replace'),
    baseSettingVersion: Schema.Union(Schema.Number, Schema.Null),
    setting: ScreenAnalysisParentSettingSchema,
  })
);

export const ScreenSettingsUpdateRequestSchema = withParser(
  Schema.Union(ScreenSettingsGetRequestSchema, ScreenSettingsReplaceRequestSchema)
);

export const ScreenSettingsUpdateResponseSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ScreenSettingsSchemaVersion),
    requestId: NonEmptyStringSchema,
    kind: ScreenSettingsUpdateKindSchema,
    status: ScreenSettingsUpdateStatusSchema,
    setting: Schema.Union(ScreenAnalysisParentSettingSchema, Schema.Null),
    auditEventId: Schema.Union(NonEmptyStringSchema, Schema.Null),
    rejectionReason: Schema.Union(ScreenSettingsRejectionReasonSchema, Schema.Null),
    message: Schema.Union(NonEmptyStringSchema, Schema.Null),
  })
);

export type ScreenSettingsUpdateKind = Infer<typeof ScreenSettingsUpdateKindSchema>;
export type ScreenSettingsUpdateStatus = Infer<typeof ScreenSettingsUpdateStatusSchema>;
export type ScreenSettingsRejectionReason = Infer<typeof ScreenSettingsRejectionReasonSchema>;
export type ScreenSettingsGetRequest = Infer<typeof ScreenSettingsGetRequestSchema>;
export type ScreenSettingsReplaceRequest = Infer<typeof ScreenSettingsReplaceRequestSchema>;
export type ScreenSettingsUpdateRequest = Infer<typeof ScreenSettingsUpdateRequestSchema>;
export type ScreenSettingsUpdateResponse = Infer<typeof ScreenSettingsUpdateResponseSchema>;

export const ScreenSettingsUpdateKindValue = {
  Get: ScreenSettingsUpdateKindSchema.parse('get'),
  Replace: ScreenSettingsUpdateKindSchema.parse('replace'),
} as const;

export const ScreenSettingsUpdateStatus = {
  Accepted: ScreenSettingsUpdateStatusSchema.parse('accepted'),
  Rejected: ScreenSettingsUpdateStatusSchema.parse('rejected'),
} as const;
