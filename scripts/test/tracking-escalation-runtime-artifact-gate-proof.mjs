import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { tsImport } from 'tsx/esm/api';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofMode = 'tracking-escalation-runtime-artifact-gate-proof';
const generatedAt = '2026-06-08T00:30:00.000Z';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const focusedProofDir = path.join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const output27 = path.join(repoRoot, 'output', 'tracking-plan-proof', '27-escalation-engine');
const output33 = path.join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(output27, { recursive: true });
  await mkdir(output33, { recursive: true });

  run('node', ['scripts/test/tracking-escalation-runtime-readiness-blocker-proof.mjs']);
  runNpmCommand(run, [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/tracking-domain',
    '--',
    'tests/contract/tracking-escalation-runtime-artifact-gate-proof.test.ts',
  ]);

  const proofModule = await tsImport(
    pathToFileURL(
      path.join(repoRoot, 'packages', 'tracking-domain', 'src', 'tracking-escalation-runtime-artifact-gate-proof.ts')
    ).href,
    import.meta.url
  );
  const sourceProof = await assertSourceRuntimeReadinessProofExists(
    proofModule.RequiredTrackingEscalationRuntimeArtifactPlan.sourceRuntimeReadinessProofRef
  );
  const requiredArtifacts = [...new Set(sourceProof.readModel.blockers.flatMap((row) => row.blockingArtifactRefs))];
  const inventory = {
    presentArtifacts: await presentArtifactsForRoot(
      path.join(repoRoot, proofModule.RequiredTrackingEscalationRuntimeArtifactPlan.proofRoot),
      requiredArtifacts
    ),
  };
  const readModel = proofModule.buildTrackingEscalationRuntimeArtifactGateProof(
    generatedAt,
    sourceProof.readModel,
    inventory
  );
  const proof = buildProof({ readModel });

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-escalation-runtime-artifact-gate-proof-ok');
  console.log(`evidence=${relativePath(path.join(resultDir, 'proof.json'))}`);
}

async function assertSourceRuntimeReadinessProofExists(proofRef) {
  const sourceProof = JSON.parse(await readFile(path.join(repoRoot, proofRef), 'utf8'));
  assert.equal(sourceProof.proofMode, 'tracking-escalation-runtime-readiness-blocker-proof');
  assert.equal(sourceProof.productClaims.productClaimReady, false);
  return sourceProof;
}

async function presentArtifactsForRoot(rootPath, requiredArtifacts) {
  const present = [];
  for (const artifact of requiredArtifacts) {
    const artifactPath = path.join(rootPath, artifact);
    if (await pathExists(artifactPath)) present.push(artifact);
  }
  return present;
}

async function pathExists(filePath) {
  try {
    await stat(filePath);
    return true;
  } catch (error) {
    if (error?.code === 'ENOENT') return false;
    throw error;
  }
}

function buildProof({ readModel }) {
  return {
    schemaVersion: 1,
    proofMode,
    generatedAt,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    workpackIds: ['27-escalation-engine', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_MANUAL_ESCALATION_RUNTIME',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: readModel.rows.every((row) => row.escalationRuntimeArtifactSetComplete)
      ? 'artifact_set_present'
      : 'manual_required',
    readModel,
    summary: {
      rowCount: readModel.rows.length,
      completeRows: readModel.rows.filter((row) => row.escalationRuntimeArtifactSetComplete).length,
      manualRequiredRows: readModel.rows.filter((row) => !row.escalationRuntimeArtifactSetComplete).length,
      missingArtifactCount: readModel.rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      requiredArtifactCount: readModel.rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      productionEscalationWorkerClaimedRows: readModel.rows.filter(
        (row) => row.productionEscalationWorkerRuntimeClaimed
      ).length,
      productionQuietHoursTimerClaimedRows: readModel.rows.filter((row) => row.productionQuietHoursTimerRuntimeClaimed)
        .length,
      productReadyRows: readModel.rows.filter((row) => row.productClaimReady).length,
    },
    proofLabels: [
      'tracking-escalation-runtime.artifact-gate',
      'tracking-escalation-runtime.required-artifacts-from-runtime-blockers',
      'tracking-escalation-runtime.no-runtime-claim-from-file-presence',
      'tracking-escalation-runtime.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    missingProofReason:
      'Actual escalation runtime still requires production escalation workers, quiet-hours timers, provider delivery/receipt runtime, parent notification history runtime, child-device delivery runtime, durable escalation storage, emergency auto-contact policy, physical-device proof, and authority proof artifacts. This gate validates artifact presence only and keeps runtime and product-ready claims false.',
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.rowCount, 1, 'expected one escalation runtime artifact gate row');
  assert.equal(proof.summary.requiredArtifactCount > 0, true, 'expected escalation runtime artifact refs');
  assert.equal(proof.summary.productionEscalationWorkerClaimedRows, 0, 'no escalation worker claims');
  assert.equal(proof.summary.productionQuietHoursTimerClaimedRows, 0, 'no quiet-hours timer claims');
  assert.equal(proof.summary.productReadyRows, 0, 'no product-ready rows');
  assert.deepEqual(Object.values(proof.productClaims), [
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
    false,
  ]);
}

async function writeArtifacts(proof) {
  await writeJson(path.join(resultDir, 'proof.json'), proof);
  await writeJson(path.join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(path.join(focusedProofDir, 'proof.json'), proof);
  await writeJson(path.join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    path.join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Escalation Runtime Artifact Gate Source Snapshot',
      '',
      `- generatedAt: ${generatedAt}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- requiredArtifactCount: ${proof.summary.requiredArtifactCount}`,
      `- missingArtifactCount: ${proof.summary.missingArtifactCount}`,
      '- required artifact refs are derived from the escalation runtime-readiness blocker proof',
      '- artifact presence alone does not claim escalation workers, quiet-hours timers, provider receipt runtime, durable escalation storage, or product readiness',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(path.join(output27, '11-escalation-runtime-artifact-gate-proof.json'), proof);
  await writeJson(path.join(output33, '62-escalation-runtime-artifact-gate-proof.json'), proof);
  await writeFile(
    path.join(output27, '11-escalation-runtime-artifact-gate-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
  await writeFile(
    path.join(output33, '62-escalation-runtime-artifact-gate-validation.log'),
    `${commands.map((entry) => entry.command).join('\n')}\n`,
    'utf8'
  );
}

function run(command, args) {
  commands.push({ command: [command, ...args].join(' ') });
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) throw new Error(`Command failed: ${command} ${args.join(' ')}`);
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replaceAll(path.sep, '/');
}
