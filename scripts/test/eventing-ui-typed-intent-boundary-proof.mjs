import { spawn } from 'node:child_process';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { extname, join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'eventing-ui-typed-intent-boundary-proof');
const planOutputDir = join(repoRoot, 'output', 'eventing-plan-proof', '52-ui-typed-intent-boundary');
const proofPath = join(testOutputDir, 'proof.json');
const planProofPath = join(planOutputDir, 'proof-summary.json');
const commands = [];
const proofLabels = [];

const sourceRoots = ['apps/portal/src', 'packages/portal-domain/src'];
const sourceExtensions = new Set(['.ts', '.tsx']);
const forbiddenPublisherPatterns = [
  {
    label: 'no reusable Rust eventing import in portal',
    pattern: /ocentra-eventing|@ocentra-parent\/eventing|EventBus|NetworkEventBus/u,
  },
  {
    label: 'no portal event publish function',
    pattern: /(?:^|[^\w])(?:publishEvent|publishBusinessEvent|publishDomainEvent|createEventPublisher)\s*\(/u,
  },
  {
    label: 'no portal event bus publish call',
    pattern: /(?:eventBus|bus|publisher)\.publish\s*\(/u,
  },
  {
    label: 'no portal event subscription ownership',
    pattern: /(?:eventBus|bus)\.subscribe\s*\(/u,
  },
  {
    label: 'no portal event envelope send',
    pattern: /AgentEventEnvelopeSchema\.parse\s*\(\s*\{/u,
  },
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
      'src/agent-client.ts',
      'src/transport.ts',
      'src/main.ts',
      'src/portal-actions.ts',
      'src/portal-command-controls.ts',
      'src/TrackingStatusRoutePanel.tsx',
      '../../packages/agent-protocol-domain/src/agent-message-codec.ts',
      '../../packages/portal-domain/src/commands.ts',
    ])
  );
  await runCommand('node', ['scripts/check-source-shape.mjs']);

  const scannedFiles = await assertPortalTypedIntentBoundary();

  const proof = {
    schemaVersion: 1,
    proofMode: 'eventing-ui-typed-intent-boundary-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      portalAgentClient: 'apps/portal/src/agent-client.ts',
      protocolAgentMessageCodec: 'packages/agent-protocol-domain/src/agent-message-codec.ts',
      portalTransport: 'apps/portal/src/transport.ts',
      portalActions: 'apps/portal/src/portal-actions.ts',
      portalCommandControls: 'apps/portal/src/portal-command-controls.ts',
      portalCommandInventory: 'packages/portal-domain/src/commands.ts',
      portalTransportTests: 'apps/portal/tests/transport-lan-target.test.ts',
      proofHarness: 'scripts/test/eventing-ui-typed-intent-boundary-proof.mjs',
      scannedSourceRoots: sourceRoots,
      scannedFiles,
    },
    claimsProved: [
      'portal outbound messages are typed AgentCommandEnvelope values built through AgentCommandEnvelopeSchema',
      'portal inbound service messages are parsed as AgentEventEnvelope read models before rendering',
      'portal command controls send AgentCommandName intents and keep AgentEventName values as result-selection metadata',
      'portal source contains no event bus imports, event publish calls, or event subscription ownership',
      'Rust service remains the owner of business event publishing for this boundary',
    ],
    claimsNotProved: [
      'Parent-specific event contracts for rows 42-50',
      'Rust parent/controller validated intent publisher for row 51',
      'child-agent command transport and local publish for rows 53-54',
      'journal-before-action enforcement or adapter-result audit/read-model integration for rows 55-56',
    ],
  };

  const serialized = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, serialized);
  await writeFile(planProofPath, serialized);
  console.log(`eventing-ui-typed-intent-boundary-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
  console.log(`planEvidence=${relative(repoRoot, planProofPath)}`);
}

async function assertPortalTypedIntentBoundary() {
  const agentClient = await readText('apps/portal/src/agent-client.ts');
  const protocolAgentMessageCodec = await readText('packages/agent-protocol-domain/src/agent-message-codec.ts');
  const transport = await readText('apps/portal/src/transport.ts');
  const main = await readText('apps/portal/src/main.ts');
  const portalActions = await readText('apps/portal/src/portal-actions.ts');
  const commandControls = await readText('apps/portal/src/portal-command-controls.ts');
  const trackingPanel = await readText('apps/portal/src/TrackingStatusRoutePanel.tsx');
  const portalCommands = await readText('packages/portal-domain/src/commands.ts');

  assertIncludes(
    agentClient,
    '@ocentra-parent/agent-protocol-domain/agent-message-codec',
    'portal agent client delegates to protocol-domain codec'
  );
  assertIncludes(
    protocolAgentMessageCodec,
    'AgentCommandEnvelopeSchema.parse',
    'agent message codec validates command envelopes'
  );
  assertIncludes(
    protocolAgentMessageCodec,
    'command: AgentCommandName',
    'agent message codec command type is AgentCommandName'
  );
  assertIncludes(
    protocolAgentMessageCodec,
    'serializeAgentCommand(command: AgentCommandEnvelope)',
    'serializer accepts command envelope'
  );
  assertIncludes(
    protocolAgentMessageCodec,
    'JSON.stringify(command)',
    'serializer only serializes validated command envelope'
  );
  assertIncludes(
    protocolAgentMessageCodec,
    'AgentEventEnvelopeSchema.parse(payload)',
    'event parser validates inbound service events'
  );
  proofLabels.push('agent-protocol-domain.codec.command-envelope-only');
  proofLabels.push('agent-protocol-domain.codec.event-readmodel-parse-only');

  const socketSendCount = countMatches(transport, 'socket.send(');
  if (socketSendCount !== 1) {
    throw new Error(`transport socket.send count: expected 1, got ${socketSendCount}`);
  }
  assertIncludes(
    transport,
    '@ocentra-parent/agent-protocol-domain/agent-message-codec',
    'transport consumes protocol-domain codec'
  );
  assertIncludes(
    transport,
    'socket.send(serializeAgentCommand(createAgentCommand(command, payload, target)))',
    'transport sends only serialized typed command envelopes'
  );
  assertIncludes(transport, 'parseAgentEventMessage(message.data)', 'transport parses incoming events');
  assertIncludes(transport, 'sendCommand(', 'transport exposes command intent sender');
  proofLabels.push('portal.transport.serialized-command-only');

  assertIncludes(main, 'sendCommand(command, payload)', 'main delegates only typed commands');
  assertIncludes(portalActions, 'sendCommand(command: AgentCommandName', 'portal actions expose command names only');
  assertIncludes(
    commandControls,
    'actions.selectCommandResult(command.resultEvent)',
    'command controls select result event'
  );
  assertIncludes(
    commandControls,
    'actions.sendCommand(command.command, command.payload)',
    'command controls send commands'
  );
  assertIncludes(
    trackingPanel,
    'actions.sendCommand(AgentCommand.ActivityTrackingReadModelGet, {})',
    'tracking panel sends read-model command'
  );
  proofLabels.push('portal.actions.typed-command-intents');

  assertDoesNotInclude(portalCommands, 'command: AgentEvent.', 'command inventory cannot use events as commands');
  assertIncludes(portalCommands, 'resultEvent: AgentEvent.', 'command inventory keeps events as result metadata');
  proofLabels.push('portal.command-inventory.events-are-result-metadata');

  const scannedFiles = await sourceFiles(sourceRoots);
  for (const file of scannedFiles) {
    const source = await readText(file);
    for (const forbidden of forbiddenPublisherPatterns) {
      if (forbidden.pattern.test(source)) {
        throw new Error(`${forbidden.label}: ${file}`);
      }
    }
  }
  proofLabels.push('portal.source.no-business-event-publisher');
  return scannedFiles;
}

async function sourceFiles(roots) {
  const files = [];
  for (const root of roots) {
    await collectSourceFiles(root, files);
  }
  return files.sort();
}

async function collectSourceFiles(path, files) {
  const entries = await readdir(join(repoRoot, path), { withFileTypes: true });
  for (const entry of entries) {
    const entryPath = `${path}/${entry.name}`;
    if (entry.isDirectory()) {
      await collectSourceFiles(entryPath, files);
      continue;
    }
    if (sourceExtensions.has(extname(entry.name))) {
      files.push(entryPath);
    }
  }
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

function countMatches(text, needle) {
  let count = 0;
  let index = text.indexOf(needle);
  while (index !== -1) {
    count += 1;
    index = text.indexOf(needle, index + needle.length);
  }
  return count;
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
