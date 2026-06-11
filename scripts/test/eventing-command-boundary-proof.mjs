import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'eventing-command-boundary-proof');
const planOutputDir = join(repoRoot, 'output', 'eventing-plan-proof', '60-61-command-boundary');
const proofPath = join(testOutputDir, 'proof.json');
const planProofPath = join(planOutputDir, 'proof-summary.json');
const commands = [];
const proofLabels = [];

const directEnforcementActionCommands = [
  'AgentCommand.EnforcementExecute',
  'AgentCommand.EnforcementTimerRecover',
  'AgentCommand.EnforcementTimerExpire',
  'AgentCommand.EnforcementOverrideCancel',
];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(planOutputDir, { recursive: true });

  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/transport-lan-target.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'eslint',
      'src/transport.ts',
      'tests/transport-lan-target.test.ts',
    ])
  );
  await runCommand('node', ['scripts/check-source-shape.mjs']);

  await assertSourceContracts();

  const proof = {
    schemaVersion: 1,
    proofMode: 'eventing-command-boundary-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      portalTransport: 'apps/portal/src/transport.ts',
      portalTransportTest: 'apps/portal/tests/transport-lan-target.test.ts',
      portalCommandInventory: 'packages/portal-domain/src/commands.ts',
      parentAssistantAdapter: 'packages/agent-protocol-domain/src/parent-assistant-adapter.ts',
      aiRouter: 'crates/agent-service/src/websocket.rs',
      proofHarness: 'scripts/test/eventing-command-boundary-proof.mjs',
    },
    claimsProved: [
      'portal transport rejects direct enforcement action commands before WebSocket serialization',
      'portal command inventory contains no direct enforcement action commands',
      'portal read-model/proof commands remain allowed separately from enforcement action commands',
      'parent-assistant command mapping does not map AI or assistant requests to enforcement action commands',
      'agent-service AI command router does not call enforcement command handlers',
    ],
    claimsNotProved: [
      'Parent protocol event contracts for enforcement, policy, audit, or portal read-model events',
      'journal-before-action enforcement execution',
      'adapter apply, rollback, or audit artifact production',
      'complete network-to-AI-to-policy-to-enforcement production chain',
    ],
  };

  const serialized = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, serialized);
  await writeFile(planProofPath, serialized);
  console.log(`eventing-command-boundary-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
  console.log(`planEvidence=${relative(repoRoot, planProofPath)}`);
}

async function assertSourceContracts() {
  const transportSource = await readText('apps/portal/src/transport.ts');
  const transportTests = await readText('apps/portal/tests/transport-lan-target.test.ts');
  const portalCommands = await readText('packages/portal-domain/src/commands.ts');
  const parentAssistantAdapter = await readText('packages/agent-protocol-domain/src/parent-assistant-adapter.ts');
  const websocket = await readText('crates/agent-service/src/websocket.rs');

  assertIncludes(
    transportSource,
    'export function isPortalDirectEnforcementActionCommand',
    'portal enforcement action predicate'
  );
  for (const command of directEnforcementActionCommands) {
    assertIncludes(transportSource, command, `portal predicate covers ${command}`);
    assertIncludes(transportTests, command, `portal test covers ${command}`);
    assertDoesNotInclude(portalCommands, command, `portal command inventory excludes ${command}`);
  }
  assertIncludes(transportTests, 'PortalOverviewCommands', 'portal command inventory test includes overview commands');
  assertIncludes(transportTests, 'PortalCommandButtons', 'portal command inventory test includes command buttons');
  proofLabels.push('portal.transport.direct-enforcement-action-guard');
  proofLabels.push('portal.inventory.no-direct-enforcement-action');

  const commandForKind = sourceBetween(parentAssistantAdapter, 'function commandForKind', 'function parseJsonPayload');
  assertDoesNotInclude(
    commandForKind,
    'AgentCommand.Enforcement',
    'parent assistant command mapping does not target enforcement commands'
  );
  proofLabels.push('ai.parent-assistant.no-enforcement-command-map');

  const aiRouter = sourceBetween(websocket, 'async fn build_ai_command_report', 'fn build_dev_echo_report');
  assertDoesNotInclude(
    aiRouter,
    'AgentCommandName::AgentEnforcement',
    'AI command router does not match enforcement commands'
  );
  assertDoesNotInclude(aiRouter, 'build_enforcement', 'AI command router does not call enforcement builders');
  proofLabels.push('ai.service-router.no-enforcement-handler');
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], {
      cwd: repoRoot,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

async function readText(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

function sourceBetween(source, start, end) {
  const startIndex = source.indexOf(start);
  if (startIndex === -1) {
    throw new Error(`missing source start marker ${start}`);
  }
  const endIndex = source.indexOf(end, startIndex);
  if (endIndex === -1) {
    throw new Error(`missing source end marker ${end}`);
  }
  return source.slice(startIndex, endIndex);
}

function assertIncludes(text, expected, label) {
  if (!text.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertDoesNotInclude(text, unexpected, label) {
  if (text.includes(unexpected)) {
    throw new Error(`${label}: found ${unexpected}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
