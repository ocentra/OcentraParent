import { spawn } from 'node:child_process';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { extname, join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'network-portal-source-gate-proof');
const planOutputDir = join(repoRoot, 'output', 'network-plan-proof', '36-portal-source-gate');
const proofPath = join(testOutputDir, 'proof.json');
const planProofPath = join(planOutputDir, 'proof-summary.json');
const sourceRoots = ['apps/portal/src', 'packages/portal-domain/src'];
const sourceExtensions = new Set(['.ts', '.tsx']);
const commands = [];
const proofLabels = [];

const forbiddenSourcePatterns = [
  {
    label: 'no Rust or private event bus ownership in portal source',
    pattern: /(?:ocentra-eventing|NetworkEventBus|EventContext<|EventPublisher|createEventPublisher)/u,
  },
  {
    label: 'no portal business event publish function',
    pattern: /(?:^|[^\w])(?:publishEvent|publishBusinessEvent|publishDomainEvent|publishNetworkEvent)\s*\(/u,
  },
  {
    label: 'no portal event bus publish call',
    pattern: /(?:eventBus|networkBus|bus|publisher)\.publish\s*\(/u,
  },
  {
    label: 'no portal event subscription ownership',
    pattern: /(?:eventBus|networkBus|bus)\.subscribe\s*\(/u,
  },
  {
    label: 'no portal-owned network policy evaluator',
    pattern: /(?:evaluateNetworkPolicy|decideNetworkPolicy|computeNetworkPolicy|authorizeNetworkAdapter)\s*\(/u,
  },
  {
    label: 'no portal-owned adapter or enforcement execution',
    pattern: /(?:executeNetworkAdapter|applyNetworkAdapter|publishEnforcementCommand|dispatchEnforcement)\s*\(/u,
  },
  {
    label: 'no portal-owned network evidence grade computation',
    pattern: /(?:computeNetworkEvidenceGrade|ActivityNetworkEvidenceGradeSchema|NetworkEvidenceGradeSchema)/u,
  },
];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(planOutputDir, { recursive: true });

  await runCommand(...npmCommand(['--workspace', '@ocentra-parent/agent-protocol-domain', 'run', 'build']));
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/live-activity-network-flow.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'eslint',
      '../../packages/portal-domain/src/live-activity-state.ts',
      '../../packages/portal-domain/src/network-evidence-drawer.ts',
      '../../packages/agent-protocol-domain/src/network-flow-read-model.ts',
      'src/NetworkEvidenceDrawerRoutePanel.tsx',
      'tests/live-activity-network-flow.test.ts',
      '../../packages/portal-domain/src/commands.ts',
    ])
  );
  await runCommand('node', ['scripts/check-source-shape.mjs']);

  const scannedFiles = await assertNetworkPortalSourceGate();
  const proof = {
    schemaVersion: 1,
    proofMode: 'network-portal-source-gate-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      portalCommandInventory: 'packages/portal-domain/src/commands.ts',
      liveActivityState: 'packages/portal-domain/src/live-activity-state.ts',
      networkFlowReadModelParser: 'packages/agent-protocol-domain/src/network-flow-read-model.ts',
      networkEvidenceDrawer: 'packages/portal-domain/src/network-evidence-drawer.ts',
      networkEvidenceDrawerRoutePanel: 'apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx',
      portalNetworkFlowTest: 'apps/portal/tests/live-activity-network-flow.test.ts',
      proofHarness: 'scripts/test/network-portal-source-gate-proof.mjs',
      scannedSourceRoots: sourceRoots,
      scannedFiles,
    },
    claimsProved: [
      'portal network commands request service-backed read models, status, and stream views only',
      'portal command inventory uses AgentEvent values only as result-event metadata, not outbound commands',
      'portal parses agent.network.flow.read-model.reported through ActivityNetworkFlowReadModelSchema before rendering',
      'network evidence drawer renders service-provided endpoint, domain, process, custody, and evidence refs',
      'network evidence drawer leaves exact URL, AI audit, policy decision, risk budget, intervention, and evidence grade facets as Not reported when service refs are absent',
      'network evidence drawer renders service-backed retention state when the read model provides it',
      'portal source contains no event bus import, event publish call, event subscription ownership, adapter execution, enforcement dispatch, or local network policy/evidence-grade computation',
      'network evidence drawer is mounted on the Activity product route only',
    ],
    claimsNotProved: [
      'live packet capture, raw PCAP custody, quota rotation, deletion, or export',
      'broker or family-hub network event-chain delivery',
      'local AI model execution or portal AI audit rendering',
      'policy engine execution, rule authoring, or final policy authority',
      'adapter execution, host DNS mutation, firewall mutation, packet blocking, or enforcement command publication',
      'new portal UI behavior beyond the existing service-backed network evidence drawer',
    ],
  };

  const serialized = `${JSON.stringify(proof, null, 2)}\n`;
  await writeFile(proofPath, serialized);
  await writeFile(planProofPath, serialized);
  console.log(`network-portal-source-gate-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
  console.log(`planEvidence=${relative(repoRoot, planProofPath)}`);
}

async function assertNetworkPortalSourceGate() {
  const portalCommands = await readText('packages/portal-domain/src/commands.ts');
  const liveActivityState = await readText('packages/portal-domain/src/live-activity-state.ts');
  const networkFlowParser = await readText('packages/agent-protocol-domain/src/network-flow-read-model.ts');
  const evidenceDrawer = await readText('packages/portal-domain/src/network-evidence-drawer.ts');
  const drawerRoutePanel = await readText('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx');
  const networkFlowTest = await readText('apps/portal/tests/live-activity-network-flow.test.ts');

  assertIncludes(portalCommands, 'command: AgentCommand.NetworkFlowReadModelGet', 'network flow query command exists');
  assertIncludes(
    portalCommands,
    'resultEvent: AgentEvent.NetworkFlowReadModelReported',
    'network flow result event is result metadata'
  );
  assertIncludes(
    portalCommands,
    'command: AgentCommand.PolicyPreviewReadModelGet',
    'policy preview query remains a read-model command'
  );
  assertDoesNotInclude(portalCommands, 'command: AgentEvent.', 'portal cannot send events as commands');
  assertPatternAbsent(
    portalCommands,
    /command:\s*AgentCommand\.Network\w*(?:Execute|Apply|Block|Enforce|Authorize|Dispatch|Mutate|Delete|Write|Set)\w*/u,
    'portal command inventory has no network mutation or enforcement command'
  );
  proofLabels.push('portal.commands.network-query-only');

  assertIncludes(
    liveActivityState,
    'const latestNetworkFlow = latestParsedNetworkFlowEvent(events, false)',
    'live activity selects latest parsed service network read-model event'
  );
  assertIncludes(
    liveActivityState,
    'const readModel = parseNetworkFlowReadModel(event.payload)',
    'live activity parses network event payload before rendering'
  );
  assertIncludes(
    liveActivityState,
    'networkFlowReadModel: networkFlowState.readModel',
    'live activity exposes parsed network read model to rendering'
  );
  proofLabels.push('portal.state.service-event-to-schema-parser');

  assertIncludes(
    networkFlowParser,
    'ActivityNetworkFlowReadModelSchema.safeParse',
    'network read-model parser validates full read-model'
  );
  assertIncludes(
    networkFlowParser,
    'ActivityNetworkFlowDigestSchema.safeParse',
    'network read-model parser validates embedded activity digest'
  );
  assertIncludes(
    networkFlowParser,
    'AgentProtocolDefaults.Field.ActivityDigest',
    'network read-model parser consumes service digest field'
  );
  proofLabels.push('portal.network-flow.schema-validated');

  for (const expectedUnsupportedFacet of [
    'browserRef: notReported()',
    'analyzerAlertRef: notReported()',
    'detectionResultRef: notReported()',
    'aiAuditRef: notReported()',
    'riskBudgetRef: notReported()',
    'policyDecisionRef: notReported()',
    'interventionResultRef: notReported()',
    'evidenceGrade: notReported()',
    'confidence: notReported()',
    'exactUrlClaim: notReported()',
  ]) {
    assertIncludes(
      evidenceDrawer,
      expectedUnsupportedFacet,
      `unsupported facet stays not reported: ${expectedUnsupportedFacet}`
    );
  }
  assertIncludes(evidenceDrawer, 'retentionState: retentionState(readModel)', 'drawer renders service retention state');
  assertIncludes(evidenceDrawer, 'evidenceReferenceDetail(row)', 'drawer renders service evidence refs');
  assertIncludes(
    drawerRoutePanel,
    'networkEvidenceDrawerSummary(liveActivity.networkFlowReadModel)',
    'network route uses drawer summary'
  );
  proofLabels.push('portal.drawer.unsupported-claims-not-reported');

  assertIncludes(
    drawerRoutePanel,
    'return isPortalNetworkEvidenceDrawerRoute(route);',
    'network drawer route uses portal-domain route predicate'
  );
  assertIncludes(
    networkFlowTest,
    "expect(summary.policyDecisionRef).toBe('Not reported')",
    'network drawer test proves policy facet is unsupported without service ref'
  );
  assertIncludes(
    networkFlowTest,
    "expect(summary.interventionResultRef).toBe('Not reported')",
    'network drawer test proves intervention facet is unsupported without service ref'
  );
  proofLabels.push('portal.route-and-test.network-drawer-boundary');

  const scannedFiles = await sourceFiles(sourceRoots);
  for (const file of scannedFiles) {
    const source = await readText(file);
    for (const forbidden of forbiddenSourcePatterns) {
      assertPatternAbsent(source, forbidden.pattern, `${forbidden.label}: ${file}`);
    }
  }
  proofLabels.push('portal.source.no-network-business-authority');
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

function assertPatternAbsent(text, pattern, label) {
  if (pattern.test(text)) {
    throw new Error(`${label}: matched ${pattern}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
