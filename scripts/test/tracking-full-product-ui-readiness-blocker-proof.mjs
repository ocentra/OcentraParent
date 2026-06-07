import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-full-product-ui-readiness-blocker-proof';
const timestamp = '2026-06-07T22:05:00.000Z';
const resultDir = join(repoRoot, 'test-results', proofMode);
const focusedProofDir = join(repoRoot, 'output', 'tracking-plan-proof', proofMode);
const wp30ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '30-parent-and-child-ui-ux-surfaces');
const wp33ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '33-proof-gates-fixtures-rollout-and-pr-gate');
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

const sourceProofRefs = [
  'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json',
  'test-results/tracking-child-runtime-artifact-gate-proof/proof.json',
];

const requiredFullProductUiArtifactRefs = [
  'output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/02-parent-device-detail-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/03-parent-notification-history-preferences-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/04-retention-settings-production-write-result.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/05-child-device-rendered-check-in-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/06-child-device-rendered-location-consent-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/07-child-device-safe-help-response-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/08-cross-surface-accessibility-report.json',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/09-product-ui-end-to-end-trace.json',
];

await main();

async function main() {
  await rm(resultDir, { recursive: true, force: true });
  await rm(focusedProofDir, { recursive: true, force: true });
  await mkdir(resultDir, { recursive: true });
  await mkdir(focusedProofDir, { recursive: true });
  await mkdir(wp30ProofDir, { recursive: true });
  await mkdir(wp33ProofDir, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  run('cmd', ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/parent-domain', '--', proofMode]);

  const uiBlockerModule = await importDist('tracking-full-product-ui-readiness-blocker-proof.js');
  const hostedUiProof = await readProofJson(sourceProofRefs[0]);
  const childRuntimeArtifactGateProof = await readProofJson(sourceProofRefs[1]);
  const readModel = uiBlockerModule.buildTrackingFullProductUiReadinessBlockerProof(
    {
      generatedAt: timestamp,
      proofId: proofMode,
      sourceProofRefs,
      hostedScreenshotRefs: hostedUiProof.screenshots.map((screenshot) => screenshot.path),
      hostedAssertionRefs: hostedUiProof.requiredAssertions,
      fullProductUiArtifactRefs: requiredFullProductUiArtifactRefs,
    },
    childRuntimeArtifactGateProof.readModel
  );
  const proof = buildProof(readModel);

  assertProof(proof);
  await writeArtifacts(proof);

  console.log('tracking-full-product-ui-readiness-blocker-proof-ok');
  console.log(`evidence=${relativePath(join(resultDir, 'proof.json'))}`);
}

function buildProof(readModel) {
  return {
    proofMode,
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '33-proof-gates-fixtures-rollout-and-pr-gate'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P2_HOSTED_CI',
    status: 'manual_required',
    sourceProofRefs,
    summary: {
      hostedScreenshotCount: readModel.hostedScreenshotRefs.length,
      hostedAssertionCount: readModel.hostedAssertionRefs.length,
      childRuntimeArtifactRows: readModel.childRuntimeArtifactRows,
      missingChildRuntimeArtifactCount: readModel.missingChildRuntimeArtifactCount,
      missingFullProductUiArtifactCount: readModel.missingFullProductUiArtifactCount,
      blockerCount: readModel.blockers.length,
      productReadyBlockers: readModel.blockers.filter((row) => row.blockerId === 'product-ready-tracking-ui').length,
    },
    proofLabels: [
      'tracking-full-product-ui.hosted-route-only-boundary',
      'tracking-full-product-ui.child-runtime-artifact-gate-linked',
      'tracking-full-product-ui.manual-required-until-product-ui-artifacts',
      'tracking-full-product-ui.product-ready-false',
    ],
    productClaims: readModel.productClaims,
    readModel,
    commands,
  };
}

function assertProof(proof) {
  assert.equal(proof.summary.hostedScreenshotCount > 0, true, 'expected hosted screenshot refs');
  assert.equal(proof.summary.hostedAssertionCount > 0, true, 'expected hosted assertion refs');
  assert.equal(proof.summary.childRuntimeArtifactRows > 0, true, 'expected child runtime artifact rows');
  assert.equal(proof.summary.missingChildRuntimeArtifactCount > 0, true, 'expected child runtime artifact gaps');
  assert.equal(proof.summary.missingFullProductUiArtifactCount > 0, true, 'expected full product UI gaps');
  assert.equal(proof.summary.blockerCount, 13, 'expected every full product UI blocker');
  assert.equal(proof.summary.productReadyBlockers, 1, 'expected product-ready UI blocker');
  assert.equal(proof.productClaims.hostedRouteOnlyClaimed, true, 'hosted route evidence should stay acknowledged');
  assert.equal(
    Object.entries(proof.productClaims)
      .filter(([key]) => key !== 'hostedRouteOnlyClaimed')
      .every(([, claim]) => claim === false),
    true,
    'no full product UI claims'
  );
}

async function writeArtifacts(proof) {
  await writeJson(join(resultDir, 'proof.json'), proof);
  await writeJson(join(resultDir, 'read-model.json'), proof.readModel);
  await writeJson(join(focusedProofDir, 'proof.json'), proof);
  await writeJson(join(focusedProofDir, 'read-model.json'), proof.readModel);
  await writeFile(
    join(focusedProofDir, '00-source-snapshot.md'),
    [
      '# Tracking Full Product UI Readiness Blocker Source Snapshot',
      '',
      `- generatedAt: ${timestamp}`,
      `- commit: ${proof.commit}`,
      `- status: ${proof.status}`,
      `- missingFullProductUiArtifactCount: ${proof.summary.missingFullProductUiArtifactCount}`,
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(wp30ProofDir, '31-full-product-ui-readiness-blocker-proof.json'), proof);
  await writeJson(join(wp33ProofDir, '56-full-product-ui-readiness-blocker-proof.json'), proof);
  await writeFile(
    join(wp33ProofDir, '56-full-product-ui-readiness-blocker-validation-commands.log'),
    commandLog(),
    'utf8'
  );
}

async function readProofJson(relativePath) {
  return JSON.parse(await readFile(join(repoRoot, relativePath), 'utf8'));
}

function importDist(name) {
  return import(pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', name)).href);
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

function commandLog() {
  return `${commands.map((entry) => entry.command).join('\n')}\n`;
}

function relativePath(filePath) {
  return filePath
    .replace(repoRoot, '')
    .replace(/^[/\\]/, '')
    .replaceAll('\\', '/');
}
