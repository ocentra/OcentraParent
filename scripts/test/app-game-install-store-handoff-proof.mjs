import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-install-store-handoff-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '26-install-uninstall-purchase-store-handoffs');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '25-install-and-uninstall-approval-handoff');
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
    'app-game-install-store-handoff',
  ]);

  const { AppGameInstallStoreHandoffProofMatrix } =
    await import('../../packages/parent-domain/dist/app-game-install-store-handoff-proof.js');
  const { AppGameInstallStoreHandoffMatrixSchema, AppGameInstallStoreHandoffRowSchema } =
    await import('../../packages/parent-domain/dist/app-game-install-store-handoff.js');
  const matrix = AppGameInstallStoreHandoffMatrixSchema.parse(AppGameInstallStoreHandoffProofMatrix);
  const summary = summarizeMatrix(matrix);
  assertProof(matrix, summary, AppGameInstallStoreHandoffRowSchema);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-install-store-handoff',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-install-store-handoff.ts',
      rules: 'packages/parent-domain/src/app-game-install-store-handoff-rules.ts',
      proofMatrix: 'packages/parent-domain/src/app-game-install-store-handoff-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-install-store-handoff.test.ts',
      proofHarness: 'scripts/test/app-game-install-store-handoff-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/26-install-uninstall-purchase-store-handoffs',
      appProofPack: 'output/app-plan-proof/25-install-and-uninstall-approval-handoff',
    },
    claimsProved: [
      'new app inventory, installer/updater, store package, game purchase, uninstall, and tamper signals route through one typed handoff matrix',
      'store and purchase signals are context-only and cannot become supported policy decisions',
      'install approval handoff refs must carry evidence references and route to the install/purchase approval feature',
      'uninstall and tamper signals route to the enforcement integrity/tamper feature without approval refs',
      'manual-required rows carry parent-visible manual state and cannot claim adapter execution',
    ],
    claimsNotProved: [
      'live store integration',
      'Google Play, Apple App Store, Microsoft Store, or package manager interception',
      'billing entitlement logic',
      'portal approval UI',
      'platform adapter execution',
      'uninstall blocking or anti-tamper behavior',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP26');
  await writeProofPack(appProofDir, proof, 'app WP25');

  console.log(`app-game-install-store-handoff-proof-ok:${summary.rowCount}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function summarizeMatrix(matrix) {
  return {
    rowCount: matrix.rows.length,
    bySignalKind: countBy(matrix.rows.map((row) => row.signalKind)),
    byProductSlice: countBy(matrix.rows.map((row) => row.productSlice)),
    byDecisionAuthority: countBy(matrix.rows.map((row) => row.decisionAuthority)),
    contextOnlyStoreSignals: matrix.rows.filter((row) => row.storeSignalUse === 'context-only-not-decision').length,
    evidenceBackedApprovalHandoffs: matrix.rows.filter(
      (row) => row.approvalRequestRef !== null && row.evidenceReferences.length > 0
    ).length,
    tamperHandoffs: matrix.rows.filter((row) =>
      row.destinationFeatureDocs.includes('docs/features/enforcement-integrity-tamper.md')
    ).length,
    adapterClaims: matrix.rows.filter((row) => row.adapterExecutionClaim !== 'not-claimed').length,
  };
}

function assertProof(matrix, summary, rowSchema) {
  assertEqual(String(matrix.matrixId), 'app-game-install-store-handoff-proof', 'matrix id');
  assertEqual(summary.rowCount, 6, 'row count');
  assertEqual(summary.contextOnlyStoreSignals, 2, 'context-only store signals');
  assertEqual(summary.evidenceBackedApprovalHandoffs, 4, 'evidence-backed approval handoffs');
  assertEqual(summary.tamperHandoffs, 2, 'tamper handoffs');
  assertEqual(summary.adapterClaims, 0, 'adapter claims');

  const storeInstall = matrix.rows.find((row) => row.handoffId === 'store-package-install-context-handoff');

  if (storeInstall === undefined) {
    throw new Error('missing store-package-install-context-handoff row');
  }

  assertEqual(rowSchema.safeParse({ ...storeInstall, capabilityState: 'supported' }).success, false, 'store support');
}

async function writeProofPack(proofDir, proof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      '- Scope: install, uninstall, purchase, and store handoff contract proof only.',
      '- No package export map changed because E-B owns the parent-domain package export lock.',
      '- No app-install-purchase approval source changed because E-C owns the child-facing approval proof lock.',
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
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-install-store-handoff: PASS',
      '- Matrix rows: 6',
      '- Store/purchase context-only rows: 2',
      '- Evidence-backed approval handoff rows: 4',
      '- Tamper/uninstall handoff rows: 2',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust/service protocol not changed. This workpack adds parent-domain handoff contracts only; runtime and WebSocket parity remain future work.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeFile(
    join(proofDir, '04-journal-sqlite-proof.json'),
    `${JSON.stringify({ schemaVersion: 1, journalSqliteChanged: false, reason: 'Handoff contract proof only; no journal or SQLite behavior changed.' }, null, 2)}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '05-policy-action-proof.json'),
    `${JSON.stringify({ schemaVersion: 1, policyDecisionClaim: 'not-claimed', adapterExecutionClaim: 'not-claimed', contextOnlyStoreSignals: proof.summary.contextOnlyStoreSignals }, null, 2)}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo parent portal, child UI, policy authoring, approval, or evidence drawer source changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright not applicable: no UI source changed. Parent-visible states are domain-contract proof only.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Store and purchase signals are context-only, not safety decisions.',
      '- Approval handoffs require evidence refs.',
      '- Uninstall/tamper rows route to tamper docs without anti-tamper claims.',
      '- Adapter execution and policy decision claims remain not-claimed.',
      '- Billing entitlement logic remains out of child safety decisions.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    [
      '# Manual Platform Proof',
      '',
      'No live platform proof is attached in this workpack.',
      'Google Play, Apple App Store, Microsoft Store, package-manager, MDM, Device Owner/Profile Owner, and uninstall-block claims remain manual-required until separate platform artifacts exist.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/parent-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-install-store-handoff: PASS',
      '- node scripts/test/app-game-install-store-handoff-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
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
  const output = await gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']);
  return output.trim();
}

async function gitHead() {
  const output = await gitOutput(['rev-parse', 'HEAD']);
  return output.trim();
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

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, got ${actual}`);
  }
}
