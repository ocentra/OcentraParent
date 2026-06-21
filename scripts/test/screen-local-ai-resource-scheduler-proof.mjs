import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const proofOutputDir = join(process.cwd(), 'output', 'screen-plan-proof', 'local-ai-resource-scheduler');
const proofOutputPath = join(proofOutputDir, 'proof-summary.json');
const validationLogPath = join(proofOutputDir, 'validation-commands.log');
const successfulCommands = [];

await runPackageCommand([
  'run',
  'test',
  '--workspace',
  '@ocentra-parent/screen-domain',
  '--',
  'screen-evidence-resource-scheduler.test.ts',
]);
await runPackageCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
await runCommand('node', ['scripts/test/local-ai-provider-scheduler-proof.mjs']);
await writeScreenSchedulerProof();

console.log(`screen-local-ai-resource-scheduler-proof-ok: ${proofOutputPath}`);

async function writeScreenSchedulerProof() {
  const { screenLocalAiResourceSchedulerProof, screenLocalAiResourceSchedulerProofSummary } =
    await import('@ocentra-parent/schema-domain/screen-evidence-resource-scheduler-proof');
  const summary = screenLocalAiResourceSchedulerProofSummary(screenLocalAiResourceSchedulerProof.decisions);

  await mkdir(proofOutputDir, { recursive: true });
  await writeFile(
    proofOutputPath,
    `${JSON.stringify(
      {
        proofGeneratedAt: new Date().toISOString(),
        proofTopic: 'screen-local-ai-resource-scheduler',
        workpack: 'docs/plans/screen-plan/workpacks/38-local-ai-resource-scheduler-priority-queue.md',
        claimsProven: [
          'screen OCR and VLM jobs are typed before scheduler admission',
          'only one heavy local screen AI job can be active per child device',
          'policy-blocking jobs are admitted ahead of cadence and background summaries',
          'queued jobs expose queue position and duplicate runtime blocking',
          'timeout, skipped, degraded, and unavailable states stay policy-ineligible',
          'image pixel, OCR snippet, local-only, no-remote-AI, and no-raw-retention caps are enforced',
          'screen resource proof reuses the existing local provider singleton scheduler proof path',
        ],
        nonClaims: [
          'this proof does not claim final real capture to AI to policy pipeline completion',
          'this proof does not retain raw screenshots or call remote AI providers',
        ],
        validationCommands: successfulCommands,
        screenSchedulerSummary: summary,
        screenSchedulerProof: screenLocalAiResourceSchedulerProof,
        underlyingProviderProof: 'test-results/local-ai-provider-scheduler-proof/proof.json',
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  await writeFile(validationLogPath, `${successfulCommands.join('\n')}\n`, 'utf8');
}

function runCommand(command, args) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd: process.cwd(),
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const output = collectOutput(child);
    child.on('error', reject);
    child.on('exit', (code) => {
      const commandLine = `${command} ${args.join(' ')}`;
      if (code === 0) {
        successfulCommands.push(commandLine);
        resolve();
        return;
      }
      reject(new Error(`${commandLine} failed with ${code}\n${output()}`));
    });
  });
}

function runPackageCommand(args) {
  if (process.platform === 'win32') {
    return runCommand(...npmCommand([...args]));
  }

  return runCommand('npm', args);
}

function collectOutput(child) {
  const chunks = [];
  child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
  child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
  return () => chunks.join('');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
