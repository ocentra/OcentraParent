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

const expectedSectionIds = [
  BrowserControlManifestDefaults.Section.Management,
  BrowserControlManifestDefaults.Section.ManagedBrowser,
  BrowserControlManifestDefaults.Section.UnmanagedBrowser,
  BrowserControlManifestDefaults.Section.UrlTabEvidence,
  BrowserControlManifestDefaults.Section.WebRules,
  BrowserControlManifestDefaults.Section.Budgets,
  BrowserControlManifestDefaults.Section.Downloads,
  BrowserControlManifestDefaults.Section.Approvals,
  BrowserControlManifestDefaults.Section.Reports,
  BrowserControlManifestDefaults.Section.Audit,
];

const expectedFieldIds = [
  BrowserControlManifestDefaults.Field.Enabled,
  BrowserControlManifestDefaults.Field.DefaultPosture,
  BrowserControlManifestDefaults.Field.ManagementMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserAllowedFamilies,
  BrowserControlManifestDefaults.Field.ManagedBrowserLaunchMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserProfileMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserBridgeRequirements,
  BrowserControlManifestDefaults.Field.ManagedBrowserIntegrationMechanisms,
  BrowserControlManifestDefaults.Field.UnmanagedBrowserMode,
  BrowserControlManifestDefaults.Field.UnmanagedBrowserGraceSeconds,
  BrowserControlManifestDefaults.Field.UnmanagedBrowserAllowRecoverLaunchUrl,
  BrowserControlManifestDefaults.Field.UnmanagedBrowserClassificationTargets,
  BrowserControlManifestDefaults.Field.EvidenceUrlScope,
  BrowserControlManifestDefaults.Field.RequiredProof,
  BrowserControlManifestDefaults.Field.WhenProofUnavailable,
  BrowserControlManifestDefaults.Field.EvidenceNeverCollect,
  BrowserControlManifestDefaults.Field.AllowedTargetTypes,
  BrowserControlManifestDefaults.Field.AllowedActions,
  BrowserControlManifestDefaults.Field.RuleItems,
  BrowserControlManifestDefaults.Field.BudgetsEnabled,
  BrowserControlManifestDefaults.Field.DailyBudgetMinutes,
  BrowserControlManifestDefaults.Field.BudgetCountingMode,
  BrowserControlManifestDefaults.Field.DownloadMode,
  BrowserControlManifestDefaults.Field.DownloadBlockedTypes,
  BrowserControlManifestDefaults.Field.ApprovalRequiredFor,
  BrowserControlManifestDefaults.Field.ApprovalUnansweredDefault,
  BrowserControlManifestDefaults.Field.ReportVisibleFields,
  BrowserControlManifestDefaults.Field.RetentionExactUrl,
  BrowserControlManifestDefaults.Field.CustodyAllowedUses,
  BrowserControlManifestDefaults.Field.AuditRequiredFields,
];

describe('browser-control contracts', () => {
  registerManifestAcceptanceCases();
  registerPolicyShapeCases();
  registerRejectionCases();
  registerVisibilityCases();
  registerPatchSafetyCases();
});

function registerManifestAcceptanceCases() {
  it('accepts the baseline authoring manifest with proposal section and field coverage', () => {
    const parsed = BrowserControlAuthoringManifestSchema.safeParse(BaselineBrowserControlAuthoringManifest);
    const sectionIds = BaselineBrowserControlAuthoringManifest.sections.map((section) => section.sectionId);
    const fieldIds = BaselineBrowserControlAuthoringManifest.sections.flatMap((section) =>
      section.fields.map((field) => field.fieldId)
    );
    const writesToPaths = BaselineBrowserControlAuthoringManifest.sections.flatMap((section) =>
      section.fields.map((field) => field.writesTo)
    );

    expect(parsed.success).toBe(true);
    expect(sectionIds).toEqual(expectedSectionIds);
    expect(fieldIds).toEqual(expectedFieldIds);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.CustodyAllowedUses);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.RuleItems);
  });
}

function registerPolicyShapeCases() {
  it('accepts proposal-shaped policy, effective policy, capability registry, and update command shapes', () => {
    expect(BrowserControlPolicyValueSchema.safeParse(validPolicy()).success).toBe(true);
    expect(BrowserControlEffectivePolicySchema.safeParse(validEffectivePolicy()).success).toBe(true);
    expect(BrowserControlCapabilityRegistrySchema.safeParse(validCapabilityRegistry()).success).toBe(true);
    expect(BrowserControlUpdateRequestSchema.safeParse(validPatchRequest()).success).toBe(true);
  });
}

function registerRejectionCases() {
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

  it('rejects limit posture without an enabled budget or fallback posture', () => {
    expect(
      BrowserControlPolicyValueSchema.safeParse({
        ...validPolicy(),
        fallbackPosture: null,
        budgets: {
          enabled: false,
          defaultDailyMinutes: null,
          countingMode: 'foreground-browser-time',
        },
      }).success
    ).toBe(false);
  });

  it('rejects exact URL rules without managed browser proof requirement or fallback', () => {
    expect(
      BrowserControlPolicyValueSchema.safeParse({
        ...validPolicy(),
        managedBrowser: {
          ...validPolicy().managedBrowser,
          mode: 'available-for-exact-rules',
        },
        evidence: {
          ...validPolicy().evidence,
          requiredProof: 'network-domain',
          proofFallback: null,
          whenProofUnavailable: 'mark-unavailable',
        },
      }).success
    ).toBe(false);
  });
}

function registerVisibilityCases() {
  it('keeps disabled and posture-specific sections hidden until their contract conditions match', () => {
    const disabledSections = browserControlVisibleSectionIds(BaselineBrowserControlAuthoringManifest, {
      [BrowserControlWritesToPath.Enabled]: false,
      [BrowserControlWritesToPath.DefaultPosture]: 'allow',
      [BrowserControlWritesToPath.ManagementMode]: 'authoring-only',
    });
    const enabledLimitSections = browserControlVisibleSectionIds(BaselineBrowserControlAuthoringManifest, {
      [BrowserControlWritesToPath.Enabled]: true,
      [BrowserControlWritesToPath.DefaultPosture]: 'limit',
      [BrowserControlWritesToPath.ManagementMode]: 'local-child-agent',
      [BrowserControlWritesToPath.BudgetsEnabled]: true,
    });
    const enabledAllowSections = browserControlVisibleSectionIds(BaselineBrowserControlAuthoringManifest, {
      [BrowserControlWritesToPath.Enabled]: true,
      [BrowserControlWritesToPath.DefaultPosture]: 'allow',
      [BrowserControlWritesToPath.ManagementMode]: 'local-child-agent',
    });

    expect(disabledSections).toEqual([BrowserControlManifestDefaults.Section.Management]);
    expect(enabledLimitSections).toContain(BrowserControlManifestDefaults.Section.Budgets);
    expect(enabledLimitSections).toContain(BrowserControlManifestDefaults.Section.WebRules);
    expect(enabledAllowSections).not.toContain(BrowserControlManifestDefaults.Section.WebRules);
  });
}

function registerPatchSafetyCases() {
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

  it('allows proposal rule-list patches only through manifest-owned field and writesTo ids', () => {
    const parsed = BrowserControlPatchPolicyRequestSchema.parse({
      ...validPatchRequest(),
      patches: [
        {
          op: 'replace',
          fieldId: BrowserControlManifestDefaults.Field.RuleItems,
          writesTo: BrowserControlWritesToPath.RuleItems,
          value: [
            {
              ruleId: 'browser-rule-school',
              targetType: 'domain-origin',
              targetValue: 'school.example.invalid',
              enabled: true,
            },
          ],
        },
      ],
    });

    expect(browserControlManifestAllowsPatchRequest(BaselineBrowserControlAuthoringManifest, parsed)).toBe(true);
  });
}

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
    ...validPolicyCore(),
    ...validBrowserBoundaryPolicy(),
    ...validRulePolicy(),
    ...validSupportPolicy(),
    ...validAuxiliaryPolicy(),
  };
}

function validPolicyCore() {
  return {
    schemaVersion: 'v0.6',
    policyId: 'browser-policy-child-1',
    enabled: true,
    defaultPosture: 'limit',
    fallbackPosture: null,
    managementMode: 'local-child-agent',
  };
}

function validBrowserBoundaryPolicy() {
  return {
    managedBrowser: {
      mode: 'required-for-exact-rules',
      allowedFamilies: ['edge-stable', 'chrome-stable', 'chrome-for-testing'],
      launchMode: 'ocentra-launcher',
      profileMode: 'persistent-managed-profile',
      bridgeRequirements: ['owned-profile', 'loopback-only', 'random-port', 'reject-default-profile'],
      integrationMechanisms: ['chromium-cdp', 'browser-policy'],
    },
    unmanagedBrowser: {
      mode: 'relaunch-managed',
      graceSeconds: 15,
      allowRecoverLaunchUrl: true,
      classificationTargets: ['known-browser', 'portable-browser', 'browser-like-process'],
    },
    evidence: {
      urlScope: 'domain-origin-title',
      requiredProof: 'fresh-managed-active-tab',
      proofFallback: null,
      whenProofUnavailable: 'ask',
      neverCollect: ['page-body', 'chat-content', 'screenshots', 'keystrokes', 'form-values', 'secrets'],
    },
  };
}

function validRulePolicy() {
  return {
    rules: {
      allowedTargetTypes: ['exact-url', 'domain-origin', 'site-category', 'browser-session', 'browser-process'],
      allowedActions: ['allow', 'warn', 'ask', 'limit', 'block'],
      items: [
        {
          ruleId: 'browser-rule-1',
          enabled: true,
          priority: 100,
          target: {
            kind: 'domain-origin',
            values: ['school.example.invalid'],
            matchMode: 'origin',
          },
          action: {
            kind: 'allow',
            reasonCode: 'school-domain',
          },
          proofRequirement: 'domain-or-managed-url',
          scheduleId: 'school-hours',
          auditLevel: 'decision',
        },
      ],
    },
  };
}

function validSupportPolicy() {
  return {
    budgets: {
      enabled: true,
      defaultDailyMinutes: 60,
      countingMode: 'foreground-browser-time',
    },
    downloads: {
      mode: 'ask',
      blockedTypes: ['executable', 'script', 'unknown'],
    },
    approvals: {
      requiredFor: ['blocked-site', 'new-domain', 'unmanaged-browser', 'download', 'time-extension'],
      unansweredDefault: 'deny',
    },
    reports: {
      visibleFields: ['managed-status', 'recent-domain-title', 'unmanaged-use', 'policy-decisions', 'time-budget'],
    },
    audit: {
      requiredFields: ['policy-decision', 'evidence-ref', 'adapter-result', 'timer-state', 'policy-version'],
    },
    retention: {
      exactUrl: '7-days',
    },
    custody: {
      allowedUses: ['child-local', 'lan-live', 'parent-cache', 'parent-report'],
    },
  };
}

function validAuxiliaryPolicy() {
  return {
    schedules: [
      {
        scheduleId: 'always',
        kind: 'always',
      },
      {
        scheduleId: 'school-hours',
        kind: 'weekly-window',
        timezone: 'America/Toronto',
      },
    ],
    childFacing: {
      showWarnText: true,
      showBlockReason: true,
      showAskParentState: true,
      showTimeLeft: true,
      showUseManagedBrowserAction: true,
      hideParentDiagnostics: true,
    },
    portalAi: {
      allowSummaries: true,
      allowPolicyExplanation: true,
      allowRuleSuggestions: false,
      allowEvidenceRefs: true,
      allowRawContent: false,
      requiresManualReview: true,
      fallbackWhenUnavailable: 'manual-view',
    },
    platforms: {
      windows: {
        enabled: true,
        allowedAdapters: ['managed-edge', 'managed-chrome', 'chrome-for-testing', 'chromium-cdp'],
        manualRequiredAdapters: ['app-control-blocking', 'wfp-domain-filtering'],
      },
      webPortal: {
        authoringOnly: true,
        mayRunCapture: false,
        mayConnectToBrowserBridge: false,
      },
    },
    fallbacks: {
      managedProfileMissing: 'ask',
      bridgeMissing: 'ask',
      extensionDisabled: 'warn',
      unsupportedBrowser: 'monitor',
      staleEvidence: 'report-only',
      platformUnsupported: 'show-unavailable',
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
      enabled: true,
      defaultDailyMinutes: 60,
      countingMode: 'foreground-browser-time',
    },
    rules: [
      {
        ruleId: 'browser-rule-1',
        targetType: 'domain-origin',
        targetValue: 'school.example.invalid',
        defaultPosture: 'limit',
        evidence: {
          urlScope: 'domain-origin-title',
          requiredProof: 'fresh-managed-active-tab',
          proofFallback: null,
          whenProofUnavailable: 'ask',
          neverCollect: ['page-body'],
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
        state: 'unknown',
        label: 'Managed browser active tab proof',
        affectedWritesTo: [BrowserControlWritesToPath.RequiredProof],
        checkedAt: '2026-05-28T17:15:00Z',
        reason: null,
      },
    ],
  };
}
