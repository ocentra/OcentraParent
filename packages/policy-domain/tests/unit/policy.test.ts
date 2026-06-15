import { describe, expect, it } from 'vitest';
import { FamilyPolicySetSchema, PolicyRuleSchema } from '../../src/policy';

describe('parent policy contracts', () => {
  it('FamilyPolicySetSchema: parses parent-authored rules, schedules, children, and devices', () => {
    const parsed = FamilyPolicySetSchema.parse({
      schemaVersion: 'v0.6',
      family: { familyId: 'family-main' },
      childProfiles: [{ childProfileId: 'child-1', displayName: 'Sam' }],
      devices: [
        {
          deviceId: 'device-1',
          childProfileId: 'child-1',
          label: 'Sam Windows PC',
          platform: 'windows',
        },
      ],
      policyVersion: 'policy-v1',
      rules: [
        {
          ruleId: 'rule-1',
          target: { targetId: 'target-1', targetType: 'domain', targetValue: 'video.example' },
          action: 'ask-parent',
          scheduleId: 'school-night',
          priority: 10,
          reasonCode: 'school-night-video',
          createdBy: { actorId: 'parent-1', role: 'parent' },
          enabled: true,
          effectiveFrom: '2026-05-20T00:00:00.000Z',
          effectiveUntil: null,
        },
      ],
      schedules: [
        {
          scheduleId: 'school-night',
          timeZone: 'America/Toronto',
          windows: [{ days: ['monday', 'tuesday'], startLocalTime: '18:00', endLocalTime: '21:00' }],
        },
      ],
    });

    expect(parsed.rules[0]?.action).toBe('ask-parent');
    expect(parsed.schedules[0]?.windows[0]?.days).toEqual(['monday', 'tuesday']);
  });

  it('PolicyRuleSchema: rejects actions outside the local policy decision set', () => {
    const result = PolicyRuleSchema.safeParse({
      ruleId: 'rule-1',
      target: { targetId: 'target-1', targetType: 'domain', targetValue: 'video.example' },
      action: 'auto-escalate',
      scheduleId: null,
      priority: 10,
      reasonCode: 'bad-action',
      createdBy: { actorId: 'parent-1', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['action']);
    }
  });
});
