import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'social-video-ai-signal-aggregate-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'tests/social-video-ai-signal-aggregate.test.ts',
  ]);

  const packageExport = await assertPackageExport();
  const aggregate = await assertBuiltAggregateContract();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/activity-domain/src/social-video-ai-signal-aggregate.ts',
      contractTest: 'packages/activity-domain/tests/social-video-ai-signal-aggregate.test.ts',
      packageExport,
      documentation,
      output: relativePath(proofPath),
    },
    aggregateState: aggregate.aggregateState,
    sourcePrivacyEvidenceId: aggregate.sourcePrivacyEvidenceId,
    socialAiAnalysisIds: aggregate.socialAiAnalysisIds,
    socialRiskBenefitSignalSetIds: aggregate.socialRiskBenefitSignalSetIds,
    routeGatePlanIds: aggregate.routeGatePlanIds,
    actionCandidateRefs: aggregate.actionCandidateRefs,
    nonClaims: [
      'raw content capture',
      'raw message capture',
      'raw video capture',
      'screenshot capture',
      'connector token storage',
      'connector API calls',
      'native app control',
      'final policy decisions',
      'alert delivery',
      'rendered UI',
      'enforcement',
    ],
    knownGaps: [
      'rendered social/video parent UI',
      'notification delivery',
      'connector OAuth/API runtime',
      'native social app adapters',
      'final policy execution',
      'enforcement',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`social-video-ai-signal-aggregate-proof-ok:${relativePath(proofPath)}`);
}

async function assertPackageExport() {
  const packageJson = JSON.parse(await readRepoFile('packages/activity-domain/package.json'));
  assert.deepEqual(packageJson.exports['./social-video-ai-signal-aggregate'], {
    import: './dist/social-video-ai-signal-aggregate.js',
    types: './dist/social-video-ai-signal-aggregate.d.ts',
  });
  return 'packages/activity-domain/package.json#exports[./social-video-ai-signal-aggregate]';
}

async function assertBuiltAggregateContract() {
  const modulePath = pathToFileURL(
    join(repoRoot, 'packages', 'activity-domain', 'dist', 'social-video-ai-signal-aggregate.js')
  );
  const module = await import(modulePath.href);
  const aggregate = module.SocialVideoAiSignalAggregateSchema.parse({
    schemaVersion: 1,
    aggregateId: 'social-video-ai-signal-aggregate-proof-row',
    aggregatedAt: '2026-06-04T04:25:00.000Z',
    sourcePrivacyEvidenceId: 'source-privacy-proof-youtube',
    childProfileRef: 'child-profile-proof',
    deviceId: 'device-proof-managed-browser',
    platform: 'youtube',
    targetKind: 'video-url',
    sourceEvidenceIds: ['source-privacy-proof-youtube', 'social-route-proof', 'social-video-metadata-proof'],
    socialRouteEvidenceIds: ['social-route-proof'],
    socialVideoMetadataEvidenceIds: ['social-video-metadata-proof'],
    socialAiAnalysisIds: ['social-ai-analysis-proof-youtube'],
    socialRiskBenefitSignalSetIds: ['social-riskbenefit-signal-set-proof-youtube'],
    routeGatePlanIds: ['social-route-gate-proof-youtube'],
    actionCandidateRefs: ['social-parent-video-review-request-proof'],
    recommendedPolicyInput: 'parent-review-candidate',
    aggregateState: 'candidate-ready',
    custodyLabel: 'child-device-local',
    confidence: 'medium',
    degradedState: 'none',
    permittedDownstreamUses: ['ai-analysis-input', 'policy-candidate-input', 'parent-explanation', 'audit-summary'],
    rawContentCaptured: false,
    rawMessageContentCaptured: false,
    rawVideoCaptured: false,
    screenshotCaptured: false,
    connectorTokenStored: false,
    connectorApiCalled: false,
    nativeAppControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    alertDeliveryClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
  });

  assert.equal(aggregate.rawContentCaptured, false);
  assert.equal(aggregate.rawMessageContentCaptured, false);
  assert.equal(aggregate.rawVideoCaptured, false);
  assert.equal(aggregate.screenshotCaptured, false);
  assert.equal(aggregate.connectorTokenStored, false);
  assert.equal(aggregate.connectorApiCalled, false);
  assert.equal(aggregate.nativeAppControlClaimed, false);
  assert.equal(aggregate.finalPolicyDecisionClaimed, false);
  assert.equal(aggregate.alertDeliveryClaimed, false);
  assert.equal(aggregate.uiRenderedClaimed, false);
  assert.equal(aggregate.enforcementClaimed, false);

  return aggregate;
}

async function assertDocumentationProof() {
  const featureDoc = await readRepoFile('docs/features/social-video-control.md');
  const expectationDoc = await readRepoFile('docs/expectations/social-video-control.md');
  const packageReadme = await readRepoFile('packages/activity-domain/README.md');
  assertIncludes(featureDoc, proofMode, 'social/video feature proof note');
  assertIncludes(expectationDoc, proofMode, 'social/video expectation proof note');
  assertIncludes(packageReadme, proofMode, 'activity-domain README proof note');
  return [
    'docs/features/social-video-control.md',
    'docs/expectations/social-video-control.md',
    'packages/activity-domain/README.md',
  ];
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
