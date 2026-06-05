import { spawn, spawnSync } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'tracking-plan-policy-escalation-runtime-proof';
const resultDir = path.join(repoRoot, 'test-results', proofMode);
const proofPath = path.join(resultDir, 'proof.json');
const workpack25 = path.join(repoRoot, 'output', 'tracking-plan-proof', '25-policy-compiler-for-tracking-rules');
const workpack27 = path.join(repoRoot, 'output', 'tracking-plan-proof', '27-escalation-engine');
const commands = [];

await main();

async function main() {
  await mkdir(resultDir, { recursive: true });
  await mkdir(workpack25, { recursive: true });
  await mkdir(workpack27, { recursive: true });

  await runNpm(['--workspace', '@ocentra-parent/parent-domain', 'run', 'build']);
  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'vitest',
    'run',
    'tests/tracking-policy-escalation-runtime-proof.test.ts',
  ]);

  const runtimeProof = await importRuntimeProof();
  const proof = buildProof(runtimeProof);
  await writeProofFiles(proof);

  console.log('tracking-plan-policy-escalation-runtime-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function importRuntimeProof() {
  const moduleUrl = pathToFileURL(
    path.join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-policy-escalation-runtime-proof.js')
  );
  const runtime = await import(moduleUrl.href);
  const proof = runtime.TrackingPolicyEscalationRuntimeProofReadModel;
  const summary = runtime.summarizeTrackingPolicyEscalationRuntimeProof(proof);

  assertSummary(summary);
  assertNonClaims(proof.nonClaims);

  return { proof, summary };
}

function buildProof({ proof, summary }) {
  return {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: gitHead(),
    proofMode,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    parentDomainModule: 'packages/parent-domain/src/tracking-policy-escalation-runtime-proof.ts',
    parentDomainTest: 'packages/parent-domain/tests/tracking-policy-escalation-runtime-proof.test.ts',
    runtimeSummary: summary,
    runtimeProof: proof,
    workpacks: {
      policyCompiler: workpackProof('25-policy-compiler-for-tracking-rules', summary),
      escalationEngine: workpackProof('27-escalation-engine', summary),
    },
    productClaimReady: false,
    nonClaims: [
      'AI risk analysis is advisory only and cannot directly trigger alerts',
      'Provider notification delivery is not attempted',
      'Emergency contact automation is not implemented or claimed',
      'Child-device runtime prompt delivery is not implemented or claimed',
      'Physical Android/iOS background behavior remains unproved',
      'Parent policy decisions remain the final authority in this proof',
    ],
    knownGaps: proof.knownGaps,
    commands,
  };
}

function workpackProof(workpackId, summary) {
  return {
    workpackId,
    requiredProofTier: 'P1_FIXTURE_SIMULATION',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    currentStatus: 'simulated',
    artifactPath: relativePath(proofPath),
    runtimeSummary: summary,
    productClaimReady: false,
    missingProofReason:
      'This is parent-domain runtime proof only. Provider delivery, emergency automation, child-device runtime, hosted UI, and physical-device proof remain pending.',
  };
}

async function writeProofFiles(proof) {
  await writeJson(proofPath, proof);
  await writeJson(path.join(workpack25, '09-policy-alert-proof.json'), proof.workpacks.policyCompiler);
  await writeJson(path.join(workpack27, '09-policy-alert-proof.json'), proof.workpacks.escalationEngine);
  await writeText(path.join(workpack25, '13-security-negative-proof.log'), securityNegativeLog(proof));
  await writeText(path.join(workpack27, '13-security-negative-proof.log'), securityNegativeLog(proof));
  await writeText(path.join(workpack25, '16-validation-commands.log'), validationLog());
  await writeText(path.join(workpack27, '16-validation-commands.log'), validationLog());
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args]);
    return;
  }
  await runCommand('npm', args);
}

async function runCommand(commandName, args) {
  const result = await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('error', reject);
    child.once('exit', (exitCode) => resolve({ exitCode: exitCode ?? 1 }));
  });
  commands.push({ command: `${commandName} ${args.join(' ')}`, exitCode: result.exitCode });
  if (result.exitCode !== 0) {
    throw new Error(`${commandName} ${args.join(' ')} failed with exit ${result.exitCode}`);
  }
}

function assertSummary(summary) {
  const expected = {
    outcomes: 5,
    parentPolicyAuthorityRows: 5,
    aiAuthorityRows: 0,
    providerDeliveryRows: 0,
    emergencyContactRows: 0,
    deviceRuntimeRows: 0,
    escalationRows: 2,
    resolvedRows: 3,
  };
  if (JSON.stringify(summary) !== JSON.stringify(expected)) {
    throw new Error(`Unexpected tracking policy escalation runtime summary: ${JSON.stringify(summary)}`);
  }
}

function assertNonClaims(nonClaims) {
  const required = [
    'no-provider-delivery-attempted',
    'no-emergency-contact-automation',
    'no-child-device-runtime',
    'no-background-location-claim',
    'no-physical-device-proof',
    'no-ai-final-authority',
  ];
  for (const requiredNonClaim of required) {
    if (!nonClaims.includes(requiredNonClaim)) {
      throw new Error(`Missing tracking policy escalation runtime non-claim: ${requiredNonClaim}`);
    }
  }
}

function securityNegativeLog(proof) {
  return [
    'workpacks=25-policy-compiler-for-tracking-rules,27-escalation-engine',
    'Existing contract proof keeps No Zod, manual brands, test doubles, app string literals, and schema-boundary violations out of the tracking contracts.',
    'Existing contract proof rejects precise coordinates from LAN/Wi-Fi/IP/manual/unknown hint-only tracking sources.',
    'Existing contract proof keeps AI location analysis as evidence only and prevents direct alert/final-authority claims.',
    'Existing contract proof keeps remote AI data disabled unless the route is parent-approved remote.',
    'tracking policy escalation runtime negative proof',
    `checkedAt=${proof.checkedAt}`,
    'providerDeliveryAttempted=true is rejected by TrackingPolicyEscalationRuntimeOutcomeSchema',
    'emergencyContactClaimed=true is rejected by TrackingPolicyEscalationRuntimeOutcomeSchema',
    'deviceRuntimeClaimed=true is rejected by TrackingPolicyEscalationRuntimeOutcomeSchema',
    'critical-still-alert with parentAlertSuppressed=true is rejected by runtime honesty filter',
    'missing required outcome coverage is rejected by TrackingPolicyEscalationRuntimeProofSchema',
    'missing required non-claim coverage is rejected by TrackingPolicyEscalationRuntimeProofSchema',
    '',
  ].join('\n');
}

function validationLog() {
  return [
    'Previous contract-proof validation is owned by node scripts/test/tracking-plan-contract-proof.mjs.',
    'Runtime proof validation commands:',
    ...commands.map((entry) => `${entry.command} exit=${entry.exitCode}`),
    '',
  ].join('\n');
}

async function writeJson(filePath, value) {
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

async function writeText(filePath, value) {
  await mkdir(path.dirname(filePath), { recursive: true });
  await writeFile(filePath, value);
}

function gitHead() {
  const result = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' });
  if (result.status !== 0) {
    return 'unknown';
  }
  return result.stdout.trim();
}

function relativePath(filePath) {
  return path.relative(repoRoot, filePath).replace(/\\/gu, '/');
}
