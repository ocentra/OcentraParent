import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { runNpmCommand } from './run-npm-command.mjs';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const gateRoot = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  'tracking-child-runtime-snapshot-requirements-proof'
);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-child-runtime-snapshot-requirements-proof');
const sourceReadinessProofPath = join(proofRoot, '27-child-runtime-execution-readiness-proof.json');
const commands = [];

await main();

async function main() {
  await runNpm(['--workspace', '@ocentra-parent/parent-domain', 'run', 'build']);
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'vitest',
    'run',
    'tests/tracking-child-runtime-snapshot-requirements-proof.test.ts',
    'tests/tracking-child-runtime-execution-readiness-proof.test.ts',
  ]);

  const snapshotProof = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-child-runtime-snapshot-requirements-proof.js')
    )
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const sourceReadinessProof = JSON.parse(await readFile(sourceReadinessProofPath, 'utf8'));
  const readModel = snapshotProof.buildTrackingChildRuntimeSnapshotRequirementsReadModel(
    {
      generatedAt: '2026-06-07T15:05:00.000Z',
      snapshotRequirementsId: 'tracking-child-runtime-snapshot-requirements-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-child-runtime-execution-readiness-proof.ts',
        'packages/parent-domain/src/tracking-child-runtime-snapshot-requirements-proof.ts',
        'docs/plans/tracking-plan/workpacks/30-parent-and-child-ui-ux-surfaces.md',
      ],
    },
    sourceReadinessProof.readModel
  );

  assert.equal(readModel.rows.length, sourceReadinessProof.readModel.rows.length, 'readiness row count preserved');
  assert.equal(readModel.requiredSnapshotKindCount, readModel.rows.length * 5, 'snapshot kind coverage');
  assert.equal(readModel.deliveryEnvelopeRequirementCount, readModel.rows.length, 'delivery envelopes');
  assert.equal(readModel.executionResultRequirementCount, readModel.rows.length, 'execution result refs');
  assert.equal(readModel.visibleSnapshotRequirementCount, readModel.rows.length, 'visible snapshot refs');
  assert.equal(readModel.parentReceiptRequirementCount, readModel.rows.length, 'parent receipt refs');
  assert.equal(readModel.runtimeObservationRequirementCount, readModel.rows.length * 2, 'runtime observation refs');
  assert.equal(readModel.childDeviceDeliveryRuntimeClaimed, false, 'no child delivery runtime');
  assert.equal(readModel.childDeviceExecutionRuntimeClaimed, false, 'no child execution runtime');
  assert.equal(readModel.renderedChildDeviceUiRuntimeClaimed, false, 'no rendered child runtime UI');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical-device proof');
  assert.equal(readModel.authorityProofClaimed, false, 'no authority proof');
  assert.equal(readModel.productReadyClaimed, false, 'not product ready');

  await writeProofArtifacts({ checkedAt, commit, readModel, sourceReadinessProof });

  console.log('tracking-child-runtime-snapshot-requirements-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

async function writeProofArtifacts({ checkedAt, commit, readModel, sourceReadinessProof }) {
  await mkdir(proofRoot, { recursive: true });
  await mkdir(gateRoot, { recursive: true });
  await mkdir(namedProofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackId: '30-parent-and-child-ui-ux-surfaces',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    status: 'proved',
    artifactPath: relative(repoRoot, join(proofRoot, '28-child-runtime-snapshot-requirements-proof.json')),
    rolloutGateArtifactPath: relative(repoRoot, join(gateRoot, '40-child-runtime-snapshot-requirements-proof.json')),
    sourceReadinessProofPath: relative(repoRoot, sourceReadinessProofPath),
    sourceSnapshotPath: relative(repoRoot, join(namedProofRoot, '00-source-snapshot.md')),
    validationLogPath: relative(repoRoot, join(namedProofRoot, '16-validation-commands.log')),
    sourceReadinessStatus: sourceReadinessProof.status,
    sourceReadinessArtifactPath: sourceReadinessProof.artifactPath,
    readModel,
    proofLabels: [
      'tracking-child-runtime.snapshot-requirements',
      'tracking-child-runtime.delivery-envelope-ref',
      'tracking-child-runtime.execution-result-ref',
      'tracking-child-runtime.visible-snapshot-ref',
      'tracking-child-runtime.parent-receipt-ref',
      'tracking-child-runtime.runtime-observation-ref',
      'tracking-child-runtime.no-child-device-runtime-execution',
    ],
    productClaims: {
      childRuntimeSnapshotRequirementsRowsClaimed: true,
      deliveryEnvelopeRequirementRefsClaimed: true,
      executionResultRequirementRefsClaimed: true,
      visibleSnapshotRequirementRefsClaimed: true,
      parentReceiptRequirementRefsClaimed: true,
      runtimeObservationRequirementRefsClaimed: true,
      executionReadinessProofConsumed: true,
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptIngestionClaimed: false,
      liveLocationRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productReadyClaimed: false,
    },
    missingProofReason:
      'P2 hosted snapshot-requirements proof consumes the child runtime execution readiness proof and requires delivery-envelope, execution-result, visible-snapshot, parent-receipt, and runtime-observation refs for each child check-in state. It proves requirement coverage only. Actual child-device delivery, child-device execution, rendered child-device runtime UI, provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior remain unclaimed.',
    commands,
  };

  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeJson(join(proofRoot, '28-child-runtime-snapshot-requirements-proof.json'), proof);
  await writeJson(join(gateRoot, '40-child-runtime-snapshot-requirements-proof.json'), proof);
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(
    join(namedProofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeJson(join(namedProofRoot, 'proof.json'), proof);
  await writeJson(join(testResultRoot, 'proof.json'), proof);
  await writeJson(join(testResultRoot, 'tracking-child-runtime-snapshot-requirements-read-model.json'), readModel);
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# Tracking Child Runtime Snapshot Requirements Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P2_HOSTED_CI',
    '- currentProofTier: P2_HOSTED_CI',
    '- status: proved',
    '- consumes: output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/27-child-runtime-execution-readiness-proof.json',
    '- proves requirement refs for delivery-envelope, execution-result, visible-snapshot, parent-receipt, and runtime-observation rows',
    '- proof module: packages/parent-domain/src/tracking-child-runtime-snapshot-requirements-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-child-runtime-snapshot-requirements-proof.test.ts',
    '- proof harness: scripts/test/tracking-child-runtime-snapshot-requirements-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=30-parent-and-child-ui-ux-surfaces',
    'Child runtime snapshot requirement rows consume the execution-readiness proof and emit deterministic requirement refs.',
    'Rows are requirement coverage only and do not claim actual child-device delivery, execution, or rendered child UI.',
    'Provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior are explicit non-claims.',
    '',
  ].join('\n');
}

async function runNpm(args) {
  const command = `npm ${args.join(' ')}`;
  commands.push({ command });
  await runNpmCommand(run, args);
}

async function run(command, args) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit' });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited ${code}`));
    });
  });
}

async function gitHead() {
  let stdout = '';
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.pipe(process.stderr);
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`git rev-parse HEAD exited ${code}`));
    });
  });
  return stdout.trim();
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
