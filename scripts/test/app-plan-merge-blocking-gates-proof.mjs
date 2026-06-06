import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-plan-merge-blocking-gates-proof');
const proofDir = join(repoRoot, 'output', 'app-plan-proof', 'merge-gates', 'app-plan-merge-blocking-gates');
const commands = [];

const gates = [
  {
    appPlanGate: 'Inventory evidence is displayed as app usage.',
    appGameProofDir: 'inventory-display',
    appGameGate: 'Inventory evidence is displayed as app/game usage.',
    preventedBy: 'inventory-display-boundary',
  },
  {
    appPlanGate: 'Running evidence is displayed as foreground usage.',
    appGameProofDir: 'running-foreground-display',
    appGameGate: 'Running evidence is displayed as foreground usage.',
    preventedBy: 'running-foreground-boundary',
  },
  {
    appPlanGate: 'Foreground evidence is displayed as content knowledge.',
    appGameProofDir: 'foreground-content-boundary',
    appGameGate: 'Foreground evidence is displayed as content knowledge.',
    preventedBy: 'foreground-content-boundary',
  },
  {
    appPlanGate: 'AI output can directly enforce.',
    appGameProofDir: 'ai-output-direct-enforcement',
    appGameGate: 'AI output directly enforces.',
    preventedBy: 'ai-evidence-only-boundary',
  },
  {
    appPlanGate: 'Dry-run terminates or blocks app.',
    appGameProofDir: 'dry-run-no-action',
    appGameGate: 'Dry-run terminates or blocks app/game.',
    preventedBy: 'dry-run-no-action-boundary',
  },
  {
    appPlanGate: 'Manual-required action calls an adapter.',
    appGameProofDir: 'manual-required-no-adapter',
    appGameGate: 'Manual-required action calls an adapter.',
    preventedBy: 'manual-required-no-adapter-boundary',
  },
  {
    appPlanGate: 'Android normal mode claims package suspend/hide.',
    appGameProofDir: 'android-normal-mode-no-suspend-hide',
    appGameGate: 'Android normal mode claims package suspend/hide.',
    preventedBy: 'android-normal-mode-no-claim-boundary',
  },
  {
    appPlanGate: 'iOS claims process scanning/killing.',
    appGameProofDir: 'ios-no-process-scan-kill',
    appGameGate: 'iOS claims process scanning/killing.',
    preventedBy: 'ios-no-process-control-boundary',
  },
  {
    appPlanGate: 'macOS hard block is claimed without entitlement/profile proof.',
    appGameProofDir: 'macos-hard-block-proof',
    appGameGate: 'macOS hard block is claimed without MDM/Endpoint/System Extension proof.',
    preventedBy: 'macos-hard-block-proof-boundary',
  },
  {
    appPlanGate: 'Linux universal block is claimed without mechanism/distro proof.',
    appGameProofDir: 'linux-universal-block-proof',
    appGameGate: 'Linux universal block is claimed without mechanism/distro proof.',
    preventedBy: 'linux-universal-block-proof-boundary',
  },
  {
    appPlanGate: 'Session duration changes after journal replay.',
    appGameProofDir: 'session-duration-replay',
    appGameGate: 'Session duration changes after journal replay.',
    preventedBy: 'session-replay-stability-boundary',
  },
  {
    appPlanGate: 'Portal hides stale, permission-limited, manual-required, or not-claimed states.',
    appGameProofDir: 'portal-state-visibility',
    appGameGate: 'Portal hides stale, permission-limited, manual-required, or not-claimed states.',
    preventedBy: 'portal-state-visibility-boundary',
  },
  {
    appPlanGate: 'Raw private executable paths leak into parent UI.',
    appGameProofDir: 'raw-executable-path-ui-leak',
    appGameGate: 'Raw private executable paths leak into parent UI.',
    preventedBy: 'raw-executable-path-redaction-boundary',
  },
  {
    appPlanGate: 'Malicious app metadata causes XSS or layout breakage.',
    appGameProofDir: 'malicious-metadata-ui-safety',
    appGameGate: 'Malicious app/game metadata causes XSS or layout breakage.',
    preventedBy: 'malicious-metadata-ui-safety-boundary',
  },
];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  const appPlanChecklist = await readFile(
    join(repoRoot, 'docs', 'plans', 'app-plan', 'implementation-checklist.md'),
    'utf8'
  );
  const appGameFeatureDoc = await readFile(join(repoRoot, 'docs', 'features', 'app-game-control.md'), 'utf8');
  const proofRows = [];

  for (const gate of gates) {
    const proofPath = join(
      repoRoot,
      'output',
      'app-game-plan-proof',
      'merge-gates',
      gate.appGameProofDir,
      'proof.json'
    );
    const proof = JSON.parse(await readFile(proofPath, 'utf8'));
    assertEqual(proof.gate, gate.appGameGate, `${gate.appPlanGate} source gate`);
    assertNonEmptyString(proof.gateState, `${gate.appPlanGate} gate state`);
    assertEqual(proof.productBoundaries?.sharedEvidenceSpine, true, `${gate.appPlanGate} shared evidence spine`);
    assertEqual(proof.productBoundaries?.nativeAppMeaningProven, true, `${gate.appPlanGate} native app meaning`);
    assertEqual(proof.productBoundaries?.browserGameWorkDuplicated, false, `${gate.appPlanGate} browser-game boundary`);
    assertChecklistGateChecked(appPlanChecklist, gate.appPlanGate, `${gate.appPlanGate} checklist row checked`);
    proofRows.push({
      appPlanGate: gate.appPlanGate,
      appGameProof: `output/app-game-plan-proof/merge-gates/${gate.appGameProofDir}/proof.json`,
      appGameGate: proof.gate,
      appGameGateState: proof.gateState,
      appPlanPreventedBy: gate.preventedBy,
      proofMode: proof.proofMode,
      commandCount: Array.isArray(proof.commands) ? proof.commands.length : 0,
    });
  }

  assertIncludes(
    appGameFeatureDoc,
    'app-plan merge-blocking gate reconciliation',
    'feature doc records app-plan merge-blocking gate reconciliation'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-plan-merge-blocking-gates-proof',
    generatedAt: new Date().toISOString(),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    gateSet: 'docs/plans/app-plan/implementation-checklist.md#merge-blocking-failure-gates',
    gates: proofRows,
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningPreservedInSharedProofs: true,
      browserGameWorkDuplicated: false,
      productStatusMoved: false,
      packageExportsChanged: false,
      adapterDispatchClaimed: false,
      policyExecutionClaimed: false,
    },
    proofPaths: {
      proof: 'test-results/app-plan-merge-blocking-gates-proof/proof.json',
      appPlanProofPack: 'output/app-plan-proof/merge-gates/app-plan-merge-blocking-gates',
      harness: 'scripts/test/app-plan-merge-blocking-gates-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-plan merge-blocking gates source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- The app-plan merge-blocking gates are cross-recorded from the current shared app/game proof spine.',
      '- Each source proof has `sharedEvidenceSpine=true`, `nativeAppMeaningProven=true`, and `browserGameWorkDuplicated=false`.',
      '- This proof changes no product status, package exports, adapter dispatch, policy execution, or browser-game path.',
      '',
      'Gate proof map:',
      ...proofRows.map((row) => `- ${row.appPlanGate} -> ${row.appGameProof}`),
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-plan-merge-blocking-gates-proof-ok');
  console.log('evidence=test-results/app-plan-merge-blocking-gates-proof/proof.json');
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
  }
}

function assertIncludes(source, needle, label) {
  if (!String(source).includes(needle)) {
    throw new Error(`Missing ${label}: ${needle}`);
  }
}

function assertNonEmptyString(value, label) {
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new Error(`${label}: expected a non-empty string, got ${JSON.stringify(value)}`);
  }
}

function assertChecklistGateChecked(source, gate, label) {
  const normalizedSource = source.replace(/\s+/gu, ' ');
  const normalizedGate = `- [x] ${gate}`.replace(/\s+/gu, ' ');
  if (!normalizedSource.includes(normalizedGate)) {
    throw new Error(`Missing ${label}: ${normalizedGate}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push(`git ${args.join(' ')}\nexit=${result.status}\n${result.stdout}${result.stderr}`);
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}
