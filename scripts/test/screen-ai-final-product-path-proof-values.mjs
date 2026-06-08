import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

export const RepoRoot = process.cwd();
export const OutputRoot = resolve(RepoRoot, 'output', 'screen-ai-pipeline-proof', 'final-product-path');
export const ProofPath = join(OutputRoot, 'proof-summary.json');
export const SnapshotPath = join(OutputRoot, '00-source-snapshot.md');
export const CommandsPath = join(OutputRoot, '14-validation-commands.log');

export const LiveScenarioIds = [
  'youtube-ordinary-video',
  'youtube-education-video',
  'vimeo-video',
  'facebook-social-surface',
  'browser-game',
  'shopping-page',
  'school-productivity',
  'native-app',
  'protected-unsupported-state',
];

export const BrowserScenarioIds = new Set([
  'youtube-ordinary-video',
  'youtube-education-video',
  'vimeo-video',
  'facebook-social-surface',
  'browser-game',
  'shopping-page',
  'school-productivity',
]);

export const SourcePaths = {
  actionDispatch: 'output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json',
  blockActionDispatch: 'output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json',
  deletionRetentionCustody: 'output/screen-ai-pipeline-proof/deletion-retention-custody/proof-summary.json',
  finalAdapterAudit: 'output/screen-ai-pipeline-proof/final-adapter-dependency-audit/proof-summary.json',
  liveOperator: 'output/screen-ai-pipeline-proof/live-operator/proof-summary.json',
  liveOperatorAi: 'output/ai-plan-proof/live-operator/proof-summary.json',
  portalChain: 'output/screen-ai-pipeline-proof/portal-chain/proof-summary.json',
  protectedSurface: 'output/screen-ai-pipeline-proof/protected-surface/proof-summary.json',
  readModel: 'output/ai-plan-proof/screen-summary-parent-explanation-read-model/proof-summary.json',
  retentionSweeper: 'output/screen-ai-pipeline-proof/service-retention-sweeper/proof-summary.json',
  serviceReadModel: 'output/ai-plan-proof/screen-summary-parent-explanation-service-read-model/proof-summary.json',
};

export function readJson(path, assert) {
  const absolute = resolve(RepoRoot, path);
  assert(existsSync(absolute), `missing artifact ${path}`);
  return JSON.parse(readFileSync(absolute, 'utf8'));
}

export function existsPath(path) {
  return Boolean(path) && existsSync(path);
}

export function repoPath(path) {
  return resolve(RepoRoot, path);
}

export function writeProofOutputs(proof) {
  mkdirSync(OutputRoot, { recursive: true });
  writeFileSync(ProofPath, `${JSON.stringify(proof, null, 2)}\n`);
  writeFileSync(SnapshotPath, sourceSnapshot(proof));
  writeFileSync(CommandsPath, validationCommands());
}

function sourceSnapshot(proof) {
  const rows = Object.entries(SourcePaths)
    .map(([name, path]) => `- ${name}: \`${path}\``)
    .join('\n');
  return `# Screen AI Final Product Path Proof\n\nGenerated: ${proof.generatedAt}\n\n## Source Artifacts\n\n${rows}\n\n## Closure\n\n\`\`\`json\n${JSON.stringify(proof.closure, null, 2)}\n\`\`\`\n`;
}

function validationCommands() {
  return [
    'node --check scripts/test/screen-ai-final-product-path-proof.mjs',
    'node scripts/test/screen-ai-final-product-path-proof.mjs',
    'git diff --check',
    'npm run lanes:guard',
    'npm run hub:guard',
    '',
  ].join('\n');
}
