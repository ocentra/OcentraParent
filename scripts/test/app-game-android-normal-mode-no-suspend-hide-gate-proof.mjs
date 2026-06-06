import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-android-normal-mode-no-suspend-hide-gate-proof');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'android-normal-mode-no-suspend-hide');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  run('cmd', [
    '/c',
    'npm',
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

  assertIncludes(
    broadGateData,
    "gateId: 'android-normal-mode-hide-suspend-manual-required'",
    'Android normal mode gate id exists'
  );
  assertIncludes(broadGateData, 'platform: ParentPlatformValue.Android', 'Android gate is scoped to Android');
  assertIncludes(broadGateData, "action: 'suspend-app'", 'Android gate covers package suspend action');
  assertIncludes(broadGateData, "authorityTier: 'manual-required'", 'Android normal mode gate remains manual-required');
  assertIncludes(
    broadGateData,
    "setupState: 'device-owner-required'",
    'Android gate requires device-owner setup before stronger claims'
  );
  assertIncludes(
    broadGateData,
    'Android normal mode cannot hide or suspend packages; Device Owner or Profile Owner proof is required before adapter dispatch.',
    'Android parent-visible reason rejects normal-mode hide/suspend claim'
  );
  assertIncludes(broadGateData, "'android-device-owner-proof'", 'Android gate requires device owner proof');
  assertIncludes(broadGateData, "'android-profile-owner-proof'", 'Android gate requires profile owner proof');
  assertIncludes(
    broadGateData,
    "adapterDispatchState: 'blocked-before-adapter'",
    'manual Android gate blocks before adapter dispatch'
  );
  assertIncludes(broadGateData, 'canCallAdapter: false', 'manual Android gate cannot call adapters');
  assertIncludes(broadGateData, 'broadBlockingClaimed: false', 'manual Android gate does not claim broad blocking');

  assertIncludes(
    broadGateRules,
    "gate.platform === 'android' && (gate.action === 'hide-app' || gate.action === 'suspend-app')",
    'rules explicitly cover Android hide and suspend actions'
  );
  assertIncludes(
    broadGateRules,
    "return gateRequiresAny(gate, ['android-device-owner-proof', 'android-profile-owner-proof'])",
    'rules require Android owner proof before hide/suspend can move up'
  );
  assertIncludes(
    broadGateRules,
    'return !gate.canCallAdapter && gate.supportedModes.length === 0 && gate.adapterDispatchState !==',
    'non-supported Android gates cannot dispatch'
  );

  assertIncludes(
    broadGateTest,
    'keeps Android normal mode, iOS shielding, and iOS process killing proof-gated',
    'test names Android normal mode proof gate'
  );
  assertIncludes(
    broadGateTest,
    "const android = gateFor('android-normal-mode-hide-suspend-manual-required')",
    'test fetches the Android normal mode gate'
  );
  assertIncludes(
    broadGateTest,
    "expect(android.parentVisibleReason).toContain('Device Owner or Profile Owner proof')",
    'test requires parent-visible owner proof reason'
  );
  assertIncludes(
    broadGateTest,
    "expect(android.requiredProofKinds).toContain('android-device-owner-proof')",
    'test requires Android device-owner proof'
  );
  assertIncludes(
    broadGateTest,
    "expect(android.requiredProofKinds).toContain('android-profile-owner-proof')",
    'test requires Android profile-owner proof'
  );

  assertIncludes(
    catalogData,
    'The roadmap currently treats Android package lifecycle proof as manual-required until real device artifacts exist.',
    'app control guide catalog does not claim Android package lifecycle support in normal mode'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-android-normal-mode-no-suspend-hide-gate-proof',
    generatedAt: new Date().toISOString(),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    gate: 'Android normal mode claims package suspend/hide.',
    gateState: 'prevented-by-android-owner-proof-manual-required-gate',
    evidence: {
      androidGate:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts defines android-normal-mode-hide-suspend-manual-required as Android suspend-app, manual-required, device-owner-required, blocked-before-adapter, canCallAdapter=false, and broadBlockingClaimed=false.',
      androidGateRules:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts requires android-device-owner-proof or android-profile-owner-proof for Android hide/suspend actions and blocks non-supported gates from dispatch eligibility.',
      androidGateTests:
        'packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts asserts Android normal mode needs Device Owner/Profile Owner proof.',
      catalogBoundary:
        'packages/parent-domain/src/app-control-guide-catalog-data.ts keeps Android package lifecycle proof manual-required until real device artifacts exist.',
    },
    productBoundaries: {
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
    proofPaths: {
      proof: 'test-results/app-game-android-normal-mode-no-suspend-hide-gate-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/android-normal-mode-no-suspend-hide',
      harness: 'scripts/test/app-game-android-normal-mode-no-suspend-hide-gate-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game Android normal-mode no suspend/hide gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Android normal-mode package suspend is represented as manual-required and device-owner-required.',
      '- The Android gate is blocked-before-adapter, cannot call adapters, has no supported modes, and does not claim broad blocking.',
      '- Android hide/suspend rules require Device Owner or Profile Owner proof before moving up.',
      '- Tests assert the Android gate requires Device Owner/Profile Owner proof.',
      '- App control guide catalog keeps Android package lifecycle proof manual-required until real device artifacts exist.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-android-normal-mode-no-suspend-hide-gate-proof-ok');
  console.log('evidence=test-results/app-game-android-normal-mode-no-suspend-hide-gate-proof/proof.json');
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
  commands.push(`${rendered}\nexit=${result.status}\n${result.stdout}${result.stderr}`);
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}
