import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-ai-classifier-boundary-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '24-ai-classifier-digest-boundary');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '23-app-ai-classifier-digest-boundary');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'app-game-ai-classifier-boundary',
    ])
  );
  await runCommand(
    ...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'app-game-evidence-claim'])
  );

  const { AppGameAiClassifierBoundaryProofMatrix } =
    await import('../../packages/parent-domain/dist/app-game-ai-classifier-boundary-data.js');
  const { appGameAiClassifierForbiddenOutputKeyPaths, safeParseAppGameAiClassifierResult } =
    await import('../../packages/parent-domain/dist/app-game-ai-classifier-boundary.js');
  const summary = summarizeMatrix(AppGameAiClassifierBoundaryProofMatrix);
  assertProof(AppGameAiClassifierBoundaryProofMatrix, summary, {
    appGameAiClassifierForbiddenOutputKeyPaths,
    safeParseAppGameAiClassifierResult,
  });

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-ai-classifier-boundary',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    counts: summary,
    evidence: {
      tsContract: 'packages/parent-domain/src/app-game-ai-classifier-boundary.ts',
      tsContractValues: 'packages/parent-domain/src/app-game-ai-classifier-boundary-values.ts',
      tsContractData: 'packages/parent-domain/src/app-game-ai-classifier-boundary-data.ts',
      tsContractTest: 'packages/parent-domain/tests/app-game-ai-classifier-boundary.test.ts',
      activityDigestSource: 'packages/activity-domain/src/app-game.ts',
      activityDigestTest: 'packages/activity-domain/tests/app-game-evidence-claim.test.ts',
      proofHarness: 'scripts/test/app-game-ai-classifier-boundary-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/24-ai-classifier-digest-boundary',
      appProofPack: 'output/app-plan-proof/23-app-ai-classifier-digest-boundary',
    },
    claimsProved: [
      'classifier results must cite stored evidence refs',
      'confidence remains bounded from zero through one',
      'runtime, prompt template, prompt version, and fallback state are explicit',
      'AI output is evidence-only and cannot request direct action',
      'forbidden action, duration, and raw scan fields are rejected before policy handoff',
      'existing activity-domain digest proof remains the source spine and was run without editing the locked package',
    ],
    claimsNotProved: [
      'live local model quality',
      'provider lifecycle execution',
      'runtime classifier service events',
      'portal classifier result rendering',
      'policy evaluator runtime consumption',
      'platform adapter enforcement',
    ],
  };
  const policyProof = {
    schemaVersion: 1,
    policyProofMode: 'app-game-ai-classifier-policy-handoff',
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    directActionCandidates: summary.directActionCandidates,
    missingEvidenceCandidates: summary.missingEvidenceCandidates,
    forbiddenFieldNegativePathPassed: true,
    policyHandoffCounts: summary.byPolicyHandoff,
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(appGameProofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(appGameProofDir, '05-policy-action-proof.json'), policyProof);
  await writeJson(join(appProofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(appProofDir, '05-policy-action-proof.json'), policyProof);

  console.log(`app-game-ai-classifier-boundary-proof-ok:${Object.keys(summary.byProductKind).join(',')}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function summarizeMatrix(matrix) {
  return {
    resultCount: matrix.results.length,
    byProductKind: countBy(matrix.results.map((result) => result.productKind)),
    byCandidateKind: countBy(matrix.results.map((result) => result.candidateKind)),
    byClassifierState: countBy(matrix.results.map((result) => result.classifierState)),
    byFallbackState: countBy(matrix.results.map((result) => result.fallbackState)),
    byPolicyHandoff: countBy(matrix.results.map((result) => result.policyHandoff)),
    directActionCandidates: matrix.results.filter((result) => result.directActionRequested).length,
    missingEvidenceCandidates: matrix.results.filter((result) => result.sourceEvidenceRefs.length === 0).length,
  };
}

function assertProof(matrix, summary, helpers) {
  assertEqual(String(matrix.matrixId), 'app-game-ai-classifier-boundary-proof', 'matrix id');
  assertEqual(summary.resultCount, 3, 'result count');
  assertEqual(summary.byProductKind.unknownApp, 1, 'unknown app count');
  assertEqual(summary.byProductKind.unknownGame, 1, 'unknown game count');
  assertEqual(summary.directActionCandidates, 0, 'direct action candidate count');
  assertEqual(summary.missingEvidenceCandidates, 0, 'missing evidence count');

  const forbiddenOutput = {
    ...matrix.results[0],
    durationMs: 60000,
    rawOsScanResult: { processScanRows: ['private-process.exe'] },
    modelDecision: { terminate: true },
  };
  assertEqual(helpers.safeParseAppGameAiClassifierResult(forbiddenOutput).success, false, 'forbidden parse');
  assertEqual(
    helpers.appGameAiClassifierForbiddenOutputKeyPaths(forbiddenOutput).includes('modelDecision.terminate'),
    true,
    'forbidden nested key'
  );
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit' });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`git rev-parse HEAD exited with ${code}`));
    });
  });
  return Buffer.concat(chunks).toString('utf8').trim();
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, got ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
