import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofDir = join(repoRoot, 'output', 'tracking-plan-proof', 'authority-hard-control-proof');
const resultDir = join(repoRoot, 'test-results', 'tracking-authority-hard-control-proof');
const timestamp = '2026-06-05T19:30:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(resultDir, { recursive: true, force: true });
await mkdir(resultDir, { recursive: true });
await mkdir(proofDir, { recursive: true });

run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
run('cmd', [
  '/c',
  'npm',
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/parent-domain',
  '--',
  'tracking-authority-hard-control-proof',
]);

const authority = await importDist('tracking-authority-hard-control-proof.js');
const readModel = authority.buildTrackingAuthorityHardControlReadModel(timestamp);

const proof = {
  proofMode: 'tracking-authority-hard-control-proof',
  generatedAt: timestamp,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: readModel.summary,
  productClaims: productClaims(),
  proofPaths: {
    source: 'packages/parent-domain/src/tracking-authority-hard-control-proof.ts',
    test: 'packages/parent-domain/tests/tracking-authority-hard-control-proof.test.ts',
    harness: 'scripts/test/tracking-authority-hard-control-proof.mjs',
    evidence: 'test-results/tracking-authority-hard-control-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/authority-hard-control-proof',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(resultDir, 'authority-hard-control-read-model.json'), readModel);
await writeJson(join(resultDir, 'proof.json'), proof);
await writeProofPack(proofDir, proof);

console.log('tracking-authority-hard-control-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-authority-hard-control-proof', 'proof.json')}`);

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
}

function assertProof(proof) {
  if (
    proof.summary.surfaceCount !== 5 ||
    proof.summary.authorityRequiredRows !== 5 ||
    proof.summary.authorityEnrolledRows !== 0 ||
    proof.summary.hardControlClaimedRows !== 0 ||
    proof.summary.childDeviceRuntimeClaimedRows !== 0 ||
    proof.summary.physicalDeviceClaimedRows !== 0 ||
    proof.summary.productClaimReadyRows !== 0
  ) {
    throw new Error(`Unexpected tracking authority proof summary: ${JSON.stringify(proof.summary)}`);
  }

  if (Object.values(proof.productClaims).some((value) => value !== false)) {
    throw new Error(`Tracking authority proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

async function writeProofPack(path, proof) {
  await writeFile(
    join(path, '00-source-snapshot.md'),
    [
      '# Tracking Authority Hard-Control Proof Source Snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Base commit at generation: ${proof.baseCommitAtGeneration}`,
      '- Git status at proof generation:',
      '',
      '```text',
      proof.gitStatusShort.length === 0 ? 'clean' : proof.gitStatusShort,
      '```',
      '',
      '- Scope: negative proof gate for authority-enrolled tracking hard-control behavior.',
      '- Source inspected: location/geofence feature doc, tracking plan checklist, platform expectations, and parent-domain README.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(path, '01-authority-gate-proof.log'),
    [
      'Authority gate proof:',
      '',
      '- Android Device Owner location control: authority_required.',
      '- Android managed profile location control: authority_required.',
      '- iOS supervised/MDM location control: authority_required.',
      '- macOS MDM location control: authority_required.',
      '- Windows AppLocker/App Control location control: authority_required.',
      '',
      'No row claims enrollment, hard-control runtime behavior, child-device runtime execution, physical-device proof, or product readiness.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(path, '09-authority-surface-summary.json'), proof.summary);
  await writeFile(
    join(path, '13-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Device Owner, managed profile, supervised/MDM, and AppLocker/App Control remain authority_required until real artifacts are attached.',
      '- CI rejects hard-control and product-ready overclaims through Effect Schema literal and read-model honesty filters.',
      '- Physical-device, child-device runtime, platform adapter execution, live/background location, and product UI claims remain false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(join(path, '16-validation-commands.log'), `${proof.commands.join('\n')}\n`, 'utf8');
  await writeFile(
    join(path, 'README.md'),
    '# Tracking Authority Hard-Control Proof\n\nThis proof pack records the CI-checkable negative proof gate for tracking hard-control authority. It keeps Android Device Owner, Android managed profile, iOS supervised/MDM, macOS MDM, and Windows AppLocker/App Control rows `authority_required` until real enrolled-device or managed-policy artifacts are attached.\n',
    'utf8'
  );
  await writeJson(join(path, 'proof.json'), proof);
}

function productClaims() {
  return {
    authorityEnrolledClaimed: false,
    hardControlClaimed: false,
    childDeviceRuntimeClaimed: false,
    platformAdapterClaimed: false,
    physicalDeviceClaimed: false,
    liveLocationRuntimeClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    productUiCompleteClaimed: false,
    productClaimReadyClaimed: false,
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
