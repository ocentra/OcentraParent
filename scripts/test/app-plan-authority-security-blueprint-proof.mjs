import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'output', 'app-plan-proof', 'security-blueprint', 'authority-guards');
const testResultDir = join(repoRoot, 'test-results', 'app-plan-authority-security-blueprint-proof');
const proofPath = join(testResultDir, 'proof.json');
const commands = [];

const sourceProofs = [
  {
    appPlanRow: 'Security: weak evidence no-upgrade.',
    sourceProofPath: join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      'unknown-process-auto-promotion',
      'proof.json'
    ),
    expectedMode: 'app-game-unknown-process-auto-promotion-gate-proof',
    expectedGate: 'Unknown process is auto-promoted to known game.',
    requiredProductBoundaryFlags: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      unknownProcessPromotedToKnownGame: false,
      heuristicLabelDispatchesAdapter: false,
      manualReviewRequired: true,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
      runtimeAdapterClaimed: false,
    },
  },
  {
    appPlanRow: 'Security: manual-required guard.',
    sourceProofPath: join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      'manual-required-no-adapter',
      'proof.json'
    ),
    expectedMode: 'app-game-manual-required-no-adapter-gate-proof',
    expectedGate: 'Manual-required action calls an adapter.',
    requiredProductBoundaryFlags: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      manualRequiredCallsAdapter: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      runtimeAdapterClaimed: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
    },
  },
  {
    appPlanRow: 'Security: platform authority guard.',
    sourceProofPath: join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      'android-normal-mode-no-suspend-hide',
      'proof.json'
    ),
    expectedMode: 'app-game-android-normal-mode-no-suspend-hide-gate-proof',
    expectedGate: 'Android normal mode claims package suspend/hide.',
    requiredProductBoundaryFlags: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      androidNormalModeSuspendHideClaimed: false,
      androidDeviceOwnerProofAttached: false,
      androidProfileOwnerProofAttached: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
    },
  },
  {
    appPlanRow: 'Security: platform authority guard.',
    sourceProofPath: join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      'macos-hard-block-proof',
      'proof.json'
    ),
    expectedMode: 'app-game-macos-hard-block-proof-gate',
    expectedGate: 'macOS hard block is claimed without MDM/Endpoint/System Extension proof.',
    requiredProductBoundaryFlags: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      macosHardBlockClaimed: false,
      macosMdmProfileProofAttached: false,
      macosEndpointSecurityProofAttached: false,
      macosSystemExtensionProofAttached: false,
      rollbackProofAttached: false,
      auditProofAttached: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      browserGameWorkDuplicated: false,
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
    '@ocentra-parent/activity-domain',
    '--',
    'app-game-identity.test.ts',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/activity-domain',
    '--',
    'app-game-category-risk.test.ts',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-broad-blocking-proof-gates.test.ts',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-policy-preview-handoff.test.ts',
  ]);

  const checklist = await readText(join(repoRoot, 'docs', 'plans', 'app-plan', 'implementation-checklist.md'));
  const featureDoc = await readText(join(repoRoot, 'docs', 'features', 'app-game-control.md'));
  const mappedProofs = [];

  for (const proofConfig of sourceProofs) {
    const proof = JSON.parse(await readText(proofConfig.sourceProofPath));
    assert.equal(proof.proofMode, proofConfig.expectedMode);
    assert.equal(proof.gate, proofConfig.expectedGate);
    assert.equal(typeof proof.gateState, 'string');
    assert.notEqual(proof.gateState.trim(), '');
    assert.ok(proof.productBoundaries);

    for (const [flag, expected] of Object.entries(proofConfig.requiredProductBoundaryFlags)) {
      assert.equal(
        proof.productBoundaries[flag],
        expected,
        `${proofConfig.expectedMode} expected productBoundaries.${flag}=${expected}`
      );
    }

    assertChecklistRowChecked(checklist, proofConfig.appPlanRow);
    mappedProofs.push({
      appPlanRow: proofConfig.appPlanRow,
      sourceProof: relative(repoRoot, proofConfig.sourceProofPath),
      sourceProofMode: proof.proofMode,
      gate: proof.gate,
      gateState: proof.gateState,
      productBoundaries: proof.productBoundaries,
      proofPaths: proof.proofPaths,
    });
  }

  assert.match(
    featureDoc,
    /app-plan authority security blueprint/i,
    'Expected feature doc to record app-plan authority security blueprint reconciliation'
  );
  assertChecklistRowOpen(checklist, 'Security: stale evidence rejection.');

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-plan-authority-security-blueprint-proof',
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
      appPlanRows: [
        'Security: weak evidence no-upgrade.',
        'Security: manual-required guard.',
        'Security: platform authority guard.',
      ],
      intentionallyOpenRows: ['Security: stale evidence rejection.'],
      productChecklistChanged: false,
      packageExportsChanged: false,
      adapterDispatchClaimed: false,
      policyExecutionClaimed: false,
      platformSupportClaimed: false,
    },
    sourceProofs: mappedProofs,
    proofPaths: {
      proof: relative(repoRoot, proofPath),
      appPlanProofPack: relative(repoRoot, outputDir),
      harness: 'scripts/test/app-plan-authority-security-blueprint-proof.mjs',
    },
  };

  await writeFile(join(outputDir, '00-source-snapshot.md'), sourceSnapshot(branch, commit, status, mappedProofs));
  await writeFile(join(outputDir, '10-validation-commands.log'), `${commands.join('\n\n')}\n`);
  await writeFile(join(outputDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);

  console.log(`app-plan-authority-security-blueprint-proof-ok:${relative(repoRoot, proofPath)}`);
}

function sourceSnapshot(branch, commit, status, mappedProofs) {
  return [
    '# App-Plan Authority Security Blueprint Proof Source Snapshot',
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
    ...mappedProofs.flatMap((entry) => [
      `- ${entry.appPlanRow}: ${entry.sourceProof}`,
      `  - mode: ${entry.sourceProofMode}`,
      `  - gate: ${entry.gate}`,
      `  - gate state: ${entry.gateState}`,
    ]),
    '',
  ].join('\n');
}

function assertChecklistRowChecked(checklist, row) {
  const normalized = checklist.replace(/\s+/g, ' ');
  assert.match(normalized, new RegExp(`- \\[x\\] ${escapeRegExp(row)}`), `Expected checked app-plan row: ${row}`);
}

function assertChecklistRowOpen(checklist, row) {
  const normalized = checklist.replace(/\s+/g, ' ');
  assert.match(
    normalized,
    new RegExp(`- \\[ \\] ${escapeRegExp(row)}`),
    `Expected intentionally open app-plan row: ${row}`
  );
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
