import { describe, expect, it } from 'vitest';

import {
  BrowserGamePolicyCompilerInputSchema,
  BrowserGamePolicyDecisionCandidateSchema,
} from '../../src/browser-game-policy-compiler';
import {
  BaselineBrowserControlAuthoringManifest,
} from '../../src/browser-control-baseline-manifest';
import { BrowserControlManifestDefaults } from '../../src/browser-control-manifest';
import {
  BaselineBrowserControlFullCatalog,
  browserControlFullCatalogSettingCount,
  browserControlFullCatalogSettings,
} from '../../src/browser-control-full-catalog';

const invalidBrowserGameCompilerInput = {
  schemaVersion: 'v0.6',
  compileRequestId: 'compile-1',
  familyId: 'family-1',
  childProfileId: 'child-1',
  deviceId: 'device-1',
  requestedAt: '2026-06-29T05:00:00.000Z',
  policyVersionRef: 'policy-1',
  targetKind: 'browser-game-url',
  sourceEvidenceRefs: ['evidence-1'],
  analysisRefs: ['analysis-1'],
  mobileCapabilityRefs: [],
  parentRuleRefs: ['rule-1'],
  scheduleContextRefs: [],
  compilerMode: 'contract-only',
  rawGamePayloadIncluded: false,
  rawModelTextIncluded: false,
  activityDomainObjectIncluded: false,
  finalDecisionClaimedByInput: true,
  runtimeGateClaimedByInput: false,
  uiClaimedByInput: false,
  enforcementClaimedByInput: false,
  nativeGameControlClaimed: false,
  cloudFrameAnalysisClaimed: false,
} as const;

const invalidBrowserGameDecisionCandidate = {
  schemaVersion: 'v0.6',
  decisionCandidateId: 'candidate-1',
  compileRequestId: 'compile-1',
  decidedAt: '2026-06-29T05:00:00.000Z',
  expiresAt: null,
  policyVersionRef: 'policy-1',
  targetKind: 'browser-game-url',
  sourceEvidenceRefs: ['evidence-1'],
  analysisRefs: ['analysis-1'],
  mobileCapabilityRefs: [],
  parentRuleRefs: ['rule-1'],
  scheduleContextRefs: [],
  actionCandidate: 'warn-candidate',
  reasonCodes: ['schedule-context'],
  confidence: 'medium',
  compilerMode: 'contract-only',
  compilerCapabilityState: 'ready',
  fallbackUsed: false,
  parentApprovalRequired: false,
  finalPolicyDecisionClaimed: true,
  runtimeGateExecutedClaimed: false,
  uiRenderedClaimed: false,
  enforcementClaimed: false,
  nativeGameControlClaimed: false,
  cloudFrameAnalysisClaimed: false,
  rawGamePayloadStored: false,
  rawModelTextUsed: false,
} as const;

describe('browser policy/control schema adapters stay generated-backed', () => {
  it('rejects browser game compiler inputs that claim runtime or final authority', () => {
    expect(BrowserGamePolicyCompilerInputSchema.safeParse(invalidBrowserGameCompilerInput).success).toBe(false);
  });

  it('keeps browser game decision candidates non-final and contract-only', () => {
    expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(invalidBrowserGameDecisionCandidate).success).toBe(false);
  });

  it('flows generated manifest ids into the baseline authoring manifest', () => {
    expect(BaselineBrowserControlAuthoringManifest.manifestId).toBe(BrowserControlManifestDefaults.ManifestId);
    expect(BaselineBrowserControlAuthoringManifest.sections[0]?.sectionId).toBe(
      BrowserControlManifestDefaults.Section.Management
    );
    expect(BaselineBrowserControlAuthoringManifest.sections[2]?.fields[1]?.defaultValue).toEqual([
      'edge-stable',
      'chrome-stable',
      'chrome-for-testing',
    ]);
  });

  it('keeps proof and capability states explicit in the full browser catalog', () => {
    const settings = browserControlFullCatalogSettings();
    const proofSetting = settings.find(
      (setting) =>
        setting.effectStatus === 'proof-required' &&
        setting.capabilityState === 'protected' &&
        setting.proofRequirement === 'managed-browser-or-explicit-browser-integration'
    );
    const childAgentSetting = settings.find((setting) => setting.runtimeOwner === 'child-agent');
    const manualSetting = settings.find(
      (setting) =>
        setting.effectStatus === 'manual-required' &&
        setting.capabilityState === 'manual-required'
    );

    expect(browserControlFullCatalogSettingCount()).toBe(BaselineBrowserControlFullCatalog.settingCount);
    expect(proofSetting).toMatchObject({
      effectStatus: 'proof-required',
      capabilityState: 'protected',
      proofRequirement: 'managed-browser-or-explicit-browser-integration',
    });
    expect(childAgentSetting?.runtimeOwner).toBe('child-agent');
    expect(manualSetting).toMatchObject({
      effectStatus: 'manual-required',
      capabilityState: 'manual-required',
    });
  });
});
