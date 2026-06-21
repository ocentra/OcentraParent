import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'app-game-scoped-adapter-dispatch-execute-command-proof');
const proofPath = join(outputDir, 'proof.json');
const planOutputDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '173-app-game-scoped-adapter-dispatch-execute-command'
);
const planProofPath = join(planOutputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(planOutputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'app-game-adapter-dispatch-result',
      'contracts',
    ])
  );
  await runCommand(...npmCommand(['run', 'test', '--workspace', '@ocentra-parent/text-domain', '--', 'portal-dev']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'app-game-adapter-dispatch-route-panel',
    ])
  );
  await runCommand('cargo', [
    'test',
    '-p',
    'ocentra-parent-agent-protocol',
    'app_game_adapter_dispatch_execute_command_and_event_names_serialize_to_contract_shape',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'app_game_adapter_dispatch_execute']);
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/portal']));

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-scoped-adapter-dispatch-execute-command-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    serviceCommand: 'agent.activity.app-game.adapter-dispatch.execute',
    serviceEvent: 'agent.activity.app-game.adapter-dispatch.executed',
    payloadField: 'appGameAdapterDispatchExecuteResult',
    executionCommand: 'agent.enforcement.execute',
    readbackCommand: 'agent.activity.app-game.adapter-dispatch-result.read-model.get',
    evidence: {
      agentProtocolDomain: 'packages/agent-protocol-domain/src/app-game-adapter-dispatch-result.ts',
      protocolContracts: 'packages/agent-protocol-domain/src/contracts.ts',
      rustProtocol: 'crates/agent-protocol/src/app_game_adapter_dispatch_result.rs',
      servicePayload: 'crates/agent-service/src/activity_api/app_game_adapter_dispatch_result_payload.rs',
      serviceWebsocket: 'crates/agent-service/src/websocket.rs',
      portalCommands: 'apps/portal/src/AppGameAdapterDispatchRoutePanel.tsx',
      portalText: 'packages/text-domain/src/portal-dev.ts',
    },
    summary: {
      expectedRows: 8,
      dispatchExecuteRows: 1,
      blockedBeforeExecutionRows: 7,
      executionStatus: 'actually-enforced',
      readModelCommandSideEffectFree: true,
      portalOverviewAutoExecute: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
    },
    claimsProved: [
      'agent-protocol-domain parses a dedicated app/game adapter dispatch executed event',
      'Rust protocol exposes stable command and event names for the manual dispatch execute command',
      'Rust service dispatch execute command invokes the existing scoped agent.enforcement.execute path',
      'portal-domain exposes the execute command as a manual command button',
      'portal overview commands do not auto-run the execute command',
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
  console.log('app-game-scoped-adapter-dispatch-execute-command-proof-ok');
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
