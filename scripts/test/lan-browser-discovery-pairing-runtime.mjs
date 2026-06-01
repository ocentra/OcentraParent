import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofId = 'lan-browser-discovery-pairing-runtime';
const outputDir = join(repoRoot, 'test-results', proofId);
const proofPath = join(outputDir, 'proof.json');
const checks = [
  {
    label: 'parent-domain browser runtime request contracts',
    command: 'npm',
    args: [
      '--workspace',
      '@ocentra-parent/parent-domain',
      'run',
      'test',
      '--',
      '--run',
      'tests/lan-pairing-browser-runtime.test.ts',
    ],
  },
  {
    label: 'agent-protocol-domain browser runtime command contracts',
    command: 'npm',
    args: [
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      'run',
      'test',
      '--',
      '--run',
      'tests/lan-pairing-browser-runtime.test.ts',
    ],
  },
  {
    label: 'Rust protocol browser runtime parity',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-protocol', 'lan_pairing_browser_runtime', '--quiet'],
  },
  {
    label: 'Rust service browser runtime events',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'lan_pairing_browser_runtime', '--quiet'],
  },
];

const commands = [];
const proofLabels = [];

for (const check of checks) {
  console.log(`[lan-browser-runtime] ${check.label}`);
  commands.push([check.command, ...check.args].join(' '));
  const result = spawnSync(check.command, check.args, {
    cwd: repoRoot,
    shell: true,
    stdio: 'inherit',
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
  proofLabels.push(check.label);
}

mkdirSync(outputDir, { recursive: true });
writeFileSync(proofPath, `${JSON.stringify(proof(), null, 2)}\n`);
console.log(`[lan-browser-runtime] proof harness passed evidence=${proofPath}`);

function proof() {
  return {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: gitHead(),
    proofMode: proofId,
    commands,
    proofLabels,
    evidence: {
      parentContract: 'packages/parent-domain/src/lan-pairing-browser-runtime.ts',
      agentProtocolContract: 'packages/agent-protocol-domain/src/lan-pairing-browser-runtime.ts',
      rustProtocolContract: 'crates/agent-protocol/src/lan_pairing_browser_runtime.rs',
      rustServiceAdapter: 'crates/agent-service/src/lan_pairing_browser_runtime.rs',
      rustServiceTest: 'crates/agent-service/src/lan_pairing_browser_runtime_tests.rs',
      output: relativePath(proofPath),
    },
    runtimeEvents: {
      discoveryScan: 'agent.lan-pairing.browser-discovery.scan -> agent.lan-pairing.browser-discovery.reported',
      addDeviceRequest: 'agent.lan-pairing.add-device.request -> agent.lan-pairing.add-device.reported',
      localServiceDiscovery: 'real command target/local service state',
      rejectedPairing: 'wrong-origin add-device request rejects without trusting a device',
      selectedReadiness: 'trusted selected route reports readyForControl from service registry state',
    },
    honestBoundaries: [
      'physical household LAN scan remains manual-required until real device/router/firewall artifacts exist',
      'cloud relay is unavailable/not implemented',
      'remote desktop/control is not implemented in this slice',
      'no fake household devices are emitted',
    ],
  };
}

function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: true,
  });
  if (result.status !== 0) {
    throw new Error('git rev-parse HEAD failed');
  }
  return result.stdout.trim();
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
