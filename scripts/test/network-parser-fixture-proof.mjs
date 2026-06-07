import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const proofRoot = join('output', 'network-plan-proof', '03-parser-fixture-proof');
const testRoot = join('test-results', 'network-parser-fixture-proof');
const parserRoots = [
  proofRoot,
  testRoot,
  join('output', 'network-plan-proof', '11-rust-crate-and-tooling-evaluation'),
  join('output', 'network-plan-proof', '12-pcap-file-replay-harness'),
  join('output', 'network-plan-proof', '14-packet-parser'),
  join('output', 'network-plan-proof', '15-dns-query-response-parser'),
  join('output', 'network-plan-proof', '16-tls-clienthello-sni-parser'),
  join('output', 'network-plan-proof', '17-http-host-parser'),
  join('output', 'network-plan-proof', '18-quic-http3-limitation-detector'),
  join('output', 'network-plan-proof', '19-doh-dot-detector'),
];

const commands = [
  {
    name: 'network-pcap-replay-proof',
    command: 'node',
    args: ['scripts/test/network-pcap-replay-proof.mjs'],
  },
  {
    name: 'network-packet-dns-parser-proof',
    command: 'node',
    args: ['scripts/test/network-packet-dns-parser-proof.mjs'],
  },
  {
    name: 'network-visibility-parser-proof',
    command: 'node',
    args: ['scripts/test/network-visibility-parser-proof.mjs'],
  },
];

const commandResults = commands.map(runCommand);
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const requiredArtifacts = [
  join('output', 'network-plan-proof', '11-rust-crate-and-tooling-evaluation', 'tooling-decision.md'),
  join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'proof-summary.json'),
  join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'fixtures', 'dns-example.pcap'),
  join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'expected-domain-evidence.json'),
  join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'must-not-claim.json'),
  join('output', 'network-plan-proof', '14-packet-parser', 'expected-packet-fixtures.json'),
  join('output', 'network-plan-proof', '15-dns-query-response-parser', 'expected-dns-response.json'),
  join('output', 'network-plan-proof', '15-dns-query-response-parser', 'proof-summary.json'),
  join('output', 'network-plan-proof', '16-tls-clienthello-sni-parser', 'expected-sni-visibility.json'),
  join('output', 'network-plan-proof', '17-http-host-parser', 'expected-http-host.json'),
  join('output', 'network-plan-proof', '18-quic-http3-limitation-detector', 'expected-quic-limitation.json'),
  join('output', 'network-plan-proof', '19-doh-dot-detector', 'must-not-claim.json'),
  join('output', 'network-plan-proof', '19-doh-dot-detector', 'proof-summary.json'),
];
const missingArtifacts = requiredArtifacts.filter((path) => !existsSync(path));
if (missingArtifacts.length > 0) {
  throw new Error(`network parser fixture proof missing artifacts: ${missingArtifacts.join(', ')}`);
}

const parserFixtureProof = {
  pcapFixtureInput: join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'fixtures', 'dns-example.pcap'),
  parserOutputs: [
    join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'expected-domain-evidence.json'),
    join('output', 'network-plan-proof', '14-packet-parser', 'expected-packet-fixtures.json'),
    join('output', 'network-plan-proof', '15-dns-query-response-parser', 'expected-dns-response.json'),
    join('output', 'network-plan-proof', '16-tls-clienthello-sni-parser', 'expected-sni-visibility.json'),
    join('output', 'network-plan-proof', '17-http-host-parser', 'expected-http-host.json'),
    join('output', 'network-plan-proof', '18-quic-http3-limitation-detector', 'expected-quic-limitation.json'),
    join('output', 'network-plan-proof', '19-doh-dot-detector', 'must-not-claim.json'),
  ],
  externalComparisonState: {
    tshark: 'manual-unavailable',
    wireshark: 'manual-unavailable',
    reason:
      'This deterministic proof runs local Rust parser fixtures only; no external TShark or Wireshark binary is invoked on this worker branch.',
    followUpOwner: 'primary-lab-host-with-analyzer-tooling',
  },
  mustNotClaimRecords: [
    join('output', 'network-plan-proof', '12-pcap-file-replay-harness', 'must-not-claim.json'),
    join('output', 'network-plan-proof', '15-dns-query-response-parser', 'must-not-claim.json'),
    join('output', 'network-plan-proof', '19-doh-dot-detector', 'must-not-claim.json'),
  ],
};

const proof = {
  proof: 'network-parser-fixture-proof',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  sourceCommit: runText('git', ['rev-parse', 'HEAD']).trim(),
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    parserFixtureProof: join(proofRoot, '03-parser-fixture-proof.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  requiredArtifacts,
  parserFixtureProof,
  provenRows: [
    '11 Rust crate and tooling evaluation',
    '12 PCAP file replay harness',
    '14 Packet parser',
    '15 DNS query/response parser',
    '16 TLS ClientHello/SNI parser',
    '17 HTTP Host parser',
    '18 QUIC/HTTP3 limitation detector',
    '19 DoH/DoT detector',
  ],
  provenRootGate: '03-parser-fixture-proof.json',
  notClaimed: [
    'live Npcap/libpcap capture',
    'external TShark or Wireshark execution',
    'TCP stream reassembly beyond named fixtures',
    'live Zeek, Suricata, or Snort analyzer execution',
    'exact URL, page content, private message, search query, video, screen activity, or decrypted payload visibility',
    'policy, adapter, portal, broker, family-hub, or enforcement runtime integration',
  ],
};

writeFileSync(
  join(proofRoot, '03-parser-fixture-proof.json'),
  `${JSON.stringify(parserFixtureProof, null, 2)}\n`
);
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-parser-fixture-proof-ok:pcap,packet,dns,tls,http,quic,doh-dot');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${entry.name} failed with exit ${result.status}`);
  }
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
  };
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function sourceStatusShort() {
  return runText('git', [
    'status',
    '--short',
    '--',
    '.',
    ...parserRoots.map((path) => `:(exclude)${path.replaceAll('\\', '/')}`),
  ]);
}
