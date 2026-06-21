import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-platform-extension-routing-proof');
const appGameProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '25-platform-extension-checklist-and-proof-routing'
);
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '24-platform-extension-checklist-and-proof-routing');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-platform-extension-routing',
    ])
  );

  const { AppGamePlatformExtensionRoutingMatrix } =
    await import('../../packages/app-game-domain/dist/app-game-platform-extension-routing-data.js');
  const { AppGamePlatformExtensionRoutingRowSchema } =
    await import('../../packages/app-game-domain/dist/app-game-platform-extension-routing.js');
  const summary = summarizeMatrix(AppGamePlatformExtensionRoutingMatrix);
  assertProof(AppGamePlatformExtensionRoutingMatrix, summary, AppGamePlatformExtensionRoutingRowSchema);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-platform-extension-routing',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    counts: summary,
    evidence: {
      tsContract: 'packages/app-game-domain/src/app-game-platform-extension-routing.ts',
      tsContractRules: 'packages/schema-domain/src/app-game-platform-extension-routing-rules.ts',
      tsContractData: 'packages/app-game-domain/src/app-game-platform-extension-routing-data.ts',
      tsContractDataSupport: 'packages/app-game-domain/src/app-game-platform-extension-routing-data-support.ts',
      tsContractPlatformData: [
        'packages/app-game-domain/src/app-game-platform-extension-routing-macos-data.ts',
        'packages/app-game-domain/src/app-game-platform-extension-routing-ios-data.ts',
        'packages/app-game-domain/src/app-game-platform-extension-routing-android-data.ts',
        'packages/app-game-domain/src/app-game-platform-extension-routing-linux-data.ts',
      ],
      tsContractTest: 'packages/app-game-domain/tests/unit/app-game-platform-extension-routing.test.ts',
      proofHarness: 'scripts/test/app-game-platform-extension-routing-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/25-platform-extension-checklist-and-proof-routing',
      appProofPack: 'output/app-plan-proof/24-platform-extension-checklist-and-proof-routing',
    },
    claimsProved: [
      'MAC-01 through MAC-12, IOS-01 through IOS-12, ANDROID-01 through ANDROID-14, and LINUX-01 through LINUX-14 are represented',
      'each extension row names platform, action scope, authority tier, setup state, manual tags, and proof pack refs',
      'each extension row cross-links the app-plan and app-game-plan proof routes',
      'stronger-than-observe rows name authority, setup, and rollback proof files before promotion',
      'promotion-ready rows are rejected unless every required proof file is attached',
      'bare unsupported labels are rejected',
    ],
    claimsNotProved: [
      'live macOS adapter execution',
      'live iOS FamilyControls or ManagedSettings execution',
      'live Android Device Owner or Profile Owner execution',
      'live Linux distro/session/mechanism execution',
      'portal platform matrix rendering',
      'store signing or entitlement approval',
    ],
  };
  const policyProof = {
    schemaVersion: 1,
    policyProofMode: 'app-game-platform-extension-promotion-gates',
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    canPromoteCount: summary.canPromoteCount,
    missingProofPromotionRejected: true,
    promotedCandidateRequiresAllProofFiles: true,
    strongRowsWithAuthoritySetupRollbackProofFiles: summary.strongRowsWithAuthoritySetupRollbackProofFiles,
    crossPlanRows: summary.rowCount,
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, policyProof, 'app-game WP25');
  await writeProofPack(appProofDir, proof, policyProof, 'app WP24');

  console.log(`app-game-platform-extension-routing-proof-ok:${summary.rowCount}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function summarizeMatrix(matrix) {
  const strongRows = matrix.rows.filter((row) => row.actionProofRequired || row.rollbackProofRequired);
  return {
    rowCount: matrix.rows.length,
    byPlatform: countBy(matrix.rows.map((row) => row.platform)),
    byProductScope: countBy(matrix.rows.map((row) => row.productScope)),
    byActionScope: countBy(matrix.rows.map((row) => row.actionScope)),
    byPromotionState: countBy(matrix.rows.map((row) => row.promotionState)),
    canPromoteCount: matrix.rows.filter((row) => row.canPromote).length,
    proofReferenceCount: matrix.rows.reduce((count, row) => count + row.proofReferences.length, 0),
    strongRowsWithAuthoritySetupRollbackProofFiles: strongRows.filter((row) =>
      ['11-authority-tier-proof.md', '12-permission-setup-proof.md', '13-rollback-proof.md'].every((proofFile) =>
        row.requiredProofFiles.includes(proofFile)
      )
    ).length,
    strongRowCount: strongRows.length,
    rowsWithCrossPlanRefs: matrix.rows.filter(
      (row) => row.appPlanProofPackRef.includes(row.rowId) && row.appGameProofPackRef.includes(row.rowId)
    ).length,
  };
}

function assertProof(matrix, summary, rowSchema) {
  assertEqual(String(matrix.matrixId), 'app-game-platform-extension-proof-routing', 'matrix id');
  assertEqual(summary.rowCount, 52, 'row count');
  assertEqual(summary.byPlatform.macos, 12, 'macOS count');
  assertEqual(summary.byPlatform.ios, 12, 'iOS count');
  assertEqual(summary.byPlatform.android, 14, 'Android count');
  assertEqual(summary.byPlatform.linux, 14, 'Linux count');
  assertEqual(summary.canPromoteCount, 0, 'promoted row count');
  assertEqual(summary.proofReferenceCount, 0, 'attached proof count');
  assertEqual(summary.strongRowsWithAuthoritySetupRollbackProofFiles, summary.strongRowCount, 'strong row proof gates');
  assertEqual(summary.rowsWithCrossPlanRefs, summary.rowCount, 'cross-plan proof refs');

  const androidSuspend = matrix.rows.find((row) => row.rowId === 'ANDROID-09');

  if (androidSuspend === undefined) {
    throw new Error('missing ANDROID-09 row');
  }

  assertEqual(
    rowSchema.safeParse({
      ...androidSuspend,
      capabilityState: 'supported',
      canPromote: true,
      promotionState: 'promotion-ready',
      proofReferences: [],
    }).success,
    false,
    'missing proof promotion'
  );
}

async function writeProofPack(proofDir, proof, policyProof, label) {
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `- Branch: ${await gitBranch()}`,
      `- Commit: ${proof.commit}`,
      '- Worktree status: tracked files generated by this proof pack; unrelated local `.codex` proof artifacts intentionally excluded.',
      '- Scope: platform extension proof-routing contract only; no live adapter support moved up.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '01-contract-proof.log'),
    [
      'Contract proof:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/app-game-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/app-game-domain -- app-game-platform-extension-routing: PASS',
      '- Matrix rows: 52',
      '- Platforms: macOS 12, iOS 12, Android 14, Linux 14',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '02-rust-protocol-proof.log'),
    'Rust/service protocol not changed. This workpack adds app-game-domain proof routing backed by centralized schema rules only; platform service parity remains future work.\n',
    'utf8'
  );
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(proofDir, '05-policy-action-proof.json'), policyProof);
  await writeFile(
    join(proofDir, '04-journal-sqlite-proof.json'),
    `${JSON.stringify({ schemaVersion: 1, journalSqliteChanged: false, reason: 'Extension routing contract only; no journal or SQLite behavior changed.' }, null, 2)}\n`,
    'utf8'
  );
  await writeFile(
    join(proofDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo parent portal, child UI, policy authoring, or platform setup screen changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '07-playwright-ui-proof.log'),
    'Playwright not applicable: no UI source changed. Platform labels are contract-only proof routes.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '08-security-negative-proof.log'),
    [
      'Security/no-claim proof:',
      '',
      '- Promotion-ready rows without attached required proof are rejected.',
      '- Bare Unsupported / Not supported labels are rejected.',
      '- All current extension rows have canPromote=false and proofReferences=[].',
      '- Android VPN/DNS row is not-claimed in app/game scope and routes to network/browser proof before product claims.',
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
      'Every MAC/IOS/ANDROID/LINUX extension row remains extension-checklist, manual-required, or not-claimed until the row-specific proof pack contains setup, action, rollback, and audit evidence.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '10-validation-commands.log'),
    [
      'Validation run:',
      '',
      '- cmd /c npm run build --workspace @ocentra-parent/app-game-domain: PASS',
      '- cmd /c npm run test --workspace @ocentra-parent/app-game-domain -- app-game-platform-extension-routing: PASS',
      '- node scripts/test/app-game-platform-extension-routing-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(proofDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nRows name authority tier and setup state. No row moves to supported because no row-specific live authority proof is attached.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '12-permission-setup-proof.md'),
    '# Permission Setup Proof\n\nRows name setup requirements such as permission, MDM, supervision, Device Owner, Profile Owner, system extension, admin/root, kiosk, and store/signing. Setup proof is not attached in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(proofDir, '13-rollback-proof.md'),
    '# Rollback Proof\n\nRows that are stronger than observe-only require rollback proof before promotion. No live rollback proof is attached in this workpack.\n',
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
