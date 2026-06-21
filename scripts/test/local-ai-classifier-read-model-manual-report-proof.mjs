import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-classifier-read-model-manual-report-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-classifier-read-model-manual-report-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const GeneratedAt = '2026-06-06T11:05:00.000Z';
const ObservedAt = '2026-06-06T07:30:00.000Z';
const SourceProofRefs = ['output/ai-plan-proof/local-ai-deterministic-classifier-proof/proof-summary.json'];
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

runCommand(...npmCommand(['run', 'build:contracts']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/ai-domain',
    '--',
    'local-ai-classifier-read-model-manual-report-proof',
  ])
);

const { buildLocalAiClassifierReportSnapshot } = await import(
  '@ocentra-parent/schema-domain/local-ai-classifier-read-model-manual-report-proof'
);
const { runLocalAiDeterministicClassifier } = await import(
  '@ocentra-parent/schema-domain/local-ai-deterministic-classifier-proof'
);

const snapshot = buildLocalAiClassifierReportSnapshot({
  generatedAt: GeneratedAt,
  snapshotId: 'local-ai-classifier-report:deterministic-lane',
  sourceProofRefs: SourceProofRefs,
  classifierResults: [
    classifierResult('video'),
    classifierResult('app'),
    classifierResult('process'),
    classifierResult('network'),
    missingEvidenceResult(),
    runtimeUnavailableResult(),
  ],
});
const failures = validateSnapshot(snapshot);

if (failures.length > 0) {
  throw new Error(`Local AI classifier read-model manual report proof failed:\n${failures.join('\n')}`);
}

const proof = {
  status: 'ok',
  proofKind: 'local-ai-classifier-read-model-manual-report-proof',
  generatedAt: new Date().toISOString(),
  output: relativePath(ProofPath),
  snapshot,
  summary: {
    rowCount: snapshot.rows.length,
    readyRowCount: snapshot.readyRowCount,
    manualRequiredRowCount: snapshot.manualRequiredRowCount,
    unavailableRowCount: snapshot.unavailableRowCount,
    reportOnlyRows: snapshot.rows.filter((row) => row.reportOnly).length,
    dryRunRows: snapshot.rows.filter((row) => row.dryRun).length,
    modelExecuted: snapshot.rows.some((row) => row.modelExecuted),
    rawEvidenceRetained: snapshot.rows.some((row) => row.rawEvidenceRetained),
    rawModelOutputRetained: snapshot.rows.some((row) => row.rawModelOutputRetained),
    remoteAiUsed: snapshot.rows.some((row) => row.remoteApiClaimed),
    policyAuthorityClaimed: snapshot.rows.some((row) => row.policyAuthorityClaimed),
    enforcementClaimed: snapshot.rows.some((row) => row.enforcementClaimed),
    productionModelQualityClaimed: snapshot.rows.some((row) => row.productionModelQualityClaimed),
    failures: failures.length,
  },
  assertions: [
    'Deterministic classifier dry-run rows can project into parent-facing read-model report rows.',
    'Low-confidence, missing-evidence, and unavailable classifier rows remain manual-required or unavailable for parent review.',
    'Report rows preserve evidence, parent rule, runtime, provider, model, prompt, proof, and classifier trace refs.',
    'The bridge rejects model execution, remote/API AI, raw retention, policy authority, enforcement, and model-quality overclaims.',
  ],
  nonClaims: [
    'This proof does not create fresh capture, execute a model, or prove production model quality.',
    'This proof does not render portal UI, make policy decisions, dispatch enforcement, or use remote/API AI.',
    'This proof is stacked on the deterministic classifier contract proof until the parent-rule and classifier PRs land.',
  ],
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build:contracts',
    'cmd /c npm run test --workspace @ocentra-parent/ai-domain -- local-ai-classifier-read-model-manual-report-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(`local-ai-classifier-read-model-manual-report-proof-ok:${snapshot.rows.length}`);
console.log(`proof=${relativePath(ProofPath)}`);

function validateSnapshot(snapshot) {
  const failures = [];
  if (snapshot.readyRowCount !== 3) {
    failures.push(`readyRowCount was ${snapshot.readyRowCount}`);
  }
  if (snapshot.manualRequiredRowCount !== 2) {
    failures.push(`manualRequiredRowCount was ${snapshot.manualRequiredRowCount}`);
  }
  if (snapshot.unavailableRowCount !== 1) {
    failures.push(`unavailableRowCount was ${snapshot.unavailableRowCount}`);
  }
  for (const row of snapshot.rows) {
    if (!row.reportOnly || !row.dryRun) {
      failures.push(`${row.reportRowId} lost report-only dry-run boundary`);
    }
    if (
      row.modelExecuted ||
      row.rawEvidenceRetained ||
      row.rawModelOutputRetained ||
      row.remoteApiClaimed ||
      row.policyAuthorityClaimed ||
      row.enforcementClaimed ||
      row.productionModelQualityClaimed
    ) {
      failures.push(`${row.reportRowId} overclaimed execution, raw retention, remote AI, policy, or enforcement`);
    }
    if (row.reportState !== 'ready' && row.manualRequiredReasons.length === 0) {
      failures.push(`${row.reportRowId} did not expose manual-required reasons`);
    }
  }
  return failures;
}

function classifierResult(contextKind) {
  return runLocalAiDeterministicClassifier(classifierInput(contextKind));
}

function missingEvidenceResult() {
  const input = classifierInput('page');
  return runLocalAiDeterministicClassifier({
    ...input,
    evaluationInput: {
      ...input.evaluationInput,
      evidenceReferences: [],
    },
  });
}

function runtimeUnavailableResult() {
  return runLocalAiDeterministicClassifier({
    ...classifierInput('video'),
    modelRuntime: {
      ...Runtime,
      executionState: 'disabled',
      loadState: 'unavailable',
      degradedState: 'provider-unavailable',
      unavailableReason: 'local-ai-deterministic-classifier-unavailable',
    },
  });
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
      childProfile: {
        childProfileId: 'child:maya',
        displayName: 'Maya',
      },
      device: {
        deviceId: 'device:maya-windows',
        childProfileId: 'child:maya',
        label: 'Maya Windows laptop',
        platform: 'windows',
      },
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
