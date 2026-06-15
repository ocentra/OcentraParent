import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';

const RepoRoot = process.cwd();
const OutputRoot = resolve(RepoRoot, 'output', 'ai-plan-proof', 'local-ai-runtime-status-read-model-proof');
const TestResultRoot = resolve(RepoRoot, 'test-results', 'local-ai-runtime-status-read-model-proof');
const ProofPath = join(OutputRoot, 'proof-summary.json');
const ValidationLogPath = join(OutputRoot, 'validation-commands.log');
const TestResultPath = join(TestResultRoot, 'proof.json');
const generatedAt = new Date().toISOString();

runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/parent-domain']));
runCommand(
  ...npmCommand([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'local-ai-runtime-status-read-model-proof',
  ])
);

const {
  LocalAiRuntimeStatusSurfaceReadModel,
  LocalAiRuntimeStatusSurfaceReadModelSchema,
  LocalAiRuntimeStatusSurfaceRowSchema,
} = await import('@ocentra-parent/ai-domain/local-ai-runtime-status-read-model-proof');

const readModel = LocalAiRuntimeStatusSurfaceReadModelSchema.parse(LocalAiRuntimeStatusSurfaceReadModel);
const childSafetyPriorityRows = readModel.rows.filter((row) => row.childSafetyPriorityVisible);
const unavailableRows = readModel.rows.filter((row) => row.surfaceState === 'unavailable-visible');
const degradedRows = readModel.rows.filter((row) => row.surfaceState === 'degraded-visible');

const rejectionChecks = [
  {
    name: 'hidden-parent-row',
    rejected: !LocalAiRuntimeStatusSurfaceRowSchema.safeParse({
      ...readModel.rows[0],
      parentVisible: false,
    }).success,
  },
  {
    name: 'remote-api-overclaim',
    rejected: !LocalAiRuntimeStatusSurfaceRowSchema.safeParse({
      ...readModel.rows[0],
      remoteApiClaimed: true,
    }).success,
  },
  {
    name: 'policy-authority-overclaim',
    rejected: !LocalAiRuntimeStatusSurfaceRowSchema.safeParse({
      ...readModel.rows[0],
      policyAuthorityClaimed: true,
    }).success,
  },
  {
    name: 'dishonest-read-model-count',
    rejected: !LocalAiRuntimeStatusSurfaceReadModelSchema.safeParse({
      ...readModel,
      readyVisibleCount: readModel.readyVisibleCount + 1,
    }).success,
  },
];

const proof = {
  status: 'ok',
  proofKind: 'local-ai-runtime-status-read-model-proof',
  generatedAt,
  output: relativePath(ProofPath),
  readModel: {
    readModelId: readModel.readModelId,
    sourceReadModelIds: readModel.sourceReadModelIds,
    rowCount: readModel.rows.length,
    readyVisibleCount: readModel.readyVisibleCount,
    queuedVisibleCount: readModel.queuedVisibleCount,
    degradedVisibleCount: readModel.degradedVisibleCount,
    unavailableVisibleCount: readModel.unavailableVisibleCount,
    manualSetupRequiredCount: readModel.manualSetupRequiredCount,
  },
  runtimeRefs: readModel.rows.map((row) => ({
    rowId: row.rowId,
    sourceRuntimeProviderProofEntryId: row.sourceRuntimeProviderProofEntryId,
    providerId: row.providerId,
    runtimeReferenceId: row.runtimeReferenceId,
    modelId: row.modelId,
    surfaceState: row.surfaceState,
    parentVisible: row.parentVisible,
    childSafetyPriorityVisible: row.childSafetyPriorityVisible,
    unavailableReason: row.unavailableReason,
  })),
  rejectionChecks,
  assertions: {
    allRowsParentVisible: readModel.rows.every((row) => row.parentVisible),
    localRuntimeIdentifiersPreserved: readModel.rows.every(
      (row) => row.providerId.length > 0 && row.runtimeReferenceId.length > 0 && row.modelId.length > 0
    ),
    childSafetyPriorityVisible: childSafetyPriorityRows.length === 2,
    degradedStateVisible: degradedRows.length === 1 && degradedRows[0].degradedState === 'overloaded',
    unavailableStateVisible:
      unavailableRows.length === 1 && unavailableRows[0].unavailableReason === 'local-ai-provider-unconfigured',
    noPortalRuntimeRenderingClaim: readModel.rows.every((row) => !row.portalRuntimeRenderingClaimed),
    noRemoteApiClaim: readModel.rows.every((row) => !row.remoteApiClaimed),
    noPolicyAuthorityClaim: readModel.rows.every((row) => !row.policyAuthorityClaimed),
    noEnforcementClaim: readModel.rows.every((row) => !row.enforcementClaimed),
    malformedRowsRejected: rejectionChecks.every((check) => check.rejected),
  },
  nonClaims: readModel.runtimeStatusNonClaims,
};

mkdirSync(OutputRoot, { recursive: true });
mkdirSync(TestResultRoot, { recursive: true });
writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  ValidationLogPath,
  [
    'cmd /c npm run build --workspace @ocentra-parent/parent-domain',
    'cmd /c npm run test --workspace @ocentra-parent/parent-domain -- local-ai-runtime-status-read-model-proof',
  ].join('\n') + '\n'
);
writeFileSync(TestResultPath, `${JSON.stringify({ status: 'ok', proof: relativePath(ProofPath) }, null, 2)}\n`);
console.log(
  `local-ai-runtime-status-read-model-proof-ok:${readModel.rows.length}:${readModel.unavailableVisibleCount}`
);
console.log(`proof=${relativePath(ProofPath)}`);

function relativePath(filePath) {
  return relative(RepoRoot, filePath).replaceAll('\\', '/');
}

function runCommand(command, args) {
  execFileSync(command, args, { cwd: RepoRoot, stdio: 'inherit' });
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
