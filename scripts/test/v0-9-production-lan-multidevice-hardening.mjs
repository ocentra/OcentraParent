import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const outputDir = join(process.cwd(), 'test-results', 'v0-9-production-lan-multidevice-hardening');
const evidencePath = join(outputDir, 'proof.json');

const proofSteps = [
  {
    label: 'discovery-challenge',
    command: ['node', 'scripts/test/v0-9-lan-discovery-challenge-mvp.mjs'],
    evidencePath: join(process.cwd(), 'test-results', 'v0-9-lan-discovery-challenge-mvp', 'proof.json'),
    requiredAssertions: [
      'wrong-origin-websocket-rejected-before-upgrade',
      'first-discovery-agent:challenge-preview-issued',
      'first-discovery-agent:challenge-proof-accepted',
      'first-discovery-agent:route-selected-after-challenge',
      'second-discovery-agent:challenge-preview-issued',
      'second-discovery-agent:challenge-proof-accepted',
      'second-discovery-agent:route-selected-after-challenge',
      'wrong-agent-port-challenge-rejected-as-wrong-device',
    ],
  },
  {
    label: 'pairing-control',
    command: ['node', 'scripts/test/v0-9-lan-pairing-control-mvp.mjs'],
    evidencePath: join(process.cwd(), 'test-results', 'v0-9-lan-pairing-control-mvp', 'proof.json'),
    requiredAssertions: [
      'wrong-origin-websocket-rejected-before-upgrade',
      'first-child-agent:route-selected',
      'first-child-agent:observer-write-rejected',
      'first-child-agent:controller-lease-takeover-denied',
      'first-child-agent:route-revoked',
      'second-child-agent:controller-lease-takeover-accepted',
      'second-child-agent:restart-restores-selected-route',
      'second-child-agent:restart-recovered-approval-accepted',
      'wrong-agent-port-rejected-as-wrong-device',
    ],
  },
  {
    label: 'lan-ai-provider-pool',
    command: ['node', 'scripts/test/platform-roles-lan-ai-provider-pool.mjs'],
    evidencePath: join(process.cwd(), 'test-results', 'platform-roles-lan-ai-provider-pool', 'proof.json'),
    requiredAssertions: [
      'parent-desktop-controller-ai-provider:provider-advertised-available',
      'parent-desktop-controller-ai-provider:controller-job-completed-observer-job-rejected',
      'parent-desktop-controller-ai-provider:unsupported-capability-rejected',
      'parent-mobile-observer-scaffold:provider-unavailable',
      'parent-mobile-observer-scaffold:controller-job-degraded-with-provider-unavailable',
      'parent-desktop-busy-ai-provider:provider-busy',
      'parent-desktop-busy-ai-provider:busy-job-degraded',
    ],
  },
];

const manualTwoDeviceChecklist = [
  {
    label: 'two-physical-hosts',
    commands: [
      'cargo build -p ocentra-parent-agent-service',
      'set OCENTRA_PARENT_AGENT_ADDR=0.0.0.0:4477',
      'set OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS=http://127.0.0.1:4478,http://<parent-lan-ip>:4478',
      'set OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED=true',
      'target\\debug\\ocentra-parent-agent-service.exe',
      'node scripts/test/v0-9-production-lan-multidevice-hardening.mjs',
    ],
    requiredArtifacts: [
      'child service stdout/stderr with listening address and no secret-bearing payloads',
      'test-results/v0-9-production-lan-multidevice-hardening/proof.json',
      'test-results/v0-9-lan-discovery-challenge-mvp/proof.json',
      'test-results/v0-9-lan-pairing-control-mvp/proof.json',
      'test-results/platform-roles-lan-ai-provider-pool/proof.json',
      'parent and child host names or IPs showing two distinct LAN devices',
      'firewall/router note proving the child port is reachable from the parent host',
    ],
    currentStatus: 'manual-required-physical-devices-not-claimed-by-local-harness',
  },
];

await mkdir(outputDir, { recursive: true });

const checkedSteps = [];
for (const step of proofSteps) {
  await runStep(step);
  const evidence = JSON.parse(await readFile(step.evidencePath, 'utf8'));
  assertRequiredAssertions(step, evidence.assertions ?? []);
  checkedSteps.push({
    label: step.label,
    command: step.command.join(' '),
    evidencePath: relativeToWorkspace(step.evidencePath),
    assertionCount: evidence.assertions?.length ?? 0,
    requiredAssertions: step.requiredAssertions,
  });
}

const proof = {
  schemaVersion: 1,
  checkedAt: new Date().toISOString(),
  proofMode: 'local-multi-service-production-lan-hardening',
  checkedSteps,
  claimsProvedLocally: [
    'production LAN states use explicit discovered/pending/paired/revoked/stale/offline/unavailable contract values',
    'trusted registry persists selected route and recovers it after restart',
    'active controller write authority rejects observer writes, stale intents, replay, wrong device, and denied takeover',
    'LAN AI provider routing covers authorized result, unsupported capability, busy, unavailable, and observer rejection',
  ],
  claimsNotProvedLocally: [
    'real household router discovery across two physical devices',
    'OS firewall prompts and mobile background behavior on Windows/macOS/Linux/Android/iOS',
    'device-owner policy, iOS Family Controls, app-store or MDM deployment behavior',
  ],
  manualTwoDeviceChecklist,
};

await writeFile(evidencePath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(`v0-9-production-lan-multidevice-hardening-ok:${checkedSteps.map((step) => step.label).join(',')}`);

async function runStep(step) {
  await new Promise((resolve, reject) => {
    const child = spawn(step.command[0], step.command.slice(1), {
      cwd: process.cwd(),
      env: process.env,
      shell: false,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const chunks = [];
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${step.label} failed with exit code ${code}\n${chunks.join('')}`));
    });
  });
}

function assertRequiredAssertions(step, assertions) {
  for (const required of step.requiredAssertions) {
    if (!assertions.includes(required)) {
      throw new Error(`${step.label} evidence is missing required assertion ${required}`);
    }
  }
}

function relativeToWorkspace(path) {
  return path.replace(`${process.cwd()}\\`, '').replaceAll('\\', '/');
}
