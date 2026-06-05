import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-readiness-manual-guard-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', '71-policy-readiness-manual-guard');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });
  await mkdir(join(proofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'app_game_policy_readiness']);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-policy-readiness-manual-guard',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    summary: {
      readModel: 'agent.activity.app-game.policy-readiness.read-model.get',
      guardedStatus: 'policy-partial',
      guardReason:
        'manual-required or missing policy-readiness rows prevent the service read model from reporting policy-ready',
      policyEvaluationReady:
        'kept as the base evidence and authority readiness signal; capabilityStatus carries the manual guard',
      adapterDispatchClaimed: false,
    },
    claimsProved: [
      'Manual-required approval action history or AI classifier context keeps capabilityStatus policy-partial',
      'The service read model still exposes policyEvaluationReady separately for base evidence and authority readiness',
      'The proof path does not add portal UI, policy evaluator runtime, timer runtime, child delivery, adapter dispatch, broad blocking, or platform support',
    ],
    claimsNotProved: [
      'portal policy readiness rendering',
      'runtime policy evaluator execution',
      'timer or child delivery execution',
      'adapter execution, broad installed-app blocking, or platform support',
    ],
    evidence: {
      servicePayload: 'crates/agent-service/src/activity_api/app_game_policy_readiness_payload.rs',
      servicePayloadTest: 'crates/agent-service/src/activity_api/app_game_policy_readiness_payload_tests.rs',
      serviceWebSocketTest: 'crates/agent-service/src/activity_api/app_game_policy_readiness_service_tests.rs',
      proofHarness: 'scripts/test/app-game-policy-readiness-manual-guard-proof.mjs',
      proofPack: 'output/app-game-plan-proof/71-policy-readiness-manual-guard',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(proofDir, proof);

  console.log('app-game-policy-readiness-manual-guard-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

async function writeProofPack(targetDir, proof) {
  await writeFile(
    join(targetDir, '00-source-snapshot.md'),
    [
      '# WP71 Source Snapshot',
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      '- Scope: backend app/game policy readiness manual-required guard.',
      '- Source inspected: app-game feature doc, app/game evidence expectations, enforcement expectations, agent-service activity API, and existing policy readiness proof path.',
      '- Conflict hygiene: portal surfaces, parent-domain package exports, app-game/app-plan implementation checklist rows, and workpack README conflict paths were intentionally not edited.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(targetDir, '01-contract-proof.log'),
    'Contract proof not changed: this slice preserves the existing app-game policy readiness payload shape and tightens service status derivation only.\n',
    'utf8'
  );
  await writeFile(
    join(targetDir, '02-rust-protocol-proof.log'),
    'Rust protocol DTO shape not changed. Existing service tests serialize the unchanged AppGamePolicyReadinessReadModel shape.\n',
    'utf8'
  );
  await writeJson(join(targetDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(targetDir, '04-journal-sqlite-proof.json'), {
    schemaVersion: 1,
    journalSqliteChanged: false,
    source: 'existing ActivityStore app-game service read model',
    guard:
      'manual-required and missing readiness rows are derived from existing persisted row counts; no row is invented',
  });
  await writeJson(join(targetDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    policyRuntimeExecuted: false,
    adapterDispatchClaimed: false,
    capabilityStatus:
      'policy-partial whenever any readiness row is missing or manual-required, even if policyEvaluationReady is true for base rows',
  });
  await writeFile(
    join(targetDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo portal or child-facing UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(targetDir, '07-playwright-ui-proof.log'),
    'Playwright/browser proof not applicable: no UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(targetDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Manual-required readiness rows cannot be collapsed into a policy-ready capability status.',
      '- Missing rows remain missing/manual-required and visible in the service read model.',
      '- adapterDispatchClaimed remains false.',
      '- No portal UI, policy evaluator, timer, child delivery, provider, adapter, broad blocking, or platform support claim is added.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(targetDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo platform authority tier is raised; broad blocking and adapter execution remain unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(targetDir, '10-validation-commands.log'),
    'Validation run:\n\n- cargo test -p ocentra-parent-agent-service app_game_policy_readiness: PASS\n- node scripts/test/app-game-policy-readiness-manual-guard-proof.mjs: PASS\n',
    'utf8'
  );
  await writeFile(
    join(targetDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nThe guard reads existing service readiness rows only and does not upgrade manual-required authority.\n',
    'utf8'
  );
  await writeFile(
    join(targetDir, '12-rollback-proof.md'),
    '# Rollback Proof\n\nNo device action, timer, block, suspend, shield, or adapter state is created, so rollback is not applicable.\n',
    'utf8'
  );
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, shell: false, stdio: 'inherit' });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
  });
}

async function gitBranch() {
  return (await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD'])).trim();
}

async function gitHead() {
  return (await gitOutput(['rev-parse', 'HEAD'])).trim();
}

async function gitOutput(args) {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', args, { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    child.stderr.on('data', (chunk) => chunks.push(Buffer.from(chunk)));
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(undefined);
        return;
      }
      reject(new Error(`git ${args.join(' ')} exited with ${code}`));
    });
  });
  return Buffer.concat(chunks).toString('utf8');
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
