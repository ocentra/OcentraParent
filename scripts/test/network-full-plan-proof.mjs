import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';

const proofRoot = 'output/network-plan-proof/full-network-plan';
const testRoot = 'test-results/network-full-plan-proof';
const commandLogRoot = `${proofRoot}/command-logs`;

mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });
mkdirSync(commandLogRoot, { recursive: true });

const freshnessCommandSpecs = [
  ['network-contract-boundary-proof', 'node', ['scripts/test/network-contract-boundary-proof.mjs']],
  ['eventing-full-plan-proof', 'node', ['scripts/test/eventing-full-plan-proof.mjs']],
  ['network-pcap-replay-proof', 'node', ['scripts/test/network-pcap-replay-proof.mjs']],
  ['network-packet-dns-parser-proof', 'node', ['scripts/test/network-packet-dns-parser-proof.mjs']],
  ['network-visibility-parser-proof', 'node', ['scripts/test/network-visibility-parser-proof.mjs']],
  ['network-live-capture-proof-gate', 'node', ['scripts/test/network-live-capture-proof-gate.mjs']],
  ['network-live-capture-storage-proof', 'node', ['scripts/test/network-live-capture-storage-proof.mjs']],
  ['network-zeek-analyzer-comparison-proof', 'node', ['scripts/test/network-zeek-analyzer-comparison-proof.mjs']],
  ['network-signature-alert-ingestion-proof', 'node', ['scripts/test/network-signature-alert-ingestion-proof.mjs']],
  ['network-ai-detection-fixture-proof', 'node', ['scripts/test/network-ai-detection-fixture-proof.mjs']],
  ['network-ai-audit-narrative-proof', 'node', ['scripts/test/network-ai-audit-narrative-proof.mjs']],
  ['network-risk-budget-threshold-proof', 'node', ['scripts/test/network-risk-budget-threshold-proof.mjs']],
  ['network-evidence-policy-mapping-proof', 'node', ['scripts/test/network-evidence-policy-mapping-proof.mjs']],
  [
    'network-policy-preview-stored-flow-evidence-proof',
    'node',
    ['scripts/test/network-policy-preview-stored-flow-evidence-proof.mjs'],
  ],
  ['network-dns-adapter-proof', 'node', ['scripts/test/network-dns-adapter-proof.mjs']],
  ['network-windows-firewall-adapter-proof', 'node', ['scripts/test/network-windows-firewall-adapter-proof.mjs']],
  ['network-manual-platform-proof', 'node', ['scripts/test/network-manual-platform-proof.mjs']],
  ['network-platform-claims-proof', 'node', ['scripts/test/network-platform-claims-proof.mjs']],
  ['network-adapter-capability-status-proof', 'node', ['scripts/test/network-adapter-capability-status-proof.mjs']],
  ['network-action-result-state-proof', 'node', ['scripts/test/network-action-result-state-proof.mjs']],
  [
    'network-remote-delivery-event-chain-journal-proof',
    'node',
    ['scripts/test/network-remote-delivery-event-chain-journal-proof.mjs'],
  ],
  [
    'network-remote-delivery-receipt-ledger-proof',
    'node',
    ['scripts/test/network-remote-delivery-receipt-ledger-proof.mjs'],
  ],
  [
    'network-remote-delivery-durable-envelope-proof',
    'node',
    ['scripts/test/network-remote-delivery-durable-envelope-proof.mjs'],
  ],
  [
    'network-remote-delivery-outbox-handoff-proof',
    'node',
    ['scripts/test/network-remote-delivery-outbox-handoff-proof.mjs'],
  ],
  [
    'network-remote-delivery-outbox-status-bridge-proof',
    'node',
    ['scripts/test/network-remote-delivery-outbox-status-bridge-proof.mjs'],
  ],
  [
    'network-remote-delivery-dispatch-readiness-proof',
    'node',
    ['scripts/test/network-remote-delivery-dispatch-readiness-proof.mjs'],
  ],
  [
    'network-remote-delivery-no-enforcement-invariant-proof',
    'node',
    ['scripts/test/network-remote-delivery-no-enforcement-invariant-proof.mjs'],
  ],
  [
    'network-remote-delivery-transport-dispatch-state-proof',
    'node',
    ['scripts/test/network-remote-delivery-transport-dispatch-state-proof.mjs'],
  ],
  [
    'network-remote-delivery-fixture-transport-proof',
    'node',
    ['scripts/test/network-remote-delivery-fixture-transport-proof.mjs'],
  ],
  [
    'network-remote-delivery-delete-export-propagation-proof',
    'node',
    ['scripts/test/network-remote-delivery-delete-export-propagation-proof.mjs'],
  ],
  [
    'network-remote-delivery-delete-export-status-bridge-proof',
    'node',
    ['scripts/test/network-remote-delivery-delete-export-status-bridge-proof.mjs'],
  ],
  ['network-parent-ui-evidence-drawer-proof', 'node', ['scripts/test/network-parent-ui-evidence-drawer-proof.mjs']],
  [
    'network-portal-risk-performance-platform-proof',
    'node',
    ['scripts/test/network-portal-risk-performance-platform-proof.mjs'],
  ],
  ['network-security-readiness-proof', 'node', ['scripts/test/network-security-readiness-proof.mjs']],
  ['network-performance-benchmark-proof', 'node', ['scripts/test/network-performance-benchmark-proof.mjs']],
  ['network-broker-delivery-proof', 'node', ['scripts/test/network-broker-delivery-proof.mjs']],
  [
    'network-broker-family-hub-delivery-status-proof',
    'node',
    ['scripts/test/network-broker-family-hub-delivery-status-proof.mjs'],
  ],
  ['network-vpn-proxy-tunnel-classifier-proof', 'node', ['scripts/test/network-vpn-proxy-tunnel-classifier-proof.mjs']],
  [
    'network-remote-torrent-download-classifier-proof',
    'node',
    ['scripts/test/network-remote-torrent-download-classifier-proof.mjs'],
  ],
  ['network-end-to-end-pipeline-proof', 'node', ['scripts/test/network-end-to-end-pipeline-proof.mjs']],
  [
    'agent-service-clippy',
    'cargo',
    ['clippy', '-p', 'ocentra-parent-agent-service', '--all-targets', '--', '-D', 'warnings'],
  ],
];

const refreshedProofByName = new Map([
  ['network-contract-boundary', 'network-contract-boundary-proof'],
  ['eventing-full-plan', 'eventing-full-plan-proof'],
  ['network-pcap-replay', 'network-pcap-replay-proof'],
  ['network-packet-dns-parser', 'network-packet-dns-parser-proof'],
  ['network-visibility-parser', 'network-visibility-parser-proof'],
  ['network-live-capture-proof-gate', 'network-live-capture-proof-gate'],
  ['network-live-capture-storage', 'network-live-capture-storage-proof'],
  ['network-zeek-analyzer-comparison', 'network-zeek-analyzer-comparison-proof'],
  ['network-signature-alert-ingestion', 'network-signature-alert-ingestion-proof'],
  ['network-ai-detection-fixture-proof', 'network-ai-detection-fixture-proof'],
  ['network-ai-audit-narrative-proof', 'network-ai-audit-narrative-proof'],
  ['network-risk-budget-threshold-proof', 'network-risk-budget-threshold-proof'],
  ['network-evidence-policy-mapping', 'network-evidence-policy-mapping-proof'],
  ['network-policy-preview-stored-flow-evidence-proof', 'network-policy-preview-stored-flow-evidence-proof'],
  ['network-dns-adapter', 'network-dns-adapter-proof'],
  ['network-windows-firewall-adapter', 'network-windows-firewall-adapter-proof'],
  ['network-windows-wfp-proof-gate', 'network-manual-platform-proof'],
  ['network-android-vpnservice-proof-gate', 'network-manual-platform-proof'],
  ['network-apple-network-extension-proof-gate', 'network-manual-platform-proof'],
  ['network-linux-adapter-proof-gate', 'network-manual-platform-proof'],
  ['network-platform-claims', 'network-platform-claims-proof'],
  ['network-adapter-capability-status', 'network-adapter-capability-status-proof'],
  ['network-action-result-state-proof', 'network-action-result-state-proof'],
  ['network-remote-delivery-event-chain-journal-proof', 'network-remote-delivery-event-chain-journal-proof'],
  ['network-remote-delivery-receipt-ledger-proof', 'network-remote-delivery-receipt-ledger-proof'],
  ['network-remote-delivery-durable-envelope-proof', 'network-remote-delivery-durable-envelope-proof'],
  ['network-remote-delivery-outbox-handoff-proof', 'network-remote-delivery-outbox-handoff-proof'],
  ['network-remote-delivery-outbox-status-bridge-proof', 'network-remote-delivery-outbox-status-bridge-proof'],
  ['network-remote-delivery-dispatch-readiness-proof', 'network-remote-delivery-dispatch-readiness-proof'],
  ['network-remote-delivery-no-enforcement-invariant-proof', 'network-remote-delivery-no-enforcement-invariant-proof'],
  ['network-remote-delivery-transport-dispatch-state-proof', 'network-remote-delivery-transport-dispatch-state-proof'],
  ['network-remote-delivery-fixture-transport-proof', 'network-remote-delivery-fixture-transport-proof'],
  [
    'network-remote-delivery-delete-export-propagation-proof',
    'network-remote-delivery-delete-export-propagation-proof',
  ],
  [
    'network-remote-delivery-delete-export-status-bridge-proof',
    'network-remote-delivery-delete-export-status-bridge-proof',
  ],
  ['network-portal-risk-performance-platform-proof', 'network-portal-risk-performance-platform-proof'],
  ['network-security-readiness-proof', 'network-security-readiness-proof'],
  ['network-performance-benchmark-proof', 'network-performance-benchmark-proof'],
  ['network-vpn-proxy-tor-tunnel-classifier', 'network-vpn-proxy-tunnel-classifier-proof'],
  ['network-remote-torrent-download-classifier', 'network-remote-torrent-download-classifier-proof'],
  ['network-end-to-end-pipeline', 'network-end-to-end-pipeline-proof'],
]);

const refreshedArtifactById = new Map([
  ['packet-parser', 'network-packet-dns-parser-proof'],
  ['tls-sni-parser', 'network-visibility-parser-proof'],
  ['http-host-parser', 'network-visibility-parser-proof'],
  ['quic-visibility', 'network-visibility-parser-proof'],
  ['portal-drawer', 'network-parent-ui-evidence-drawer-proof'],
  ['portal-drawer-screenshot', 'network-parent-ui-evidence-drawer-proof'],
  ['broker-delivery', 'network-broker-delivery-proof'],
  ['remote-delivery-status', 'network-broker-family-hub-delivery-status-proof'],
]);

const proofArtifacts = [
  [
    'contract-boundary',
    '01-contract-proof',
    'output/network-plan-proof/03-contract-boundary-and-effect-schemas/proof-summary.json',
  ],
  ['eventing-full-plan', '02-eventing-proof', 'output/eventing-plan-proof/full-eventing-plan/proof-summary.json'],
  [
    'pcap-replay',
    '03-parser-fixture-proof',
    'output/network-plan-proof/12-pcap-file-replay-harness/proof-summary.json',
  ],
  [
    'packet-parser',
    '03-parser-fixture-proof',
    'output/network-plan-proof/14-packet-parser/expected-packet-fixtures.json',
  ],
  [
    'dns-parser',
    '03-parser-fixture-proof',
    'output/network-plan-proof/15-dns-query-response-parser/proof-summary.json',
  ],
  [
    'tls-sni-parser',
    '03-parser-fixture-proof',
    'output/network-plan-proof/16-tls-clienthello-sni-parser/expected-sni-visibility.json',
  ],
  [
    'http-host-parser',
    '03-parser-fixture-proof',
    'output/network-plan-proof/17-http-host-parser/expected-http-host.json',
  ],
  [
    'quic-visibility',
    '03-parser-fixture-proof',
    'output/network-plan-proof/18-quic-http3-limitation-detector/expected-quic-limitation.json',
  ],
  ['encrypted-dns', '03-parser-fixture-proof', 'output/network-plan-proof/19-doh-dot-detector/proof-summary.json'],
  [
    'live-capture-gate',
    '03-parser-fixture-proof',
    'output/network-plan-proof/13-live-capture-proof-gate/proof-summary.json',
  ],
  [
    'live-capture-storage',
    '03-parser-fixture-proof',
    'output/network-plan-proof/03a-live-capture-storage-proof/proof-summary.json',
  ],
  [
    'zeek-analyzer',
    '04-analyzer-alert-proof',
    'output/network-plan-proof/43-zeek-structured-log-analyzer-comparison/proof-summary.json',
  ],
  [
    'signature-alerts',
    '04-analyzer-alert-proof',
    'output/network-plan-proof/44-signature-alert-ingestion-proof/proof-summary.json',
  ],
  ['ai-detection', '05-ai-policy-proof', 'output/network-plan-proof/46-ai-detection-fixture-proof/proof-summary.json'],
  ['ai-audit', '05-ai-policy-proof', 'output/network-plan-proof/47-ai-audit-narrative-proof/proof-summary.json'],
  ['risk-budget', '05-ai-policy-proof', 'output/network-plan-proof/48-risk-budget-threshold-proof/proof-summary.json'],
  [
    'policy-mapping',
    '05-ai-policy-proof',
    'output/network-plan-proof/34-evidence-grade-policy-mapping/proof-summary.json',
  ],
  [
    'policy-preview',
    '05-ai-policy-proof',
    'output/network-plan-proof/policy-preview-stored-flow-evidence/proof-summary.json',
  ],
  [
    'dns-adapter',
    '06-adapter-action-proof',
    'output/network-plan-proof/37-dns-proxy-block-redirect-adapter/proof-summary.json',
  ],
  [
    'windows-firewall-adapter',
    '06-adapter-action-proof',
    'output/network-plan-proof/38-windows-firewall-adapter/proof-summary.json',
  ],
  [
    'windows-wfp-gate',
    '06-adapter-action-proof',
    'output/network-plan-proof/39-windows-wfp-proof-gate/proof-summary.json',
  ],
  [
    'android-vpnservice-gate',
    '06-adapter-action-proof',
    'output/network-plan-proof/40-android-vpnservice-proof-gate/proof-summary.json',
  ],
  [
    'apple-network-extension-gate',
    '06-adapter-action-proof',
    'output/network-plan-proof/41-apple-network-extension-proof-gate/proof-summary.json',
  ],
  [
    'linux-adapter-gate',
    '06-adapter-action-proof',
    'output/network-plan-proof/42-linux-adapter-proof-gate/proof-summary.json',
  ],
  [
    'platform-claims',
    '06-adapter-action-proof',
    'output/network-plan-proof/52-platform-claims-proof/proof-summary.json',
  ],
  [
    'adapter-capability-status',
    '06-adapter-action-proof',
    'output/network-plan-proof/adapter-capability-status/proof-summary.json',
  ],
  [
    'action-result-state',
    '06-adapter-action-proof',
    'output/network-plan-proof/53-action-result-state-proof/proof-summary.json',
  ],
  [
    'event-chain-journal',
    '07-journal-sqlite-proof',
    'output/network-plan-proof/10c-remote-delivery-event-chain-journal-status/proof-summary.json',
  ],
  [
    'receipt-ledger',
    '07-journal-sqlite-proof',
    'output/network-plan-proof/10d-remote-delivery-receipt-ledger-status/proof-summary.json',
  ],
  [
    'durable-envelope',
    '07-journal-sqlite-proof',
    'output/network-plan-proof/10e-remote-delivery-durable-envelope-status/proof-summary.json',
  ],
  [
    'stored-flow-policy-preview',
    '07-journal-sqlite-proof',
    'output/network-plan-proof/policy-preview-stored-flow-evidence/proof-summary.json',
  ],
  [
    'portal-drawer',
    '08-ui-snapshots',
    'output/network-plan-proof/36-parent-ui-network-evidence-drawer/proof-summary.json',
  ],
  [
    'portal-drawer-screenshot',
    '08-ui-snapshots',
    'output/network-plan-proof/36-parent-ui-network-evidence-drawer/08-ui-snapshots/network-evidence-drawer.png',
  ],
  [
    'portal-risk-platform-status',
    '08-ui-snapshots',
    'output/network-plan-proof/portal-risk-performance-platform-status/proof-summary.json',
  ],
  [
    'security-readiness',
    '09-security-negative-proof',
    'output/network-plan-proof/50-security-readiness-proof/proof-summary.json',
  ],
  [
    'performance-benchmark',
    '10-performance-proof',
    'output/network-plan-proof/49-performance-benchmark-proof/proof-summary.json',
  ],
  [
    'broker-delivery',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10a-broker-delivery-proof/10a-broker-delivery-proof.log',
  ],
  [
    'remote-delivery-status',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10b-broker-family-hub-delivery-status/10b-remote-delivery-status.log',
  ],
  [
    'remote-outbox-handoff',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10g-remote-delivery-outbox-handoff/proof-summary.json',
  ],
  [
    'remote-outbox-status-bridge',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10h-remote-delivery-outbox-status-bridge/proof-summary.json',
  ],
  [
    'remote-dispatch-readiness',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10i-remote-delivery-dispatch-readiness/proof-summary.json',
  ],
  [
    'remote-no-enforcement',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10j-remote-delivery-no-enforcement-invariant/proof-summary.json',
  ],
  [
    'remote-transport-state',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/proof-summary.json',
  ],
  [
    'remote-fixture-transport',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10l-remote-delivery-fixture-transport/proof-summary.json',
  ],
  [
    'remote-delete-export-readiness',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10m-remote-delivery-delete-export-propagation/proof-summary.json',
  ],
  [
    'remote-delete-export-status-bridge',
    '10-remote-delivery-proof',
    'output/network-plan-proof/10n-remote-delivery-delete-export-status-bridge/proof-summary.json',
  ],
  [
    'vpn-proxy-tunnel',
    'classification-proof',
    'output/network-plan-proof/24-vpn-proxy-tor-tunnel-classifier/proof-summary.json',
  ],
  [
    'remote-torrent-download',
    'classification-proof',
    'output/network-plan-proof/25-remote-torrent-download-classifier/proof-summary.json',
  ],
  [
    'end-to-end-pipeline',
    'product-path-proof',
    'output/network-plan-proof/51-end-to-end-pipeline-proof/proof-summary.json',
  ],
];

const commandSpecs = [
  ['network-evidence-tests', 'cargo', ['test', '-p', 'ocentra-network-evidence']],
  ['agent-core-network-runtime-tests', 'cargo', ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime']],
  ['adapter-proof-script-check', 'node', ['--check', 'scripts/test/network-adapter-capability-status-proof.mjs']],
  ['no-test-doubles', 'node', ['scripts/check-no-test-doubles.mjs']],
  ['source-shape', 'node', ['scripts/check-source-shape.mjs']],
  ['git-diff-check', 'git', ['diff', '--check']],
];

const sideEffectSnapshot = snapshotTrackedProofSideEffects();
let refreshCommands = [];
try {
  refreshCommands = freshnessCommandSpecs.map(runCommand);
} finally {
  restoreTrackedProofSideEffects(sideEffectSnapshot);
}
const artifacts = proofArtifacts.map(readArtifact);
const commands = [...refreshCommands, ...commandSpecs.map(runCommand)];
const groupedArtifacts = groupArtifacts(artifacts);
const freshnessAudit = summarizeFreshness(artifacts);

writeFileSync(`${proofRoot}/00-source-snapshot.md`, sourceSnapshot());
writeGroupedLog('01-contract-proof.log', groupedArtifacts['01-contract-proof'], commands);
writeGroupedLog('02-eventing-proof.log', groupedArtifacts['02-eventing-proof'], commands);
writeJsonArtifact('03-parser-fixture-proof.json', groupedArtifacts['03-parser-fixture-proof']);
writeJsonArtifact('04-analyzer-alert-proof.json', groupedArtifacts['04-analyzer-alert-proof']);
writeJsonArtifact('05-ai-policy-proof.json', groupedArtifacts['05-ai-policy-proof']);
writeJsonArtifact('06-adapter-action-proof.json', groupedArtifacts['06-adapter-action-proof']);
writeJsonArtifact('07-journal-sqlite-proof.json', groupedArtifacts['07-journal-sqlite-proof']);
writeFileSync(`${proofRoot}/08-ui-snapshots.md`, uiSnapshotSummary(groupedArtifacts['08-ui-snapshots']));
writeGroupedLog('09-security-negative-proof.log', groupedArtifacts['09-security-negative-proof'], commands);
writeGroupedLog('10-performance-proof.log', groupedArtifacts['10-performance-proof'], commands);
writeJsonArtifact('11-classification-proof.json', groupedArtifacts['classification-proof']);
writeJsonArtifact('12-remote-delivery-proof.json', groupedArtifacts['10-remote-delivery-proof']);
writeJsonArtifact('13-product-path-proof.json', groupedArtifacts['product-path-proof']);

const proof = {
  schemaVersion: 1,
  proof: 'network-full-plan',
  proofRoot,
  testRoot,
  commands,
  freshnessAudit,
  artifactCount: artifacts.length,
  artifacts,
  requiredProofPack: [
    `${proofRoot}/00-source-snapshot.md`,
    `${proofRoot}/01-contract-proof.log`,
    `${proofRoot}/02-eventing-proof.log`,
    `${proofRoot}/03-parser-fixture-proof.json`,
    `${proofRoot}/04-analyzer-alert-proof.json`,
    `${proofRoot}/05-ai-policy-proof.json`,
    `${proofRoot}/06-adapter-action-proof.json`,
    `${proofRoot}/07-journal-sqlite-proof.json`,
    `${proofRoot}/08-ui-snapshots.md`,
    `${proofRoot}/09-security-negative-proof.log`,
    `${proofRoot}/10-performance-proof.log`,
    `${proofRoot}/11-classification-proof.json`,
    `${proofRoot}/12-remote-delivery-proof.json`,
    `${proofRoot}/13-product-path-proof.json`,
  ],
  provenPlanGates: [
    'metadata-only network evidence and no exact URL/content from network-only evidence',
    'reusable Rust eventing dependency for network runtime routing without a network-only bus',
    'PCAP/parser/analyzer/classifier fixture proof with unsupported-claim records',
    'AI detection/audit and risk-budget outputs remain advisory until typed policy handoff',
    'adapter action is gated by policy, capability proof, result state, rollback/unavailable refs, and audit refs',
    'journal, replay, receipt-ledger, durable envelope, outbox, and local read-model proof are present',
    'remote fixture transport, delete/export readiness, and delete/export status bridge proof are present without live propagation',
    'portal evidence drawer, risk/performance/platform status, and row51 stored-flow product-path bridge are service/read-model backed and non-authoritative',
    'security readiness and performance benchmark proof roots are present',
  ],
  manualRequiredBoundaries: [
    'live packet capture driver invocation remains proof-gated/manual-required',
    'broker/family-hub remote delivery, provider delivery, and child-device delivery remain manual-required',
    'remote delete/export propagation remains proof-local readiness/status and future transport work',
    'production platform support and external audit/penetration-test signoff remain unclaimed',
  ],
  notClaimed: [
    'decrypted payload capture',
    'exact URL, page content, video content, private message content, or search query from network-only evidence',
    'raw PCAP upload to AI or cloud by default',
    'AI direct policy authority',
    'portal/UI policy or adapter authority',
    'live adapter execution, host DNS/firewall/VPN/WFP/NetworkExtension/Linux mutation, or host filtering',
    'enforcement command publication from network proof paths',
  ],
};

writeFileSync(`${proofRoot}/proof-summary.json`, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(`${testRoot}/proof.json`, `${JSON.stringify(proof, null, 2)}\n`);

console.log(`network-full-plan-proof-ok:artifacts=${artifacts.length},commands=${commands.length}`);
console.log(`proof=${proofRoot}/proof-summary.json`);

function readArtifact([id, group, path]) {
  if (!existsSync(path)) {
    throw new Error(`required artifact missing: ${path}`);
  }
  const text = readFileSync(path, isBinary(path) ? undefined : 'utf8');
  const parsed = isJson(path) ? JSON.parse(text) : null;
  return {
    id,
    group,
    path,
    kind: artifactKind(path),
    parsedProof: parsed?.proof ?? null,
    freshness: artifactFreshness(id, path, parsed),
  };
}

function runCommand([name, command, args]) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  const log = `${commandLogRoot}/${safeName(name)}.log`;
  writeFileSync(log, normalizeCommandOutput(`${result.stdout ?? ''}${result.stderr ?? ''}`));
  if (result.status !== 0) {
    throw new Error(`${name} failed with exit ${result.status}; log=${log}`);
  }
  return { name, command: [command, ...args].join(' '), status: result.status, log };
}

function snapshotTrackedProofSideEffects() {
  const result = spawnSync(
    'git',
    ['ls-files', 'output/eventing-plan-proof', 'output/network-plan-proof', 'test-results'],
    {
      encoding: 'utf8',
      shell: false,
    }
  );
  if (result.status !== 0) {
    throw new Error(`git ls-files failed with exit ${result.status}`);
  }
  return result.stdout
    .split(/\r?\n/u)
    .filter(Boolean)
    .filter((path) => !isAggregateProofPath(path))
    .map((path) => [path, readFileSync(path)]);
}

function restoreTrackedProofSideEffects(snapshot) {
  for (const [path, contents] of snapshot) {
    writeFileSync(path, contents);
  }
}

function isAggregateProofPath(path) {
  const normalized = path.replace(/\\/gu, '/');
  return normalized.startsWith(`${proofRoot}/`) || normalized.startsWith(`${testRoot}/`);
}

function groupArtifacts(artifactsToGroup) {
  return artifactsToGroup.reduce((groups, artifact) => {
    groups[artifact.group] = [...(groups[artifact.group] ?? []), artifact];
    return groups;
  }, {});
}

function writeGroupedLog(filename, artifactGroup = [], commandGroup = []) {
  const artifactLines = artifactGroup.map(
    (artifact) => `artifact=${artifact.id}\npath=${artifact.path}\nkind=${artifact.kind}`
  );
  const commandLines = commandGroup.map(
    (entry) => `command=${entry.command}\nstatus=${entry.status}\nlog=${entry.log}`
  );
  writeFileSync(`${proofRoot}/${filename}`, `${[...artifactLines, ...commandLines].join('\n\n')}\n`);
}

function writeJsonArtifact(filename, artifactGroup = []) {
  writeFileSync(`${proofRoot}/${filename}`, `${JSON.stringify({ artifacts: artifactGroup }, null, 2)}\n`);
}

function sourceSnapshot() {
  return [
    '# Network Full Plan Source Snapshot',
    '',
    'Deterministic full-network-plan closure proof for the current E-D network branch.',
    '',
    '## Source Paths Inspected',
    '',
    '- docs/features/network-domain-control.md',
    '- docs/expectations/network-flow-evidence.md',
    '- docs/expectations/policy.md',
    '- docs/expectations/enforcement.md',
    '- docs/plans/eventing-plan',
    '- docs/plans/network-plan',
    '- crates/ocentra-network-evidence',
    '- crates/ocentra-eventing',
    '- crates/agent-core',
    '- crates/agent-service',
    '- packages/activity-domain',
    '- packages/agent-protocol-domain',
    '- apps/portal',
    '',
    '## Before-State Gap',
    '',
    'Row-level network proof artifacts existed, but the required network proof pack rows did not have one aggregate artifact that ties contracts, eventing, parser/analyzer fixtures, AI/policy, adapter state, journal/read-model, UI, security negatives, performance, remote delivery, and explicit non-claims together.',
    '',
    'Run-specific branch, commit, pushed state, and validation command output are reported in the worker DONE/PR-ready handoff; this committed artifact is kept deterministic so rerunning the proof does not dirty the checkout.',
    '',
  ].join('\n');
}

function uiSnapshotSummary(artifactGroup = []) {
  return [
    '# Network Full Plan UI Snapshot Summary',
    '',
    'The full-plan proof references the existing service-backed parent UI proof artifacts instead of producing new UI pixels in this closure slice.',
    '',
    ...artifactGroup.map((artifact) => `- ${artifact.id}: ${artifact.path}`),
    '',
    'UI non-claims: the portal remains a read-model/status surface and does not own policy authority, adapter authority, event publishing, AI authority, or enforcement-command publication.',
    '',
  ].join('\n');
}

function isJson(path) {
  return path.endsWith('.json');
}

function isBinary(path) {
  return path.endsWith('.png');
}

function artifactKind(path) {
  if (path.endsWith('.json')) return 'json';
  if (path.endsWith('.md')) return 'markdown';
  if (path.endsWith('.log')) return 'log';
  if (path.endsWith('.png')) return 'png';
  return 'file';
}

function safeName(value) {
  return value.replace(/[^a-zA-Z0-9_.-]/g, '-');
}

function normalizeCommandOutput(value) {
  const lines = value
    .replace(/\r\n/gu, '\n')
    .replace(/\\/gu, '/')
    .replace(/target\/debug\/deps\/[^\s)]+/gu, 'target/debug/deps/<test-binary>')
    .replace(/\b\d+\.\d+s\b/gu, '<duration>s')
    .replace(/\b\d+\.\d{2}ms\b/gu, '<duration>ms')
    .replace(/target\(s\) in [^\n]+/gu, 'target(s) in <duration>')
    .replace(/finished in [^\n]+/giu, 'finished in <duration>')
    .replace(/Duration [^\n]+/gu, 'Duration <duration>')
    .split('\n')
    .filter((line) => !/^\s+Compiling /u.test(line))
    .filter((line) => !/^\s+Blocking waiting for file lock on build directory$/u.test(line));
  return `${stableRustTestLines(lines).join('\n').trim()}\n`;
}

function artifactFreshness(id, path, parsed) {
  if (refreshedArtifactById.has(id)) {
    return { kind: 'refreshed-by-aggregate-command', command: refreshedArtifactById.get(id) };
  }
  if (!parsed) {
    return isBinary(path) ? { kind: 'binary-reference' } : { kind: 'file-reference' };
  }
  const proof = parsed.proof ?? null;
  if (proof && refreshedProofByName.has(proof)) {
    return { kind: 'refreshed-by-aggregate-command', command: refreshedProofByName.get(proof) };
  }
  if (typeof parsed.sourceFingerprint === 'string' && Array.isArray(parsed.sourceRefs)) {
    const actualFingerprints = sourceFingerprintsForRefs(parsed.sourceRefs);
    const matchingFingerprint = actualFingerprints.find(
      (fingerprint) => fingerprint.value === parsed.sourceFingerprint
    );
    if (!matchingFingerprint) {
      throw new Error(
        `source fingerprint mismatch for ${path}: expected ${parsed.sourceFingerprint}, actual ${actualFingerprints
          .map((fingerprint) => fingerprint.value)
          .join(' or ')}`
      );
    }
    return {
      kind: 'source-fingerprint-match',
      refCount: parsed.sourceRefs.length,
      convention: matchingFingerprint.name,
    };
  }
  if (typeof parsed.checkedAt === 'string' && parsed.checkedAt.startsWith('deterministic:')) {
    return { kind: 'deterministic-proof-reference' };
  }
  if (parsed.branch || parsed.commit || parsed.sourceCommit || parsed.statusShort || parsed.sourceStatusShort) {
    return {
      kind: 'legacy-run-metadata-reference',
      branch: parsed.branch ?? null,
      commit: parsed.commit ?? parsed.sourceCommit ?? null,
      hasDirtyStatus: Boolean(parsed.statusShort || parsed.sourceStatusShort),
    };
  }
  return { kind: 'json-proof-reference' };
}

function sourceFingerprintsForRefs(sourceRefs) {
  return [
    { name: 'all-source-refs', value: `source-tree:${sourceFingerprintForRefs(sourceRefs)}` },
    {
      name: 'source-refs-excluding-proof-harness',
      value: `source-tree:${sourceFingerprintForRefs(sourceRefs.filter((sourceRef) => !sourceRef.startsWith('scripts/test/')))}`,
    },
  ];
}

function sourceFingerprintForRefs(sourceRefs) {
  const hash = createHash('sha256');
  for (const filePath of sourceRefs) {
    if (!existsSync(filePath)) {
      throw new Error(`source fingerprint ref missing: ${filePath}`);
    }
    hash.update(filePath);
    hash.update('\0');
    hash.update(readFileSync(filePath, 'utf8').replace(/\r\n/gu, '\n'));
    hash.update('\0');
  }
  return hash.digest('hex');
}

function summarizeFreshness(artifactsToSummarize) {
  const counts = artifactsToSummarize.reduce((summary, artifact) => {
    const kind = artifact.freshness.kind;
    summary[kind] = (summary[kind] ?? 0) + 1;
    return summary;
  }, {});
  const legacyMetadataArtifacts = artifactsToSummarize
    .filter((artifact) => artifact.freshness.kind === 'legacy-run-metadata-reference')
    .map((artifact) => artifact.id);
  const refreshedArtifacts = artifactsToSummarize
    .filter((artifact) => artifact.freshness.kind === 'refreshed-by-aggregate-command')
    .map((artifact) => artifact.id);
  const fingerprintCheckedArtifacts = artifactsToSummarize
    .filter((artifact) => artifact.freshness.kind === 'source-fingerprint-match')
    .map((artifact) => artifact.id);
  return {
    policy:
      'Aggregate proof reruns current critical proof scripts, verifies source-tree fingerprints where artifacts provide sourceRefs, and labels legacy row artifacts instead of silently treating old branch metadata as fresh.',
    counts,
    refreshedArtifacts,
    fingerprintCheckedArtifacts,
    legacyMetadataArtifacts,
  };
}

function stableRustTestLines(lines) {
  const sortedTestLines = lines.filter(isRustTestLine).sort();
  let nextTestLine = 0;
  return lines.map((line) => {
    if (!isRustTestLine(line)) {
      return line;
    }
    const sortedLine = sortedTestLines[nextTestLine];
    nextTestLine += 1;
    return sortedLine;
  });
}

function isRustTestLine(line) {
  return /^test .+ \.\.\. ok$/u.test(line);
}
