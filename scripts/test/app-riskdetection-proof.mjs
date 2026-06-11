import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const testOutputDir = join(repoRoot, 'test-results', 'app-riskdetection-proof');
const proofDir = join(repoRoot, 'output', 'app-plan-proof', '17-riskapp-detection');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'app-riskdetection',
      'app-game-policy-target-compiler',
    ])
  );

  const { AppRiskDetectionMatrix } = await import('../../packages/parent-domain/dist/app-riskdetection-data.js');
  const summary = summarizeMatrix(AppRiskDetectionMatrix);
  assertMatrix(AppRiskDetectionMatrix, summary);

  const proof = {
    schemaVersion: 1,
    proofMode: 'app-riskdetection-proof',
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    counts: summary,
    evidence: {
      tsContract: 'packages/parent-domain/src/app-riskdetection.ts',
      tsContractRules: 'packages/parent-domain/src/app-riskdetection-rules.ts',
      tsContractData: 'packages/parent-domain/src/app-riskdetection-data.ts',
      tsContractTest: 'packages/parent-domain/tests/app-riskdetection.test.ts',
      proofHarness: 'scripts/test/app-riskdetection-proof.mjs',
      proofPack: 'output/app-plan-proof/17-riskapp-detection',
    },
    claimsProved: [
      'known VPN, remote desktop, torrent/download, and AI chatbot native-app risks classify with evidence refs',
      'unknown risklike names and hash-derived candidates remain review candidates instead of facts',
      'unknown publisher state lowers confidence to review-level candidates',
      'local AI risk candidates cite a digest and stay ask parent or review routed',
      'risk candidates cannot directly enforce and risk app policy targets require category proof',
      'parent-visible disclosure includes confidence, evidence count, and no-content-captured state',
    ],
    claimsNotProved: [
      'live OS app scanning or live catalog enrichment',
      'portal evidence drawer rendering for risk app rows',
      'runtime app risk classifier service events',
      'local model quality or provider execution',
      'platform adapter enforcement for risk categories',
    ],
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, '03-runtime-evidence.json'), proof);
  await writeJson(join(proofDir, '05-policy-action-proof.json'), {
    schemaVersion: 1,
    policyProofMode: 'app-riskpolicy-candidate-proof',
    checkedAt: proof.checkedAt,
    commit: proof.commit,
    riskAppTargetsRequireCategoryProof: true,
    directEnforcementCandidates: summary.directEnforcementCandidates,
    askParentRoutedCandidates: summary.askParentRoutedCandidates,
    manualReviewCandidates: summary.manualReviewCandidates,
  });

  console.log(`app-riskdetection-proof-ok:${Object.keys(summary.byRiskSignal).join(',')}`);
  console.log(`evidence=${relative(repoRoot, join(testOutputDir, 'proof.json'))}`);
}

function summarizeMatrix(matrix) {
  return {
    candidateCount: matrix.candidates.length,
    byRiskSignal: countBy(matrix.candidates.map((candidate) => candidate.riskSignal)),
    bySourceKind: countBy(matrix.candidates.map((candidate) => candidate.sourceKind)),
    byPolicyCandidateAction: countBy(matrix.candidates.map((candidate) => candidate.policyCandidateAction)),
    directEnforcementCandidates: matrix.candidates.filter((candidate) => !candidate.notDirectEnforcement).length,
    noContentClaimCandidates: matrix.candidates.filter((candidate) => candidate.noContentClaim).length,
    askParentRoutedCandidates: matrix.candidates.filter((candidate) => candidate.askParentRouting === 'available')
      .length,
    manualReviewCandidates: matrix.candidates.filter((candidate) => candidate.askParentRouting === 'manual-review')
      .length,
  };
}

function assertMatrix(matrix, summary) {
  assertEqual(String(matrix.matrixId), 'app-riskdetection-proof-matrix', 'matrix id');
  assertEqual(summary.candidateCount, 8, 'candidate count');
  assertEqual(summary.bySourceKind.knownCatalog, 4, 'known catalog count');
  assertEqual(summary.byRiskSignal.vpnProxy, 2, 'VPN/proxy risk count');
  assertEqual(summary.byRiskSignal.remoteDesktop, 1, 'remote desktop risk count');
  assertEqual(summary.byRiskSignal.downloadTorrent, 1, 'torrent/download risk count');
  assertEqual(summary.byRiskSignal.aiChatbot, 2, 'AI chatbot risk count');
  assertEqual(summary.directEnforcementCandidates, 0, 'direct enforcement candidate count');
  assertEqual(summary.noContentClaimCandidates, summary.candidateCount, 'no content claim count');

  const unknownName = candidateFor(matrix, 'unknown-vpn-name-candidate');
  assertEqual(unknownName.candidateState, 'heuristicCandidate', 'unknown name candidate state');
  assertEqual(unknownName.identityRef, null, 'unknown name identity ref');
  assertEqual(unknownName.confidence <= 0.5, true, 'unknown publisher confidence');

  const aiCandidate = candidateFor(matrix, 'local-ai-social-video-messaging-risk');
  assertEqual(aiCandidate.localAiDigestRef !== null, true, 'AI digest ref');
}

function candidateFor(matrix, candidateId) {
  const candidate = matrix.candidates.find((entry) => String(entry.candidateId) === candidateId);
  if (candidate === undefined) {
    throw new Error(`missing app risk detection candidate ${candidateId}`);
  }
  return candidate;
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${command} exited with ${code}`))));
    child.once('error', reject);
  });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function countBy(values) {
  return values.reduce((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
