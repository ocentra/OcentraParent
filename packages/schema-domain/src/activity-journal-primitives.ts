import { Schema, brandedNonEmptyStringSchema } from './effect';

export const ActivityJournalCiphertextSchema = brandedNonEmptyStringSchema('ActivityJournalCiphertext');
export const ActivityJournalEntryIdSchema = brandedNonEmptyStringSchema('ActivityJournalEntryId');
export const ActivityJournalNonceSchema = brandedNonEmptyStringSchema('ActivityJournalNonce');
export const ActivityJournalSegmentIdSchema = brandedNonEmptyStringSchema('ActivityJournalSegmentId');

export type ActivityJournalCiphertext = typeof ActivityJournalCiphertextSchema.Type;
export type ActivityJournalEntryId = typeof ActivityJournalEntryIdSchema.Type;
export type ActivityJournalNonce = typeof ActivityJournalNonceSchema.Type;
export type ActivityJournalSegmentId = typeof ActivityJournalSegmentIdSchema.Type;

export const decodeActivityJournalCiphertext = Schema.decodeUnknownSync(ActivityJournalCiphertextSchema);
export const decodeActivityJournalEntryId = Schema.decodeUnknownSync(ActivityJournalEntryIdSchema);
export const decodeActivityJournalNonce = Schema.decodeUnknownSync(ActivityJournalNonceSchema);
export const decodeActivityJournalSegmentId = Schema.decodeUnknownSync(ActivityJournalSegmentIdSchema);
