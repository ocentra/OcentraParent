import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { extname } from 'node:path';

const proofRoot = 'output/network-plan-proof/adapter-capability-status';
const testRoot = 'test-results/network-adapter-capability-status-proof';
const proofPath = `${testRoot}/proof.json`;
const planProofPath = `${proofRoot}/proof-summary.json`;
const sourceRoots = ['apps/portal/src', 'packages/portal-domain/src'];
const sourceExtensions = new Set(['.ts', '.tsx']);
const commands = [];
const proofLabels = [];

mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

runCommand('cargo', ['test', '-p', 'ocentra-network-evidence', 'adapter_capability_status']);
runCommand('cargo', ['test', '-p', 'ocentra-network-evidence', 'platform_claims']);
runCommand('cmd', [
  '/c',
  'npm',
  'exec',
  '--workspace',
  '@ocentra-parent/portal',
  '--',
  'eslint',
  'src/network-evidence-drawer.ts',
  'src/NetworkEvidenceDrawerRoutePanel.tsx',
  'tests/live-activity-network-flow.test.ts',
  '../../packages/portal-domain/src/details.ts',
]);
runCommand('node', ['scripts/check-source-shape.mjs']);
runCommand('git', ['diff', '--check']);

writeFileSync(
  `${proofRoot}/expected-adapter-capability-status.json`,
  `${JSON.stringify(expectedAdapterCapabilityStatus(), null, 2)}\n`
);

const scannedFiles = assertPortalCapabilityStatusProjection();
const proof = {
  schemaVersion: 1,
  proof: 'network-adapter-capability-status',
  sourceBranch: runText('git', ['branch', '--show-current']).trim(),
  proofRoot,
  testRoot,
  commands,
  artifacts: {
    expectedAdapterCapabilityStatus: `${proofRoot}/expected-adapter-capability-status.json`,
    proofSummary: planProofPath,
    testProof: proofPath,
  },
  evidence: {
    adapterCapabilityStatusModule: 'crates/ocentra-network-evidence/src/adapter_capability_status.rs',
    adapterCapabilityStatusTests: 'crates/ocentra-network-evidence/src/tests/adapter_capability_status.rs',
    platformClaimManifestModule: 'crates/ocentra-network-evidence/src/platform_claims.rs',
    platformClaimManifestTests: 'crates/ocentra-network-evidence/src/tests/platform_claims.rs',
    portalNetworkDrawer: 'apps/portal/src/network-evidence-drawer.ts',
    portalNetworkDrawerTest: 'apps/portal/tests/live-activity-network-flow.test.ts',
    scannedSourceRoots: sourceRoots,
    scannedFiles,
  },
  provenRows: ['Network feature checklist: Adapter capability status'],
  provenRootGates: [
    'adapter capability status derives from the existing Row52 platform manifest instead of a duplicate matrix',
    'Windows Firewall ready maps to supported status',
    'Windows WFP ready maps to lab-ready status',
    'Android VpnService ready maps to physical-device-ready status',
    'Apple Network Extension macOS/iOS ready maps to Apple-device-ready status',
    'Linux nftables/eBPF/TUN ready maps to distro-ready status',
    'manual-required and unavailable rows preserve missing-artifact follow-ups',
    'Row52 platform manifest rejects non-ready adapter authorization before status projection',
    'adapter authorization is rejected on dry-run, research-only, manual-required, or unavailable status rows',
    'portal drawer renders service-backed capability/platform status for the current network read model',
    'generic platform support, live adapter execution, broader platform capability UX, UI policy authority, and enforcement command publication claims are rejected',
  ],
  notClaimed: [
    'live host adapter mutation',
    'packet blocking or host filtering',
    'production platform support',
    'broader platform capability UX beyond the current network drawer',
    'exact URL, page content, private message, search query, or decrypted payload from network-only evidence',
    'policy engine execution',
    'enforcement command publication',
  ],
  proofLabels,
};

const serialized = `${JSON.stringify(proof, null, 2)}\n`;
writeFileSync(planProofPath, serialized);
writeFileSync(proofPath, serialized);
console.log(`network-adapter-capability-status-proof-ok:${proofLabels.join(',')}`);
console.log(`proof=${planProofPath}`);

function expectedAdapterCapabilityStatus() {
  return {
    sourceOfTruth: 'Row52 platform claim manifest proof entries',
    targetStatusMapping: {
      WindowsFirewall: 'supported',
      WindowsWfp: 'lab-ready',
      AndroidVpnService: 'physical-device-ready',
      AppleNetworkExtensionMacOs: 'apple-device-ready',
      AppleNetworkExtensionIos: 'apple-device-ready',
      LinuxNftables: 'distro-ready',
      LinuxEbpf: 'distro-ready',
      LinuxTun: 'distro-ready',
    },
    reportableNonReadyStates: ['dry-run', 'research-only', 'manual-required', 'unavailable'],
    authorizationInvariant: 'adapter_authorized_by_proof is accepted only for Row52 ready claim rows',
    requiredRefs: [
      'platform manifest ref',
      'adapter capability refs or missing-artifact follow-ups',
      'OS/device refs',
      'permission or entitlement refs when available',
      'audit refs',
      'portal service-backed status proof ref',
    ],
    notClaimed: [
      'generic platform support',
      'live adapter execution',
      'enforcement command publication',
      'UI policy authority',
      'broader platform capability UX',
    ],
  };
}

function assertPortalCapabilityStatusProjection() {
  const drawer = readText('apps/portal/src/network-evidence-drawer.ts');
  const test = readText('apps/portal/tests/live-activity-network-flow.test.ts');

  for (const expected of [
    'readModelPlatformState(readModel)',
    'readModelRows(readModel)',
    'degradedState(row, readModel)',
    'return joinedDetail([readModel.custody, readModel.capabilityStatus])',
    'return joinedDetail([row.capabilityStatus, row.domainAttributionStatus, row.processAttributionStatus])',
    'policyDecisionRef: notReported()',
    'interventionResultRef: notReported()',
    'exactUrlClaim: notReported()',
  ]) {
    assertIncludes(drawer, expected, `portal capability projection: ${expected}`);
  }
  proofLabels.push('portal.projects-service-backed-capability-status');

  for (const expected of [
    "expect(summary.platformState).toBe('child-device-query-store | available')",
    "expect(summary.platformState).toBe('child-device-query-store | no-network-observations')",
    "expect(summary.platformState).toBe('child-device-query-store | adapter-error')",
    "expect(summary.degradedState).toBe('available | domain-observed | process-attributed')",
    "expect(summary.degradedState).toBe('no-network-observations')",
    "expect(summary.degradedState).toBe('adapter-error')",
    "expect(summary.policyDecisionRef).toBe('Not reported')",
    "expect(summary.interventionResultRef).toBe('Not reported')",
  ]) {
    assertIncludes(test, expected, `portal test assertion: ${expected}`);
  }
  proofLabels.push('portal.tests-capability-and-non-authority-states');

  const scannedFiles = sourceFiles(sourceRoots);
  for (const file of scannedFiles) {
    const source = readText(file);
    for (const forbidden of [
      /(?:executeNetworkAdapter|applyNetworkAdapter|dispatchEnforcement|authorizeNetworkAdapter)\s*\(/u,
      /(?:evaluateNetworkPolicy|decideNetworkPolicy|computeNetworkPolicy)\s*\(/u,
      /(?:publishNetworkEvent|publishEnforcementCommand|publishDomainEvent|publishBusinessEvent)\s*\(/u,
    ]) {
      assertPatternAbsent(source, forbidden, `portal authority pattern absent: ${file}`);
    }
  }
  proofLabels.push('portal.source-no-adapter-policy-or-event-authority');
  return scannedFiles;
}

function sourceFiles(roots) {
  return roots.flatMap((root) => collectSourceFiles(root)).sort();
}

function collectSourceFiles(path) {
  const entries = readdirSync(path, { withFileTypes: true });
  return entries.flatMap((entry) => {
    const entryPath = `${path}/${entry.name}`;
    if (entry.isDirectory()) {
      return collectSourceFiles(entryPath);
    }
    return sourceExtensions.has(extname(entry.name)) ? [entryPath] : [];
  });
}

function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  const log = `${proofRoot}/${commands.length + 1}-${safeName(commandLine)}.log`;
  writeFileSync(log, normalizeCommandOutput(`${result.stdout ?? ''}${result.stderr ?? ''}`));
  commands.push({
    command: commandLine,
    status: result.status ?? 1,
    log,
  });
  if (result.status !== 0) {
    throw new Error(`${commandLine} failed with exit ${result.status}`);
  }
}

function safeName(value) {
  return value
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-|-$/gu, '')
    .slice(0, 80);
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

function readText(path) {
  return readFileSync(path, 'utf8');
}

function assertIncludes(source, expected, label) {
  if (!source.includes(expected)) {
    throw new Error(`${label} missing`);
  }
}

function assertPatternAbsent(source, pattern, label) {
  if (pattern.test(source)) {
    throw new Error(`${label} failed`);
  }
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}
