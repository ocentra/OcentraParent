import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-deterministic-classifier-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-deterministic-classifier-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const ObservedAt = '2026-06-06T07:30:00.000Z';
const ChildProfile = {
  childProfileId: 'child:maya',
  displayName: 'Maya',
};
const Device = {
  deviceId: 'device:maya-windows',
  childProfileId: 'child:maya',
  label: 'Maya Windows laptop',
  platform: 'windows',
};
const Runtime = {
  runtimeReferenceId: 'local-ai-runtime-deterministic-classifier',
  providerId: 'local-provider-deterministic-classifier',
  modelId: 'deterministic-classifier-v1',
  modelReference: 'artifact:deterministic_classifier_v1',
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

runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
runCommand('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'local-ai-deterministic-classifier-proof',
]);

const { runLocalAiDeterministicClassifier } = await import(
  pathToFileURL(resolve(RepoRoot, 'packages', 'parent-domain', 'dist', 'local-ai-deterministic-classifier-proof.js'))
    .href
);

const rows = [
  rowFor('video-warning', classifierInput('video')),
  rowFor('productivity-allow', classifierInput('window')),
  rowFor('app-time-limit', classifierInput('app')),
  rowFor('process-block', classifierInput('process')),
  rowFor('network-review', classifierInput('network')),
  rowFor('missing-evidence-unknown', {
    ...classifierInput('page'),
    evaluationInput: {
      ...classifierInput('page').evaluationInput,
      evidenceReferences: [],
    },
  }),
  rowFor('runtime-unavailable-ask-parent', {
    ...classifierInput('video'),
    modelRuntime: {
      ...Runtime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-deterministic-classifier-unavailable',
    },
  }),
];
const failures = rows.flatMap(validateRow);

if (failures.length > 0) {
  throw new Error(`Local AI deterministic classifier proof failed:\n${failures.join('\n')}`);
}

const proof = {
  status: 'ok',
  proofKind: 'local-ai-deterministic-classifier-proof',
  generatedAt: new Date().toISOString(),
  output: relativePath(ProofPath),
  rows,
  summary: {
    classifiedRows: rows.filter((row) => row.state === 'classified').length,
    lowConfidenceRows: rows.filter((row) => row.state === 'low-confidence').length,
    missingEvidenceRows: rows.filter((row) => row.state === 'missing-evidence').length,
    runtimeUnavailableRows: rows.filter((row) => row.state === 'runtime-unavailable').length,
    dryRunRows: rows.filter((row) => row.dryRun).length,
    deterministicOnly: rows.every((row) => row.deterministicOnly),
    localOnly: rows.every((row) => row.localOnly),
    modelExecuted: rows.some((row) => row.modelExecuted),
    remoteAiUsed: rows.some((row) => row.remoteApiClaimed),
    rawEvidenceRetained: rows.some((row) => row.rawEvidenceRetained),
    policyAuthorityClaimed: rows.some((row) => row.policyAuthorityClaimed),
    enforcementClaimed: rows.some((row) => row.enforcementClaimed),
    failures: failures.length,
  },
  assertions: [
    'Typed local AI inputs can produce deterministic allow/warn/ask-parent/time-limit/block dry-run rows without executing a model.',
    'Low-confidence network/page/domain classifications degrade before policy can treat them as model-quality proof.',
    'Missing evidence and unavailable local runtime remain typed non-enforcing unknown or ask-parent results.',
    'All rows preserve evidence, parent rule, runtime, prompt, and trace refs while rejecting raw evidence retention.',
  ],
  nonClaims: [
    'This proof does not execute a model or prove production model quality.',
    'This proof does not use remote/API AI, grant policy authority, render portal UI, or dispatch enforcement.',
    'This proof does not create fresh screen capture; it proves a deterministic classifier lane over typed evidence refs.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-deterministic-classifier-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-deterministic-classifier-proof-ok:${rows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function rowFor(rowId, input) {
  const result = runLocalAiDeterministicClassifier(input);
  return {
    rowId,
    state: result.state,
    contextKind: result.contextKind,
    action: result.result.action,
    confidence: result.result.confidence,
    unknownState: result.result.unknownState,
    degradedState: result.result.degradedState,
    reasonCodes: result.result.reasonCodes,
    evidenceRefs: result.result.evidenceReferences.map((reference) => reference.evidenceReferenceId),
    parentRuleRefs: result.result.parentRuleReferences,
    runtimeRef: result.modelRuntime.runtimeReferenceId,
    providerId: result.modelRuntime.providerId,
    modelId: result.modelRuntime.modelId,
    promptVersion: result.promptVersion,
    traceRefs: result.classifierTraceRefs,
    dryRun: result.dryRun,
    deterministicOnly: result.deterministicOnly,
    localOnly: result.localOnly,
    modelExecuted: result.modelExecuted,
    remoteApiClaimed: result.remoteApiClaimed,
    rawEvidenceRetained: result.rawEvidenceRetained,
    policyAuthorityClaimed: result.policyAuthorityClaimed,
    enforcementClaimed: result.enforcementClaimed,
  };
}

function validateRow(row) {
  const failures = [];
  if (!row.deterministicOnly || !row.localOnly) {
    failures.push(`${row.rowId} lost deterministic/local-only boundary`);
  }
  if (!row.dryRun) {
    failures.push(`${row.rowId} was not marked as a dry-run classifier row`);
  }
  if (
    row.modelExecuted ||
    row.remoteApiClaimed ||
    row.rawEvidenceRetained ||
    row.policyAuthorityClaimed ||
    row.enforcementClaimed
  ) {
    failures.push(`${row.rowId} overclaimed model, remote, raw-retention, policy, or enforcement authority`);
  }
  if (row.rowId === 'video-warning' && row.action !== 'warn') {
    failures.push(`${row.rowId} action was ${row.action}`);
  }
  if (row.rowId === 'productivity-allow' && row.action !== 'allow') {
    failures.push(`${row.rowId} action was ${row.action}`);
  }
  if (row.rowId === 'app-time-limit' && row.action !== 'time-limit') {
    failures.push(`${row.rowId} action was ${row.action}`);
  }
  if (row.rowId === 'process-block' && row.action !== 'block') {
    failures.push(`${row.rowId} action was ${row.action}`);
  }
  if (row.rowId === 'network-review' && row.state !== 'low-confidence') {
    failures.push(`${row.rowId} state was ${row.state}`);
  }
  if (row.rowId === 'missing-evidence-unknown' && row.unknownState !== 'missing-evidence') {
    failures.push(`${row.rowId} unknownState was ${row.unknownState}`);
  }
  if (row.rowId === 'runtime-unavailable-ask-parent' && row.unknownState !== 'model-unavailable') {
    failures.push(`${row.rowId} unknownState was ${row.unknownState}`);
  }
  return failures;
}

function classifierInput(contextKind) {
  const evidence = evidenceReference(contextKind);
  return {
    schemaVersion: 'v0.6',
    classifierRunId: `local-ai-deterministic-classifier:${contextKind}`,
    rawEvidenceRetained: false,
    modelRuntime: Runtime,
    evaluationInput: {
      schemaVersion: 'v0.6',
      requestId: `local-ai-eval:deterministic-classifier:${contextKind}`,
      childProfile: ChildProfile,
      device: Device,
      currentObservation: {
        contextKind,
        evidence,
      },
      evidenceReferences: [evidence],
      parentRuleReferences: ['policy-rule:screen-video-warn'],
      recentActivityWindow: [evidence],
      memoryReferences: [],
      graphReferences: [],
      modelRequest: {
        providerId: Runtime.providerId,
        modelId: Runtime.modelId,
        promptVersion: 'prompt:deterministic-classifier:v1',
      },
    },
  };
}

function evidenceReference(contextKind) {
  return {
    evidenceReferenceId: `evidence:${contextKind}`,
    kind: 'query-store-summary',
    observedAt: ObservedAt,
  };
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}
