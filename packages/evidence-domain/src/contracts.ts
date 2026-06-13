import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ActivityEventKindSchema,
  ActivityEvidenceKindSchema,
  ActivityObserverSchema,
  ActivitySubjectKindSchema,
} from './kinds';
import {
  ActivityDeviceIdSchema,
  ActivityEvidenceDigestSchema,
  ActivityEvidenceIdSchema,
  ActivityEvidenceUriSchema,
  ActivityEventIdSchema,
  ActivityPlatformSchema,
  ActivitySourceIdSchema,
  ActivitySubjectIdSchema,
  ActivitySubjectNameSchema,
  ActivityTimestampSchema,
} from '@ocentra-parent/evidence-domain/primitives';

export const ActivitySchemaVersion = 1;

export const ActivityFieldValueSchema = withParser(
  Schema.Union(Schema.String, Schema.Number, Schema.Boolean, Schema.Null)
);

export const ActivityFieldsSchema = withParser(
  Schema.Record({
    key: Schema.String,
    value: ActivityFieldValueSchema,
  })
);

export const ActivitySourceSchema = withParser(
  Schema.Struct({
    deviceId: ActivityDeviceIdSchema,
    platform: ActivityPlatformSchema,
    observer: ActivityObserverSchema,
    sourceId: ActivitySourceIdSchema,
  })
);

export const ActivitySubjectSchema = withParser(
  Schema.Struct({
    kind: ActivitySubjectKindSchema,
    subjectId: ActivitySubjectIdSchema,
    displayName: Schema.Union(ActivitySubjectNameSchema, Schema.Null),
  })
);

export const ActivityEvidenceRefSchema = withParser(
  Schema.Struct({
    evidenceId: ActivityEvidenceIdSchema,
    kind: ActivityEvidenceKindSchema,
    digest: Schema.Union(ActivityEvidenceDigestSchema, Schema.Null),
    uri: Schema.Union(ActivityEvidenceUriSchema, Schema.Null),
  })
);

export const ActivityEventSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivitySchemaVersion),
    eventId: ActivityEventIdSchema,
    observedAt: ActivityTimestampSchema,
    source: ActivitySourceSchema,
    kind: ActivityEventKindSchema,
    subject: ActivitySubjectSchema,
    fields: ActivityFieldsSchema,
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export type ActivityFieldValue = Infer<typeof ActivityFieldValueSchema>;
export type ActivityFields = Infer<typeof ActivityFieldsSchema>;
export type ActivitySource = Infer<typeof ActivitySourceSchema>;
export type ActivitySubject = Infer<typeof ActivitySubjectSchema>;
export type ActivityEvidenceRef = Infer<typeof ActivityEvidenceRefSchema>;
export type ActivityEvent = Infer<typeof ActivityEventSchema>;
