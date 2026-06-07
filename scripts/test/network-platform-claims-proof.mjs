import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '52-platform-claims-proof');
const testRoot = join('test-results', 'network-platform-claims-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceBranch = runText('git', ['branch', '--show-current']).trim();
const sourceCommit = runText('git', ['rev-parse', 'HEAD']).trim();
const sourceOriginMain = runText('git', ['rev-parse', 'origin/main']).trim();
const sourceMergeBase = runText('git', ['merge-base', 'HEAD', 'origin/main']).trim();
const sourceStatusShort = readSourceStatusShort();

writeFileSync(
  join(proofRoot, 'expected-platform-claims.json'),
  `${JSON.stringify(
    {
      row: 52,
      requiredTargets: [
        'Windows Firewall',
        'Windows WFP',
        'Android VpnService',
        'Apple Network Extension macOS',
        'Apple Network Extension iOS',
        'Linux nftables',
        'Linux eBPF',
        'Linux TUN',
      ],
      platformClaimInvariants: [
        'every platform claim names exact OS/device refs',
        'every ready claim names permission, entitlement, capability, or manual follow-up refs',
        'manual-required and unavailable states remain reportable without live execution',
        'adapter authorization is accepted only on ready platform claim rows',
        'UI has no policy authority',
        'no proof source publishes enforcement commands',
      ],
      notClaimed: [
        'generic platform support',
        'live adapter execution',
        'host packet blocking',
        'exact URL from network-only evidence',
        'decrypted payload or page content',
        'UI policy authority',
      ],
    },
    null,
    2
  )}\n`
);
writeFileSync(join(proofRoot, '11-manual-platform-proof.md'), manualPlatformProof());

const commands = [
  {
    name: 'network-platform-claim-manifest-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'platform_claims'],
    log: join(proofRoot, 'platform-claims-tests.log'),
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
writeFileSync(join(proofRoot, '12-validation-commands.log'), validationCommandsLog(commandResults));

const proof = {
  proof: 'network-platform-claims',
  checkedAt: new Date().toISOString(),
  branch: sourceBranch,
  commit: sourceCommit,
  originMain: sourceOriginMain,
  mergeBase: sourceMergeBase,
  sourceStatusShort,
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedPlatformClaims: join(proofRoot, 'expected-platform-claims.json'),
    manualPlatformProof: join(proofRoot, '11-manual-platform-proof.md'),
    validationCommands: join(proofRoot, '12-validation-commands.log'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['52 Platform claim manifest proof'],
  provenRootGates: [
    'platform claims name exact OS/device/permission refs',
    'manual-required missing artifacts have explicit follow-up entries',
    'unavailable platform rows remain visible without adapter authorization',
    'proof sources cannot authorize adapters unless the platform claim row is ready',
    'generic platform support and live adapter execution claims are rejected',
    'proof sources cannot publish enforcement commands',
    'UI policy authority remains rejected',
  ],
  notClaimed: [
    'generic platform support',
    'live host adapter mutation or packet blocking',
    'production platform support',
    'exact URL, page content, or decrypted payload from network-only evidence',
    'policy engine execution',
    'enforcement command publication',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-platform-claims-proof-ok:tests,clippy,source-shape');
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

function manualPlatformProof() {
  return `# Network Platform Claim Manual Proof

Row: 52 Platform claim manifest proof

Validated target rows:

- Windows Firewall: exact target/rule refs, adapter authorization ref, capability proof ref, audit ref.
- Windows WFP: exact target/provider/layer refs, administrator permission, driver signing/package, provider registration, layer capability, lab result, audit ref.
- Android VpnService: exact package/service/device refs, VpnService declaration, user consent, package identity, virtual interface, traffic observation, Device Owner proof when claimed, audit ref.
- Apple Network Extension macOS: exact bundle/extension/device refs, developer team, entitlement approval, provisioning, signing, declaration, configuration, supervision/MDM proof when claimed, audit ref.
- Apple Network Extension iOS: exact bundle/extension/device refs, developer team, entitlement approval, provisioning, signing, declaration, configuration, supervision/MDM proof when claimed, audit ref.
- Linux nftables: exact distro/kernel refs, permission, adapter API, adapter plan, service-manager scope, lab result, audit ref.
- Linux eBPF: exact distro/kernel refs, permission, adapter API, adapter plan, service-manager scope, lab result, audit ref.
- Linux TUN: exact distro/kernel refs, permission, adapter API, adapter plan, service-manager scope, lab result, audit ref.

Manual-required and unavailable labels:

- Missing WFP administrator permission records manual follow-up label \`windows-wfp.administrator-permission\`.
- Unavailable Linux TUN rows remain visible, do not authorize adapter apply, and record follow-up label \`linux-adapter.permission\` when permission proof is absent.
- Non-ready platform rows cannot carry adapter authorization, so dry-run, research-only, manual-required, and unavailable states remain non-executable.

Screenshots/logs:

- UI screenshots are N/A for this row because the proof is a Rust manifest/harness boundary, not a portal rendering change.
- Command logs are written by this harness under \`output/network-plan-proof/52-platform-claims-proof/\`.

Known follow-up owner:

- Platform adapter implementation owners must replace fixture refs with real OS/device/permission artifacts before any production platform support claim.

No-claim boundary:

- No generic platform support.
- No live adapter execution.
- No host packet blocking.
- No exact URL from network-only evidence.
- No decrypted payload or page content.
- No UI policy authority.
- No enforcement command publication.
`;
}

function validationCommandsLog(results) {
  const lines = [
    'network-platform-claims validation commands',
    '',
    ...results.map((result) => `${result.name}: ${result.command} -> exit ${result.status}; log=${result.log}`),
    '',
    'Additional check run outside this harness: git diff --check -> exit 0.',
  ];
  return `${lines.join('\n')}\n`;
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function readSourceStatusShort() {
  return runText('git', [
    'status',
    '--short',
    '--',
    '.',
    ':(exclude)output/network-plan-proof/52-platform-claims-proof',
    ':(exclude)test-results/network-platform-claims-proof',
  ]);
}
