import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const gateRoot = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const namedProofRoot = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  'tracking-child-runtime-execution-readiness-proof'
);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-child-runtime-execution-readiness-proof');
const sourceBoundaryProofPath = join(proofRoot, '26-child-runtime-delivery-boundary-proof.json');
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
    'tests/tracking-child-runtime-execution-readiness-proof.test.ts',
    'tests/tracking-child-runtime-delivery-boundary-proof.test.ts',
  ]);

  const readinessProof = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-child-runtime-execution-readiness-proof.js')
    )
  );
  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const sourceBoundaryProof = JSON.parse(await readFile(sourceBoundaryProofPath, 'utf8'));
  const readModel = readinessProof.buildTrackingChildRuntimeExecutionReadinessReadModel(
    {
      generatedAt: '2026-06-07T14:45:00.000Z',
      readinessId: 'tracking-child-runtime-execution-readiness-proof',
      sourceContractRefs: [
        'packages/parent-domain/src/tracking-child-runtime-delivery-boundary-proof.ts',
        'packages/parent-domain/src/tracking-child-runtime-execution-readiness-proof.ts',
        'docs/plans/tracking-plan/workpacks/30-parent-and-child-ui-ux-surfaces.md',
      ],
    },
    sourceBoundaryProof.readModel
  );

  assert.equal(readModel.rows.length, sourceBoundaryProof.readModel.rows.length, 'boundary row count preserved');
  assert.equal(readModel.deliveryEnvelopeReadyCount, readModel.rows.length, 'delivery envelope rows');
  assert.equal(readModel.safeResponseExecutionReadyCount, 1, 'safe response execution-ready rows');
  assert.equal(readModel.escalationExecutionReadyCount, 3, 'escalation execution-ready rows');
  assert.equal(readModel.executionRequirementRefCount, readModel.rows.length * 4, 'execution requirement refs');
  assert.equal(
    readModel.runtimeObservationRequirementRefCount,
    readModel.rows.length * 2,
    'runtime observation requirement refs'
  );
  assert.equal(readModel.childDeviceDeliveryRuntimeClaimed, false, 'no child delivery runtime');
  assert.equal(readModel.childDeviceExecutionRuntimeClaimed, false, 'no child execution runtime');
  assert.equal(readModel.renderedChildDeviceUiRuntimeClaimed, false, 'no rendered child runtime UI');
  assert.equal(readModel.physicalDeviceProofClaimed, false, 'no physical-device proof');
  assert.equal(readModel.authorityProofClaimed, false, 'no authority proof');
  assert.equal(readModel.productReadyClaimed, false, 'not product ready');

  await writeProofArtifacts({ checkedAt, commit, readModel, sourceBoundaryProof });

  console.log('tracking-child-runtime-execution-readiness-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

async function writeProofArtifacts({ checkedAt, commit, readModel, sourceBoundaryProof }) {
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
    artifactPath: relative(repoRoot, join(proofRoot, '27-child-runtime-execution-readiness-proof.json')),
    rolloutGateArtifactPath: relative(repoRoot, join(gateRoot, '39-child-runtime-execution-readiness-proof.json')),
    sourceBoundaryProofPath: relative(repoRoot, sourceBoundaryProofPath),
    sourceSnapshotPath: relative(repoRoot, join(namedProofRoot, '00-source-snapshot.md')),
    validationLogPath: relative(repoRoot, join(namedProofRoot, '16-validation-commands.log')),
    sourceBoundaryStatus: sourceBoundaryProof.status,
    sourceBoundaryArtifactPath: sourceBoundaryProof.artifactPath,
    readModel,
    proofLabels: [
      'tracking-child-runtime.execution-readiness',
      'tracking-child-runtime.delivery-envelope-refs',
      'tracking-child-runtime.execution-requirement-refs',
      'tracking-child-runtime.runtime-observation-requirement-refs',
      'tracking-child-runtime.no-child-device-delivery-runtime',
      'tracking-child-runtime.no-child-device-execution-runtime',
      'tracking-child-runtime.no-physical-device-or-authority-proof',
    ],
    productClaims: {
      childRuntimeExecutionReadinessRowsClaimed: true,
      deliveryEnvelopeRefsClaimed: true,
      executionRequirementRefsClaimed: true,
      runtimeObservationRequirementRefsClaimed: true,
      childRuntimeBoundaryProofConsumed: true,
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
      'P2 hosted execution-readiness proof consumes the child runtime delivery boundary proof and emits deterministic delivery-envelope, execution-result, visible-snapshot, parent-receipt, and runtime-observation requirement refs for each child check-in state. It proves readiness accounting only. Actual child-device delivery, child-device execution, rendered child-device runtime UI, provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior remain unclaimed.',
    commands,
  };

  await writeFile(join(namedProofRoot, '00-source-snapshot.md'), sourceSnapshot({ checkedAt, commit }));
  await writeFile(
    join(proofRoot, '27-child-runtime-execution-readiness-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(
    join(gateRoot, '39-child-runtime-execution-readiness-proof.json'),
    `${JSON.stringify(proof, null, 2)}\n`
  );
  await writeFile(join(namedProofRoot, '13-security-negative-proof.log'), securityNegativeProof());
  await writeFile(
    join(namedProofRoot, '16-validation-commands.log'),
    commands.map((entry) => entry.command).join('\n') + '\n'
  );
  await writeFile(join(namedProofRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    join(testResultRoot, 'tracking-child-runtime-execution-readiness-read-model.json'),
    `${JSON.stringify(readModel, null, 2)}\n`
  );
}

function sourceSnapshot({ checkedAt, commit }) {
  return [
    '# Tracking Child Runtime Execution Readiness Source Snapshot',
    '',
    `- checkedAt: ${checkedAt}`,
    `- commit: ${commit}`,
    '- requiredProofTier: P2_HOSTED_CI',
    '- currentProofTier: P2_HOSTED_CI',
    '- status: proved',
    '- consumes: output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/26-child-runtime-delivery-boundary-proof.json',
    '- proves delivery-envelope, execution-result, visible-snapshot, parent-receipt, and runtime-observation requirement refs',
    '- proof module: packages/parent-domain/src/tracking-child-runtime-execution-readiness-proof.ts',
    '- proof tests: packages/parent-domain/tests/tracking-child-runtime-execution-readiness-proof.test.ts',
    '- proof harness: scripts/test/tracking-child-runtime-execution-readiness-proof.mjs',
    '',
  ].join('\n');
}

function securityNegativeProof() {
  return [
    'workpack=30-parent-and-child-ui-ux-surfaces',
    'Child runtime execution readiness rows consume the delivery-boundary proof and emit deterministic runtime requirement refs.',
    'Rows are readiness accounting only and do not claim actual child-device delivery, execution, or rendered child UI.',
    'Provider delivery, notification receipt ingestion, live location runtime, physical-device proof, authority proof, production workers, and product-ready behavior are explicit non-claims.',
    '',
  ].join('\n');
}

async function runNpm(args) {
  const command = `npm ${args.join(' ')}`;
  commands.push({ command });
  await run('cmd', ['/c', 'npm', ...args]);
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
