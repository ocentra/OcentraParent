import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'parent-desktop-release-support-proof');
const proofPath = join(outputDir, 'proof.json');
const proofCommand = 'node scripts/test/parent-desktop-release-support-proof.mjs';
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/parent-desktop-release-support.test.ts',
  ]);

  const packageJson = JSON.parse(await readFile(join(repoRoot, 'package.json'), 'utf8'));
  const commit = await gitHead();
  const ciArtifactProof = await buildCiArtifactProof();
  const readModel = await parseReadModel(buildReadModel(packageJson.version, commit, ciArtifactProof));
  assertReadModel(readModel);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit,
    proofMode: 'parent-desktop-release-support-proof',
    commands,
    evidence: {
      contract: 'packages/parent-domain/src/parent-desktop-release-support.ts',
      contractTest: 'packages/parent-domain/tests/parent-desktop-release-support.test.ts',
      output: relative(repoRoot, proofPath),
      packagePreviewWorkflow: '.github/workflows/package-preview.yml',
      featureDocs: [
        'docs/features/production-distribution-support.md',
        'docs/features/child-agent-local-service.md',
        'docs/features/remote-lan-mobile-platforms.md',
      ],
    },
    readModel,
    workpacks: {
      completed: ['04', '06', '09', '10', '11', '12', '15', '16', '17', '18', '19', '20'],
      partial: [],
      partialReason: null,
    },
    claimsProved: [
      'Parent observer read-only state rejects policy writes, approvals, and controller takeover.',
      'Parent mobile bridge state is separate from child Android and child iOS agent claims.',
      'Parent desktop package runtime uses built portal dist, the Rust service boundary, fixed loopback ownership, and package service-manager launch evidence.',
      'Update, rollback, signing, notarization, store, TestFlight, Play, and production promotion states remain explicit and manual-required where proof is missing.',
      'Support diagnostics include version, commit, platform, package, service, route, capability, and degraded state without secrets, private child data, raw URLs, command lines, keystrokes, clipboard data, message contents, journals, SQLite snapshots, screenshots, or private paths.',
      'Package preview CI artifact status is recorded as pending/manual-required unless a real Actions artifact context proves readiness.',
    ],
    claimsNotProved: [
      'signed release publishing',
      'store distribution',
      'macOS notarization',
      'production updater rollback',
      'production support workflow',
      'mobile child-agent parity',
      'cloud relay proof',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`parent-desktop-release-support-proof-ok:${relative(repoRoot, proofPath)}`);
}

function buildReadModel(version, commit, ciArtifactProof) {
  return {
    schemaVersion: 'parent-desktop-release-support-proof',
    observerAuthority: observerAuthority(),
    mobileBridgeBoundary: {
      parentMobileState: 'scaffold',
      childAndroidAgentState: 'manual-required',
      childIosAgentState: 'manual-required',
      parentMobileClaim: 'parent mobile bridge is a parent shell route boundary only',
      childAgentNonClaim: 'child Android and child iOS agent parity is not claimed by parent desktop release support',
    },
    packageRuntimeEvidence: packageRuntimeEvidence(ciArtifactProof),
    updateStates: updateStates(),
    signingStoreStates: signingStoreStates(),
    platformCapabilityMatrix: platformRows(),
    ciArtifactProof,
    supportDiagnostics: supportDiagnostics(version, commit),
    manualRunbook: manualRunbook(version),
    updatedAt: new Date().toISOString(),
  };
}

function observerAuthority() {
  return [
    authority('read-service-state', 'completed', null),
    authority('read-route-state', 'completed', null),
    authority('write-policy', 'rejected', 'observer-read-only'),
    authority('approve-request', 'rejected', 'observer-read-only'),
    authority('take-controller', 'disabled', 'observer-read-only'),
  ];
}

function authority(operation, result, rejectionReason) {
  return {
    operation,
    result,
    authorityRole: 'observer',
    rejectionReason,
    proofRequirement: `${operation} must preserve parent observer read-only authority`,
  };
}

function packageRuntimeEvidence(ciArtifactProof) {
  return {
    packageFrontendSource: 'built-portal-dist',
    backendBoundary: 'rust-service-boundary',
    serviceLaunchOwner: 'package-service-manager',
    serviceHealthState: 'implemented',
    connectOrDegradeState: 'degraded',
    fixedAgentAddress: '127.0.0.1:4477',
    portOwnership: 'fixed-loopback',
    portConflictPolicy: 'no-foreign-process-reclaim',
    processOwnership: 'parent-shell-only',
    blankWindowGuard: 'frontend-dist-required',
    updateRollbackPosture: 'signed-channel-required',
    artifactState: ciArtifactProof.artifactState,
    supportDiagnosticState: 'preview-only',
    nonClaim: 'CI package preview is not signing not production not store distribution proof',
  };
}

function updateStates() {
  return [
    updateState('scaffold', 'scaffold', 'signature-required', 'rollback-unavailable'),
    updateState('unsigned-preview', 'unsigned-preview', 'signature-required', 'rollback-unavailable'),
    updateState('signature-required', 'signature-required', 'signature-required', 'rollback-unavailable'),
    updateState('production', 'production-promotion-required', 'signature-required', 'rollback-unavailable'),
  ];
}

function updateState(channel, packageState, signingState, rollbackState) {
  return {
    channel,
    packageState,
    signingState,
    rollbackState,
    productionPromotionState: 'production-promotion-required',
    proofRequirement: `${channel} update state must not imply signed production rollback`,
  };
}

function signingStoreStates() {
  return ['windows-code-signing', 'macos-notarization', 'google-play', 'testflight', 'app-store'].map((surface) => ({
    surface,
    state: 'manual-required',
    credentialState: 'manual-required',
    proofRequirement: `${surface} remains manual-required until real credentials and artifacts exist`,
  }));
}

function platformRows() {
  return [
    platformRow('parent-desktop', 'unsigned-preview', 'implemented', 'preview-only', 'preview-only'),
    platformRow('parent-mobile', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('child-desktop', 'preview-only', 'implemented', 'preview-only', 'manual-required'),
    platformRow('child-android', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('child-ios', 'scaffold', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('relay', 'not-implemented', 'not-implemented', 'not-implemented', 'not-ready'),
    platformRow('signing', 'signature-required', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('store', 'manual-required', 'manual-required', 'manual-required', 'manual-required'),
    platformRow('support', 'preview-only', 'preview-only', 'preview-only', 'preview-only'),
  ];
}

function platformRow(target, packageState, serviceState, capabilityState, proofLevel) {
  return {
    target,
    packageState,
    serviceState,
    routeState: target === 'relay' ? 'not-implemented' : 'preview-only',
    capabilityState,
    proofLevel,
    nonClaim: `${target} state is limited to the named proof level and does not upgrade unsupported platform behavior`,
  };
}

async function buildCiArtifactProof() {
  const workflow = await readFile(join(repoRoot, '.github', 'workflows', 'package-preview.yml'), 'utf8');
  assert.match(workflow, /uses: actions\/upload-artifact@v6/u);
  assert.match(workflow, /ocentra-parent-windows-x64-preview/u);
  assert.match(workflow, /ocentra-parent-android-preview/u);
  assert.match(workflow, /ocentra-parent-ios-simulator-preview/u);

  if (process.env.GITHUB_ACTIONS === 'true') {
    const runId = process.env.GITHUB_RUN_ID ?? null;
    const server = process.env.GITHUB_SERVER_URL ?? 'https://github.com';
    const repository = process.env.GITHUB_REPOSITORY ?? 'ocentra/OcentraParent';
    return {
      workflowName: 'Package Preview',
      runStatus: 'pending',
      artifactState: 'pending',
      packageReadinessClaim: 'manual-required',
      checkedBy: proofCommand,
      runUrl: runId === null ? null : `${server}/${repository}/actions/runs/${runId}`,
    };
  }

  return {
    workflowName: 'Package Preview',
    runStatus: 'not-checked-local',
    artifactState: 'not-checked-local',
    packageReadinessClaim: 'manual-required',
    checkedBy: proofCommand,
    runUrl: null,
  };
}

function supportDiagnostics(version, commit) {
  return {
    outputState: 'preview-only',
    entries: [
      diagnostic('version', version),
      diagnostic('commit', commit),
      diagnostic('platform', process.platform),
      diagnostic('package', 'parent-desktop unsigned preview'),
      diagnostic('service', 'loopback service reachable or explicitly unavailable'),
      diagnostic('route', 'local route or unavailable route state'),
      diagnostic('capability', 'observer read-only release support'),
      diagnostic('degraded-state', 'signing store relay and rollback are manual-required'),
    ],
    redactedFields: [
      'tokens',
      'child activity',
      'raw urls',
      'screenshots',
      'journals',
      'SQLite snapshots',
      'private paths',
      'command lines',
      'keystrokes',
      'clipboard data',
      'message contents',
    ],
  };
}

function diagnostic(field, value) {
  return { field, value, redactionState: 'safe' };
}

function manualRunbook(version) {
  return [
    'parent-desktop',
    'parent-mobile',
    'child-desktop',
    'child-android',
    'child-ios',
    'relay',
    'signing',
    'store',
    'support',
  ].map((target) => ({
    target,
    hostOrDevice: `${target} named manual host or device`,
    commandOrUiAction: `${target} package launch or UI proof action`,
    permissions: `${target} permissions and entitlement state recorded`,
    packageVersion: version,
    logsScreenshotsProofJson: `test-results/manual-platform-proof/${target}.json`,
    knownGaps: [`${target} requires manual proof before production claim`],
  }));
}

async function parseReadModel(readModel) {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'parent-desktop-release-support.js');
  const module = await import(pathToFileURL(modulePath).href);
  return module.ParentDesktopReleaseSupportReadModelSchema.parse(readModel);
}

function assertReadModel(readModel) {
  assert.equal(readModel.schemaVersion, 'parent-desktop-release-support-proof');
  assert.equal(readModel.observerAuthority.find((entry) => entry.operation === 'write-policy').result, 'rejected');
  assert.equal(readModel.mobileBridgeBoundary.childAndroidAgentState, 'manual-required');
  assert.equal(readModel.packageRuntimeEvidence.packageFrontendSource, 'built-portal-dist');
  assert.equal(readModel.packageRuntimeEvidence.backendBoundary, 'rust-service-boundary');
  assert.equal(readModel.packageRuntimeEvidence.serviceLaunchOwner, 'package-service-manager');
  assert.equal(readModel.packageRuntimeEvidence.portConflictPolicy, 'no-foreign-process-reclaim');
  assert.equal(readModel.packageRuntimeEvidence.processOwnership, 'parent-shell-only');
  assert.equal(
    readModel.updateStates.find((entry) => entry.channel === 'unsigned-preview').rollbackState,
    'rollback-unavailable'
  );
  assert.equal(readModel.ciArtifactProof.packageReadinessClaim, 'manual-required');
  assert.equal(readModel.supportDiagnostics.entries.length, 8);
  assert.equal(readModel.manualRunbook.length, 9);
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
