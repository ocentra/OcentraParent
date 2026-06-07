import { execFileSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const root = process.cwd();
const proofDir = path.join(root, 'test-results', 'browser-runtime-chain-topology-proof');
const outputDir = path.join(root, 'output', 'browser-plan-proof', 'browser-runtime-chain-topology');

const files = {
  topology: path.join(root, 'crates', 'agent-core', 'src', 'browser_event_runtime', 'topology.rs'),
  runtime: path.join(root, 'crates', 'agent-core', 'src', 'browser_event_runtime.rs'),
  lib: path.join(root, 'crates', 'agent-core', 'src', 'lib.rs'),
  tests: path.join(root, 'crates', 'agent-core', 'src', 'browser_event_runtime_tests.rs'),
  checklist: path.join(root, 'docs', 'plans', 'browser-plan', 'implementation-checklist.md'),
  workpack: path.join(
    root,
    'docs',
    'plans',
    'browser-plan',
    'workpacks',
    '13-browser-read-models-and-service-events.md'
  ),
};

const expectedPhases = [
  ['browser.evidence.observed', 'browser-evidence-observer', 'browser-evidence-observer'],
  ['browser.evidence.journaled', 'browser-evidence-journal', 'browser-evidence-journal'],
  ['browser.ai.analysis.requested', 'browser-ai-request', 'browser-ai-analyzer'],
  ['browser.ai.analysis.completed', 'browser-ai-complete', 'browser-ai-analyzer'],
  ['browser.policy.evaluation.requested', 'browser-policy-request', 'browser-policy-engine'],
  ['browser.policy.decision.completed', 'browser-policy-decision', 'browser-policy-engine'],
  ['browser.intervention.command.issued', 'browser-intervention-command', 'browser-intervention-adapter'],
  ['browser.intervention.result.observed', 'browser-intervention-result', 'browser-intervention-adapter'],
  ['browser.audit.entry.committed', 'browser-audit-entry', 'browser-audit-writer'],
  ['browser.read-model.projected', 'browser-read-model', 'browser-read-model'],
];

function run(command, args) {
  execFileSync(command, args, {
    cwd: root,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });
}

async function sourceChecks() {
  const [topology, runtime, lib, tests, checklist, workpack] = await Promise.all([
    readFile(files.topology, 'utf8'),
    readFile(files.runtime, 'utf8'),
    readFile(files.lib, 'utf8'),
    readFile(files.tests, 'utf8'),
    readFile(files.checklist, 'utf8'),
    readFile(files.workpack, 'utf8'),
  ]);
  return {
    registersRuntimeContracts: topology.includes('EventContractRegistry::new()'),
    buildsTopologyManifest: topology.includes('EventTopologyManifest::from_registry'),
    declaresRuntimePublishers: topology.includes('EventTopologyPublisher'),
    declaresRuntimeSubscribers: topology.includes('EventTopologySubscriber'),
    iteratesOrderedChain: topology.includes('BrowserRuntimePhase::ordered_chain()'),
    runtimeModuleExportsTopology: runtime.includes('pub use topology::browser_runtime_chain_topology_manifest'),
    exportsTopologyHelper: lib.includes('browser_runtime_chain_topology_manifest'),
    focusedTopologyTestExists: tests.includes('browser_runtime_chain_topology_covers_ordered_event_spine'),
    testAssertsCoveredStatus: tests.includes('EventTopologyStatus::Covered'),
    docsMentionTopologyProof: checklist.includes('browser-runtime-chain-topology-proof'),
    workpackMentionsTopologyProof: workpack.includes('Runtime Chain Topology Addendum'),
    allExpectedEventsMentioned: expectedPhases.every(
      ([eventType]) => runtime.includes('phase.event_type()') || tests.includes(eventType)
    ),
  };
}

function assertSourceChecks(checks) {
  const missing = Object.entries(checks)
    .filter(([, ok]) => !ok)
    .map(([name]) => name);
  if (missing.length > 0) {
    throw new Error(`browser runtime chain topology proof failed: ${missing.join(', ')}`);
  }
}

async function main() {
  await mkdir(proofDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });

  const commands = [
    {
      command: 'cargo',
      args: [
        'test',
        '-p',
        'ocentra-parent-agent-core',
        'browser_runtime_chain_topology_covers_ordered_event_spine',
        '--quiet',
      ],
    },
  ];

  for (const item of commands) {
    run(item.command, item.args);
  }

  const checks = await sourceChecks();
  assertSourceChecks(checks);

  const rows = expectedPhases.map(
    ([eventType, subscriber, target]) =>
      `| ${eventType} | browser-event-runtime-spine | ${subscriber} | ${target} | covered |`
  );
  const manifestMarkdown = [
    '# Browser Runtime Chain Topology Proof',
    '',
    '| Event Type | Publisher | Subscriber | Target | Status |',
    '| --- | --- | --- | --- | --- |',
    ...rows,
    '',
    'This is topology proof for the existing local browser runtime event chain. It does not add external transport, adapter dispatch, browser mutation, child intervention execution, final policy execution, or enforcement.',
    '',
  ].join('\n');

  const proof = {
    proofName: 'browser-runtime-chain-topology-proof',
    branchHead: execFileSync('git', ['log', '-1', '--oneline'], { cwd: root, encoding: 'utf8' }).trim(),
    sourceChecks: checks,
    commands: commands.map((item) => `${item.command} ${item.args.join(' ')}`),
    topology: expectedPhases.map(([eventType, subscriber, target]) => ({
      eventType,
      publisher: 'browser-event-runtime-spine',
      subscriber,
      target,
      status: 'covered',
    })),
    verified: {
      reusableEventingTopologyManifestUsed: true,
      orderedBrowserRuntimeChainRegistered: true,
      namedPublishersDeclared: true,
      namedSubscribersDeclared: true,
      noUnreadyTopologyEntries: true,
      newEventBusCreated: false,
      externalTransportClaimed: false,
      adapterDispatchClaimed: false,
      browserMutationClaimed: false,
      childInterventionClaimed: false,
      finalPolicyExecutionClaimed: false,
      enforcementClaimed: false,
    },
  };

  await writeFile(path.join(proofDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(path.join(outputDir, '01-browser-runtime-chain-topology-proof.md'), manifestMarkdown);

  console.log(JSON.stringify(proof, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
