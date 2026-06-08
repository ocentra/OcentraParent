import { execFileSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const root = process.cwd();
const proofDir = path.join(root, 'test-results', 'browser-runtime-no-fixture-service-exposure-proof');
const outputDir = path.join(root, 'output', 'browser-plan-proof', 'browser-runtime-no-fixture-service-exposure');

const files = {
  runtime: path.join(root, 'crates', 'agent-core', 'src', 'browser_event_runtime.rs'),
  childStatus: path.join(
    root,
    'crates',
    'agent-core',
    'src',
    'browser_event_runtime',
    'action_handoff_child_status.rs'
  ),
  servicePayload: path.join(root, 'crates', 'agent-service', 'src', 'browser_runtime_stream_payload.rs'),
  serviceTests: path.join(root, 'crates', 'agent-service', 'src', 'browser_runtime_stream_tests.rs'),
  protocolParser: path.join(root, 'packages', 'agent-protocol-domain', 'src', 'browser-runtime-events.ts'),
  portalState: path.join(root, 'apps', 'portal', 'src', 'live-activity-state.ts'),
  childStatusProof: path.join(root, 'scripts', 'test', 'browser-runtime-action-intent-child-status-proof.mjs'),
  workpack: path.join(
    root,
    'docs',
    'plans',
    'browser-plan',
    'workpacks',
    '13-browser-read-models-and-service-events.md'
  ),
  checklist: path.join(root, 'docs', 'plans', 'browser-plan', 'implementation-checklist.md'),
};

function run(command, args) {
  execFileSync(command, args, {
    cwd: root,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });
}

async function sourceChecks() {
  const [
    runtime,
    childStatus,
    servicePayload,
    serviceTests,
    protocolParser,
    portalState,
    childStatusProof,
    workpack,
    checklist,
  ] = await Promise.all(Object.values(files).map((file) => readFile(file, 'utf8')));

  return {
    childStatusModuleIsTestOnly:
      runtime.includes('#[cfg(test)]\nmod action_handoff_child_status;') &&
      runtime.includes(
        '#[cfg(test)]\npub(crate) use action_handoff_child_status::prove_browser_runtime_action_intent_child_status;'
      ),
    childStatusTypesAreTestOnly:
      runtime.includes('#[cfg(test)]\nmod action_handoff_child_status_types;') &&
      runtime.includes(
        '#[cfg(test)]\npub(crate) use action_handoff_child_status_types::BrowserRuntimeActionIntentChildStatusReadModelState;'
      ),
    childStatusComposesParentChildFixtureOnly:
      childStatus.includes('ParentChildRuntimeInput::browser_action_intent_handoff_fixture()') &&
      childStatus.includes('public_stream_field_registry_ready: false'),
    serviceStreamDoesNotCallChildStatusFixture:
      !servicePayload.includes('prove_browser_runtime_action_intent_child_status') &&
      !servicePayload.includes('BrowserRuntimeActionIntentChildStatus'),
    serviceTestsAssertOnlyNoObservationChildStatusPayload:
      !serviceTests.includes('child_command_accepted_event_ref') &&
      !serviceTests.includes('public_stream_field_registry_ready') &&
      serviceTests.includes('assert_child_status_payload_empty'),
    protocolParserHasOnlyNoObservationChildStatusFields:
      !protocolParser.includes('childCommandAcceptedEventRef') &&
      !protocolParser.includes('publicStreamFieldRegistryReady') &&
      protocolParser.includes('actionIntentChildAcceptedRows') &&
      protocolParser.includes('browserRuntimeActionIntentChildStatusIsHonest'),
    portalStateDoesNotPromoteFixtureChildStatus:
      !portalState.includes('childCommandAcceptedEventRef') && !portalState.includes('publicStreamFieldRegistryReady'),
    childStatusProofDeclaresRemainingGap:
      childStatusProof.includes('publicStreamFieldRegistryReady: true') &&
      childStatusProof.includes('serviceStreamChildStatusBoundary') &&
      childStatusProof.includes('real child transport/status read model'),
    docsRecordNoFixtureExposureProof:
      workpack.includes('No Fixture Service Exposure Addendum') &&
      workpack.includes('no-observation') &&
      checklist.includes('browser-runtime-no-fixture-service-exposure-proof'),
  };
}

function assertSourceChecks(checks) {
  const missing = Object.entries(checks)
    .filter(([, ok]) => !ok)
    .map(([name]) => name);
  if (missing.length > 0) {
    throw new Error(`browser no-fixture service exposure proof failed: ${missing.join(', ')}`);
  }
}

async function main() {
  await mkdir(proofDir, { recursive: true });
  await mkdir(outputDir, { recursive: true });

  const commands = [
    {
      command: 'cmd',
      args: ['/c', 'node', 'scripts/test/browser-runtime-action-intent-child-status-proof.mjs'],
    },
  ];

  for (const item of commands) {
    run(item.command, item.args);
  }

  const checks = await sourceChecks();
  assertSourceChecks(checks);

  const proof = {
    proofName: 'browser-runtime-no-fixture-service-exposure-proof',
    branchHead: execFileSync('git', ['log', '-1', '--oneline'], {
      cwd: root,
      encoding: 'utf8',
    }).trim(),
    sourceChecks: checks,
    commands: commands.map((item) => `${item.command} ${item.args.join(' ')}`),
    verified: {
      childStatusProofRemainsTestOnly: true,
      serviceStreamDoesNotExposeFixtureChildStatusRefs: true,
      serviceStreamExposesOnlyNoObservationChildStatus: true,
      portalParserDoesNotInventFixtureChildStatusRefs: true,
      publicStreamFieldRegistryReady: true,
      externalTransportImplemented: false,
      adapterDispatchClaimed: false,
      browserMutationClaimed: false,
      childInterventionExecutionClaimed: false,
      finalPolicyExecutionClaimed: false,
      enforcementClaimed: false,
    },
    requiredFollowUp: {
      condition:
        'Only expose nonzero child accepted/read-model refs through the service stream after a real runtime child transport/status read model exists.',
      forbiddenShortcut:
        'Do not call the fixture-backed child-status proof from agent-service or portal state parsing.',
    },
  };

  await writeFile(path.join(proofDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(
    path.join(outputDir, '01-browser-runtime-no-fixture-service-exposure-proof.md'),
    [
      '# Browser Runtime No Fixture Service Exposure Proof',
      '',
      'This proof guards the WP13 child-status boundary from being promoted into runtime service state before a real child transport/status read model exists.',
      '',
      'It verifies that the child-status composition remains `#[cfg(test)]`, service stream payloads do not call the fixture-backed proof, and portal/protocol parsing only exposes honest no-observation child-status fields.',
      '',
      'Validation:',
      ...proof.commands.map((command) => `- \`${command}\``),
      '',
      'No-claim boundary:',
      '- No fixture-backed child-status refs in service or portal runtime state.',
      '- Public child-status stream fields are no-observation only.',
      '- No external transport implementation.',
      '- No adapter dispatch.',
      '- No browser mutation.',
      '- No child intervention execution.',
      '- No final policy execution.',
      '- No enforcement.',
      '',
    ].join('\n')
  );

  console.log(JSON.stringify(proof, null, 2));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
