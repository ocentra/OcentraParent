import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof', 'unsupported-platform-manual-proof');
const resultRoot = join(repoRoot, 'test-results', 'tracking-unsupported-platform-manual-proof');
const generatedAt = '2026-06-05T19:55:00.000Z';
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(proofRoot, { recursive: true, force: true });
await rm(resultRoot, { recursive: true, force: true });
await mkdir(proofRoot, { recursive: true });
await mkdir(resultRoot, { recursive: true });

runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
runNpm([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/tracking-domain',
  '--',
  'tracking-unsupported-platform-manual-proof',
]);

const proofModule = await import(
  pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'tracking-unsupported-platform-manual-proof.js'))
    .href
);
const readModel = proofModule.buildTrackingUnsupportedPlatformManualProof(generatedAt);
const proof = {
  proofMode: 'tracking-unsupported-platform-manual-proof',
  generatedAt,
  branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
  baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
  gitStatusShort: initialGitStatusShort,
  commands,
  summary: summarize(readModel.rows),
  productClaims: {
    portalScreenshotClaimed: readModel.portalScreenshotClaimed,
    physicalDeviceProofClaimed: readModel.physicalDeviceProofClaimed,
    authorityProofClaimed: readModel.authorityProofClaimed,
    productionClaimReady: false,
  },
  proofPaths: {
    source: 'packages/schema-domain/src/tracking-unsupported-platform-manual-proof.ts',
    test: 'packages/tracking-domain/tests/contract/tracking-unsupported-platform-manual-proof.test.ts',
    harness: 'scripts/test/tracking-unsupported-platform-manual-proof.mjs',
    evidence: 'test-results/tracking-unsupported-platform-manual-proof/proof.json',
    trackingProofPack: 'output/tracking-plan-proof/unsupported-platform-manual-proof',
  },
  readModel,
};

assertProof(proof);
await writeJson(join(resultRoot, 'unsupported-platform-manual-read-model.json'), readModel);
await writeJson(join(resultRoot, 'proof.json'), proof);
await writeJson(join(proofRoot, 'proof.json'), proof);
await writeFile(
  join(proofRoot, 'manual-proof-plan.md'),
  [
    '# Unsupported Platform Manual Proof Plan',
    '',
    'This proof records UI-ready manual-required, authority-required, and unavailable tracking rows.',
    'It does not claim portal screenshots, physical-device behavior, authority enrollment, or production readiness.',
    '',
    ...readModel.rows.map(
      (row) =>
        `- ${row.rowId}: ${row.platform}/${row.surface} renders ${row.renderedState}; command: ${row.manualProofCommand}.`
    ),
    '',
  ].join('\n')
);

console.log('tracking-unsupported-platform-manual-proof-ok');
console.log('evidence=test-results/tracking-unsupported-platform-manual-proof/proof.json');

function summarize(rows) {
  return {
    rowCount: rows.length,
    renderedStates: countBy(rows.map((row) => row.renderedState)),
    supportStates: countBy(rows.map((row) => row.supportState)),
    fakeCapabilityRows: rows.filter((row) => row.fakeCapabilityRendered).length,
    productClaimReadyRows: rows.filter((row) => row.productClaimReady).length,
    physicalDeviceClaimedRows: rows.filter((row) => row.physicalDeviceClaimed).length,
    authorityClaimedRows: rows.filter((row) => row.authorityClaimed).length,
  };
}

function assertProof(proof) {
  if (
    proof.summary.rowCount !== 7 ||
    proof.summary.renderedStates['manual-required'] !== 5 ||
    proof.summary.renderedStates.unavailable !== 1 ||
    proof.summary.renderedStates['authority-required'] !== 1 ||
    proof.summary.fakeCapabilityRows !== 0 ||
    proof.summary.productClaimReadyRows !== 0 ||
    proof.summary.physicalDeviceClaimedRows !== 0 ||
    proof.summary.authorityClaimedRows !== 0
  ) {
    throw new Error(`Unexpected unsupported-platform proof summary: ${JSON.stringify(proof.summary)}`);
  }

  if (Object.values(proof.productClaims).some((claim) => claim !== false)) {
    throw new Error(`Unsupported-platform proof overclaimed product behavior: ${JSON.stringify(proof.productClaims)}`);
  }
}

function run(command, args) {
  const printable = [command, ...args].join(' ');
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: printable,
    status: result.status,
    stdout: result.stdout.trim(),
    stderr: result.stderr.trim(),
  });
  if (result.status !== 0) {
    throw new Error(`${printable} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) return '';
  return result.stdout.trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
