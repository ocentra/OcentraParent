import { Schema } from '@ocentra-parent/schema-domain/effect';

const NonEmptyJournalText = Schema.String.pipe(Schema.minLength(1));

export const ActivityJournalCiphertextSchema = NonEmptyJournalText.pipe(Schema.brand('ActivityJournalCiphertext'));
export const ActivityJournalEntryIdSchema = NonEmptyJournalText.pipe(Schema.brand('ActivityJournalEntryId'));
export const ActivityJournalNonceSchema = NonEmptyJournalText.pipe(Schema.brand('ActivityJournalNonce'));
export const ActivityJournalSegmentIdSchema = NonEmptyJournalText.pipe(Schema.brand('ActivityJournalSegmentId'));

export type ActivityJournalCiphertext = typeof ActivityJournalCiphertextSchema.Type;
export type ActivityJournalEntryId = typeof ActivityJournalEntryIdSchema.Type;
export type ActivityJournalNonce = typeof ActivityJournalNonceSchema.Type;
export type ActivityJournalSegmentId = typeof ActivityJournalSegmentIdSchema.Type;

export const decodeActivityJournalCiphertext = Schema.decodeUnknownSync(ActivityJournalCiphertextSchema);
export const decodeActivityJournalEntryId = Schema.decodeUnknownSync(ActivityJournalEntryIdSchema);
export const decodeActivityJournalNonce = Schema.decodeUnknownSync(ActivityJournalNonceSchema);
export const decodeActivityJournalSegmentId = Schema.decodeUnknownSync(ActivityJournalSegmentIdSchema);
