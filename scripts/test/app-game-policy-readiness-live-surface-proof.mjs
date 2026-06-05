import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-readiness-live-surface-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '69-policy-readiness-live-parent-surface');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '69-policy-readiness-live-parent-surface');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/text-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/portal-domain']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'exec',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'vitest',
    'run',
    'tests/live-activity-state.test.ts',
    'tests/activity-ui-app-game-dashboard-intent.test.ts',
  ]);

  const sourceAssertions = await collectSourceAssertions();
  assertSourceAssertions(sourceAssertions);
  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-policy-readiness-live-parent-surface',
    checkedAt: new Date().toISOString(),
    implementationCommit: await gitHead(),
    commands,
    sourceAssertions,
    claimsProved: [
      'portal overview commands request the existing app/game policy readiness service read model',
      'portal live activity state parses the policy readiness service event through the existing protocol parser',
      'the shared App/Game Sessions dashboard intent exposes policy input readiness metrics and evidence rows',
      'missing and manual-required policy inputs remain visible instead of being treated as ready',
    ],
    claimsNotProved: [
      'runtime policy evaluator execution',
      'adapter dispatch, broad installed-app blocking, or platform support',
      'notification delivery, provider receipt ingestion, or child-device delivery',
      'parent rule authoring, preference mutation, timer execution, or rollback',
      'new backend service contracts beyond the existing WP52 policy readiness event',
    ],
    evidence: {
      commandList: 'packages/portal-domain/src/commands.ts',
      eventResultList: 'apps/portal/src/event-results.ts',
      liveStateParser: 'apps/portal/src/live-activity-state.ts',
      dashboardIntent: 'vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts',
      activityIntent: 'vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts',
      liveStateTest: 'apps/portal/tests/live-activity-state.test.ts',
      dashboardIntentTest: 'apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts',
      proofHarness: 'scripts/test/app-game-policy-readiness-live-surface-proof.mjs',
      appGameProofPack: 'output/app-game-plan-proof/69-policy-readiness-live-parent-surface',
      appProofPack: 'output/app-plan-proof/69-policy-readiness-live-parent-surface',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP69');
  await writeProofPack(appProofDir, proof, 'app WP69');

  console.log(`app-game-policy-readiness-live-surface-proof-ok:${sourceAssertions.policyMetric}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

async function collectSourceAssertions() {
  const commandsSource = await readFile(join(repoRoot, 'packages', 'portal-domain', 'src', 'commands.ts'), 'utf8');
  const liveStateSource = await readFile(join(repoRoot, 'apps', 'portal', 'src', 'live-activity-state.ts'), 'utf8');
  const dashboardSource = await readFile(
    join(repoRoot, 'vendor', 'ocentra-parent-core-ui', 'AppPages', 'ParentPortal', 'app-game-dashboard-intent.ts'),
    'utf8'
  );
  return {
    commandRequested: commandsSource.includes('ActivityAppGamePolicyReadinessReadModelGet'),
    eventParsed: liveStateSource.includes('parseAgentAppGamePolicyReadinessEvent'),
    policyMetric: dashboardSource.includes('Policy inputs'),
    noAdapterClaim: dashboardSource.includes('policyEvaluationReady') && dashboardSource.includes('manual-required'),
  };
}

function assertSourceAssertions(sourceAssertions) {
  for (const [key, value] of Object.entries(sourceAssertions)) {
    if (value !== true) {
      throw new Error(`Policy readiness live surface proof assertion failed: ${key}`);
    }
  }
}

async function writeProofPack(outputDir, proof, label) {
  await writeFile(
    join(outputDir, 'README.md'),
    [
      `# ${label} Policy Readiness Live Parent Surface`,
      '',
      `Checked at: ${proof.checkedAt}`,
      `Implementation commit: ${proof.implementationCommit}`,
      '',
      '## Claims Proved',
      ...proof.claimsProved.map((claim) => `- ${claim}`),
      '',
      '## Claims Not Proved',
      ...proof.claimsNotProved.map((claim) => `- ${claim}`),
      '',
      '## Evidence',
      ...Object.entries(proof.evidence).map(([key, value]) => `- ${key}: ${value}`),
      '',
    ].join('\n'),
    'utf8'
  );
  await writeJson(join(outputDir, 'proof.json'), proof);
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
      commands.push({
        command: commandLine,
        exitCode: code,
      });
      if (code === 0) {
        resolve();
      } else {
        reject(new Error(`${commandLine} exited ${code}`));
      }
    });
  });
}

async function gitHead() {
  return new Promise((resolve) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], {
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

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
