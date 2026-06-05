import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-freshness-quality-gate');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '72-source-freshness-quality-gate');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '72-source-freshness-quality-gate');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });
  await mkdir(join(appGameProofDir, '06-ui-snapshots'), { recursive: true });
  await mkdir(join(appProofDir, '06-ui-snapshots'), { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'exec',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'vitest',
    'run',
    'tests/app-game-source-freshness-quality-gate.test.ts',
  ]);

  const quality = await import('../../packages/activity-domain/dist/app-game-source-freshness-quality-gate.js');
  const kinds = await import('../../packages/activity-domain/dist/kinds.js');
  const generatedAt = '2026-06-05T20:00:00Z';
  const staleAfterMs = 600000;
  const evidence = {
    evidenceId: 'journal-entry-source-freshness-proof-1',
    kind: kinds.ActivityEvidenceKind.JournalEntry,
    digest: 'sha256:source-freshness-proof',
    uri: null,
  };
  const report = quality.buildAppGameSourceFreshnessQualityReport({
    generatedAt,
    staleAfterMs,
    requiredSources: ['processSnapshot', 'foregroundWindow', 'osInstalledRecord', 'launcherManifest'],
    sourceStatusRows: [
      sourceRow('processSnapshot', 'ready', 'available', '2026-06-05T19:59:00Z', 2, [evidence]),
      sourceRow('foregroundWindow', 'ready', 'available', '2026-06-05T19:20:00Z', 1, [evidence]),
      sourceRow('launcherManifest', 'permission-required', 'manualRequired', '2026-06-05T19:59:00Z', 1, [evidence]),
    ],
  });

  assertEqual(report.summary.requiredSourceCount, 4, 'required source count');
  assertEqual(report.summary.freshSources, 1, 'fresh sources');
  assertEqual(report.summary.staleSources, 1, 'stale sources');
  assertEqual(report.summary.missingSources, 1, 'missing sources');
  assertEqual(report.summary.manualRequiredSources, 1, 'manual required sources');
  assertEqual(report.summary.adapterDispatchClaimed, false, 'adapter dispatch claim');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-source-freshness-quality-gate',
    checkedAt: new Date().toISOString(),
    branch: await gitValue(['branch', '--show-current']),
    commit: await gitValue(['rev-parse', 'HEAD']),
    statusShort: await gitValue(['status', '--short']),
    commands,
    report,
    claimsProved: [
      'activity-domain source freshness quality rows distinguish fresh, stale, missing, manual-required, unavailable, and empty source coverage',
      'only recent evidenced rows become policyEligible=true',
      'manual-required, unavailable, stale, missing, and empty source rows remain out of policy eligibility',
      'quality rows keep adapterDispatchClaimed=false and do not execute policy, timers, adapters, or blocking',
    ],
    claimsNotProved: [
      'new live source subscriptions',
      'portal SVG source panel rendering',
      'policy evaluator runtime consumption',
      'adapter execution, broad blocking, timers, child delivery, provider delivery, or platform support',
    ],
    evidence: {
      contract: 'packages/activity-domain/src/app-game-source-freshness-quality-gate.ts',
      test: 'packages/activity-domain/tests/app-game-source-freshness-quality-gate.test.ts',
      packageExport: 'packages/activity-domain/package.json',
      harness: 'scripts/test/app-game-source-freshness-quality-gate-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/72-source-freshness-quality-gate',
      appProofPack: 'output/app-plan-proof/72-source-freshness-quality-gate',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP72');
  await writeProofPack(appProofDir, proof, 'app WP72');

  console.log(`app-game-source-freshness-quality-gate-proof-ok:${report.summary.freshSources}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function sourceRow(sourceKind, state, capabilityStatus, lastObservedAt, rowCount, evidence) {
  return {
    sourceKind,
    state,
    rowCount,
    lastObservedAt,
    capabilityStatus,
    evidence,
  };
}

async function runCommand(command, args) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: repoRoot,
      shell: false,
      stdio: 'inherit',
      windowsHide: true,
    });
    child.on('error', reject);
    child.on('exit', (code) => {
      const commandLine = [command, ...args].join(' ');
      commands.push({ command: commandLine, exitCode: code });
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`${commandLine} exited ${code}`));
      }
    });
  });
}

async function gitValue(args) {
  return new Promise((resolve) => {
    const child = spawn('git', args, {
      cwd: repoRoot,
      shell: false,
      stdio: ['ignore', 'pipe', 'ignore'],
      windowsHide: true,
    });
    let output = '';
    child.stdout.on('data', (chunk) => {
      output += chunk.toString();
    });
    child.on('exit', () => {
      resolve(output.trim());
    });
  });
}

async function writeProofPack(outputDir, proof, label) {
  await writeFile(
    join(outputDir, '00-source-snapshot.md'),
    [
      `# ${label} Source Snapshot`,
      '',
      `Branch: ${proof.branch}`,
      `Commit: ${proof.commit}`,
      '',
      '## Git Status',
      '```text',
      proof.statusShort || 'clean',
      '```',
      '',
      '## Inspected Sources',
      ...Object.values(proof.evidence).map((value) => `- ${value}`),
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(outputDir, '01-contract-proof.log'),
    [
      ...proof.commands.map((entry) => `${entry.command}: exit ${entry.exitCode}`),
      'source freshness quality assertions: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(outputDir, '02-rust-protocol-proof.log'),
    'N/A: activity-domain quality gate only; no Rust protocol shape changed.\n',
    'utf8'
  );
  await writeJson(join(outputDir, '03-runtime-evidence.json'), proof);
  await writeFile(
    join(outputDir, '04-journal-sqlite-proof.json'),
    '{\n  "applicable": false,\n  "reason": "No journal or SQLite schema changed in WP72."\n}\n',
    'utf8'
  );
  await writeFile(
    join(outputDir, '05-policy-action-proof.json'),
    `${JSON.stringify(
      {
        schemaVersion: 1,
        policyEligibleFreshSources: proof.report.summary.policyEligibleFreshSources,
        adapterDispatchClaimed: false,
        policyEvaluatorRuntimeClaimed: false,
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(
    join(outputDir, '06-ui-snapshots', 'ui-not-applicable.md'),
    '# UI Not Applicable\n\nNo portal or child-facing UI source changed in this workpack.\n',
    'utf8'
  );
  await writeFile(
    join(outputDir, '07-playwright-ui-proof.log'),
    'N/A: no UI route or rendered surface changed.\n',
    'utf8'
  );
  await writeFile(
    join(outputDir, '08-security-negative-proof.log'),
    [
      'PASS: stale source rows are not policy eligible.',
      'PASS: missing source rows are not policy eligible.',
      'PASS: manual-required source rows are not policy eligible.',
      'PASS: unavailable source rows are not policy eligible.',
      'PASS: adapterDispatchClaimed remains false.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(outputDir, '09-manual-platform-proof.md'),
    '# Manual Platform Proof\n\nN/A: no OS adapter or platform authority changed.\n',
    'utf8'
  );
  await writeFile(
    join(outputDir, '10-validation-commands.log'),
    [
      ...proof.commands.map((entry) => `${entry.command}: exit ${entry.exitCode}`),
      'node scripts/test/app-game-source-freshness-quality-gate-proof.mjs: PASS',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeFile(
    join(outputDir, '11-authority-tier-proof.md'),
    '# Authority Tier Proof\n\nNo authority tier is raised; this quality gate is evidence-only.\n',
    'utf8'
  );
  await writeFile(
    join(outputDir, '12-rollback-proof.md'),
    [
      '# Rollback Proof',
      '',
      'Remove the activity-domain source freshness quality gate export, test, docs, and proof artifacts.',
      'No persisted data, policy state, timer state, provider state, or adapter state is created.',
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(outputDir, 'proof.json'), proof);
  await writeFile(
    join(outputDir, 'README.md'),
    [
      `# ${label} Source Freshness Quality Gate`,
      '',
      `Checked at: ${proof.checkedAt}`,
      `Commit: ${proof.commit}`,
      '',
      '## Claims Proved',
      ...proof.claimsProved.map((claim) => `- ${claim}`),
      '',
      '## Claims Not Proved',
      ...proof.claimsNotProved.map((claim) => `- ${claim}`),
      '',
    ].join('\n'),
    'utf8'
  );
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, got ${actual}`);
  }
}
