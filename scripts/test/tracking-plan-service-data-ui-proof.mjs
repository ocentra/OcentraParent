import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofRoot = join(repoRoot, 'output', 'tracking-plan-proof');
const uiProofRoot = join(proofRoot, '30-parent-and-child-ui-ux-surfaces');
const readModelProofRoot = join(proofRoot, '32-journal-sqlite-and-read-model-proof');
const testResultRoot = join(repoRoot, 'test-results', 'tracking-plan-service-data-ui-proof');
const uiProofPath = join(uiProofRoot, '18-service-data-ui-proof.json');
const readModelProofPath = join(readModelProofRoot, '20-service-data-ui-proof.json');
const testResultProofPath = join(testResultRoot, 'proof.json');
const commands = [];

await main();

async function main() {
  await runCommand('node', ['scripts/test/tracking-plan-service-read-model-proof.mjs']);
  await runNpmWorkspace('@ocentra-parent/text-domain', ['run', 'test', '--', 'portal-dev']);
  await runNpmWorkspace('@ocentra-parent/portal', ['run', 'test', '--', 'tracking-status-panel']);

  const checkedAt = new Date().toISOString();
  const commit = await gitHead();
  const proof = {
    schemaVersion: 1,
    checkedAt,
    commit,
    workpackIds: ['30-parent-and-child-ui-ux-surfaces', '32-journal-sqlite-and-read-model-proof'],
    proofMode: 'tracking-service-data-ui-proof',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    currentStatus: 'proved',
    productClaimReady: false,
    commands,
    serviceDataUiSurface: {
      route: 'policy-tracking',
      consumer: 'apps/portal/src/TrackingStatusRoutePanel.tsx',
      modelFunction: 'trackingStatusServiceDataCoverage',
      sourceModel: 'apps/portal/src/tracking-status-panel.ts',
      renderedTitle: 'Service data coverage',
      sourcePayload: 'trackingReadModel',
      sourceCommand: 'agent.activity.tracking.read-model.get',
      sourceEvent: 'agent.activity.tracking.read-model.reported',
      validatedFields: [
        'rowsReturned',
        'activeRows',
        'tombstoneRows',
        'latestTombstoneEventId',
        'latestTombstoneObservedAt',
        'capabilityStatus',
        'custodyLabel',
        'row.kind coverage',
        'row.evidenceReferenceIds',
        'row.deletedEvidenceReferenceIds',
        'productClaimReady=false',
      ],
    },
    assertions: [
      'The hosted policy-tracking route can render service-data coverage from the parsed tracking read model.',
      'The React hosted policy-tracking route renders the service-data coverage card beside the service read-model summary.',
      'The coverage row exposes active/tombstone row counts through existing portal detail fields.',
      'The coverage row exposes event-kind coverage and deleted evidence references separately from active evidence references.',
      'The coverage row keeps productClaimReady=false and does not claim physical-device, provider, notification, or production readiness.',
      'The underlying service proof still validates the Rust service command, protocol parser, portal state parser, and SQLite ActivityStore read-model path.',
    ],
    proofArtifacts: {
      serviceReadModelProof:
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      uiServiceDataProof: 'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json',
      readModelServiceDataProof:
        'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json',
      testResult: 'test-results/tracking-plan-service-data-ui-proof/proof.json',
    },
    nonClaims: [
      'This proof does not claim Android or iOS physical background tracking behavior.',
      'This proof does not claim live provider delivery, notification delivery, enrolled authority, or production pilot readiness.',
      'This proof does not claim child-device delivery/runtime UI or full parent/child UI beyond the hosted parent route.',
    ],
    remainingGapsBeforeProductClaim: [
      'Android foreground/background location and geofence runtime proof remains manual-required.',
      'iOS Core Location and background region proof remains manual-required.',
      'Physical Android/iOS proof remains manual-required.',
      'Child-device runtime UI and full parent/child screenshots remain pending.',
      'Provider delivery, notification delivery, authority-enrolled proof, and production proof remain pending.',
    ],
  };

  await mkdir(uiProofRoot, { recursive: true });
  await mkdir(readModelProofRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  await writeFile(uiProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(readModelProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(testResultProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('tracking-plan-service-data-ui-proof-ok');
  console.log(`ui=${relative(repoRoot, uiProofPath).replace(/\\/gu, '/')}`);
  console.log(`readModel=${relative(repoRoot, readModelProofPath).replace(/\\/gu, '/')}`);
  console.log(`evidence=${relative(repoRoot, testResultProofPath).replace(/\\/gu, '/')}`);
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
