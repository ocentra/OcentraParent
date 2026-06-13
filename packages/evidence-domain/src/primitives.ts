import { Schema, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const ActivityDeviceIdSchema = brandedNonEmptyStringSchema('ActivityDeviceId');
export const ActivityEvidenceDigestSchema = brandedNonEmptyStringSchema('ActivityEvidenceDigest');
export const ActivityEvidenceIdSchema = brandedNonEmptyStringSchema('ActivityEvidenceId');
export const ActivityEvidenceUriSchema = brandedNonEmptyStringSchema('ActivityEvidenceUri');
export const ActivityEventIdSchema = brandedNonEmptyStringSchema('ActivityEventId');
export const ActivityPlatformSchema = brandedNonEmptyStringSchema('ActivityPlatform');
export const ActivitySourceIdSchema = brandedNonEmptyStringSchema('ActivitySourceId');
export const ActivitySubjectIdSchema = brandedNonEmptyStringSchema('ActivitySubjectId');
export const ActivitySubjectNameSchema = brandedNonEmptyStringSchema('ActivitySubjectName');
export const ActivityTimestampSchema = brandedNonEmptyStringSchema('ActivityTimestamp');

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

export const decodeActivityDeviceId = Schema.decodeUnknownSync(ActivityDeviceIdSchema);
export const decodeActivityEvidenceDigest = Schema.decodeUnknownSync(ActivityEvidenceDigestSchema);
export const decodeActivityEvidenceId = Schema.decodeUnknownSync(ActivityEvidenceIdSchema);
export const decodeActivityEvidenceUri = Schema.decodeUnknownSync(ActivityEvidenceUriSchema);
export const decodeActivityEventId = Schema.decodeUnknownSync(ActivityEventIdSchema);
export const decodeActivityPlatform = Schema.decodeUnknownSync(ActivityPlatformSchema);
export const decodeActivitySourceId = Schema.decodeUnknownSync(ActivitySourceIdSchema);
export const decodeActivitySubjectId = Schema.decodeUnknownSync(ActivitySubjectIdSchema);
export const decodeActivitySubjectName = Schema.decodeUnknownSync(ActivitySubjectNameSchema);
export const decodeActivityTimestamp = Schema.decodeUnknownSync(ActivityTimestampSchema);
