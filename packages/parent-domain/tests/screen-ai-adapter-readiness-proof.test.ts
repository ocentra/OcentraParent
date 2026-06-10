import { describe, expect, it } from 'vitest';
import {
  screenAiFinalAdapterCompletionGate,
  screenAiFinalAdapterCompletionGateIsSatisfied,
  ScreenAiAdapterCompletionArtifactSchema,
  screenAiAdapterReadinessCoversRequiredBoundaries,
  ScreenAiAdapterReadinessReadModelSchema,
  ScreenAiAdapterReadinessRowSchema,
  summarizeScreenAiAdapterReadiness,
} from '../src/screen-ai-adapter-readiness-proof';

describe('screen AI adapter readiness proof', () => {
  it('covers real owned-process actions plus manual unavailable and not-claimed adapter states', () => {
    expectRequiredReadinessCoverage();
  });

  it('rejects rows that try to convert broad app network exact tab or mobile gaps into claims', () => {
    expectBroadClaimUpgradeRejected();
  });

  it('rejects real adapter rows without deleted screen custody and adapter proof', () => {
    expectRealAdapterCustodyRequired();
  });

  it('rejects manual required rows that claim an adapter executed', () => {
    expectManualRowExecutionRejected();
  });

  it('keeps final adapter completion closed until every broad browser network and mobile artifact is present', () => {
    expectPartialCompletionGateClosed();
  });

  it('opens final adapter completion only when every required artifact preserves apply rollback audit and custody', () => {
    expectCompleteGateOpen();
  });

  it('rejects final adapter artifacts that retain raw screen images or omit rollback custody', () => {
    expectInvalidCompletionArtifactRejected();
  });
});

function expectRequiredReadinessCoverage() {
  const parsed = parseRequiredReadModel();

  expect(parsed.rows).toHaveLength(8);
  expect(screenAiAdapterReadinessCoversRequiredBoundaries(parsed)).toBe(true);
  expect(summarizeScreenAiAdapterReadiness(parsed)).toEqual(expectedReadinessSummary());
}

function expectBroadClaimUpgradeRejected() {
  const row = requiredRows()[2];

  expect(() =>
    ScreenAiAdapterReadinessRowSchema.parse({
      ...row,
      rowId: 'invalid-broad-app-claim',
      claimFlags: {
        ...row.claimFlags,
        broadInstalledAppBlockingClaimed: true,
      },
    })
  ).toThrow(/claim upgrades/u);
}

function expectRealAdapterCustodyRequired() {
  const row = requiredRows()[0];

  expect(() =>
    ScreenAiAdapterReadinessRowSchema.parse({
      ...row,
      rowId: 'invalid-retained-image-real-adapter',
      rawImageRetained: true,
    })
  ).toThrow(/deleted-image custody/u);

  expect(() =>
    ScreenAiAdapterReadinessRowSchema.parse({
      ...row,
      rowId: 'invalid-real-adapter-without-proof',
      adapterExecutionProofArtifact: null,
    })
  ).toThrow(/adapter claim boundaries/u);
}

function expectManualRowExecutionRejected() {
  const row = requiredRows()[3];

  expect(() =>
    ScreenAiAdapterReadinessRowSchema.parse({
      ...row,
      rowId: 'invalid-manual-row-executed',
      actionExecutionState: 'executed',
    })
  ).toThrow(/adapter claim boundaries/u);
}

function expectPartialCompletionGateClosed() {
  const parsed = parseRequiredReadModel();
  const artifacts = completionArtifacts()
    .slice(0, 3)
    .map((artifact) => ScreenAiAdapterCompletionArtifactSchema.parse(artifact));

  expect(screenAiFinalAdapterCompletionGate(parsed, artifacts)).toEqual({
    completed: false,
    requiredRows: 5,
    completedRows: 3,
    missingRows: ['screen-ai-android-mobile-control-manual-required', 'screen-ai-ios-mobile-control-manual-required'],
    invalidRows: [],
    rawImageRetainedRows: 0,
  });
  expect(screenAiFinalAdapterCompletionGateIsSatisfied(parsed, artifacts)).toBe(false);
}

function expectCompleteGateOpen() {
  const parsed = parseRequiredReadModel();
  const artifacts = completionArtifacts().map((artifact) => ScreenAiAdapterCompletionArtifactSchema.parse(artifact));

  expect(screenAiFinalAdapterCompletionGate(parsed, artifacts)).toEqual({
    completed: true,
    requiredRows: 5,
    completedRows: 5,
    missingRows: [],
    invalidRows: [],
    rawImageRetainedRows: 0,
  });
  expect(screenAiFinalAdapterCompletionGateIsSatisfied(parsed, artifacts)).toBe(true);
}

function expectInvalidCompletionArtifactRejected() {
  const artifact = completionArtifacts()[0];

  expect(() =>
    ScreenAiAdapterCompletionArtifactSchema.parse({
      ...artifact,
      rawImageRetained: true,
    })
  ).toThrow(/screen-derived custody/u);

  expect(() =>
    ScreenAiAdapterCompletionArtifactSchema.parse({
      ...artifact,
      rollbackOrExpiryRef: '',
    })
  ).toThrow();
}

function parseRequiredReadModel() {
  return ScreenAiAdapterReadinessReadModelSchema.parse({
    schemaVersion: 'v0.6',
    readModelId: 'screen-ai-adapter-readiness-proof',
    generatedAt: '2026-06-05T16:13:00.000Z',
    sourceArtifacts: [
      'output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json',
      'output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json',
    ],
    rows: requiredRows(),
  });
}

function expectedReadinessSummary() {
  return {
    rowCount: 8,
    byReadinessState: {
      'real-owned-process-action-proved': 2,
      'manual-required': 4,
      'not-claimed': 1,
      unavailable: 1,
    },
    byPlatform: {
      windows: 5,
      android: 1,
      ios: 1,
      linux: 1,
    },
    executedRows: 2,
    skippedRows: 6,
    rawImageRetainedRows: 0,
    claimUpgradeRows: 0,
  };
}

function requiredRows() {
  return [...implementedRows(), ...manualAndUnavailableRows()];
}

function completionArtifacts() {
  return [
    completionArtifact('screen-ai-broad-installed-app-manual-required'),
    completionArtifact('screen-ai-host-network-domain-manual-required'),
    completionArtifact('screen-ai-managed-active-tab-not-claimed'),
    completionArtifact('screen-ai-android-mobile-control-manual-required'),
    completionArtifact('screen-ai-ios-mobile-control-manual-required'),
  ];
}

function completionArtifact(rowId: string) {
  return {
    schemaVersion: 'v0.6',
    rowId,
    sourcePolicyDecisionRef: 'policy-decision-screen-analysis-bypass-tool',
    sourceEvidenceRefs: [
      {
        evidenceReferenceId: `screen-analysis-evidence-${rowId}`,
        kind: 'activity-event',
        observedAt: '2026-06-04T08:53:32.027Z',
      },
    ],
    applyResultRef: `${rowId}-apply-result`,
    rollbackOrExpiryRef: `${rowId}-rollback-result`,
    auditRef: `${rowId}-audit-ref`,
    rawImageRetained: false,
    rawImageDeletedBeforeAdapter: true,
    screenDerivedPolicyDecision: true,
    finalAdapterCompletionClaimed: true,
  };
}

function implementedRows() {
  return [
    baseRow({
      rowId: 'screen-ai-owned-process-time-limit-real-adapter',
      sourcePolicyDecisionId: 'policy-decision-screen-analysis-native-owned-process-time-limit',
      sourcePolicyAction: 'time-limit',
      sourceProofArtifact: 'output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json',
      readinessState: 'real-owned-process-action-proved',
      actionExecutionState: 'executed',
      adapterRuntimeBoundary: 'windows-app-game-owned-process-time-limit',
      adapterCapability: 'app-game-owned-process-time-limit',
      adapterRuntimeState: 'implemented-boundary',
      adapterResult: 'supported-boundary-proved',
      platform: 'windows',
      platformSupportState: 'supported-on-windows',
      targetIdentityState: 'process-session-evidence-backed',
      rollbackReferenceState: 'timer-recovery-backed',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'none',
      adapterExecutionProofArtifact: 'output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json',
      linkedProofArtifacts: ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      manualProofRequirements: [],
    }),
    baseRow({
      rowId: 'screen-ai-owned-process-block-real-adapter',
      sourcePolicyDecisionId: 'policy-decision-screen-analysis-bypass-tool',
      sourcePolicyAction: 'block',
      sourceProofArtifact: 'output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json',
      readinessState: 'real-owned-process-action-proved',
      actionExecutionState: 'executed',
      adapterRuntimeBoundary: 'windows-screen-owned-process-block',
      adapterCapability: 'screen-owned-process-block',
      adapterRuntimeState: 'implemented-boundary',
      adapterResult: 'supported-boundary-proved',
      platform: 'windows',
      platformSupportState: 'supported-on-windows',
      targetIdentityState: 'process-session-evidence-backed',
      rollbackReferenceState: 'not-required',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'none',
      adapterExecutionProofArtifact: 'output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json',
      linkedProofArtifacts: ['output/screen-ai-pipeline-proof/block-action-dispatch/02-adapter-proof.json'],
      manualProofRequirements: [],
    }),
  ];
}

function manualAndUnavailableRows() {
  return [
    manualRow(
      'screen-ai-broad-installed-app-manual-required',
      'windows-broad-installed-app-blocking-manual-gate',
      'broad-installed-app-blocking',
      'same app identity proof'
    ),
    manualRow(
      'screen-ai-host-network-domain-manual-required',
      'windows-host-network-domain-blocking-manual-gate',
      'host-network-domain-blocking',
      'host DNS or filter apply artifact'
    ),
    managedActiveTabRow(),
    mobileManualRow(
      'screen-ai-android-mobile-control-manual-required',
      'android-mobile-control-manual-gate',
      'android'
    ),
    mobileManualRow('screen-ai-ios-mobile-control-manual-required', 'ios-mobile-control-manual-gate', 'ios'),
    linuxUnavailableRow(),
  ];
}

function managedActiveTabRow() {
  return {
    ...manualRow(
      'screen-ai-managed-active-tab-not-claimed',
      'windows-managed-exact-active-tab-not-claimed',
      'managed-exact-active-tab-enforcement',
      'managed active-tab evidence artifact'
    ),
    readinessState: 'not-claimed',
    adapterRuntimeState: 'not-claimed',
    adapterResult: 'not-claimed',
    platformSupportState: 'manual-required',
    rollbackReferenceState: 'not-claimed',
    auditReferenceState: 'not-claimed',
    refusalReason: 'not-claimed-boundary',
  };
}

function linuxUnavailableRow() {
  return baseRow({
    rowId: 'screen-ai-linux-host-adapter-unavailable',
    adapterRuntimeBoundary: 'linux-host-adapter-unavailable',
    adapterCapability: 'desktop-host-platform-adapter',
    platform: 'linux',
    readinessState: 'unavailable',
    actionExecutionState: 'skipped',
    adapterRuntimeState: 'unavailable',
    adapterResult: 'target-unavailable',
    platformSupportState: 'unavailable-on-target',
    targetIdentityState: 'unsupported-platform-target',
    rollbackReferenceState: 'unavailable',
    auditReferenceState: 'unavailable',
    refusalReason: 'target-unavailable',
    adapterExecutionProofArtifact: null,
    linkedProofArtifacts: [],
    manualProofRequirements: ['Linux service manager artifact', 'Linux rollback artifact'],
  });
}

function baseRow(overrides: Record<string, unknown>) {
  return {
    schemaVersion: 'v0.6',
    rowId: 'screen-ai-adapter-readiness-row',
    sourcePolicyDecisionId: 'policy-decision-screen-analysis-bypass-tool',
    sourcePolicyAction: 'block',
    sourcePolicyDryRun: true,
    sourceProofArtifact: 'output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json',
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: 'screen-analysis-evidence-bypass-tool',
        kind: 'activity-event',
        observedAt: '2026-06-04T08:53:32.027Z',
      },
    ],
    sourceImageDeletionState: 'deleted',
    rawImageRetained: false,
    rawImageDeletedBeforeAdapter: true,
    readinessState: 'manual-required',
    actionExecutionState: 'skipped',
    adapterRuntimeBoundary: 'windows-broad-installed-app-blocking-manual-gate',
    adapterCapability: 'broad-installed-app-blocking',
    adapterRuntimeState: 'manual-required',
    adapterResult: 'manual-proof-required',
    platform: 'windows',
    platformSupportState: 'manual-required',
    targetIdentityState: 'insufficient-for-broad-target',
    rollbackReferenceState: 'manual-required',
    auditReferenceState: 'manual-required',
    refusalReason: 'manual-artifact-required',
    adapterExecutionProofArtifact: null,
    linkedProofArtifacts: [],
    manualProofRequirements: ['adapter proof artifact'],
    claimFlags: {
      broadInstalledAppBlockingClaimed: false,
      networkDomainBlockingClaimed: false,
      exactActiveTabEnforcementClaimed: false,
      notificationDeliveryClaimed: false,
      tamperHardeningClaimed: false,
      mobileControlClaimed: false,
      unsupportedPlatformBehaviorClaimed: false,
    },
    claimBoundary: 'Screen AI action readiness does not upgrade broad adapter claims.',
    fallbackBehavior: 'Report manual-required, not-claimed, unavailable, unsupported, or degraded without executing.',
    ...overrides,
  };
}

function manualRow(rowId: string, adapterRuntimeBoundary: string, adapterCapability: string, requirement: string) {
  return baseRow({
    rowId,
    adapterRuntimeBoundary,
    adapterCapability,
    manualProofRequirements: [requirement, 'rollback artifact', 'audit custody artifact'],
  });
}

function mobileManualRow(rowId: string, adapterRuntimeBoundary: string, platform: string) {
  return baseRow({
    rowId,
    adapterRuntimeBoundary,
    adapterCapability: 'mobile-child-control-adapter',
    platform,
    targetIdentityState: 'unsupported-platform-target',
    manualProofRequirements: ['privileged mobile platform artifact', 'rollback artifact', 'device artifact'],
  });
}
