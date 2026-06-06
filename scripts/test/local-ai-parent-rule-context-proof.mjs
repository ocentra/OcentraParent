import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-parent-rule-context-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-parent-rule-context-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const ObservedAt = '2026-06-06T05:35:00.000Z';
const ChildProfile = { childProfileId: 'child:maya', displayName: 'Maya' };
const Device = {
  deviceId: 'device:maya-windows',
  childProfileId: ChildProfile.childProfileId,
  label: 'Maya Windows laptop',
  platform: 'windows',
};

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'local-ai-context-parent-rule-selection',
]);

const { buildLocalAiEvidenceContext } = await import('@ocentra-parent/parent-domain/local-ai-context-builder');

const groundedInput = contextInput({
  contextId: 'parent-rule-context-grounded',
  evidenceReferences: [evidenceReference('screen-ref-education', 'screen-summary')],
  parentRuleContextReferences: [parentRuleContext('parent-rule-context:screen-education', ['screen-ref-education'])],
});
const ungroundedInput = contextInput({
  contextId: 'parent-rule-context-ungrounded',
  evidenceReferences: [evidenceReference('screen-ref-education', 'screen-summary')],
  parentRuleContextReferences: [parentRuleContext('parent-rule-context:unselected-social', ['social-ref-private'])],
});
const missingRuleInput = contextInput({
  contextId: 'parent-rule-context-missing',
  evidenceReferences: [evidenceReference('screen-ref-education', 'screen-summary')],
  parentRuleContextReferences: [],
});

const rows = [
  rowFor('grounded-parent-rule-selected', groundedInput),
  rowFor('ungrounded-parent-rule-rejected', ungroundedInput),
  rowFor('missing-parent-rule-degraded', missingRuleInput),
];
const failures = rows.flatMap(validateRow);

if (failures.length > 0) {
  throw new Error(`Local AI parent-rule context proof failed:\n${failures.join('\n')}`);
}

const proof = {
  status: 'ok',
  proofKind: 'local-ai-parent-rule-context-proof',
  generatedAt: new Date().toISOString(),
  output: relativePath(ProofPath),
  rows,
  summary: {
    groundedRows: rows.filter((row) => row.selectedParentRuleRefs.length === 1).length,
    ungroundedRows: rows.filter((row) => row.ungroundedParentRuleReferenceCount === 1).length,
    missingRuleRows: rows.filter((row) => row.degradedReasons.includes('parent-rule-missing')).length,
    remoteAiUsed: false,
    rawEvidenceRetained: false,
    policyAuthorityClaimed: false,
    enforcementClaimed: false,
    failures: failures.length,
  },
  assertions: [
    'Grounded parent rule context refs are selected only when every target evidence ref survives context selection.',
    'Parent rule refs targeting filtered or missing evidence are rejected before model input and counted as ungrounded.',
    'Missing parent rule context degrades explicitly with parent-rule-missing instead of silently producing policy authority.',
  ],
  nonClaims: [
    'This proof exercises existing local AI context contracts; it does not execute a model or prove production model quality.',
    'This proof does not claim remote/API AI, policy authority, portal UI, or enforcement.',
    'This proof does not create fresh capture; it proves parent-rule context grounding over typed stored evidence refs.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-context-parent-rule-selection',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-parent-rule-context-proof-ok:${rows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function contextInput({ contextId, evidenceReferences, parentRuleContextReferences }) {
  return {
    contextId,
    request: {
      schemaVersion: 'v0.6',
      requestId: `request:${contextId}`,
      requestedAt: ObservedAt,
      childProfile: ChildProfile,
      device: Device,
      requestedEvaluationKind: 'screen-summary',
      requiredEvidenceKinds: ['screen-summary'],
      parentRuleContextReferences,
      modelTaskRequirements: ['classification', 'safety-decision'],
      allowedCustody: ['child-device-query-store', 'parent-device-cache'],
      promptVersion: 'prompt:screen-parent-rule:v1',
    },
    evidenceReferences,
    runtimeReferences: [runtimeReference()],
    memoryReferences: [],
    graphReferences: [],
  };
}

function evidenceReference(evidenceRefId, evidenceKind) {
  return {
    evidenceRefId,
    evidence: {
      evidenceReferenceId: `evidence:${evidenceRefId}`,
      kind: 'query-store-summary',
      observedAt: ObservedAt,
    },
    evidenceKind,
    sourceSchemaVersion: 'v0.6',
    observedAt: ObservedAt,
    ingestedAt: ObservedAt,
    freshUntil: null,
    sourceId: `source:${evidenceRefId}`,
    adapterId: 'adapter:screen-summary-parent-rule-proof',
    device: Device,
    childProfile: ChildProfile,
    custody: 'child-device-query-store',
    retentionState: 'deleted-source',
    confidence: 0.91,
    confidenceKind: 'classifier',
    capabilityStatus: 'available',
    degradedReasons: ['screen-image-deleted'],
    unknownReasons: [],
    sourceEvidenceReferences: [
      {
        evidenceReferenceId: `journal:${evidenceRefId}`,
        kind: 'journal-event',
        observedAt: ObservedAt,
      },
    ],
  };
}

function parentRuleContext(parentRuleRefId, targetEvidenceRefs) {
  return {
    parentRuleRefId,
    policyVersion: 'policy:screen-rules:v1',
    family: { familyId: 'family:maya' },
    childProfile: ChildProfile,
    device: Device,
    rule: {
      ruleId: `rule:${parentRuleRefId}`,
      target: {
        targetId: `target:${parentRuleRefId}`,
        targetType: 'category',
        targetValue: 'school',
      },
      action: 'allow',
      scheduleId: null,
      priority: 20,
      reasonCode: 'parent-rule-school-allow',
      createdBy: { actorId: 'parent:alex', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    },
    targetEvidenceRefs,
    custody: 'parent-device-cache',
    updatedAt: ObservedAt,
    expiresAt: null,
  };
}

function runtimeReference() {
  return {
    runtimeReferenceId: 'runtime:local-screen-parent-rule-proof',
    providerId: 'provider:local-screen-safety',
    modelId: 'model:screen-safety-local',
    modelReference: 'artifact:screen_safety_local',
    privacyMode: 'local-only',
    adapterBoundary: 'local-adapter-ready',
    executionState: 'dry-run-ready',
    providerSource: 'local-model-cache',
    loadState: 'loaded',
    capabilityFlags: ['classification', 'safety-decision'],
    resourceClass: 'cpu',
    degradedState: 'none',
    lastCheckedAt: ObservedAt,
    unavailableReason: null,
  };
}

function rowFor(rowId, input) {
  const result = buildLocalAiEvidenceContext(input);
  return {
    rowId,
    contextState: result.state,
    selectedEvidenceRefs: result.context?.evidenceReferences.map((reference) => reference.evidenceRefId) ?? [],
    selectedParentRuleRefs:
      result.context?.parentRuleContextReferences.map((reference) => reference.parentRuleRefId) ?? [],
    parentRuleReferences: result.context?.parentRuleReferences ?? [],
    degradedReasons: result.context?.degradedReasons ?? [],
    validationSummary: result.context?.validationSummary ?? null,
    ungroundedParentRuleReferenceCount: result.context?.validationSummary.ungroundedParentRuleReferenceCount ?? 0,
    rejectedFields: result.rejectedFields,
    missingEvidenceKinds: result.missingEvidenceKinds,
    custodyBoundarySummary: result.custodyBoundarySummary,
    validationGateSummary: result.validationGateSummary,
  };
}

function validateRow(row) {
  if (row.rowId === 'grounded-parent-rule-selected') {
    return validateGroundedRow(row);
  }
  if (row.rowId === 'ungrounded-parent-rule-rejected') {
    return validateUngroundedRow(row);
  }
  return validateMissingRuleRow(row);
}

function validateGroundedRow(row) {
  const failures = [];
  if (row.contextState !== 'ready') {
    failures.push(`${row.rowId} state was ${row.contextState}`);
  }
  if (row.selectedParentRuleRefs.length !== 1 || row.parentRuleReferences.length !== 1) {
    failures.push(`${row.rowId} selected ${row.selectedParentRuleRefs.length} parent rule context refs`);
  }
  if (row.ungroundedParentRuleReferenceCount !== 0) {
    failures.push(`${row.rowId} had ${row.ungroundedParentRuleReferenceCount} ungrounded parent rule refs`);
  }
  return failures;
}

function validateUngroundedRow(row) {
  const failures = [];
  if (row.contextState !== 'partial') {
    failures.push(`${row.rowId} state was ${row.contextState}`);
  }
  if (!row.degradedReasons.includes('parent-rule-missing')) {
    failures.push(`${row.rowId} omitted parent-rule-missing degraded reason`);
  }
  if (row.ungroundedParentRuleReferenceCount !== 1) {
    failures.push(`${row.rowId} had ${row.ungroundedParentRuleReferenceCount} ungrounded parent rule refs`);
  }
  return failures;
}

function validateMissingRuleRow(row) {
  const failures = [];
  if (row.contextState !== 'partial') {
    failures.push(`${row.rowId} state was ${row.contextState}`);
  }
  if (!row.degradedReasons.includes('parent-rule-missing')) {
    failures.push(`${row.rowId} omitted parent-rule-missing degraded reason`);
  }
  if (row.selectedParentRuleRefs.length !== 0) {
    failures.push(`${row.rowId} selected ${row.selectedParentRuleRefs.length} parent rule refs`);
  }
  return failures;
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}
