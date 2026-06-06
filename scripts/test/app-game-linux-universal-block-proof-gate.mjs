import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-linux-universal-block-proof-gate');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'linux-universal-block-proof');
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

  assertIncludes(broadGateData, "gateId: 'linux-hard-block-mechanism-unavailable'", 'Linux hard-block gate id exists');
  assertIncludes(broadGateData, 'platform: ParentPlatformValue.Linux', 'Linux gate is scoped to Linux');
  assertIncludes(broadGateData, "action: 'block-launch'", 'Linux gate covers launch block');
  assertIncludes(broadGateData, "authorityTier: 'manual-required'", 'Linux hard block remains manual-required');
  assertIncludes(broadGateData, "setupState: 'admin-or-root-required'", 'Linux gate requires admin/root setup state');
  assertIncludes(
    broadGateData,
    'Linux blocking is unavailable without a named mechanism, distro, session, rollback, and audit proof.',
    'Linux parent-visible reason rejects universal block claims without proof'
  );
  assertIncludes(broadGateData, "'linux-mechanism-proof'", 'Linux gate requires mechanism proof');
  assertIncludes(broadGateData, "'linux-distro-proof'", 'Linux gate requires distro proof');
  assertIncludes(broadGateData, "'linux-session-proof'", 'Linux gate requires session proof');
  assertIncludes(broadGateData, "'rollback-proof'", 'Linux gate requires rollback proof');
  assertIncludes(broadGateData, "'audit-state-proof'", 'Linux gate requires audit proof');
  assertIncludes(broadGateData, "outcomeState: 'unavailable'", 'unavailable Linux gate has unavailable outcome');
  assertIncludes(
    broadGateData,
    "adapterDispatchState: 'adapter-unavailable'",
    'unavailable Linux gate cannot dispatch adapters'
  );
  assertIncludes(broadGateData, "capabilityState: 'unavailable'", 'unavailable Linux gate has unavailable capability');
  assertIncludes(broadGateData, 'canCallAdapter: false', 'unavailable Linux gate cannot call adapters');
  assertIncludes(broadGateData, 'broadBlockingClaimed: false', 'unavailable Linux gate does not claim broad blocking');

  assertIncludes(broadGateRules, "case 'linux':", 'rules include Linux platform proof requirements');
  assertIncludes(
    broadGateRules,
    "return gateRequiresAll(gate, ['linux-mechanism-proof', 'linux-distro-proof', 'linux-session-proof'])",
    'rules require Linux mechanism, distro, and session proof together'
  );
  assertIncludes(
    broadGateRules,
    "'setup-proof', 'authority-tier-proof', 'rollback-proof', 'audit-state-proof'",
    'rules require base setup, authority, rollback, and audit proof'
  );

  assertIncludes(
    broadGateTest,
    "const linux = gateFor('linux-hard-block-mechanism-unavailable')",
    'test fetches the Linux hard-block gate'
  );
  assertIncludes(
    broadGateTest,
    "expect(linux.requiredProofKinds).toContain('linux-mechanism-proof')",
    'test requires Linux mechanism proof'
  );
  assertIncludes(
    broadGateTest,
    "expect(linux.requiredProofKinds).toContain('linux-distro-proof')",
    'test requires Linux distro proof'
  );
  assertIncludes(
    broadGateTest,
    "expect(linux.requiredProofKinds).toContain('linux-session-proof')",
    'test requires Linux session proof'
  );

  assertIncludes(
    catalogData,
    'Linux policy, desktop/session integration, package-manager restriction, or service-level control where proven.',
    'app control guide catalog names Linux hard-control mechanisms only where proven'
  );
  assertIncludes(
    catalogData,
    'Broad app blocking should be treated as manual-required until a concrete adapter is proven on the target distro and desktop.',
    'app control guide catalog rejects universal Linux broad-block claims'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-linux-universal-block-proof-gate',
    generatedAt: new Date().toISOString(),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    gate: 'Linux universal block is claimed without mechanism/distro proof.',
    gateState: 'prevented-by-linux-unavailable-mechanism-distro-session-proof-gate',
    evidence: {
      linuxGate:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts defines linux-hard-block-mechanism-unavailable as Linux block-launch, manual-required, adapter-unavailable, admin-or-root-required, canCallAdapter=false, and broadBlockingClaimed=false.',
      linuxGateRules:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts requires setup, authority, rollback, audit, and Linux mechanism/distro/session proof before broad block upgrades.',
      linuxGateTests:
        'packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts asserts Linux hard-block gates require mechanism, distro, and session proof.',
      catalogBoundary:
        'packages/parent-domain/src/app-control-guide-catalog-data.ts keeps Linux broad app blocking manual-required until a concrete adapter is proven on the target distro and desktop.',
    },
    productBoundaries: {
      sharedEvidenceSpine: true,
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      linuxUniversalBlockClaimed: false,
      linuxMechanismProofAttached: false,
      linuxDistroProofAttached: false,
      linuxSessionProofAttached: false,
      rollbackProofAttached: false,
      auditProofAttached: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      browserGameWorkDuplicated: false,
      packageExportsChanged: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-linux-universal-block-proof-gate/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/linux-universal-block-proof',
      harness: 'scripts/test/app-game-linux-universal-block-proof-gate.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game Linux universal block proof gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- Linux hard block launch is represented as unavailable/manual-required and admin-or-root-required.',
      '- The Linux gate is unavailable, cannot call adapters, has no supported modes, and does not claim broad blocking.',
      '- Rules require setup, authority, rollback, audit, and Linux mechanism/distro/session proof before broad block upgrades.',
      '- Tests assert the Linux gate requires mechanism, distro, and session proof.',
      '- App control guide catalog keeps Linux broad blocking manual-required until a concrete target-distro/desktop adapter is proven.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-linux-universal-block-proof-gate-ok');
  console.log('evidence=test-results/app-game-linux-universal-block-proof-gate/proof.json');
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
