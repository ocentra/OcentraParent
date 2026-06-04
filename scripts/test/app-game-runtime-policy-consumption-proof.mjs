import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-runtime-policy-consumption-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '50-runtime-policy-consumption');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '50-runtime-policy-consumption');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'enforcement-policy-dispatch',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'enforcement-policy-dispatch-adapter',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'enforcement_policy_dispatch']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'policy_dispatch']);

  const dispatch = await import('../../packages/parent-domain/dist/enforcement-policy-dispatch.js');
  const readModel = dispatch.EnforcementPolicyDispatchReadModelSchema.parse(
    dispatch.EnforcementPolicyDispatchReadModel
  );
  const dryRunEntry = readModel.entries.find(
    (entry) => entry.intent.intentId === 'dispatch-app-game-category-risk-dry-run'
  );

  assertPresent(dryRunEntry, 'app/game category-risk dry-run dispatch entry');
  assertEqual(dryRunEntry.intent.dryRun, true, 'dry-run flag');
  assertEqual(dryRunEntry.dispatchedAt, null, 'dispatchedAt');
  assertEqual(dryRunEntry.timerState, 'not-required', 'timer state');
  assertEqual(dryRunEntry.intent.target.targetValue, 'app-category:category-risk-route', 'target value');
  assertEqual(dryRunEntry.intent.requestedParentAction, 'dry-run-preview', 'requested parent action');
  assertEqual(dryRunEntry.matrixRow.surface, 'windows-policy-dry-run-preview', 'surface');
  assertEqual(dryRunEntry.matrixRow.capabilityState, 'dry-run', 'capability state');
  assertEqual(dryRunEntry.matrixRow.proofLevel, 'scaffold', 'proof level');
  assertEqual(dryRunEntry.matrixRow.outcomeState, 'dry-run-only', 'outcome state');
  assertEqual(dryRunEntry.matrixRow.rejectionReason, 'none', 'rejection reason');
  assertEqual(dryRunEntry.intent.evidenceReferences.length > 0, true, 'evidence reference count');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-runtime-policy-consumption',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    summary: {
      readModelId: readModel.readModelId,
      entryCount: readModel.entries.length,
      appGameCategoryRiskEntry: {
        intentId: dryRunEntry.intent.intentId,
        surface: dryRunEntry.matrixRow.surface,
        target: dryRunEntry.intent.target,
        requestedParentAction: dryRunEntry.intent.requestedParentAction,
        dryRun: dryRunEntry.intent.dryRun,
        outcomeState: dryRunEntry.matrixRow.outcomeState,
        adapterDispatch: 'not-dispatched',
      },
    },
    claimsProved: [
      'service-backed enforcement policy-dispatch read model carries an app/game category-risk policy consumption row',
      'category/risk policy consumption stays dry-run-only through the runtime policy-dispatch command path',
      'the app/game category-risk row cites policy-route evidence and does not set dispatchedAt',
      'the TypeScript protocol adapter accepts the service read model event with the new dry-run row',
      'Rust protocol parity serializes the policy-dispatch read model without row-state claim upgrades',
      'focused Rust service tests prove the websocket command returns the service read model',
    ],
    claimsNotProved: [
      'portal category/risk UI rendering',
      'live classifier/provider execution',
      'notification or child request delivery',
      'adapter execution, process termination, broad installed-app blocking, or platform enforcement support',
    ],
    evidence: {
      contract: 'packages/parent-domain/src/enforcement-policy-dispatch.ts',
      contractTest: 'packages/parent-domain/tests/enforcement-policy-dispatch.test.ts',
      protocolAdapterTest: 'packages/agent-protocol-domain/tests/enforcement-policy-dispatch-adapter.test.ts',
      rustConstants: 'crates/agent-protocol/src/constants/v08_enforcement_policy_dispatch.rs',
      rustProtocol: 'crates/agent-protocol/src/enforcement_policy_dispatch.rs',
      rustService: 'crates/agent-service/src/enforcement_policy_dispatch_read_model.rs',
      rustServiceTest: 'crates/agent-service/src/enforcement_policy_dispatch_read_model_tests.rs',
      harness: 'scripts/test/app-game-runtime-policy-consumption-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/50-runtime-policy-consumption',
      appProofPack: 'output/app-plan-proof/50-runtime-policy-consumption',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP50');
  await writeProofPack(appProofDir, proof, 'app WP50');

  console.log(`app-game-runtime-policy-consumption-proof-ok:${readModel.entries.length}`);
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
      '- Scope: service-backed app/game category-risk policy consumption in the enforcement policy-dispatch read model.',
      '- Source inspected: WP49 category/risk routing and existing V0.8 policy-dispatch command/event path.',
      '- Portal UI, notification delivery, live provider execution, and adapter execution are intentionally not changed.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- enforcement-policy-dispatch: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- enforcement-policy-dispatch-adapter: PASS',
      '- App/game category-risk policy row is dry-run-only, has evidence refs, and leaves dispatchedAt null.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    [
      'Rust/service protocol proof:',
      '',
      '- cargo test -p ocentra-parent-agent-protocol enforcement_policy_dispatch: PASS',
      '- cargo test -p ocentra-parent-agent-service policy_dispatch: PASS',
      '- Rust constants include the app/game category-risk dry-run intent, matrix, target, evidence, and child reason.',
      '- The websocket service command returns six policy-dispatch rows including the dry-run app/game row.',
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
        reason:
          'The runtime policy consumption row is a service read-model proof and does not create journal or SQLite mutations.',
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
        policyConsumption: proof.summary.appGameCategoryRiskEntry,
        noClaimBoundary: {
          adapterDispatch: 'not-dispatched',
          dryRunOnly: true,
          platformEnforcementClaimed: false,
          broadBlockingClaimed: false,
        },
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo portal, child-facing, or notification UI source changed in this workpack.\n',
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
      '- Category/risk policy consumption is represented as a dry-run policy preview only.',
      '- The app/game row does not set dispatchedAt and does not create timer refs.',
      '- The row cites evidence refs but does not execute adapters or broad blocking.',
      '- Portal rendering, notifications, child UX, provider execution, and platform support remain separate gaps.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nNo live platform enforcement proof is attached. Adapter execution and platform support remain unclaimed.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- enforcement-policy-dispatch: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- enforcement-policy-dispatch-adapter: PASS',
      '- cargo test -p ocentra-parent-agent-protocol enforcement_policy_dispatch: PASS',
      '- cargo test -p ocentra-parent-agent-service policy_dispatch: PASS',
      '- node scripts/test/app-game-runtime-policy-consumption-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nNo authority tier is raised. Policy consumption remains dry-run-only and evidence-backed.\n',
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

function assertPresent(value, label) {
  if (value === undefined || value === null) {
    throw new Error(`${label}: missing`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, got ${actual}`);
  }
}
