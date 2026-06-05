import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-evaluator-service-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '57-policy-evaluator-service-read-model');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '57-policy-evaluator-service-read-model');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'app-game-policy-evaluation',
    'contracts',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'app_game_policy_evaluation']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'app_game_policy_evaluation']);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-policy-evaluator-service-read-model',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    summary: {
      command: 'agent.activity.app-game.policy-evaluation.read-model.get',
      event: 'agent.activity.app-game.policy-evaluation.read-model.reported',
      payloadField: 'appGamePolicyEvaluationReadModel',
      evaluationRows: ['timeLimit', 'approvalRequest', 'categoryRiskReview', 'blockLaunch'],
      dryRun: true,
      enforcementHandoffState: 'disabled',
      adapterDispatchClaimed: false,
    },
    claimsProved: [
      'TypeScript protocol parses the dedicated app/game policy evaluation event payload',
      'Rust protocol serializes dry-run policy evaluation rows with adapterDispatchClaimed=false',
      'Agent service answers the policy evaluation command from the existing app/game service read model',
      'Evaluation rows consume the existing policy readiness model instead of creating a second query-store truth',
      'Time-limit and approval-request rows can become dry-run ready when required readiness rows exist',
      'Category/risk and block-launch rows stay manual-required when classifier or platform proof is missing',
    ],
    claimsNotProved: [
      'parent portal policy evaluation rendering',
      'parent rule authoring or durable policy persistence',
      'local classifier provider execution or model quality',
      'notification or child request delivery',
      'adapter execution, broad installed-app blocking, or platform support',
    ],
    evidence: {
      typescriptContract: 'packages/agent-protocol-domain/src/app-game-policy-evaluation.ts',
      typescriptTest: 'packages/agent-protocol-domain/tests/app-game-policy-evaluation.test.ts',
      rustProtocol: 'crates/agent-protocol/src/app_game_policy_evaluation.rs',
      rustProtocolTest: 'crates/agent-protocol/src/app_game_policy_evaluation_tests.rs',
      servicePayload: 'crates/agent-service/src/activity_api/app_game_policy_evaluation_payload.rs',
      servicePayloadTest: 'crates/agent-service/src/activity_api/app_game_policy_evaluation_payload_tests.rs',
      serviceWebSocketTest: 'crates/agent-service/src/activity_api/app_game_policy_evaluation_service_tests.rs',
      harness: 'scripts/test/app-game-policy-evaluator-service-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/57-policy-evaluator-service-read-model',
      appProofPack: 'output/app-plan-proof/57-policy-evaluator-service-read-model',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP57');
  await writeProofPack(appProofDir, proof, 'app WP57');

  console.log('app-game-policy-evaluator-service-proof-ok');
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      '- Scope: service-backed app/game policy evaluator read model.',
      '- Source inspected: existing app/game policy readiness model, activity API, WebSocket routing, and app/game service model rows.',
      '- Portal UI, parent rule persistence, notification delivery, adapters, broad blocking, and platform support are intentionally not changed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-policy-evaluation contracts: PASS',
      '- Parser accepts only the dedicated policy evaluation event and rejects adapterDispatchClaimed=true or dryRun=false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    [
      'Rust protocol proof:',
      '',
      '- cargo test -p ocentra-parent-agent-protocol app_game_policy_evaluation: PASS',
      '- DTO serialization preserves dryRun=true, enforcementHandoffState=disabled, and adapterDispatchClaimed=false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeFile(
    join(proofDir, '04-journal-sqlite-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        journalSqliteChanged: false,
        serviceModelSource: 'ActivityStore::app_game_service_read_model',
        policyReadinessSource:
          'app_game_policy_readiness_from_service_model is the single readiness source consumed by evaluation rows',
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '05-policy-action-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        dryRun: true,
        enforcementHandoffState: 'disabled',
        adapterDispatchClaimed: false,
        evaluatedRows: [
          {
            evaluationKind: 'timeLimit',
            requestedAction: 'time-limit',
            result:
              'dry-run-ready when policy evidence, approval authority, and platform authority readiness rows are ready',
          },
          {
            evaluationKind: 'approvalRequest',
            requestedAction: 'ask-parent',
            result: 'dry-run-ready when policy evidence and approval authority readiness rows are ready',
          },
          {
            evaluationKind: 'categoryRiskReview',
            requestedAction: 'warn',
            result: 'manual-required until classifier context readiness is ready',
          },
          {
            evaluationKind: 'blockLaunch',
            requestedAction: 'block-launch',
            result: 'manual-required with no adapter dispatch until platform proof exists',
          },
        ],
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo parent portal or child-facing UI source changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright/browser proof not applicable: no UI source changed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Evaluation rows are derived from the existing service policy readiness read model.',
      '- Missing readiness stays manual-required instead of fabricating evaluator authority.',
      '- dryRun is fixed true and enforcementHandoffState is fixed disabled.',
      '- adapterDispatchClaimed is fixed false in TypeScript schema, Rust DTO construction, service payload tests, and WebSocket tests.',
      '- The WebSocket command reports evaluation readiness only; it does not execute policy, call adapters, send notifications, or claim platform support.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo live platform authority tier is raised. Broad blocking and platform adapter execution remain unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-policy-evaluation contracts: PASS',
      '- cargo test -p ocentra-parent-agent-protocol app_game_policy_evaluation: PASS',
      '- cargo test -p ocentra-parent-agent-service app_game_policy_evaluation: PASS',
      '- node scripts/test/app-game-policy-evaluator-service-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nThe evaluator consumes existing readiness rows only. It does not upgrade manual-required or not-claimed platform authority.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-rollback-proof.md'),
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
