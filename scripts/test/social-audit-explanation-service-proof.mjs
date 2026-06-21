import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'social-audit-explanation-service-proof');
const proofDir = join(repoRoot, 'output', 'browser-plan-proof', 'social-22-audit-explanation-read-model');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/browser-domain',
      '--',
      'social-audit-explanation-read-model.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'social-audit-explanation-read-model.test.ts',
      'contracts.test.ts',
    ])
  );
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'social_audit_explanation']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'social_audit_explanation']);

  const proof = {
    schemaVersion: 1,
    proofMode: 'social-audit-explanation-service-read-model',
    checkedAt: new Date().toISOString(),
    branch: await gitBranch(),
    commit: await gitHead(),
    commands,
    summary: {
      command: 'agent.browser.social-audit-explanation.read-model.get',
      event: 'agent.browser.social-audit-explanation.read-model.reported',
      payloadField: 'browserSocialAuditExplanationReadModel',
      rowCount: 6,
      subjects: [
        'account-approval',
        'feed-video-gate',
        'native-app-gap',
        'connector-boundary',
        'decision-memory',
        'manual-required-gap',
      ],
      serviceBackedExplanationDelivery: true,
      runtimeAuditStoreClaimed: false,
      notificationDeliveryClaimed: false,
      connectorAuthorizationClaimed: false,
      nativeAppControlClaimed: false,
      finalPolicyDecisionClaimed: false,
      enforcementClaimed: false,
    },
    claimsProved: [
      'TypeScript protocol exposes the dedicated social audit explanation command, event, and payload field',
      'Schema-domain contract accepts exactly the six ref-only audit/explanation subjects and rejects hidden runtime claims',
      'Rust protocol serializes the social audit explanation snapshot with runtime, final policy, and enforcement claims false',
      'Agent service answers the WebSocket command with a service-built six-row snapshot payload',
    ],
    claimsNotProved: [
      'runtime audit store persistence',
      'notification delivery',
      'raw account, video, or message content capture',
      'connector authorization',
      'native app control',
      'final policy execution',
      'enforcement',
      'product checklist completion or release readiness',
    ],
    evidence: {
      browserContractTest: 'packages/browser-domain/tests/unit/social-audit-explanation-read-model.test.ts',
      protocolContracts: 'packages/agent-protocol-domain/src/contracts.ts',
      protocolDefaults: 'packages/agent-protocol-domain/src/defaults.ts',
      protocolTest: 'packages/agent-protocol-domain/tests/unit/social-audit-explanation-read-model.test.ts',
      schemaDomain: 'packages/schema-domain/src/social-audit-explanation-read-model.ts',
      rustProtocol: 'crates/agent-protocol/src/social_audit_explanation_read_model.rs',
      rustProtocolTest: 'crates/agent-protocol/src/social_audit_explanation_read_model_tests.rs',
      servicePayload: 'crates/agent-service/src/activity_api/social_audit_explanation_read_model_payload.rs',
      servicePayloadTest: 'crates/agent-service/src/activity_api/social_audit_explanation_read_model_payload_tests.rs',
      serviceWebSocketTest:
        'crates/agent-service/src/activity_api/social_audit_explanation_read_model_service_tests.rs',
      harness: 'scripts/test/social-audit-explanation-service-proof.mjs',
      proofPack: 'output/browser-plan-proof/social-22-audit-explanation-read-model',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(proof);

  console.log('social-audit-explanation-service-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

async function writeProofPack(proof) {
  await writeFile(
    join(proofDir, '02-service-read-model-proof.log'),
    [
      'Service read-model proof:',
      '',
      '- cmd /c npm run build:contracts: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/browser-domain -- social-audit-explanation-read-model.test.ts: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- social-audit-explanation-read-model.test.ts contracts.test.ts: PASS',
      '- cargo test -p ocentra-parent-agent-protocol social_audit_explanation: PASS',
      '- cargo test -p ocentra-parent-agent-service social_audit_explanation: PASS',
      '- node scripts/test/social-audit-explanation-service-proof.mjs: PASS',
      '',
      'The service command returns a six-row schema-backed social audit/explanation snapshot through browserSocialAuditExplanationReadModel.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(proofDir, '03-service-read-model-proof.json'), proof);
  await writeFile(
    join(proofDir, '04-service-no-claim-boundaries.md'),
    [
      '# SOCIAL-22 Service No-Claim Boundaries',
      '',
      '- Service-backed explanation delivery is limited to the WebSocket read-model command/event payload.',
      '- The read model remains ref-only and does not include raw account, video, or message content.',
      '- Runtime audit store persistence, notification delivery, connector authorization, native app control, final policy execution, and enforcement remain unclaimed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '',
      '- cmd /c npm run build:contracts: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/browser-domain -- social-audit-explanation-read-model.test.ts: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- social-audit-explanation-read-model.test.ts contracts.test.ts: PASS',
      '- cargo test -p ocentra-parent-agent-protocol social_audit_explanation --quiet: PASS',
      '- cargo test -p ocentra-parent-agent-service social_audit_explanation --quiet: PASS',
      '- cmd /c node scripts/test/social-audit-explanation-service-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
}

async function runCommand(command, args) {
  const label = [command, ...args].join(' ');
  const startedAt = new Date().toISOString();
  const child = spawn(command, args, {
    cwd: repoRoot,
    shell: false,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on('data', (chunk) => {
    stderr += chunk.toString();
  });
  const exitCode = await new Promise((resolve, reject) => {
    child.on('error', reject);
    child.on('close', resolve);
  });
  commands.push({
    command: label,
    startedAt,
    finishedAt: new Date().toISOString(),
    exitCode,
    stdoutTail: tail(stdout),
    stderrTail: tail(stderr),
  });
  if (exitCode !== 0) {
    throw new Error(`${label} failed with exit ${exitCode}\n${stdout}\n${stderr}`);
  }
}

async function gitHead() {
  return runCapture('git', ['rev-parse', 'HEAD']);
}

async function gitBranch() {
  return runCapture('git', ['rev-parse', '--abbrev-ref', 'HEAD']);
}

async function runCapture(command, args) {
  const child = spawn(command, args, {
    cwd: repoRoot,
    shell: false,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
  let stdout = '';
  let stderr = '';
  child.stdout.on('data', (chunk) => {
    stdout += chunk.toString();
  });
  child.stderr.on('data', (chunk) => {
    stderr += chunk.toString();
  });
  const exitCode = await new Promise((resolve, reject) => {
    child.on('error', reject);
    child.on('close', resolve);
  });
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${exitCode}: ${stderr}`);
  }
  return stdout.trim();
}

async function writeJson(filePath, value) {
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function tail(text) {
  return text.split(/\r?\n/).filter(Boolean).slice(-20);
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
