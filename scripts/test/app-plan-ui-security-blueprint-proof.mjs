import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'app-plan-proof', 'security-blueprint', 'ui-safety');
const testResultDir = join(repoRoot, 'test-results', 'app-plan-ui-security-blueprint-proof');
const proofPath = join(testResultDir, 'proof.json');
const sourceSnapshotPath = join(outputDir, '00-source-snapshot.md');
const validationLogPath = join(outputDir, '10-validation-commands.log');
const outputProofPath = join(outputDir, 'proof.json');
const commands = [];

const gateProofs = [
  {
    appPlanRow: 'Security: path redaction.',
    sourceProofPath: join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      'raw-executable-path-ui-leak',
      'proof.json'
    ),
    expectedMode: 'app-game-raw-executable-path-ui-leak-gate-proof',
    expectedGate: 'Raw private executable paths leak into parent UI.',
    requiredProductBoundaryFlags: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      rawPrivateExecutablePathExposed: false,
      executablePathRefRendered: false,
      browserGameWorkDuplicated: false,
      adapterDispatchClaimed: false,
      policyExecutionClaimed: false,
      packageExportsChanged: false,
    },
  },
  {
    appPlanRow: 'Security: malicious metadata escaping.',
    sourceProofPath: join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      'malicious-metadata-ui-safety',
      'proof.json'
    ),
    expectedMode: 'app-game-malicious-metadata-ui-safety-gate-proof',
    expectedGate: 'Malicious app/game metadata causes XSS or layout breakage.',
    requiredProductBoundaryFlags: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      maliciousMetadataExecuted: false,
      dangerousHtmlSinkUsed: false,
      layoutRowsBounded: true,
      labelsTruncated: true,
      manualRequiredPreserved: true,
      browserGameWorkDuplicated: false,
      adapterDispatchClaimed: false,
      policyExecutionClaimed: false,
      packageExportsChanged: false,
    },
  },
];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(testResultDir, { recursive: true });

  const branch = await commandOutput('git', ['branch', '--show-current']);
  const commit = await commandOutput('git', ['rev-parse', 'HEAD']);
  const status = await commandOutput('git', ['status', '--short']);

  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'activity-ui-app-game-dashboard-intent.test.ts',
  ]);

  const checklist = await readText(join(repoRoot, 'docs', 'plans', 'app-plan', 'implementation-checklist.md'));
  const featureDoc = await readText(join(repoRoot, 'docs', 'features', 'app-game-control.md'));
  const sourceProofs = [];

  for (const gateProof of gateProofs) {
    const proof = JSON.parse(await readText(gateProof.sourceProofPath));
    assert.equal(proof.proofMode, gateProof.expectedMode);
    assert.equal(proof.gate, gateProof.expectedGate);
    assert.equal(typeof proof.gateState, 'string');
    assert.notEqual(proof.gateState.trim(), '');
    assert.ok(proof.productBoundaries);

    for (const [flag, expected] of Object.entries(gateProof.requiredProductBoundaryFlags)) {
      assert.equal(
        proof.productBoundaries[flag],
        expected,
        `${gateProof.expectedMode} expected productBoundaries.${flag}=${expected}`
      );
    }

    assertChecklistRowChecked(checklist, gateProof.appPlanRow);
    sourceProofs.push({
      appPlanRow: gateProof.appPlanRow,
      sourceProof: relative(repoRoot, gateProof.sourceProofPath),
      sourceProofMode: proof.proofMode,
      gateState: proof.gateState,
      proofPaths: proof.proofPaths,
      productBoundaries: proof.productBoundaries,
    });
  }

  assert.match(
    featureDoc,
    /app-plan UI security blueprint/i,
    'Expected feature doc to record app-plan UI security blueprint reconciliation'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-plan-ui-security-blueprint-proof',
    generatedAt: new Date().toISOString(),
    branch: branch.trim(),
    commit: commit.trim(),
    gitStatusShort: status.trim(),
    commands,
    scope: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      browserGameWorkDuplicated: false,
      appPlanRows: gateProofs.map((gateProof) => gateProof.appPlanRow),
      productChecklistChanged: false,
      packageExportsChanged: false,
      adapterDispatchClaimed: false,
      policyExecutionClaimed: false,
    },
    sourceProofs,
    proofPaths: {
      proof: relative(repoRoot, proofPath),
      appPlanProofPack: relative(repoRoot, outputDir),
      harness: 'scripts/test/app-plan-ui-security-blueprint-proof.mjs',
    },
  };

  await writeFile(
    sourceSnapshotPath,
    [
      '# App-Plan UI Security Blueprint Proof Source Snapshot',
      '',
      `Branch: ${branch.trim()}`,
      `Commit: ${commit.trim()}`,
      '',
      '## Git Status',
      '',
      '```text',
      status.trim() || '(clean)',
      '```',
      '',
      '## Source Proofs',
      '',
      ...sourceProofs.flatMap((proofEntry) => [
        `- ${proofEntry.appPlanRow}: ${proofEntry.sourceProof}`,
        `  - mode: ${proofEntry.sourceProofMode}`,
        `  - gate state: ${proofEntry.gateState}`,
      ]),
      '',
    ].join('\n')
  );
  await writeFile(validationLogPath, `${commands.join('\n\n')}\n`);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(outputProofPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log(`app-plan-ui-security-blueprint-proof-ok:${relative(repoRoot, proofPath)}`);
}

function assertChecklistRowChecked(checklist, row) {
  const normalized = checklist.replace(/\s+/g, ' ');
  assert.match(normalized, new RegExp(`- \\[x\\] ${escapeRegExp(row)}`), `Expected checked app-plan row: ${row}`);
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

async function readText(path) {
  return readFile(path, 'utf8');
}

async function commandOutput(command, args) {
  const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(chunk));
  child.stderr.on('data', (chunk) => chunks.push(chunk));
  const exitCode = await new Promise((resolve) => {
    child.on('close', resolve);
  });
  const output = Buffer.concat(chunks).toString('utf8');
  if (exitCode !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${exitCode}\n${output}`);
  }
  return output;
}

async function runCommand(command, args) {
  const output = await commandOutput(command, args);
  commands.push(`${command} ${args.join(' ')}\n${output.trim()}`);
}
