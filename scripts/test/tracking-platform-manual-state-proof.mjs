import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { spawn } from 'node:child_process';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const outputRoot = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const testResultRoot = join(repoRoot, 'test-results', 'tracking-platform-manual-state-proof');
const commands = [];

await main();

async function main() {
  await runNpmWorkspace('@ocentra-parent/parent-domain', ['run', 'build']);
  await runNpmWorkspace('@ocentra-parent/parent-domain', ['run', 'test', '--', 'tracking-platform-manual-state-proof']);

  const module = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'tracking-platform-manual-state-proof.js')).href
  );
  const rows = module.trackingPlatformManualStateProofRows();
  const summary = module.summarizeTrackingPlatformManualStateProof(rows);
  const commit = await gitHead();
  const checkedAt = new Date().toISOString();
  const proof = {
    checkedAt,
    commit,
    workpackId: summary.workpackId,
    proofTier: 'P1_FIXTURE_SIMULATION',
    status: 'manual_required',
    productClaimReady: summary.productClaimReady,
    rows,
    summary,
    validationCommands: commands.map((entry) => entry.command),
    nonClaims: [
      'No Android foreground/background location sample claim',
      'No Android or iOS physical-device proof claim',
      'No iOS Core Location entitlement or region-monitoring claim',
      'No child-device runtime delivery claim',
      'No web child-agent execution claim',
    ],
  };

  await mkdir(outputRoot, { recursive: true });
  await mkdir(testResultRoot, { recursive: true });
  await writeFile(join(outputRoot, '22-platform-manual-state-proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
  await writeFile(join(outputRoot, '15-manual-platform-proof.md'), manualProofMarkdown(proof));
  await writeFile(join(outputRoot, '16-validation-commands.log'), `${proof.validationCommands.join('\n')}\n`);
  await writeFile(join(testResultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);

  console.log('tracking-platform-manual-state-proof-ok');
  console.log(`proof=${relative(repoRoot, join(outputRoot, '22-platform-manual-state-proof.json'))}`);
  console.log(`testResult=${relative(repoRoot, join(testResultRoot, 'proof.json'))}`);
}

function manualProofMarkdown(proof) {
  return [
    '# WP31 Platform Manual State Proof',
    '',
    `- checkedAt: ${proof.checkedAt}`,
    `- commit: ${proof.commit}`,
    `- proofTier: ${proof.proofTier}`,
    `- status: ${proof.status}`,
    `- productClaimReady: ${proof.productClaimReady}`,
    '',
    '## Summary',
    '',
    `- rows: ${proof.summary.rowCount}`,
    `- manualRequired: ${proof.summary.manualRequiredCount}`,
    `- unavailable: ${proof.summary.unavailableCount}`,
    `- notClaimed: ${proof.summary.notClaimedCount}`,
    `- scaffoldObserved: ${proof.summary.scaffoldObservedCount}`,
    '',
    '## Non Claims',
    '',
    ...proof.nonClaims.map((claim) => `- ${claim}`),
    '',
  ].join('\n');
}

async function runNpmWorkspace(workspace, args) {
  await run('cmd', ['/c', 'npm', '--workspace', workspace, ...args]);
}

async function run(command, args) {
  const rendered = [command, ...args].join(' ');
  commands.push({ command: rendered });
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', shell: false });
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${rendered} failed with exit code ${code}`));
    });
  });
}

async function gitHead() {
  let value = '';
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, shell: false });
    child.stdout.on('data', (chunk) => {
      value += chunk.toString();
    });
    child.on('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error('git rev-parse HEAD failed'));
    });
  });
  return value.trim();
}
