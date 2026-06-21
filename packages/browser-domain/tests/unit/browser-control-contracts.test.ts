import { describe, expect, it } from 'vitest';
import { BaselineBrowserControlAuthoringManifest } from '@ocentra-parent/schema-domain/browser-control-baseline-manifest';
import {
  BrowserControlAuthoringManifestSchema,
  BrowserControlManifestDefaults,
  browserControlVisibleSectionIds,
} from '@ocentra-parent/schema-domain/browser-control-manifest';
import {
  BrowserControlCandidateMvpItems,
  BrowserControlCatalogMajorSections,
  BrowserControlCoverageMatrix,
} from '@ocentra-parent/schema-domain/browser-control-coverage-matrix';
import {
  BrowserControlCapabilityRegistrySchema,
  BrowserControlEffectivePolicySchema,
  browserControlCreateStorageUnavailableResponse,
  browserControlManifestAllowsPatchRequest,
  BrowserControlPatchPolicyRequestSchema,
  BrowserControlPolicyValueSchema,
  BrowserControlUpdateRequestSchema,
} from '@ocentra-parent/schema-domain/browser-control-policy';
import {
  BrowserControlFieldIdSchema,
  BrowserControlRequestIdSchema,
} from '@ocentra-parent/schema-domain/browser-control-identifiers';
import {
  BrowserControlUnmanagedBrowserModeSchema,
  BrowserControlWritesToPath,
} from '@ocentra-parent/schema-domain/browser-control-values';
import { BrowserControlRuleActionSchema } from '@ocentra-parent/schema-domain/browser-control-catalog-values';

const expectedSectionIds = [
  BrowserControlManifestDefaults.Section.Management,
  BrowserControlManifestDefaults.Section.BrowserDiscovery,
  BrowserControlManifestDefaults.Section.ManagedBrowser,
  BrowserControlManifestDefaults.Section.UnmanagedBrowser,
  BrowserControlManifestDefaults.Section.UrlTabEvidence,
  BrowserControlManifestDefaults.Section.WebRules,
  BrowserControlManifestDefaults.Section.Budgets,
  BrowserControlManifestDefaults.Section.BrowserGames,
  BrowserControlManifestDefaults.Section.Downloads,
  BrowserControlManifestDefaults.Section.Approvals,
  BrowserControlManifestDefaults.Section.Reports,
  BrowserControlManifestDefaults.Section.Audit,
];

const expectedFieldIds = [
  BrowserControlManifestDefaults.Field.Enabled,
  BrowserControlManifestDefaults.Field.ExecutionMode,
  BrowserControlManifestDefaults.Field.DefaultPosture,
  BrowserControlManifestDefaults.Field.ManagementMode,
  BrowserControlManifestDefaults.Field.DiscoveryScanInstalledBrowsers,
  BrowserControlManifestDefaults.Field.DiscoveryScanRunningBrowsers,
  BrowserControlManifestDefaults.Field.DiscoveryDetectUnmanagedBrowsers,
  BrowserControlManifestDefaults.Field.ManagedBrowserMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserAllowedFamilies,
  BrowserControlManifestDefaults.Field.ManagedBrowserLaunchMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserProfileMode,
  BrowserControlManifestDefaults.Field.ManagedBrowserBridgeRequirements,
  BrowserControlManifestDefaults.Field.ManagedBrowserIntegrationMechanisms,
  BrowserControlManifestDefaults.Field.ManagedBrowserPolicyWriterControls,
  BrowserControlManifestDefaults.Field.ManagedBrowserPolicyWriterFallback,
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
  BrowserControlManifestDefaults.Field.UrlAllowList,
  BrowserControlManifestDefaults.Field.UrlBlockList,
  BrowserControlManifestDefaults.Field.BudgetsEnabled,
  BrowserControlManifestDefaults.Field.DailyBudgetMinutes,
  BrowserControlManifestDefaults.Field.BudgetCountingMode,
  BrowserControlManifestDefaults.Field.BrowserGameEducationalMode,
  BrowserControlManifestDefaults.Field.BrowserGameUnknownMode,
  BrowserControlManifestDefaults.Field.BrowserGameCloudGamingApproval,
  BrowserControlManifestDefaults.Field.BrowserGamePurchaseAccountApproval,
  BrowserControlManifestDefaults.Field.BrowserGameUnblockedPortalMode,
  BrowserControlManifestDefaults.Field.BrowserGameWebglCanvasMode,
  BrowserControlManifestDefaults.Field.BrowserGameDailyBudgetMinutes,
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
  registerCoverageMatrixCases();
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
    expect(writesToPaths).toContain(BrowserControlWritesToPath.ManagedBrowserPolicyWriterControls);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.ManagedBrowserPolicyWriterFallback);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.UrlAllowList);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.UrlBlockList);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.BrowserGameCloudGamingApproval);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.CustodyAllowedUses);
    expect(writesToPaths).toContain(BrowserControlWritesToPath.RuleItems);
    expect(optionValuesFor(BrowserControlManifestDefaults.Field.UnmanagedBrowserMode)).toEqual([
      'report-only',
      'allowed-unmanaged-exception',
      'warn-child',
      'parent-review',
      'terminate-process',
      'relaunch-managed',
      'os-block-configured',
      'os-block-manual-required',
    ]);
    expect(optionValuesFor(BrowserControlManifestDefaults.Field.AllowedActions)).toContain('terminate-process');
    expect(optionValuesFor(BrowserControlManifestDefaults.Field.AllowedActions)).toContain('relaunch-managed');
  });
}

function registerCoverageMatrixCases() {
  it('accounts for every candidate MVP item and catalog major section', () => {
    const candidateItems = BrowserControlCoverageMatrix.filter((entry) => entry.coverageKind === 'candidate-mvp').map(
      (entry) => entry.catalogItem
    );
    const catalogSections = BrowserControlCoverageMatrix.filter(
      (entry) => entry.coverageKind === 'catalog-section'
    ).map((entry) => entry.catalogSection);

    expect(candidateItems).toEqual([...BrowserControlCandidateMvpItems]);
    expect(catalogSections).toEqual([...BrowserControlCatalogMajorSections]);
    expect(
      BrowserControlCoverageMatrix.some(
        (entry) =>
          entry.catalogItem === 'Mode: observe, dry-run, warn/ask, enforce.' &&
          entry.writesTo.includes(BrowserControlWritesToPath.ExecutionMode)
      )
    ).toBe(true);
    expect(
      BrowserControlCoverageMatrix.some(
        (entry) =>
          entry.catalogSection === 'Managed Browser Setup Settings' &&
          entry.coverageStatus === 'represented-through-capability' &&
          entry.capabilityState === 'manual-required'
      )
    ).toBe(true);
  });
}

function registerPolicyShapeCases() {
  it('accepts proposal-shaped policy, effective policy, capability registry, and update command shapes', () => {
    expect(BrowserControlPolicyValueSchema.safeParse(validPolicy()).success).toBe(true);
    expect(BrowserControlEffectivePolicySchema.safeParse(validEffectivePolicy()).success).toBe(true);
    expect(BrowserControlCapabilityRegistrySchema.safeParse(validCapabilityRegistry()).success).toBe(true);
    expect(BrowserControlUpdateRequestSchema.safeParse(validPatchRequest()).success).toBe(true);
  });

  it('creates an honest storage-unavailable rejection for the implemented browser policy runtime boundary', () => {
    const response = browserControlCreateStorageUnavailableResponse(
      BrowserControlRequestIdSchema.parse('browser-policy-request-storage'),
      'patch'
    );

    expect(response.status).toBe('rejected');
    expect(response.rejectionReason).toBe('storage-unavailable');
    expect(response.effectivePolicy).toBeNull();
  });

  it('accepts policy target compiler vocabulary for social, video, and browser-game targets', () => {
    expect(BrowserControlPolicyValueSchema.safeParse(validCompilerVocabularyPolicy()).success).toBe(true);
    expect(BrowserControlEffectivePolicySchema.safeParse(validCompilerVocabularyEffectivePolicy()).success).toBe(true);
    expect(BrowserControlRuleActionSchema.safeParse('terminate-process').success).toBe(true);
    expect(BrowserControlRuleActionSchema.safeParse('relaunch-managed').success).toBe(true);
    expect(BrowserControlUnmanagedBrowserModeSchema.safeParse('terminate-process').success).toBe(true);
    expect(BrowserControlUnmanagedBrowserModeSchema.safeParse('os-block-manual-required').success).toBe(true);
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

  it('rejects unsupported managed Chrome and Edge policy-writer controls', () => {
    expect(
      BrowserControlPolicyValueSchema.safeParse({
        ...validPolicy(),
        managedBrowser: {
          ...validPolicy().managedBrowser,
          policyWriterControls: ['delete-browser-history-remotely'],
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

  it('allows managed Chrome and Edge policy-writer patches only through typed manifest controls', () => {
    const parsed = BrowserControlPatchPolicyRequestSchema.parse({
      ...validPatchRequest(),
      patches: [
        {
          op: 'replace',
          fieldId: BrowserControlManifestDefaults.Field.ManagedBrowserPolicyWriterControls,
          writesTo: BrowserControlWritesToPath.ManagedBrowserPolicyWriterControls,
          value: [
            'disable-incognito',
            'disable-guest-browsing',
            'disable-profile-adding',
            'limit-history-deletion',
            'force-safe-search',
            'force-restricted-mode',
            'url-allow-list',
            'url-block-list',
          ],
        },
        {
          op: 'replace',
          fieldId: BrowserControlManifestDefaults.Field.UrlAllowList,
          writesTo: BrowserControlWritesToPath.UrlAllowList,
          value: ['school.example.invalid'],
        },
        {
          op: 'replace',
          fieldId: BrowserControlManifestDefaults.Field.UrlBlockList,
          writesTo: BrowserControlWritesToPath.UrlBlockList,
          value: ['games.example.invalid'],
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

function optionValuesFor(fieldId: (typeof expectedFieldIds)[number]): string[] {
  return (
    BaselineBrowserControlAuthoringManifest.sections
      .flatMap((section) => section.fields)
      .find((field) => field.fieldId === fieldId)
      ?.options.map((option) => option.value) ?? []
  );
}

function validPolicyCore() {
  return {
    schemaVersion: 'v0.6',
    policyId: 'browser-policy-child-1',
    enabled: true,
    defaultPosture: 'limit',
    fallbackPosture: null,
    managementMode: 'local-child-agent',
    executionMode: 'enforce',
    discovery: {
      scanInstalledBrowsers: true,
      scanRunningBrowsers: true,
      detectUnmanagedBrowsers: true,
    },
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
      policyWriterControls: [
        'disable-incognito',
        'disable-guest-browsing',
        'disable-profile-adding',
        'limit-history-deletion',
        'force-safe-search',
        'force-restricted-mode',
        'url-allow-list',
        'url-block-list',
      ],
      policyWriterFallback: 'manual-required',
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
      allowedActions: ['allow', 'warn', 'ask', 'limit', 'block', 'terminate-process', 'relaunch-managed'],
      urlAllowList: ['school.example.invalid'],
      urlBlockList: ['games.example.invalid'],
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
    executionMode: 'enforce',
    defaultPosture: 'limit',
    fallbackPosture: null,
    discovery: {
      scanInstalledBrowsers: true,
      scanRunningBrowsers: true,
      detectUnmanagedBrowsers: true,
    },
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
        action: 'allow',
        targetProofRequirement: 'domain-or-managed-url',
        capabilityState: 'ready',
        actionExecution: 'deterministic-parent-policy',
        aiAuthority: 'parent-policy-only',
        compileNote: 'Compiled from deterministic parent policy.',
      },
    ],
  };
}

function validCompilerVocabularyPolicy() {
  return {
    ...validPolicy(),
    managedBrowser: {
      ...validPolicy().managedBrowser,
      mode: 'not-required',
    },
    evidence: {
      ...validPolicy().evidence,
      requiredProof: 'browser-game-runtime-signal',
      proofFallback: 'parent-review',
      whenProofUnavailable: 'ask',
    },
    rules: {
      ...validPolicy().rules,
      allowedTargetTypes: [
        'search-terms',
        'video-channel',
        'social-platform',
        'social-route-kind',
        'social-account-creation',
        'social-unknown-account',
        'social-secondary-account',
        'social-feed',
        'social-short-video-feed',
        'social-messaging',
        'social-upload-post',
        'social-livestream',
        'unknown-social-site',
        'browser-game',
        'browser-game-platform',
        'browser-game-portal',
        'browser-game-url',
        'educational-game',
        'cloud-gaming',
        'webgl-canvas-game',
        'multiplayer-ugc-game',
        'game-chat',
        'game-account',
        'game-purchase',
        'game-loot-box',
        'unknown-game',
        'unblocked-game-site',
      ],
      items: [
        {
          ruleId: 'browser-rule-game',
          targetType: 'cloud-gaming',
          targetValue: 'cloud.example.invalid',
          enabled: true,
          action: {
            kind: 'ask',
            approvalKind: 'unknown-category',
          },
        },
      ],
    },
  };
}

function validCompilerVocabularyEffectivePolicy() {
  return {
    ...validEffectivePolicy(),
    rules: [
      {
        ...validEffectivePolicy().rules[0],
        targetType: 'cloud-gaming',
        targetValue: 'cloud.example.invalid',
        action: 'ask',
        targetProofRequirement: 'browser-game-runtime-signal',
        capabilityState: 'manual-required',
        actionExecution: 'manual-required',
        aiAuthority: 'ai-candidate-only',
        compileNote: 'Browser game target remains manual-required until game runtime evidence exists.',
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
