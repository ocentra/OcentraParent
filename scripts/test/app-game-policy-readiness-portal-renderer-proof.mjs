import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-game-policy-readiness-portal-renderer-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '54-policy-readiness-portal-renderer');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '54-policy-readiness-portal-renderer');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(appProofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/text-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/portal']));
  await runCommand(...npmCommand(['run', 'lint:exec', '--workspace', '@ocentra-parent/text-domain']));
  await runCommand(...npmCommand(['run', 'lint:exec', '--workspace', '@ocentra-parent/portal']));
  await runCommand(...npmCommand(['run', 'lint:exec', '--workspace', '@ocentra-parent/portal']));
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/text-domain',
      '--',
      'vitest',
      'run',
      'tests/unit/portal-dev.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/app-game-policy-readiness-panel.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/app-game-policy-readiness-panel.test.ts',
    ])
  );
  await runCommand(...npmCommand(['run', 'type-check', '--workspace', '@ocentra-parent/portal']));

  const sourceAssertions = await collectSourceAssertions();
  assertSourceAssertions(sourceAssertions);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-game-policy-readiness-portal-renderer',
    checkedAt: new Date().toISOString(),
    implementationCommit: await gitHead(),
    commands,
    sourceAssertions,
    evidence: {
      routePanel: 'apps/portal/src/AppGamePolicyReadinessRoutePanel.tsx',
      liveState: 'packages/portal-domain/src/live-activity-state.ts',
      domainIntent: 'packages/portal-domain/src/app-game-policy-readiness-panel.ts',
      portalTest: 'apps/portal/tests/app-game-policy-readiness-panel.test.ts',
      domainTest: 'packages/portal-domain/tests/unit/app-game-policy-readiness-panel.test.ts',
      textTest: 'packages/text-domain/tests/unit/portal-dev.test.ts',
      featureDoc: 'docs/features/app-game-control.md',
      appGameWorkpack: 'docs/plans/app-game-plan/workpacks/54-policy-readiness-portal-renderer.md',
      appWorkpack: 'docs/plans/app-plan/workpacks/54-policy-readiness-portal-renderer.md',
      appGameProofPack: 'output/app-game-plan-proof/54-policy-readiness-portal-renderer',
      appProofPack: 'output/app-plan-proof/54-policy-readiness-portal-renderer',
    },
    claimsProved: [
      'portal live state parses the existing service-backed app/game policy readiness event',
      'App/Game Sessions renders a policy readiness route panel backed by portal-domain intent rows',
      'renderer exposes readiness summary, readiness-kind rows, evidence refs, parser-fail state, and no policy execution or adapter dispatch copy',
    ],
    claimsNotProved: [
      'new service read-model, Rust protocol, or activity-store behavior',
      'central product capability checklist update while another lane owns that lock',
      'live policy evaluator execution, policy authoring UI, persistence, timers, or enforcement',
      'notification delivery, child-device UX, adapter dispatch, broad installed-app blocking, or platform support',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeProofPack(appGameProofDir, proof, 'app-game WP54');
  await writeProofPack(appProofDir, proof, 'app WP54');

  console.log(`app-game-policy-readiness-portal-renderer-proof-ok:${sourceAssertions.routeAttached}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function assertSourceAssertions(sourceAssertions) {
  for (const [key, value] of Object.entries(sourceAssertions)) {
    if (value !== true) {
      throw new Error(`Policy readiness portal proof assertion failed: ${key}`);
    }
  }
}

async function collectSourceAssertions() {
  const routePanelSource = await readFile(join(repoRoot, 'apps', 'portal', 'src', 'ParentPortalRoute.tsx'), 'utf8');
  const panelSource = await readFile(
    join(repoRoot, 'apps', 'portal', 'src', 'AppGamePolicyReadinessRoutePanel.tsx'),
    'utf8'
  );
  const liveStateSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'src', 'live-activity-state.ts'),
    'utf8'
  );
  const intentSource = await readFile(
    join(repoRoot, 'packages', 'portal-domain', 'src', 'app-game-policy-readiness-panel.ts'),
    'utf8'
  );
  const featureDoc = await readFile(join(repoRoot, 'docs', 'features', 'app-game-control.md'), 'utf8');
  const checklistStatus = await gitOutput(['status', '--short', 'docs/product-capability-checklist.md']);

  return {
    routeAttached: routePanelSource.includes('AppGamePolicyReadinessRoutePanel'),
    routeCommand: panelSource.includes('ActivityAppGamePolicyReadinessReadModelGet'),
    eventParser: liveStateSource.includes('parseAgentAppGamePolicyReadinessEvent'),
    noProductClaimToken: intentSource.includes('AppGamePolicyReadinessNoProductClaim'),
    featureDocUpdated:
      featureDoc.includes('The portal App/Game Sessions route now renders that service-backed policy') &&
      featureDoc.includes('readiness read model as route cards'),
    centralChecklistUntouched: checklistStatus.trim().length === 0,
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
  return gitOutput(['rev-parse', 'HEAD']);
}

async function gitOutput(args) {
  return new Promise((resolve, reject) => {
    const child = spawn('git', args, {
      cwd: repoRoot,
      shell: false,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    let output = '';
    let errorOutput = '';
    child.stdout.on('data', (chunk) => {
      output += chunk.toString();
    });
    child.stderr.on('data', (chunk) => {
      errorOutput += chunk.toString();
    });
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(output.trim());
      } else {
        reject(new Error(errorOutput.trim()));
      }
    });
  });
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}

async function writeProofPack(outputDir, proof, label) {
  await writeFile(
    join(outputDir, 'README.md'),
    [
      `# ${label} Policy Readiness Portal Renderer`,
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

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
