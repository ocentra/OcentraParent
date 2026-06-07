import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '40-android-vpnservice-proof-gate');
const testRoot = join('test-results', 'network-android-vpnservice-proof-gate');
const proofRevision = 'network-android-vpnservice-proof-gate/v1';
const proofBranch = 'codex/network-adapter-gate-proof-artifacts';
const deterministicCheckedAt = `deterministic:${proofRevision}`;
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-android-vpnservice-proof-gate.json'),
  `${JSON.stringify(
    {
      requiredRefs: [
        'policy-decision-ref',
        'parent-rule-ref',
        'evidence-refs',
        'package-ref',
        'vpn-service-ref',
        'vpn-service-declaration-ref',
        'user-consent-proof-ref',
        'physical-device-proof-ref',
        'package-identity-proof-ref',
        'virtual-interface-proof-ref',
        'traffic-observation-proof-ref',
        'rollback-plan-ref',
        'audit-event-ref',
      ],
      optionalDeviceOwnerRef: 'device-owner-proof-ref is required only when Device Owner authority is claimed',
      physicalDeviceProofReadyState:
        'grade A block policy plus physical-device VpnService consent/interface proof artifacts',
      researchOnlyState: 'non-executable and allowed without device artifacts',
      manualRequiredState:
        'weak evidence, non-block policy, manual capability, missing artifacts, or missing Device Owner proof when required',
      unavailableState: 'non-executable Android VpnService capability-unavailable state',
      unsupportedClaimsRejected: [
        'exact URL',
        'decrypted payload',
        'page content',
        'emulator-only product support',
        'live VPN tunnel',
        'packet block',
        'app/package correlation',
      ],
      adapterApplyAuthorized: false,
      enforcementCommandPublished: false,
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-android-vpnservice-gate-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'android_vpn_service_gate'],
    log: join(proofRoot, 'android-vpnservice-gate-tests.log'),
  },
  {
    name: 'network-evidence-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-network-evidence', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'clippy.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
];
const commandResults = commands.map(runCommand);

const proof = {
  proof: 'network-android-vpnservice-proof-gate',
  proofRevision,
  checkedAt: deterministicCheckedAt,
  branch: proofBranch,
  sourceCommit: `source-tree:${sourceTreeFingerprint()}`,
  sourceTreeFingerprint: sourceTreeFingerprint(),
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedAndroidVpnServiceProofGate: join(proofRoot, 'expected-android-vpnservice-proof-gate.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['40 Android VpnService adapter/proof gate'],
  notClaimed: [
    'emulator-only product support',
    'live Android VpnService tunnel',
    'packet blocking',
    'app/package correlation',
    'Device Owner authority without proof',
    'adapter action authorization',
    'enforcement command publication',
    'decrypted payload or page content inspection',
    'exact URL claim from network-only evidence',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-android-vpnservice-proof-gate-ok:vpnservice-gate-tests,clippy,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  writeFileSync(entry.log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
  if (result.status !== 0) {
    throw new Error(`${entry.name} failed with exit ${result.status}`);
  }
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    log: entry.log,
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
  return runText('git', ['status', '--short', '--', '.', ':(exclude)output', ':(exclude)test-results']);
}

function sourceTreeFingerprint() {
  const sourceIndex = runText('git', ['ls-files', '-s', '--', '.', ':(exclude)output', ':(exclude)test-results']);
  return createHash('sha256').update(sourceIndex).digest('hex');
}
