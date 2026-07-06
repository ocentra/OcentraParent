import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  NotificationLocalOutboxProofRows,
  NotificationLocalOutboxSchedulerProofRows,
  NotificationLocalOutboxProofTimestamp,
  NotificationLocalOutboxSchedulerProofTimestamp,
} from '@ocentra-parent/schema-domain/notification-local-outbox';

describe('notification local outbox contracts', () => {
  it('keeps the TS surface thin over the Rust-generated notification-local-outbox artifact', () => {
    const source = readFileSync(new URL('../../src/notification-local-outbox.ts', import.meta.url), 'utf8');
    const generated = readFileSync(
      new URL('../../src/generated-notification-local-outbox.ts', import.meta.url),
      'utf8'
    );

    expect(source).toContain("from './generated-notification-local-outbox'");
    expect(source).not.toContain('export const NotificationLocalOutboxProofRows = [');
    expect(source).not.toContain('export const NotificationLocalOutboxSchedulerProofRows = [');
    expect(source).not.toContain('export const NotificationLocalOutboxAdapterProofReadModel =');
    expect(source).not.toContain('export const NotificationLocalOutboxSchedulerProofReadModel =');
    expect(source).not.toContain('export const decodeNotificationLocalOutboxRecord =');
    expect(source).not.toContain('export const decodeNotificationLocalOutboxAdapterProof =');
    expect(source).not.toContain('export const decodeNotificationLocalOutboxSchedulerRecord =');
    expect(source).not.toContain('export const decodeNotificationLocalOutboxSchedulerProof =');
    expect(generated.startsWith('/* generated from crates/schema/src/notification_local_outbox_ts.rs */')).toBe(true);
    expect(NotificationLocalOutboxProofTimestamp).toBe('2026-06-04T01:31:47.023Z');
    expect(NotificationLocalOutboxSchedulerProofTimestamp).toBe('2026-06-04T02:28:51.667Z');
    expect(NotificationLocalOutboxProofRows).toHaveLength(6);
    expect(NotificationLocalOutboxProofRows[0]?.entryId).toBe('notification-local-outbox-policy-violation-push-queued');
    expect(NotificationLocalOutboxSchedulerProofRows).toHaveLength(6);
    expect(NotificationLocalOutboxSchedulerProofRows[0]?.schedulerState).toBe('due-local');
  });
});
