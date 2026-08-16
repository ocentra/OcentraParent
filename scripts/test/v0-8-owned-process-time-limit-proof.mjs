import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const proofId = 'v0-8-owned-process-time-limit-proof';
const proofRoot = join(repoRoot, 'test-results', proofId);
const proofPath = join(proofRoot, 'proof.json');
const planProofRoot = join(repoRoot, 'output', 'v0-8-enforcement-control-plan-proof', '04-owned-process-time-limit');
const planProofPath = join(planProofRoot, 'proof.json');
const commands = [
  [
    'cargo',
    ['test', '-p', 'ocentra-parent-agent-core', 'owned_process_adapter_terminates_a_real_owned_windows_process'],
  ],
  ['cargo', ['test', '-p', 'ocentra-parent-agent-core', 'enforcement_app_time_limit']],
  ['cargo', ['test', '-p', 'ocentra-parent-agent-service', 'enforcement_timer']],
];

await mkdir(proofRoot, { recursive: true });
await mkdir(planProofRoot, { recursive: true });
for (const [command, args] of commands) {
  await run(command, args);
}

const proof = {
  schemaVersion: 1,
  proofId,
  checkedAt: new Date().toISOString(),
  commands: commands.map(([command, args]) => [command, ...args].join(' ')),
  evidence: {
    realWindowsTerminationTest:
      'crates/agent-core/tests/unit/enforcement_tests.rs::owned_process_adapter_terminates_a_real_owned_windows_process',
    timeLimitLifecycleTest: 'crates/agent-core/tests/unit/enforcement_app_time_limit_tests.rs',
    serviceTimerPersistenceTest: 'crates/agent-service/tests/unit/enforcement_timer_expiry_tests.rs',
    testArtifact: relative(repoRoot, proofPath).replaceAll('\\', '/'),
    planArtifact: relative(repoRoot, planProofPath).replaceAll('\\', '/'),
  },
  nonClaims: [
    'broad installed-app blocking',
    'restart or relaunch of a terminated process',
    'mobile or non-Windows process control',
    'browser, network, screen, or AI enforcement',
  ],
};
const serializedProof = `${JSON.stringify(proof, null, 2)}\n`;
await Promise.all([writeFile(proofPath, serializedProof), writeFile(planProofPath, serializedProof)]);

function run(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${command} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}
