import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-ios-no-process-scan-kill-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'ios-no-process-scan-kill');
const proofBranch = 'codex/app-game-ios-process-authority-gate-proof-split';
const deterministicProofRevision = 'branch-head-validated-by-harness';
const deterministicGeneratedAt = 'deterministic-proof-artifact';
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  runNpm(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
  runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-broad-blocking-proof-gates.test.ts',
  ]);

  const broadGateData = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-data.ts'),
    'utf8'
  );
  const broadGateRules = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-rules.ts'),
    'utf8'
  );
  const broadGateTest = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'tests', 'app-game-broad-blocking-proof-gates.test.ts'),
    'utf8'
  );
  const catalogData = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-control-guide-catalog-data.ts'),
    'utf8'
  );

  assertIncludes(broadGateData, "gateId: 'ios-process-kill-not-claimed'", 'iOS process-kill gate id exists');
  assertIncludes(broadGateData, 'platform: ParentPlatformValue.Ios', 'iOS process gate is scoped to iOS');
  assertIncludes(broadGateData, "action: 'terminate-process'", 'iOS process gate covers process termination');
  assertIncludes(broadGateData, "authorityTier: 'not-claimed'", 'iOS process kill remains not-claimed');
  assertIncludes(broadGateData, "setupState: 'not-claimed'", 'iOS process kill has no supported setup state');
  assertIncludes(
    broadGateData,
    'iOS process enumeration and process killing are not claimed; iOS control must use Screen Time, ManagedSettings, MDM, or App Lock proof.',
    'iOS parent-visible reason rejects process scan/kill claims'
  );
  assertIncludes(broadGateData, "'ios-family-controls-proof'", 'iOS gate requires FamilyControls proof');
  assertIncludes(broadGateData, "'ios-managed-settings-proof'", 'iOS gate requires ManagedSettings proof');
  assertIncludes(broadGateData, "'ios-supervised-mdm-proof'", 'iOS gate requires supervised MDM proof');
  assertIncludes(broadGateData, "adapterDispatchState: 'not-dispatched'", 'not-claimed iOS gate is not dispatched');
  assertIncludes(broadGateData, 'canCallAdapter: false', 'not-claimed iOS gate cannot call adapters');
  assertIncludes(broadGateData, 'broadBlockingClaimed: false', 'not-claimed iOS gate does not claim broad blocking');

  assertIncludes(broadGateRules, "case 'ios':", 'rules keep iOS platform proof requirements named');
  assertIncludes(broadGateRules, "'ios-family-controls-proof'", 'rules know iOS FamilyControls platform proof');
  assertIncludes(broadGateRules, "'ios-managed-settings-proof'", 'rules know iOS ManagedSettings platform proof');
  assertIncludes(broadGateRules, "'ios-supervised-mdm-proof'", 'rules know supervised iOS MDM proof');

  assertIncludes(
    broadGateTest,
    'keeps Android normal mode, iOS shielding, and iOS process killing proof-gated',
    'test names the iOS process kill proof gate'
  );
  assertIncludes(
    broadGateTest,
    "const iosKill = gateFor('ios-process-kill-not-claimed')",
    'test fetches the iOS process kill gate'
  );
  assertIncludes(broadGateTest, "action: 'terminate-process'", 'test asserts iOS terminate-process action');
  assertIncludes(broadGateTest, "outcomeState: 'not-claimed'", 'test asserts iOS process kill is not claimed');
  assertIncludes(
    broadGateTest,
    "adapterDispatchState: 'not-dispatched'",
    'test asserts iOS process kill is not dispatched'
  );
  assertIncludes(broadGateTest, 'canCallAdapter: false', 'test asserts iOS process kill cannot call adapters');

  assertIncludes(
    catalogData,
    'Android package lifecycle and iOS Screen Time/entitlement behavior are manual-required until real device/platform proof exists.',
    'app control guide catalog does not claim iOS process scan/kill support'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-ios-no-process-scan-kill-gate-proof',
    generatedAt: deterministicGeneratedAt,
    branch: proofBranch,
    commit: deterministicProofRevision,
    commitMetadata:
      'This proof intentionally avoids embedding HEAD because a committed artifact cannot contain its own final commit hash.',
    gitStatusShort: 'validated-by-explicit-handoff-status-check',
    commands,
    gate: 'iOS claims process scanning/killing.',
    gateState: 'prevented-by-ios-not-claimed-process-gate',
    evidence: {
      iosGate:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts defines ios-process-kill-not-claimed as iOS terminate-process, not-claimed, not-dispatched, canCallAdapter=false, and broadBlockingClaimed=false.',
      iosGateRules:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts keeps iOS proof kinds explicit before any platform upgrade can be considered.',
      iosGateTests:
        'packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts asserts iOS process killing is not claimed and cannot dispatch adapters.',
      catalogBoundary:
        'packages/parent-domain/src/app-control-guide-catalog-data.ts keeps iOS Screen Time/entitlement behavior manual-required until real device/platform proof exists.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      iosProcessScanningClaimed: false,
      iosProcessKillingClaimed: false,
      iosFamilyControlsProofAttached: false,
      iosManagedSettingsProofAttached: false,
      iosSupervisedMdmProofAttached: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-ios-no-process-scan-kill-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/ios-no-process-scan-kill',
      harness: 'scripts/test/app-game-ios-no-process-scan-kill-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game iOS no process scan/kill gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- iOS process termination is represented as a not-claimed gate.',
      '- The iOS gate is not-dispatched, cannot call adapters, has no supported modes, and does not claim broad blocking.',
      '- Required proof kinds stay tied to FamilyControls, ManagedSettings, or supervised MDM paths before any stronger iOS control claim.',
      '- Tests assert the iOS terminate-process gate remains not-claimed and cannot call adapters.',
      '- App control guide catalog keeps iOS Screen Time/entitlement behavior manual-required until real device/platform proof exists.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-ios-no-process-scan-kill-gate-proof-ok');
  console.log('evidence=test-results/app-game-ios-no-process-scan-kill-gate-proof/proof.json');
}

function assertIncludes(source, needle, label) {
  if (!source.includes(needle)) {
    throw new Error(`Missing ${label}: ${needle}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function run(command, args) {
  const rendered = `${command} ${args.join(' ')}`;
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  commands.push(
    `${rendered}\nexit=${result.status}\n${normalizeCommandOutput(result.stdout)}${normalizeCommandOutput(result.stderr)}`
  );
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
}

function normalizeCommandOutput(output) {
  const slashRepoRoot = repoRoot.replace(/\\/g, '/');
  return output
    .split(repoRoot)
    .join('<repo-root>')
    .split(slashRepoRoot)
    .join('<repo-root>')
    .replace(/Start at\s+\d{2}:\d{2}:\d{2}/g, 'Start at <normalized>')
    .replace(/\x1b\[2m[^\r\n]*?\x1b\[22m/g, '\x1b[2m<normalized>\x1b[22m')
    .replace(/Duration\s+[^\r\n]+/g, 'Duration <normalized>');
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
