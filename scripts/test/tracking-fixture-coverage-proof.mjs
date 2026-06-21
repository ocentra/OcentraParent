import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'fixture-coverage-proof');
const resultDir = join(repoRoot, 'test-results', 'tracking-fixture-coverage-proof');
const timestamp = '2026-06-05T19:45:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/tracking-domain']);
runNpm(['run', 'test', '--workspace', '@ocentra-parent/tracking-domain', '--', 'tracking-fixture-coverage-proof']);

const fixture = await importDist('tracking-fixture-coverage-proof.js');
const readModel = fixture.buildTrackingFixtureCoverageReadModel(timestamp);

const proof = {
  proofMode: 'tracking-fixture-coverage-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: readModel.summary,
  productClaims: productClaims(),
  proofPaths: {
    source: 'packages/tracking-domain/src/tracking-fixture-coverage-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-fixture-coverage-proof.test.ts',
    harness: 'scripts/test/tracking-fixture-coverage-proof.mjs',
    evidence: 'test-results/tracking-fixture-coverage-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/fixture-coverage-proof',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(resultDir, 'fixture-coverage-read-model.json'), readModel);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-fixture-coverage-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-fixture-coverage-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'tracking-domain', 'dist', name)).href);
}

function assertProof(proof) {
  if (
    proof.summary.requiredStateCount !== 14 ||
    proof.summary.coveredStateCount !== 14 ||
    proof.summary.manualRequiredStateCount !== 0 ||
    proof.summary.productClaimReadyRows !== 0 ||
    proof.summary.liveDeviceClaimedRows !== 0 ||
    proof.summary.providerDeliveryClaimedRows !== 0 ||
    proof.summary.childDeviceRuntimeClaimedRows !== 0 ||
    proof.summary.physicalDeviceClaimedRows !== 0 ||
    proof.summary.productionWorkerClaimedRows !== 0
  ) {
    throw new Error(`Unexpected tracking fixture coverage summary: ${JSON.stringify(proof.summary)}`);
  }

  if (Object.values(proof.productClaims).some((value) => value !== false)) {
    throw new Error(`Tracking fixture coverage proof overclaimed behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Fixture Coverage Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: CI-checkable matrix for required tracking fixture states.',
      '- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, tracking checklist, and tracking-domain README.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-contract-proof.log'),
    [
      'Fixture coverage contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/tracking-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/tracking-domain -- tracking-fixture-coverage-proof: PASS',
      '- Required states are fresh, stale, offline, permission-denied, low-accuracy, ambiguous-nearby-place, exception-active, parent-acknowledged, child-check-in-requested, temporary-live-expired, missing-device, retention-deleted, remote-sync-disabled, and remote-ai-disabled.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '09-fixture-coverage-summary.json'), proof.summary);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Fixture coverage does not claim live device behavior.',
      '- Fixture coverage does not claim provider delivery, child-device runtime execution, physical-device proof, production workers, or product readiness.',
      '- Remote sync and remote AI remain disabled/default contract rows, not runtime service execution claims.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# Tracking Fixture Coverage Proof\n\nThis proof pack records the required tracking fixture-state coverage matrix. It proves that current tracked artifacts cover the required state set while keeping live-device, provider-delivery, child-runtime, physical-device, production-worker, and product-ready claims false.\n',
    'utf8'
  );
  await writeJson(join(path, 'proof.json'), proof);
}

function productClaims() {
  return {
    productClaimReady: false,
    liveDeviceClaimed: false,
    providerDeliveryClaimed: false,
    childDeviceRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    productionWorkerClaimed: false,
    fullParentChildUiClaimed: false,
  };
}

function run(command, args) {
  commands.push([command, ...args].join(' '));
  const result = spawnSync(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
  if (result.status !== 0) {
    throw new Error(`Command failed: ${command} ${args.join(' ')}`);
  }
}

function gitOutput(args) {
  return spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8' }).stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
