import { type Infer, Schema, withParser } from './effect';
import {
  ActivityJournalCiphertextSchema,
  ActivityJournalEntryIdSchema,
  ActivityJournalNonceSchema,
  ActivityJournalSegmentIdSchema,
} from './activity-journal-primitives';
import {
  ActivityEventIdSchema,
  ActivityEvidenceDigestSchema,
  ActivityTimestampSchema,
} from './evidence-primitives';

export const ActivityJournalSchemaVersion = 1;

export const ActivityJournalCipherSchema = withParser(Schema.Literal('xchacha20poly1305'));

export const ActivityJournalLineSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(ActivityJournalSchemaVersion),
    entryId: ActivityJournalEntryIdSchema,
    segmentId: ActivityJournalSegmentIdSchema,
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
    activeSegmentId: ActivityJournalSegmentIdSchema,
    segmentCount: Schema.Number,
    rotationMaxBytes: Schema.Number,
    lastEntryId: Schema.Union(ActivityJournalEntryIdSchema, Schema.Null),
  })
);

export const ActivityJournalRotationPolicySchema = withParser(
  Schema.Struct({
    maxSegmentBytes: Schema.Number,
  })
);

export type ActivityJournalCipher = Infer<typeof ActivityJournalCipherSchema>;
export type ActivityJournalLine = Infer<typeof ActivityJournalLineSchema>;
export type ActivityJournalStatus = Infer<typeof ActivityJournalStatusSchema>;
export type ActivityJournalRotationPolicy = Infer<typeof ActivityJournalRotationPolicySchema>;

export const ActivityJournalCipher = {
  XChaCha20Poly1305: ActivityJournalCipherSchema.parse('xchacha20poly1305'),
} as const;
