import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-macos-hard-block-proof-gate');
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', 'merge-gates', 'macos-hard-block-proof');
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
    "gateId: 'macos-hard-block-endpoint-mdm-manual-required'",
    'macOS hard-block gate id exists'
  );
  assertIncludes(broadGateData, 'platform: ParentPlatformValue.Macos', 'macOS gate is scoped to macOS');
  assertIncludes(broadGateData, "action: 'block-launch'", 'macOS gate covers hard block launch');
  assertIncludes(broadGateData, "authorityTier: 'manual-required'", 'macOS hard block remains manual-required');
  assertIncludes(
    broadGateData,
    "setupState: 'system-extension-required'",
    'macOS gate requires system extension setup state'
  );
  assertIncludes(
    broadGateData,
    'macOS hard blocking needs MDM, Endpoint Security, or System Extension setup plus rollback and audit proof.',
    'macOS parent-visible reason rejects hard-block claims without platform proof'
  );
  assertIncludes(broadGateData, "'macos-mdm-profile-proof'", 'macOS gate requires MDM profile proof');
  assertIncludes(broadGateData, "'macos-endpoint-security-proof'", 'macOS gate requires Endpoint Security proof');
  assertIncludes(broadGateData, "'macos-system-extension-proof'", 'macOS gate requires System Extension proof');
  assertIncludes(broadGateData, "'rollback-proof'", 'macOS gate requires rollback proof');
  assertIncludes(broadGateData, "'audit-state-proof'", 'macOS gate requires audit proof');
  assertIncludes(
    broadGateData,
    "adapterDispatchState: 'blocked-before-adapter'",
    'manual macOS gate blocks before adapter dispatch'
  );
  assertIncludes(broadGateData, 'canCallAdapter: false', 'manual macOS gate cannot call adapters');
  assertIncludes(broadGateData, 'broadBlockingClaimed: false', 'manual macOS gate does not claim broad blocking');

  assertIncludes(broadGateRules, "case 'macos':", 'rules include macOS platform proof requirements');
  assertIncludes(broadGateRules, "'macos-mdm-profile-proof'", 'rules know macOS MDM proof');
  assertIncludes(broadGateRules, "'macos-endpoint-security-proof'", 'rules know macOS Endpoint Security proof');
  assertIncludes(broadGateRules, "'macos-system-extension-proof'", 'rules know macOS System Extension proof');
  assertIncludes(
    broadGateRules,
    "'setup-proof', 'authority-tier-proof', 'rollback-proof', 'audit-state-proof'",
    'rules require base setup, authority, rollback, and audit proof'
  );

  assertIncludes(
    broadGateTest,
    "const macos = gateFor('macos-hard-block-endpoint-mdm-manual-required')",
    'test fetches the macOS hard-block gate'
  );
  assertIncludes(
    broadGateTest,
    "expect(macos.requiredProofKinds).toContain('macos-endpoint-security-proof')",
    'test requires macOS Endpoint Security proof'
  );
  assertIncludes(
    broadGateTest,
    "expect(macos.requiredProofKinds).toContain('rollback-proof')",
    'test requires macOS rollback proof'
  );
  assertIncludes(
    broadGateTest,
    "expect(macos.requiredProofKinds).toContain('audit-state-proof')",
    'test requires macOS audit proof'
  );

  assertIncludes(
    catalogData,
    'macOS MDM profile, system extension, endpoint/security tooling, or managed app restriction where entitled and deployed.',
    'app control guide catalog names privileged macOS hard-control paths'
  );
  assertIncludes(
    catalogData,
    'Do not assume Windows process control maps directly to macOS.',
    'app control guide catalog rejects Windows-to-macOS hard-control assumptions'
  );

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-macos-hard-block-proof-gate',
    generatedAt: new Date().toISOString(),
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    commit: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: gitOutput(['status', '--short']),
    commands,
    gate: 'macOS hard block is claimed without MDM/Endpoint/System Extension proof.',
    gateState: 'prevented-by-macos-manual-required-platform-proof-gate',
    evidence: {
      macosGate:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts defines macos-hard-block-endpoint-mdm-manual-required as macOS block-launch, manual-required, system-extension-required, blocked-before-adapter, canCallAdapter=false, and broadBlockingClaimed=false.',
      macosGateRules:
        'packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts requires setup, authority, rollback, audit, and macOS MDM/Endpoint/System Extension proof before broad block upgrades.',
      macosGateTests:
        'packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts asserts macOS hard-block gates require Endpoint Security, rollback, and audit proof.',
      catalogBoundary:
        'packages/parent-domain/src/app-control-guide-catalog-data.ts names privileged macOS control paths and rejects assuming Windows process control maps directly to macOS.',
    },
    productBoundaries: {
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
    proofPaths: {
      proof: 'test-results/app-game-macos-hard-block-proof-gate/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/merge-gates/macos-hard-block-proof',
      harness: 'scripts/test/app-game-macos-hard-block-proof-gate.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game macOS hard-block proof gate source snapshot',
      '',
      `- Branch: ${proof.branch}`,
      `- Commit: ${proof.commit}`,
      `- Git status: ${proof.gitStatusShort.length === 0 ? 'clean before proof generation' : proof.gitStatusShort}`,
      '',
      'Evidence:',
      '- macOS hard block launch is represented as manual-required and system-extension-required.',
      '- The macOS gate is blocked-before-adapter, cannot call adapters, has no supported modes, and does not claim broad blocking.',
      '- Rules require setup, authority, rollback, audit, and macOS MDM/Endpoint/System Extension proof before broad block upgrades.',
      '- Tests assert the macOS gate requires Endpoint Security, rollback, and audit proof.',
      '- App control guide catalog names privileged macOS control paths and rejects Windows-to-macOS process-control assumptions.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.join('\n\n').trimEnd()}\n`);

  console.log('app-game-macos-hard-block-proof-gate-ok');
  console.log('evidence=test-results/app-game-macos-hard-block-proof-gate/proof.json');
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
