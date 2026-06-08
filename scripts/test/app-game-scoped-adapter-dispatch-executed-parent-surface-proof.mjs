import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-game-scoped-adapter-dispatch-executed-parent-surface-proof');
const proofPath = join(outputDir, 'proof.json');
const planOutputDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '174-app-game-scoped-adapter-dispatch-executed-parent-surface'
);
const planProofPath = join(planOutputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(planOutputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'app-game-adapter-dispatch-result',
    'contracts',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal-domain',
    '--',
    'app-game-adapter-dispatch-result-panel',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'live-activity-state',
  ]);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/portal']);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-scoped-adapter-dispatch-executed-parent-surface-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    sourceEvent: 'agent.activity.app-game.adapter-dispatch.executed',
    sourceCommand: 'agent.activity.app-game.adapter-dispatch.execute',
    readbackCommand: 'agent.activity.app-game.adapter-dispatch-result.read-model.get',
    payloadField: 'appGameAdapterDispatchExecuteResult',
    evidence: {
      portalLiveState: 'apps/portal/src/live-activity-state.ts',
      portalLiveStateTest: 'apps/portal/tests/live-activity-state.test.ts',
      portalPanel: 'packages/portal-domain/src/app-game-adapter-dispatch-result-panel.ts',
      portalPanelTest: 'packages/portal-domain/tests/app-game-adapter-dispatch-result-panel.test.ts',
    },
    summary: {
      latestExecutedEventRetained: true,
      executeResultRendered: true,
      readModelCommandSideEffectFree: true,
      overviewAutoExecute: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
    },
    claimsProved: [
      'portal live activity state parses the latest app/game adapter dispatch executed event',
      'portal-domain renders parent-safe execute command/result/status/audit/readback details',
      'executed result stays separate from the side-effect-free dispatch result read model',
      'parent surface keeps platform and child delivery claims false',
    ],
    claimsNotProved: [
      'broad installed-app blocking execution',
      'platform enforcement outside the scoped Windows owned-process boundary',
      'provider delivery or provider receipt ingestion',
      'child-device runtime delivery',
      'private diagnostics or raw target/source row exposure',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(planProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('app-game-scoped-adapter-dispatch-executed-parent-surface-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
  console.log(`planEvidence=${relative(repoRoot, planProofPath)}`);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
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
