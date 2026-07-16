/* generic helper for evidence branded primitives */

import { Schema, brandedNonEmptyStringSchema } from './effect';

function defineBrandedNonEmptyString<const TName extends string>(name: TName) {
  const schema = brandedNonEmptyStringSchema(name);
  return {
    schema,
    decode: Schema.decodeUnknownSync(schema),
  } as const;
}

const ActivityDeviceIdDefinition = defineBrandedNonEmptyString('ActivityDeviceId');
const ActivityEvidenceDigestDefinition = defineBrandedNonEmptyString('ActivityEvidenceDigest');
const ActivityEvidenceIdDefinition = defineBrandedNonEmptyString('ActivityEvidenceId');
const ActivityEvidenceUriDefinition = defineBrandedNonEmptyString('ActivityEvidenceUri');
const ActivityEventIdDefinition = defineBrandedNonEmptyString('ActivityEventId');
const ActivityPlatformDefinition = defineBrandedNonEmptyString('ActivityPlatform');
const ActivitySourceIdDefinition = defineBrandedNonEmptyString('ActivitySourceId');
const ActivitySubjectIdDefinition = defineBrandedNonEmptyString('ActivitySubjectId');
const ActivitySubjectNameDefinition = defineBrandedNonEmptyString('ActivitySubjectName');
const ActivityTimestampDefinition = defineBrandedNonEmptyString('ActivityTimestamp');

export const ActivityDeviceIdSchema = ActivityDeviceIdDefinition.schema;
export const ActivityEvidenceDigestSchema = ActivityEvidenceDigestDefinition.schema;
export const ActivityEvidenceIdSchema = ActivityEvidenceIdDefinition.schema;
export const ActivityEvidenceUriSchema = ActivityEvidenceUriDefinition.schema;
export const ActivityEventIdSchema = ActivityEventIdDefinition.schema;
export const ActivityPlatformSchema = ActivityPlatformDefinition.schema;
export const ActivitySourceIdSchema = ActivitySourceIdDefinition.schema;
export const ActivitySubjectIdSchema = ActivitySubjectIdDefinition.schema;
export const ActivitySubjectNameSchema = ActivitySubjectNameDefinition.schema;
export const ActivityTimestampSchema = ActivityTimestampDefinition.schema;

export type ActivityDeviceId = typeof ActivityDeviceIdSchema.Type;
export type ActivityEvidenceDigest = typeof ActivityEvidenceDigestSchema.Type;
export type ActivityEvidenceId = typeof ActivityEvidenceIdSchema.Type;
export type ActivityEvidenceUri = typeof ActivityEvidenceUriSchema.Type;
export type ActivityEventId = typeof ActivityEventIdSchema.Type;
export type ActivityPlatform = typeof ActivityPlatformSchema.Type;
export type ActivitySourceId = typeof ActivitySourceIdSchema.Type;
export type ActivitySubjectId = typeof ActivitySubjectIdSchema.Type;
export type ActivitySubjectName = typeof ActivitySubjectNameSchema.Type;
export type ActivityTimestamp = typeof ActivityTimestampSchema.Type;

export const decodeActivityDeviceId = ActivityDeviceIdDefinition.decode;
export const decodeActivityEvidenceDigest = ActivityEvidenceDigestDefinition.decode;
export const decodeActivityEvidenceId = ActivityEvidenceIdDefinition.decode;
export const decodeActivityEvidenceUri = ActivityEvidenceUriDefinition.decode;
export const decodeActivityEventId = ActivityEventIdDefinition.decode;
export const decodeActivityPlatform = ActivityPlatformDefinition.decode;
export const decodeActivitySourceId = ActivitySourceIdDefinition.decode;
export const decodeActivitySubjectId = ActivitySubjectIdDefinition.decode;
export const decodeActivitySubjectName = ActivitySubjectNameDefinition.decode;
export const decodeActivityTimestamp = ActivityTimestampDefinition.decode;
