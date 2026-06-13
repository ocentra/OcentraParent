import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'social-video-source-privacy-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/browser-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/browser-domain',
      '--',
      'tests/social-video-source-privacy.test.ts',
    ])
  );

  const packageExport = await assertPackageExport();
  const contract = await assertBuiltContract();
  const documentation = await assertDocumentationProof();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode,
    commands,
    evidence: {
      contract: 'packages/browser-domain/src/social-video-source-privacy.ts',
      contractTest: 'packages/browser-domain/tests/unit/social-video-source-privacy.test.ts',
      packageExport,
      documentation,
      output: relativePath(proofPath),
    },
    acceptedSources: contract.acceptedSources,
    permittedDownstreamUses: contract.permittedDownstreamUses,
    nonClaims: [
      'raw content capture',
      'raw message capture',
      'raw video capture',
      'screenshot capture',
      'connector token storage',
      'connector API calls',
      'native app control',
      'final policy decisions',
      'enforcement',
      'portal UI',
      'notifications',
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
  console.log(`social-video-source-privacy-proof-ok:${relativePath(proofPath)}`);
}

async function assertPackageExport() {
  const packageJson = JSON.parse(await readRepoFile('packages/activity-domain/package.json'));
  assert.deepEqual(packageJson.exports['./social-video-source-privacy'], {
    import: './dist/social-video-source-privacy.js',
    types: './dist/social-video-source-privacy.d.ts',
  });
  return 'packages/activity-domain/package.json#exports[./social-video-source-privacy]';
}

async function assertBuiltContract() {
  const modulePath = pathToFileURL(
    join(repoRoot, 'packages', 'activity-domain', 'dist', 'social-video-source-privacy.js')
  );
  const module = await import(modulePath.href);
  const summary = module.buildSocialVideoSourcePrivacySummary({
    sourcePrivacyEvidenceId: 'source-privacy-proof-youtube',
    summarizedAt: '2026-06-04T01:44:00.000Z',
    childProfileRef: 'child-profile-proof',
    deviceId: 'device-proof-managed-browser',
    sourceEvidenceIds: ['social-route-proof', 'social-video-metadata-proof', 'screen-summary-proof'],
    platform: 'youtube',
    targetKind: 'video-url',
    sourceTypes: [
      'managed-browser-social-route-ref',
      'managed-browser-video-metadata-ref',
      'parent-provided-url-ref',
      'parent-provided-channel-ref',
      'screen-summary-ref',
      'connector-authorization-ref',
    ],
    socialRouteEvidenceIds: ['social-route-proof'],
    socialVideoMetadataEvidenceIds: ['social-video-metadata-proof'],
    parentProvidedUrlRefs: ['parent-url-ref-proof'],
    parentProvidedChannelRefs: ['parent-channel-ref-proof'],
    screenSummaryEvidenceRefs: ['screen-summary-proof'],
    connectorAuthorizationRefs: ['connector-authorization-proof'],
    manualRequiredReason: null,
    custodyLabel: 'child-device-local',
    confidence: 'medium',
    degradedState: 'none',
    permittedDownstreamUses: ['ai-analysis-input', 'policy-candidate-input', 'parent-explanation', 'audit-summary'],
  });

  assert.equal(summary.rawContentCaptured, false);
  assert.equal(summary.rawMessageContentCaptured, false);
  assert.equal(summary.rawVideoCaptured, false);
  assert.equal(summary.screenshotCaptured, false);
  assert.equal(summary.connectorTokenStored, false);
  assert.equal(summary.connectorApiCalled, false);
  assert.equal(summary.nativeAppControlClaimed, false);
  assert.equal(summary.finalPolicyDecisionClaimed, false);
  assert.equal(summary.enforcementClaimed, false);

  return {
    acceptedSources: summary.sourceTypes,
    permittedDownstreamUses: summary.permittedDownstreamUses,
  };
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
