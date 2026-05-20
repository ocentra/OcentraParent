import { Schema } from '@ocentra-parent/schema-domain/effect';

const NonEmptyActivityText = Schema.String.pipe(Schema.minLength(1));

export const ActivityDeviceIdSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityDeviceId'));
export const ActivityEvidenceDigestSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityEvidenceDigest'));
export const ActivityEvidenceIdSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityEvidenceId'));
export const ActivityEvidenceUriSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityEvidenceUri'));
export const ActivityEventIdSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityEventId'));
export const ActivityJournalCiphertextSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityJournalCiphertext'));
export const ActivityJournalEntryIdSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityJournalEntryId'));
export const ActivityJournalNonceSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityJournalNonce'));
export const ActivityPlatformSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityPlatform'));
export const ActivitySourceIdSchema = NonEmptyActivityText.pipe(Schema.brand('ActivitySourceId'));
export const ActivitySubjectIdSchema = NonEmptyActivityText.pipe(Schema.brand('ActivitySubjectId'));
export const ActivitySubjectNameSchema = NonEmptyActivityText.pipe(Schema.brand('ActivitySubjectName'));
export const ActivityTimestampSchema = NonEmptyActivityText.pipe(Schema.brand('ActivityTimestamp'));

export type ActivityDeviceId = typeof ActivityDeviceIdSchema.Type;
export type ActivityEvidenceDigest = typeof ActivityEvidenceDigestSchema.Type;
export type ActivityEvidenceId = typeof ActivityEvidenceIdSchema.Type;
export type ActivityEvidenceUri = typeof ActivityEvidenceUriSchema.Type;
export type ActivityEventId = typeof ActivityEventIdSchema.Type;
export type ActivityJournalCiphertext = typeof ActivityJournalCiphertextSchema.Type;
export type ActivityJournalEntryId = typeof ActivityJournalEntryIdSchema.Type;
export type ActivityJournalNonce = typeof ActivityJournalNonceSchema.Type;
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
export const decodeActivityJournalCiphertext = Schema.decodeUnknownSync(ActivityJournalCiphertextSchema);
export const decodeActivityJournalEntryId = Schema.decodeUnknownSync(ActivityJournalEntryIdSchema);
export const decodeActivityJournalNonce = Schema.decodeUnknownSync(ActivityJournalNonceSchema);
export const decodeActivityPlatform = Schema.decodeUnknownSync(ActivityPlatformSchema);
export const decodeActivitySourceId = Schema.decodeUnknownSync(ActivitySourceIdSchema);
export const decodeActivitySubjectId = Schema.decodeUnknownSync(ActivitySubjectIdSchema);
export const decodeActivitySubjectName = Schema.decodeUnknownSync(ActivitySubjectNameSchema);
export const decodeActivityTimestamp = Schema.decodeUnknownSync(ActivityTimestampSchema);
