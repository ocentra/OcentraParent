import { spawn } from 'node:child_process';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { extname, join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'network-portal-source-gate-proof');
const planOutputDir = join(repoRoot, 'output', 'network-plan-proof', '36-portal-source-gate');
const proofPath = join(testOutputDir, 'proof.json');
const planProofPath = join(planOutputDir, 'proof-summary.json');
const sourceRoots = ['apps/portal/src'];
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
      '../../packages/agent-protocol-domain/src/network-flow-read-model.ts',
      'src/live-activity-state.ts',
      'src/NetworkEvidenceDrawerRoutePanel.tsx',
      'src/use-portal-network-activity-refresh.ts',
      'src/portal-route-refresh.ts',
      'src/PolicyPreviewRoutePanel.tsx',
      'src/PortalApp.tsx',
      'tests/live-activity-network-flow.test.ts',
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
      portalNetworkRefreshHook: 'apps/portal/src/use-portal-network-activity-refresh.ts',
      portalRouteRefresh: 'apps/portal/src/portal-route-refresh.ts',
      portalPolicyPreviewRoutePanel: 'apps/portal/src/PolicyPreviewRoutePanel.tsx',
      portalApp: 'apps/portal/src/PortalApp.tsx',
      liveActivityState: 'apps/portal/src/live-activity-state.ts',
      networkFlowReadModelParser: 'packages/agent-protocol-domain/src/network-flow-read-model.ts',
      networkEvidenceDrawerRoutePanel: 'apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx',
      portalNetworkFlowTest: 'apps/portal/tests/live-activity-network-flow.test.ts',
      proofHarness: 'scripts/test/network-portal-source-gate-proof.mjs',
      scannedSourceRoots: sourceRoots,
      scannedFiles,
    },
    claimsProved: [
      'portal network refresh surfaces request service-backed read models only',
      'portal policy-preview route selects the reported result event locally and sends the read-model command only',
      'portal app live-activity state delegates network read-model parsing to the shared portal-domain parser before rendering',
      'network read-model parser validates the service payload and embedded activity digest before app rendering',
      'network evidence drawer route renders service-provided endpoint, domain, process, custody, and evidence refs',
      'network evidence drawer keeps exact URL, policy decision, and intervention facets Not reported when service refs are absent',
      'portal source contains no event bus import, event publish call, event subscription ownership, adapter execution, enforcement dispatch, or local network policy/evidence-grade computation',
      'network evidence drawer is mounted on canonical network product routes only',
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
  const networkRefreshHook = await readText('apps/portal/src/use-portal-network-activity-refresh.ts');
  const routeRefresh = await readText('apps/portal/src/portal-route-refresh.ts');
  const policyPreviewRoutePanel = await readText('apps/portal/src/PolicyPreviewRoutePanel.tsx');
  const portalApp = await readText('apps/portal/src/PortalApp.tsx');
  const liveActivityState = await readText('apps/portal/src/live-activity-state.ts');
  const networkFlowParser = await readText('packages/agent-protocol-domain/src/network-flow-read-model.ts');
  const drawerRoutePanel = await readText('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx');
  const networkFlowTest = await readText('apps/portal/tests/live-activity-network-flow.test.ts');

  assertIncludes(
    networkRefreshHook,
    'actions.sendCommand(AgentCommand.NetworkFlowReadModelGet, {});',
    'network flow route requests the read-model command only'
  );
  assertIncludes(
    policyPreviewRoutePanel,
    'actions.selectCommandResult(AgentEvent.PolicyPreviewReadModelReported);',
    'policy preview route selects the reported result event locally'
  );
  assertIncludes(
    policyPreviewRoutePanel,
    'actions.sendCommand(AgentCommand.PolicyPreviewReadModelGet, {});',
    'policy preview route sends the read-model command only'
  );
  assertIncludes(
    routeRefresh,
    '!hasNetworkFlowReadModelEvent',
    'network refresh waits for a missing service-backed read-model event'
  );
  assertIncludes(
    portalApp,
    'latestPortalEvent(props.state.events, AgentEvent.NetworkFlowReadModelReported) !== null',
    'portal app tracks the latest reported network read-model event'
  );
  proofLabels.push('portal.routes.network-query-only');

  assertIncludes(
    liveActivityState,
    'resolveLiveActivityState as resolvePortalDomainLiveActivityState',
    'live activity delegates to the shared portal-domain parser'
  );
  assertIncludes(
    liveActivityState,
    'return resolvePortalDomainLiveActivityState(events);',
    'live activity returns the shared portal-domain parsing result'
  );
  proofLabels.push('portal.state.delegates-to-shared-parser');

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

  assertIncludes(
    drawerRoutePanel,
    'networkEvidenceDrawerSummary(liveActivity.networkFlowReadModel, {',
    'network route uses the current app-owned drawer surface'
  );
  assertIncludes(
    drawerRoutePanel,
    'PortalDetails.EvidenceReferences',
    'network drawer route renders service evidence refs'
  );
  assertIncludes(drawerRoutePanel, 'PortalDetails.Custody', 'network drawer route renders service custody details');
  proofLabels.push('portal.route.renders-network-surface-details');

  assertIncludes(
    drawerRoutePanel,
    'return isPortalNetworkEvidenceDrawerRoute(route);',
    'network drawer route uses portal-domain route predicate'
  );
  assertIncludes(
    networkFlowTest,
    "expect(summary.exactUrlClaim).toBe('Not reported')",
    'network drawer test proves exact URL stays unsupported without service ref'
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
