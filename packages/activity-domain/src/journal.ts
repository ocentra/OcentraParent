import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEventIdSchema, ActivityEvidenceDigestSchema, ActivityTimestampSchema } from './primitives';
import {
  ActivityJournalCiphertextSchema,
  ActivityJournalEntryIdSchema,
  ActivityJournalNonceSchema,
} from './primitives';

export const ActivityJournalSchemaVersion = 1;

export const ActivityJournalCipherSchema = withParser(Schema.Literal('xchacha20poly1305'));

export const ActivityJournalLineSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityJournalSchemaVersion),
    entryId: ActivityJournalEntryIdSchema,
    writtenAt: ActivityTimestampSchema,
    eventId: ActivityEventIdSchema,
    cipher: ActivityJournalCipherSchema,
    nonce: ActivityJournalNonceSchema,
    ciphertext: ActivityJournalCiphertextSchema,
    activityDigest: ActivityEvidenceDigestSchema,
  })
);

export const ActivityJournalStatusSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityJournalSchemaVersion),
    encrypted: Schema.Boolean,
    entriesWritten: Schema.Number,
    bytesWritten: Schema.Number,
    lastEntryId: Schema.Union(ActivityJournalEntryIdSchema, Schema.Null),
  })
);

export type ActivityJournalCipher = Infer<typeof ActivityJournalCipherSchema>;
export type ActivityJournalLine = Infer<typeof ActivityJournalLineSchema>;
export type ActivityJournalStatus = Infer<typeof ActivityJournalStatusSchema>;

export const ActivityJournalCipher = {
  XChaCha20Poly1305: ActivityJournalCipherSchema.parse('xchacha20poly1305'),
} as const;
