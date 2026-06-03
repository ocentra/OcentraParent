import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof');
const proofPath = join(proofRoot, '32-journal-sqlite-and-read-model-proof', '18-service-read-model-proof.json');
const commands = [];

await main();

async function main() {
  await runNpmWorkspace('@ocentra-parent/text-domain', ['run', 'build']);
  await runNpmWorkspace('@ocentra-parent/agent-protocol-domain', ['run', 'build']);
  await runNpmWorkspace('@ocentra-parent/agent-protocol-domain', [
    'run',
    'test',
    '--',
    'tracking-read-model',
    'contracts',
  ]);
  await runNpmWorkspace('@ocentra-parent/portal-domain', ['run', 'test', '--', 'contracts']);
  await runNpmWorkspace('@ocentra-parent/portal', ['run', 'test', '--', 'tracking-status-panel']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'tracking_read_model']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-core', 'tracking_read_model']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'tracking_read_model']);

  const checkedAt = new Date().toISOString();
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit: await gitHead(),
    workpackId: '32-journal-sqlite-and-read-model-proof',
    proofMode: 'tracking-service-read-model',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    currentStatus: 'proved',
    productClaimReady: false,
    commands,
    serviceBoundary: {
      command: 'agent.activity.tracking.read-model.get',
      event: 'agent.activity.tracking.read-model.reported',
      payloadField: 'trackingReadModel',
      sourceStore: 'ActivityStore SQLite activity_events',
      portalConsumer: 'apps/portal/src/tracking-status-panel.ts',
      coveredEventKinds: [
        'activity.location.observed',
        'activity.tracking.geofence-transition.evaluated',
        'activity.tracking.expected-place.evaluated',
        'activity.tracking.child-check-in.responded',
        'activity.tracking.retention.deleted',
      ],
      citationField: 'evidenceReferenceIds',
    },
    proofArtifacts: {
      typescriptProtocolDomain: 'packages/agent-protocol-domain/src/contracts.ts',
      rustProtocolReadModel: 'crates/agent-protocol/src/tracking_read_model.rs',
      rustCoreReadModel: 'crates/agent-core/src/activity_store_tracking.rs',
      rustCoreRows: 'crates/agent-core/src/activity_store_tracking_rows.rs',
      rustServiceDispatcher: 'crates/agent-service/src/websocket.rs',
      rustServicePayload: 'crates/agent-service/src/tracking_read_model_payload.rs',
      rustServiceTest: 'crates/agent-service/src/tracking_read_model_service_tests.rs',
      typescriptReadModelParser: 'packages/agent-protocol-domain/src/tracking-read-model.ts',
      portalLiveState: 'apps/portal/src/live-activity-state.ts',
      portalTrackingSurface: 'apps/portal/src/tracking-status-panel.ts',
      portalTrackingSurfaceTest: 'apps/portal/tests/tracking-status-panel.test.ts',
    },
    nonClaims: [
      'This proof does not claim Android or iOS physical background tracking behavior.',
      'This proof does not claim enrolled-device authority, production pilot readiness, or provider delivery.',
      'This proof only claims narrow portal summary consumption of the service-backed read model, not complete parent/child tracking UI.',
    ],
    remainingGapsBeforeProductOrPrReady: [
      'Hosted portal screenshot, accessibility, and browser-to-service proof remain pending.',
      'Richer product tracking read-model surfaces remain pending.',
      'Child-device UI and device permission screenshots remain pending.',
      'Android/iOS physical background geofence proof remains manual-required.',
      'Authority-enrolled and production-pilot proof remain absent.',
    ],
  };

  await mkdir(join(proofPath, '..'), { recursive: true });
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('tracking-plan-service-read-model-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath).replace(/\\/gu, '/')}`);
}

async function runNpmWorkspace(workspaceName, args) {
  await runNpm(['--workspace', workspaceName, ...args]);
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args]);
    return;
  }
  await runCommand('npm', args);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      commands.push({ command: commandLine, exitCode: code });
      code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`));
    });
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
