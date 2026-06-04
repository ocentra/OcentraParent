import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'eventing-network-runtime-proof');
const proofPath = join(outputDir, 'proof.json');
const planOutputDir = join(repoRoot, 'output', 'eventing-plan-proof', '62-network-proof-links');
const planProofPath = join(planOutputDir, 'proof-summary.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(planOutputDir, { recursive: true });

  await runCommand('cargo', ['test', '-p', 'ocentra-eventing']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-core', 'network_event_runtime']);
  await runCommand('node', ['scripts/check-source-shape.mjs']);

  await assertSourceContracts();

  const proof = {
    schemaVersion: 1,
    proofMode: 'eventing-network-runtime-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      eventingCrate: 'crates/ocentra-eventing',
      eventingTests: 'crates/ocentra-eventing/src/tests',
      networkRuntime: 'crates/agent-core/src/network_event_runtime.rs',
      networkRuntimeTests: 'crates/agent-core/src/network_event_runtime_tests.rs',
      networkEventConstants: 'crates/agent-protocol/src/constants/network_flow.rs',
      proofHarness: 'scripts/test/eventing-network-runtime-proof.mjs',
      eventingPlanRow: 'docs/plans/eventing-plan/implementation-checklist.md#row-62',
      networkPlanRow: 'docs/plans/network-plan/implementation-checklist.md#row-10',
      eventingPlanProofSummary: 'output/eventing-plan-proof/62-network-proof-links/proof-summary.json',
    },
    claimsProved: [
      'network runtime consumes the reusable ocentra-eventing crate instead of defining a private network bus',
      'typed live handlers receive EventContext<NetworkRuntimeEventPayload> with EventEnvelope payloads and stored JSON stays at the envelope boundary',
      'network flow events carry custody, source, target handler, aggregate key, idempotency key, and correlation metadata',
      'metadata-only network evidence can progress through AI audit, policy, enforcement dry-run, audit, and portal read-model phases',
      'weak or unavailable network evidence stays manual-required or unavailable and does not execute an adapter action',
    ],
    claimsNotProved: [
      'packet capture, raw PCAP parsing, or analyzer signature parity',
      'decrypted HTTPS payload, exact URL, search query, message, video, or page-content visibility from network metadata',
      'real DNS, firewall, WFP, VPN, nftables, or Network Extension enforcement',
      'broker-backed delivery, durable replay, TTL/retry queueing, or request-response completion',
      'parent portal network UI or product-ready network/domain blocking',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(planProofPath, `${JSON.stringify(planProof(proof), null, 2)}\n`);
  console.log(`eventing-network-runtime-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
  console.log(`planEvidence=${relative(repoRoot, planProofPath)}`);
}

async function assertSourceContracts() {
  const workspaceCargo = await readText('Cargo.toml');
  const agentCoreCargo = await readText('crates/agent-core/Cargo.toml');
  const eventingBusSource = await readText('crates/ocentra-eventing/src/bus.rs');
  const eventingPublisherSource = await readText('crates/ocentra-eventing/src/bus/publisher.rs');
  const eventingEnvelopeSource = await readText('crates/ocentra-eventing/src/envelope.rs');
  const networkSource = await readText('crates/agent-core/src/network_event_runtime.rs');
  const networkTests = await readText('crates/agent-core/src/network_event_runtime_tests.rs');
  const eventingChecklist = await readText('docs/plans/eventing-plan/implementation-checklist.md');

  assertIncludes(workspaceCargo, 'crates/ocentra-eventing', 'workspace includes eventing crate');
  assertIncludes(agentCoreCargo, 'ocentra-eventing', 'agent-core depends on eventing crate');
  assertIncludes(eventingPublisherSource, 'pub struct EventContext<E>', 'typed event context handler boundary');
  assertIncludes(eventingPublisherSource, 'EventEnvelope<E>', 'typed event envelope handler boundary');
  assertIncludes(eventingEnvelopeSource, 'serde_json::Value', 'stored envelope JSON boundary');
  assertIncludes(eventingEnvelopeSource, 'pub payload: StoredEventPayload', 'stored envelope JSON wrapper boundary');
  assertDoesNotInclude(networkSource, 'struct NetworkEventBus', 'no private NetworkEventBus');
  assertDoesNotInclude(networkSource, 'adapter_action_executed: true', 'no adapter action execution');
  assertIncludes(networkTests, 'exact_url_available', 'network tests assert exact URL non-claim');
  assertIncludes(networkTests, 'decrypted_https_payload_available', 'network tests assert HTTPS payload non-claim');
  assertIncludes(
    eventingChecklist,
    'output/eventing-plan-proof/62-network-proof-links/proof-summary.json',
    'eventing checklist row 62 links plan proof'
  );

  proofLabels.push('eventing.workspace.crate-added');
  proofLabels.push('eventing.typed-envelope.boundary');
  proofLabels.push('network.reuses-generic-eventing');
  proofLabels.push('network.metadata-only.no-exact-url');
  proofLabels.push('network.manual-required.no-adapter-action');
  proofLabels.push('eventing.row-62.network-proof-links');
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
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
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

function planProof(proof) {
  return {
    proof: 'eventing-row-62-network-proof-links',
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    commands: proof.commands,
    proofLabels: proof.proofLabels,
    linkedArtifacts: {
      runtimeProof: relative(repoRoot, proofPath),
      eventingPlanProof: relative(repoRoot, planProofPath),
      networkRuntime: proof.evidence.networkRuntime,
      networkRuntimeTests: proof.evidence.networkRuntimeTests,
      networkEventConstants: proof.evidence.networkEventConstants,
      eventingPlanChecklist: 'docs/plans/eventing-plan/implementation-checklist.md',
      networkPlanChecklist: 'docs/plans/network-plan/implementation-checklist.md',
    },
    provenRows: ['62 Network event proof artifacts linked back to eventing plan'],
    linkedPartialRows: [
      '57 Network Workpack 10 consumes reusable crate',
      '58 Network to AI to policy to enforcement event-chain proof',
      '59 Weak-network-evidence cannot publish enforcement command',
      'network-plan row 10 NetworkActivityEvent contracts and reusable Rust eventing consumption',
    ],
    claimsProved: proof.claimsProved,
    claimsNotProved: proof.claimsNotProved,
  };
}
