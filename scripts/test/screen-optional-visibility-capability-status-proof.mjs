import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(__dirname, '..', '..');
const proofDir = join(repoRoot, 'output', 'screen-plan-proof', 'optional-visibility-capability-status');
const proofPath = join(proofDir, 'proof-summary.json');

run('npm', ['run', 'build', '--workspace', '@ocentra-parent/activity-domain']);

const screenEvidence = await import(
  pathToFileURL(join(repoRoot, 'packages', 'activity-domain', 'dist', 'screen-evidence.js')).href
);

const generatedAt = new Date().toISOString();
const proof = screenEvidence.screenOptionalVisibilityCapabilityStatusProof(generatedAt);
const readinessStates = proof.rows.map((row) => row.readinessState);
const blockedLiveView = proof.rows.find((row) => row.capabilityKind === 'liveView' && row.readinessState === 'blocked');
const manualRetention = proof.rows.find(
  (row) => row.capabilityKind === 'rawScreenshotRetention' && row.readinessState === 'manualRequired'
);

if (!blockedLiveView || !manualRetention) {
  throw new Error('Expected blocked live-view and manual-required raw retention rows');
}
if (proof.rows.some((row) => row.rawFramesRetained || row.rawRemoteUploadAllowed || row.remoteInputAllowed)) {
  throw new Error(
    'Optional visibility capability status must not retain frames, upload raw screenshots, or allow remote input'
  );
}
if (blockedLiveView.liveViewPermissionGate?.permissionEvidenceKind !== 'screen-capture-only') {
  throw new Error('Expected live-view row to prove capture-only permission remains blocked');
}

mkdirSync(proofDir, { recursive: true });
writeFileSync(
  proofPath,
  `${JSON.stringify(
    {
      ...proof,
      summary: {
        readinessStates,
        blockedLiveViewReason: blockedLiveView.reason,
        manualRetentionReason: manualRetention.reason,
      },
    },
    null,
    2
  )}\n`
);

console.log(`screen-optional-visibility-capability-status-proof-ok:${proofPath}`);

function run(command, args) {
  const runner = process.platform === 'win32' ? 'cmd' : command;
  const runnerArgs = process.platform === 'win32' ? ['/c', command, ...args] : args;
  execFileSync(runner, runnerArgs, { cwd: repoRoot, stdio: 'inherit' });
}
