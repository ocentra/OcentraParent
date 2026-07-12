import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleProviderRetryContractReadModel,
  V3NotificationRuleReasonCodeSchema,
} from '@ocentra-parent/schema-domain/notification-v3-provider-retry';

describe('notification v3 provider retry contracts', () => {
  it('keeps the TS surface thin over the Rust-generated notification-v3-provider-retry artifact', () => {
    const source = readFileSync(new URL('../../src/notification-v3-provider-retry.ts', import.meta.url), 'utf8');
    const generated = readFileSync(
      new URL('../../src/generated-notification-v3-provider-retry.ts', import.meta.url),
      'utf8'
    );

    expect(source).toContain("from './generated-notification-v3-provider-retry'");
    expect(source).toContain('generatedV3NotificationRuleProviderRetryContractEntryIsHonest');
    expect(source).toContain('generatedV3NotificationRuleProviderRetryContractReadModelIsHonest');
    expect(source).not.toContain('export const V3NotificationRuleProviderRetryContractReadModel = {');
    expect(source).not.toContain('notificationRuleProviderRetryContractEntryIsHonest(');
    expect(source).not.toContain('notificationRuleProviderRetryContractReadModelIsHonest(');
    expect(generated.startsWith('/* generated from crates/schema/src/notification_v3_provider_retry_ts.rs */')).toBe(
      true
    );
    expect(generated).toContain('export function generatedV3NotificationRuleProviderRetryContractEntryIsHonest');
    expect(generated).toContain('export function generatedV3NotificationRuleProviderRetryContractReadModelIsHonest');
    expect(V3NotificationRuleReasonCodeSchema.parse('policy-violation')).toBe('policy-violation');
    expect(V3NotificationProviderChannelSchema.parse('whatsapp')).toBe('whatsapp');
    expect(V3NotificationRuleProviderRetryContractReadModel.entries).toHaveLength(6);
    expect(V3NotificationRuleProviderRetryContractReadModel.entries[0]?.contractEntryId).toBe(
      'notification-rule-policy-violation-push-queued'
    );
  });
});
