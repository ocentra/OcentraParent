import { execFileSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const root = process.cwd();
const resultDirectory = join(root, 'test-results', 'screen-ai-browser-trigger-proof');
const proofPath = join(resultDirectory, 'proof.json');

await main();

async function main() {
  buildWorkspace('@ocentra-parent/schema-domain');
  buildWorkspace('@ocentra-parent/activity-domain');
  buildWorkspace('@ocentra-parent/parent-domain');

  const activityProof = await import('@ocentra-parent/activity-domain/screen-ai-browser-trigger-proof');
  const parentContext = await import('@ocentra-parent/parent-domain/local-ai-context-builder');

  const rows = activityProof.screenAiBrowserTriggerProof.rows;
  const localAiRows = rows.map((row) => localAiRow(row, parentContext.buildLocalAiEvidenceContext));
  const failures = [
    ...validateActivityRows(rows, activityProof.screenAiBrowserTriggerProofSummary(rows)),
    ...validateLocalAiRows(localAiRows),
  ];
  const proof = {
    schemaVersion: 1,
    proofMode: 'screen-ai-browser-trigger-proof',
    generatedAt: new Date().toISOString(),
    proofContract: '@ocentra-parent/activity-domain/screen-ai-browser-trigger-proof',
    localAiContextBuilder: '@ocentra-parent/parent-domain/local-ai-context-builder',
    rows: rows.map((row) => proofRowSummary(row)),
    localAiRows,
    summary: {
      ...activityProof.screenAiBrowserTriggerProofSummary(rows),
      localAiContextRows: localAiRows.length,
      failures: failures.length,
      checklistStatusChanged: false,
    },
    nonClaims: [
      'no portal UI proof',
      'no broad browser enforcement proof',
      'no live authenticated social account proof',
      'no cloud-streamed frame analysis proof',
      'no mobile browser parity proof',
      'no remote AI child-safety path',
    ],
    failures,
  };

  if (failures.length > 0) {
    throw new Error(`Screen-AI browser trigger proof failed:\n${failures.join('\n')}`);
  }

  await mkdir(resultDirectory, { recursive: true });
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log('screen-ai-browser-trigger-proof-ok=true');
  console.log(`proof=${relativePath(proofPath)}`);
  console.log(`rows=${proof.summary.totalRows} localAiRows=${proof.summary.localAiContextRows}`);
}

function buildWorkspace(workspace) {
  execFileSync('cmd', ['/c', 'npm', 'run', 'build', '--workspace', workspace], {
    cwd: root,
    stdio: 'inherit',
  });
}

function localAiRow(row, buildLocalAiEvidenceContext) {
  const browserRefId = `${row.rowId}-local-ai-browser-ref`;
  const screenRefId = `${row.rowId}-local-ai-screen-ref`;
  const contextInput = localAiContextInput(row, browserRefId, screenRefId);
  const result = buildLocalAiEvidenceContext(contextInput);

  return {
    rowId: row.rowId,
    surface: row.surface,
    triggerState: row.triggerState,
    expectedState: row.localAiContextExpectedState,
    actualState: result.state,
    browserEvidenceRefs: result.context?.browserEvidenceRefs ?? [],
    screenSummaryRefs: result.context?.screenSummaryRefs ?? [],
    localModelRuntimeRefs: result.context?.localModelRuntimeRefs ?? [],
    missingEvidenceKinds: result.missingEvidenceKinds,
    degradedReasons: result.context?.degradedReasons ?? [],
    degradedSourceRefs: result.degradedSourceRefs,
    custodyBoundarySummary: result.custodyBoundarySummary,
    validationGateSummary: result.validationGateSummary,
    browserRefId,
    screenRefId,
  };
}

function localAiContextInput(row, browserRefId, screenRefId) {
  const childProfile = { childProfileId: 'child-browser-trigger', displayName: 'Sam' };
  const device = {
    deviceId: 'windows-child-device',
    childProfileId: 'child-browser-trigger',
    label: 'Sam Windows PC',
    platform: 'windows',
  };
  const observedAt = row.browserInput.requestedAt;
  const browserEvidence = contextEvidence(row, browserRefId, 'browser', 'child-device-query-store', 'available');
  const screenEvidence = contextEvidence(
    row,
    screenRefId,
    'screen-summary',
    row.triggerState === 'unavailable' ? 'unavailable' : 'child-device-query-store',
    contextCapabilityStatus(row)
  );

  return {
    contextId: `${row.rowId}-local-ai-context`,
    request: {
      schemaVersion: 'v0.6',
      requestId: `${row.rowId}-local-ai-request`,
      requestedAt: observedAt,
      childProfile,
      device,
      requestedEvaluationKind: row.browserResult.videoKind === 'video' ? 'video' : 'mixed-context',
      requiredEvidenceKinds: ['browser', 'screen-summary'],
      parentRuleContextReferences: [parentRuleContext(row, browserRefId, childProfile, device, observedAt)],
      modelTaskRequirements: ['classification', 'safety-decision'],
      allowedCustody: ['child-device-query-store', 'child-device-journal'],
      promptVersion: 'screen-ai-browser-trigger-prompt-v1',
    },
    evidenceReferences: [browserEvidence, screenEvidence],
    runtimeReferences: [runtimeStatus(row)],
    memoryReferences: [],
    graphReferences: [],
  };
}

function contextEvidence(row, evidenceRefId, evidenceKind, custody, capabilityStatus) {
  const observedAt = row.browserInput.requestedAt;
  const sourceEvidence = row.sourceEvidenceRefs.find((reference) =>
    evidenceKind === 'browser' ? reference.evidenceId.includes('browser') : reference.evidenceId.includes('screen')
  );

  return {
    evidenceRefId,
    evidence: {
      evidenceReferenceId: `${evidenceRefId}-query-summary`,
      kind: 'query-store-summary',
      observedAt,
    },
    evidenceKind,
    sourceSchemaVersion: 'v0.6',
    observedAt,
    ingestedAt: observedAt,
    freshUntil: row.triggerState === 'unavailable' ? null : row.browserResult.expiresAt,
    sourceId: `${evidenceRefId}-source`,
    adapterId: `${evidenceRefId}-adapter`,
    device: {
      deviceId: 'windows-child-device',
      childProfileId: 'child-browser-trigger',
      label: 'Sam Windows PC',
      platform: 'windows',
    },
    childProfile: { childProfileId: 'child-browser-trigger', displayName: 'Sam' },
    custody,
    retentionState: row.triggerState === 'unavailable' ? 'unavailable' : 'local',
    confidence: evidenceKind === 'browser' ? null : row.screenAnalysis.confidence,
    confidenceKind: evidenceKind === 'browser' ? null : 'classifier',
    capabilityStatus,
    degradedReasons: degradedReasons(row, evidenceKind),
    unknownReasons: unknownReasons(row),
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: sourceEvidence?.evidenceId ?? `${evidenceRefId}-missing-source`,
        kind: 'journal-event',
        observedAt,
      },
    ],
  };
}

function parentRuleContext(row, browserRefId, childProfile, device, observedAt) {
  return {
    parentRuleRefId: `${row.rowId}-parent-rule-context`,
    policyVersion: 'policy-browser-trigger-v1',
    family: { familyId: 'family-browser-trigger' },
    childProfile,
    device,
    rule: {
      ruleId: `${row.rowId}-parent-rule`,
      target: {
        targetId: `${row.rowId}-target`,
        targetType: 'category',
        targetValue: row.browserResult.contentCategory,
      },
      action: row.triggerState === 'ready' ? 'warn' : 'ask-parent',
      scheduleId: null,
      priority: 10,
      reasonCode: `${row.rowId}-reason`,
      createdBy: { actorId: 'parent-browser-trigger', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    },
    targetEvidenceRefs: [browserRefId],
    custody: 'parent-device-cache',
    updatedAt: observedAt,
    expiresAt: null,
  };
}

function runtimeStatus(row) {
  const ready = row.localAiContextExpectedState === 'ready';

  return {
    runtimeReferenceId: `${row.rowId}-local-model-runtime`,
    providerId: 'local-provider',
    modelId: 'screen-browser-trigger-model',
    modelReference: 'local-model-cache/screen-browser-trigger',
    privacyMode: 'local-only',
    adapterBoundary: ready ? 'local-adapter-ready' : 'local-adapter-unavailable',
    executionState: ready ? 'dry-run-ready' : 'disabled',
    providerSource: ready ? 'local-model-cache' : 'unavailable',
    loadState: ready ? 'loaded' : 'unavailable',
    capabilityFlags: ready ? ['classification', 'safety-decision'] : [],
    resourceClass: ready ? 'cpu' : 'remote-unavailable',
    degradedState: ready ? 'none' : 'provider-unavailable',
    lastCheckedAt: row.browserInput.requestedAt,
    unavailableReason: ready ? null : `${row.rowId}-manual-or-unavailable`,
  };
}

function contextCapabilityStatus(row) {
  if (row.triggerState === 'ready') {
    return 'available';
  }
  if (row.triggerState === 'manual-required') {
    return 'degraded';
  }
  return 'unavailable';
}

function degradedReasons(row, evidenceKind) {
  if (row.triggerState === 'ready') {
    return [];
  }
  if (evidenceKind === 'browser') {
    return ['browser-active-tab-unknown'];
  }
  if (row.triggerState === 'manual-required') {
    return ['model-unavailable'];
  }
  return ['protected-surface', 'model-unavailable'];
}

function unknownReasons(row) {
  if (row.triggerState === 'unavailable') {
    return ['protected-surface'];
  }
  return [];
}

function validateActivityRows(rows, summary) {
  const failures = [];
  if (summary.readyRows !== 2) {
    failures.push('expected two ready browser-trigger rows');
  }
  if (summary.manualRequiredRows !== 1 || summary.unavailableRows !== 1) {
    failures.push('expected one manual-required row and one unavailable row');
  }
  for (const row of rows) {
    if (Object.values(row.noClaimFlags).some(Boolean)) {
      failures.push(`${row.rowId} contains a claim upgrade`);
    }
    if (row.browserInput.screenEvidenceRefs.length !== 1) {
      failures.push(`${row.rowId} does not cite exactly one screen evidence ref`);
    }
  }
  return failures;
}

function validateLocalAiRows(rows) {
  const failures = [];
  for (const row of rows) {
    if (row.actualState !== row.expectedState) {
      failures.push(`${row.rowId} local-AI state ${row.actualState} did not match ${row.expectedState}`);
    }
    if (!row.browserEvidenceRefs.includes(row.browserRefId)) {
      failures.push(`${row.rowId} local-AI context dropped browser evidence ref`);
    }
    if (row.expectedState === 'ready' && !row.screenSummaryRefs.includes(row.screenRefId)) {
      failures.push(`${row.rowId} ready local-AI context dropped screen summary ref`);
    }
    if (row.triggerState === 'unavailable' && !row.missingEvidenceKinds.includes('screen-summary')) {
      failures.push(`${row.rowId} unavailable row did not expose missing screen-summary state`);
    }
  }
  return failures;
}

function proofRowSummary(row) {
  return {
    rowId: row.rowId,
    surface: row.surface,
    triggerReason: row.triggerReason,
    triggerState: row.triggerState,
    browserContentKind: row.browserResult.contentKind,
    browserCategory: row.browserResult.contentCategory,
    screenCapabilityStatus: row.screenAnalysis.capabilityStatus,
    screenPolicyEligible: row.screenAnalysis.policyEligible,
    localAiExpectedState: row.localAiContextExpectedState,
    mobileParityState: row.mobileParityState,
    noClaimFlags: row.noClaimFlags,
  };
}

function relativePath(path) {
  return relative(root, path).replaceAll('\\', '/');
}
