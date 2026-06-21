import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'activity-surface-main-backed-adapter');
const proofPath = join(outputDir, 'proof.json');
const matrixPath = join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpmScript('build:contracts');
  await runNpmWorkspace('@ocentra-parent/activity-domain', ['test', '--', 'activity-surface']);
  await runNpmWorkspace('@ocentra-parent/agent-protocol-domain', ['test', '--', 'activity-surface-adapter']);
  await runNpmWorkspace('@ocentra-parent/portal', [
    'test',
    '--',
    'live-activity-surface-adapter',
    'activity-ui-intent',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'activity_surface']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'activity_surface']);
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await runCommand('node', ['scripts/test/portal-local-smoke.mjs']);
  await runCommand('node', ['scripts/test/activity-parent-assistant-runtime-proof.mjs']);
  await runNpmScript('test:pre-ai-proof');

  const matrix = JSON.parse(await readFile(matrixPath, 'utf8'));
  assertMatrixRegistration(matrix);

  proofLabels.push('activity-surface-main-backed-adapter.matrix-registered');
  proofLabels.push('activity-surface-main-backed-adapter.contracts');
  proofLabels.push('activity-surface-main-backed-adapter.rust-protocol-service');
  proofLabels.push('activity-surface-main-backed-adapter.real-service-runtime');
  proofLabels.push('activity-surface-main-backed-adapter.portal-ui-intent');
  proofLabels.push('activity-surface-main-backed-adapter.portal-smoke');

  const proof = {
    schemaVersion: 1,
    proofMode: 'activity-surface-main-backed-adapter',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      activityDomain: 'packages/schema-domain/src/activity-surface.ts',
      activityDomainTest: 'packages/activity-domain/tests/activity-surface.test.ts',
      agentProtocolDomainAdapter: 'packages/agent-protocol-domain/src/activity-surface-adapter.ts',
      agentProtocolDomainAdapterTest: 'packages/agent-protocol-domain/tests/unit/activity-surface-adapter.test.ts',
      rustProtocol: 'crates/agent-protocol/src/activity_surface.rs',
      rustServiceAdapter: 'crates/agent-service/src/activity_surface_adapter.rs',
      rustServiceReadModels: 'crates/agent-service/src/activity_surface_read_models.rs',
      rustDispatcherTest: 'crates/agent-service/src/activity_surface_main_backed_adapter_tests.rs',
      portalStateTest: 'apps/portal/tests/live-activity-surface-adapter.test.ts',
      portalUiIntentTest: 'apps/portal/tests/activity-ui-intent.test.ts',
      portalSmoke: 'scripts/test/portal-local-smoke.mjs',
      portalPlaywright: 'apps/portal/e2e/portal-ui.spec.ts',
      runtimeProof: 'scripts/test/activity-parent-assistant-runtime-proof.mjs',
      proofMatrix: 'docs/expectations/pre-ai-proof-matrix.json',
      checkpoint: 'docs/checkpoints/activity-surface-main-backed-adapter-proof-2026-05-29.md',
    },
    productTruth: {
      viteDataOwnership:
        'The Activity product surface is proven through typed Rust service commands and read-model events; Vite remains a development shell only.',
      coveredTabs: ['reports', 'screen', 'app-use', 'browser', 'games', 'network'],
      typedStates:
        'Ready, empty, unavailable, offline, stale, permission-required, and scaffold-only states remain explicit read-model states.',
      uiScope:
        'This proof covers the Activity service-to-UI intent seam; C-owned visual polish remains outside this proof.',
    },
    matrixRegistration: {
      state: 'registered',
      path: 'docs/expectations/pre-ai-proof-matrix.json',
      claimId: 'activity-surface-main-backed-adapter',
      checkpointScenarioId: 'activity-surface-main-backed-adapter',
    },
    knownGaps: [
      'C-owned visual polish and product UX remain incomplete, but the Activity UI intent seam consumes the merged adapter surface.',
      'Family fan-out beyond local service state and data storage destination selection remain typed local or unavailable behavior.',
      'This focused proof runs portal-local smoke but not full Playwright, full validate, Android device-owner proof, or iOS entitlement proof.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`activity-surface-main-backed-adapter-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

function assertMatrixRegistration(matrix) {
  if (!matrix.requiredCompletedClaimIds?.includes('activity-surface-main-backed-adapter')) {
    throw new Error('Proof matrix is missing requiredCompletedClaimIds activity-surface-main-backed-adapter.');
  }

  const scenario = matrix.checkpointScenarios?.find((entry) => entry.id === 'activity-surface-main-backed-adapter');
  if (!scenario) {
    throw new Error('Proof matrix is missing Activity surface checkpoint scenario.');
  }
  if (!scenario.ciCommands?.includes('node scripts/test/activity-surface-main-backed-adapter-proof.mjs')) {
    throw new Error('Activity surface checkpoint scenario is missing the focused proof command.');
  }

  const claim = matrix.claims?.find((entry) => entry.id === 'activity-surface-main-backed-adapter');
  if (!claim) {
    throw new Error('Proof matrix is missing Activity surface claim.');
  }

  const coverage = claim.runtimeSurfaceCoverage ?? {};
  for (const key of ['contracts', 'rustServiceAdapter', 'typedUnavailableStates', 'viteDataOwnership']) {
    if (typeof coverage[key]?.state !== 'string') {
      throw new Error(`Activity surface claim is missing runtimeSurfaceCoverage.${key}.state.`);
    }
  }
}

async function runNpmScript(scriptName) {
  await runNpm(['run', scriptName]);
}

async function runNpmWorkspace(workspaceName, args) {
  await runNpm(['--workspace', workspaceName, ...args]);
}

async function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  await runCommand(command, commandArgs, ...rest);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
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
