import { describe, expect, it } from 'vitest';
import {
  ActivityJournalCipher,
  ActivityJournalLineSchema,
  ActivityJournalSchemaVersion,
  ActivityJournalStatusSchema,
} from '../src/journal';

describe('activity journal contracts', () => {
  it('parses encrypted journal lines without exposing activity payload fields', () => {
    const line = ActivityJournalLineSchema.parse({
      schemaVersion: ActivityJournalSchemaVersion,
      entryId: 'journal-entry-1',
      writtenAt: '2026-05-20T00:00:00Z',
      eventId: 'activity-event-1',
      cipher: ActivityJournalCipher.XChaCha20Poly1305,
      nonce: 'nonce-bytes-base64',
      ciphertext: 'ciphertext-bytes-base64',
      activityDigest: 'sha256-activity-digest-base64',
    });

    expect(line.schemaVersion).toBe(ActivityJournalSchemaVersion);
    expect(line.cipher).toBe(ActivityJournalCipher.XChaCha20Poly1305);
    expect('subject' in line).toBe(false);
    expect('fields' in line).toBe(false);
  });

  it('rejects unsigned journal cipher names', () => {
    const result = ActivityJournalLineSchema.safeParse({
      schemaVersion: ActivityJournalSchemaVersion,
      entryId: 'journal-entry-1',
      writtenAt: '2026-05-20T00:00:00Z',
      eventId: 'activity-event-1',
      cipher: 'plaintext-json',
      nonce: 'nonce-bytes-base64',
      ciphertext: 'ciphertext-bytes-base64',
      activityDigest: 'sha256-activity-digest-base64',
    });

    expect(result.success).toBe(false);
  });

  it('parses encrypted journal status reports', () => {
    const status = ActivityJournalStatusSchema.parse({
      schemaVersion: ActivityJournalSchemaVersion,
      encrypted: true,
      entriesWritten: 2,
      bytesWritten: 512,
      lastEntryId: 'journal-entry-2',
    });

    expect(status.encrypted).toBe(true);
    expect(status.entriesWritten).toBe(2);
    expect(status.lastEntryId).toBe('journal-entry-2');
  });
});
