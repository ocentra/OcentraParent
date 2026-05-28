import { describe, expect, it } from 'vitest';
import { BaselineBrowserControlAuthoringManifest } from '../src/browser-control-baseline-manifest';
import {
  BrowserControlAuthoringManifestSchema,
  BrowserControlManifestDefaults,
  browserControlVisibleSectionIds,
} from '../src/browser-control-manifest';
import {
  BrowserControlCapabilityRegistrySchema,
  BrowserControlEffectivePolicySchema,
  browserControlManifestAllowsPatchRequest,
  BrowserControlPatchPolicyRequestSchema,
  BrowserControlPolicyValueSchema,
  BrowserControlUpdateRequestSchema,
} from '../src/browser-control-policy';
import { BrowserControlFieldIdSchema } from '../src/browser-control-identifiers';
import { BrowserControlWritesToPath } from '../src/browser-control-values';

describe('browser-control contracts', () => {
  it('accepts the baseline authoring manifest and known writesTo paths', () => {
    const parsed = BrowserControlAuthoringManifestSchema.safeParse(BaselineBrowserControlAuthoringManifest);

    expect(parsed.success).toBe(true);
    expect(BaselineBrowserControlAuthoringManifest.sections).toHaveLength(4);
    expect(BaselineBrowserControlAuthoringManifest.sections[0]?.fields[0]?.writesTo).toBe(
      BrowserControlWritesToPath.Enabled
    );
  });

  it('accepts policy, effective policy, capability registry, and update command shapes', () => {
    expect(BrowserControlPolicyValueSchema.safeParse(validPolicy()).success).toBe(true);
    expect(BrowserControlEffectivePolicySchema.safeParse(validEffectivePolicy()).success).toBe(true);
    expect(BrowserControlCapabilityRegistrySchema.safeParse(validCapabilityRegistry()).success).toBe(true);
    expect(BrowserControlUpdateRequestSchema.safeParse(validPatchRequest()).success).toBe(true);
  });

  it('rejects authoring fields with unknown writesTo paths', () => {
    const invalidManifest = {
      ...BaselineBrowserControlAuthoringManifest,
      sections: [
        {
          ...BaselineBrowserControlAuthoringManifest.sections[0],
          fields: [
            {
              ...BaselineBrowserControlAuthoringManifest.sections[0]?.fields[0],
              writesTo: '/browserPolicy/freeform',
            },
          ],
        },
      ],
    };

    expect(BrowserControlAuthoringManifestSchema.safeParse(invalidManifest).success).toBe(false);
  });

  it('rejects invalid policy enum values', () => {
    expect(
      BrowserControlPolicyValueSchema.safeParse({
        ...validPolicy(),
        defaultPosture: 'always-block-everything',
      }).success
    ).toBe(false);
  });

  it('rejects limit posture without budget or fallback posture', () => {
    expect(
      BrowserControlPolicyValueSchema.safeParse({
        ...validPolicy(),
        fallbackPosture: null,
        budgets: {
          defaultDailyMinutes: null,
        },
      }).success
    ).toBe(false);
  });

  it('rejects exact URL rules without managed browser proof requirement or fallback', () => {
    expect(
      BrowserControlPolicyValueSchema.safeParse({
        ...validPolicy(),
        managedBrowser: {
          mode: 'preferred',
        },
        evidence: {
          requiredProof: 'network-domain',
          proofFallback: null,
        },
      }).success
    ).toBe(false);
  });
});

describe('browser-control manifest visibility and patch safety', () => {
  it('keeps disabled and posture-specific sections hidden until their contract conditions match', () => {
    const disabledSections = browserControlVisibleSectionIds(BaselineBrowserControlAuthoringManifest, {
      [BrowserControlWritesToPath.Enabled]: false,
      [BrowserControlWritesToPath.DefaultPosture]: 'allow',
      [BrowserControlWritesToPath.ManagementMode]: 'observe-only',
    });
    const enabledLimitSections = browserControlVisibleSectionIds(BaselineBrowserControlAuthoringManifest, {
      [BrowserControlWritesToPath.Enabled]: true,
      [BrowserControlWritesToPath.DefaultPosture]: 'limit',
      [BrowserControlWritesToPath.ManagementMode]: 'managed-browser',
    });

    expect(disabledSections).toEqual([BrowserControlManifestDefaults.Section.Management]);
    expect(enabledLimitSections).toContain(BrowserControlManifestDefaults.Section.DefaultPosture);
    expect(enabledLimitSections).toContain(BrowserControlManifestDefaults.Section.ExactUrlRules);
  });

  it('prevents portal patches for fields outside the authoring manifest', () => {
    const parsed = BrowserControlPatchPolicyRequestSchema.parse({
      ...validPatchRequest(),
      patches: [
        {
          op: 'replace',
          fieldId: BrowserControlFieldIdSchema.parse('browser.unsanctionedControl'),
          writesTo: BrowserControlWritesToPath.Enabled,
          value: true,
        },
      ],
    });

    expect(browserControlManifestAllowsPatchRequest(BaselineBrowserControlAuthoringManifest, parsed)).toBe(false);
  });
});

function validPatchRequest() {
  return {
    schemaVersion: 'v0.6',
    requestId: 'browser-control-request-1',
    kind: 'patch',
    policyId: 'browser-policy-child-1',
    baseRevisionId: 'browser-policy-revision-1',
    patches: [
      {
        op: 'replace',
        fieldId: BrowserControlManifestDefaults.Field.Enabled,
        writesTo: BrowserControlWritesToPath.Enabled,
        value: true,
      },
    ],
  };
}

function validPolicy() {
  return {
    schemaVersion: 'v0.6',
    policyId: 'browser-policy-child-1',
    enabled: true,
    defaultPosture: 'limit',
    fallbackPosture: null,
    managementMode: 'managed-browser',
    managedBrowser: {
      mode: 'required-for-exact-rules',
    },
    unmanagedBrowser: {
      mode: 'network-domain-only',
    },
    evidence: {
      requiredProof: 'fresh-managed-active-tab',
      proofFallback: null,
    },
    rules: {
      allowedTargetTypes: ['domain', 'url-prefix', 'exact-url'],
      entries: [
        {
          ruleId: 'browser-rule-1',
          targetType: 'domain',
          targetValue: 'example.test',
          enabled: true,
        },
      ],
    },
    budgets: {
      defaultDailyMinutes: 60,
    },
    downloads: {
      state: 'ask-parent',
    },
    approvals: {
      state: 'required',
    },
    reports: {
      state: 'weekly',
    },
    audit: {
      state: 'local-only',
    },
    retention: {
      state: 'seven-days',
    },
  };
}

function validEffectivePolicy() {
  return {
    schemaVersion: 'v0.6',
    policyId: 'browser-policy-child-1',
    revisionId: 'browser-policy-revision-1',
    compiledHash: 'browser-policy-sha256-1',
    compiledAt: '2026-05-28T17:15:00Z',
    defaultPosture: 'limit',
    fallbackPosture: null,
    budgets: {
      defaultDailyMinutes: 60,
    },
    rules: [
      {
        ruleId: 'browser-rule-1',
        targetType: 'domain',
        targetValue: 'example.test',
        defaultPosture: 'limit',
        evidence: {
          requiredProof: 'fresh-managed-active-tab',
          proofFallback: null,
        },
      },
    ],
  };
}

function validCapabilityRegistry() {
  return {
    schemaVersion: 'v0.6',
    generatedAt: '2026-05-28T17:15:00Z',
    capabilities: [
      {
        capabilityId: 'managed-browser-active-tab-proof',
        state: 'supported',
        label: 'Managed browser active tab proof',
        affectedWritesTo: [BrowserControlWritesToPath.RequiredProof],
        checkedAt: '2026-05-28T17:15:00Z',
        reason: null,
      },
    ],
  };
}
