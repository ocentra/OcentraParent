import { spawn } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const WORKPACK_ID = 'WP-child-agent-runtime-distribution-plan-01-child-agent-scope-and-route-boundary';
const PLAN_ID = 'PLAN-child-agent-runtime-distribution-plan';
const WORKPACK_PATH =
  'docs/plans/child-agent-runtime-distribution-plan/workpacks/01-child-agent-scope-and-route-boundary.md';
const CHECKLIST_PATH = 'docs/plans/child-agent-runtime-distribution-plan/CHECKLIST_INDEX.md';
const EXPECTED_PROOF_ROOT =
  'output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary';
const DURABLE_PROOF_MANIFEST =
  'docs/proof/child-agent-runtime-distribution-plan/WP01_CHILD_AGENT_SCOPE_AND_ROUTE_BOUNDARY_PROOF.md';
const FOCUSED_TEST_PATH = 'tests/repo-tooling/child-agent-runtime-distribution-route.test.mjs';
const PROOF_FILES = Object.freeze([
  '00-scope-summary.md',
  '01-negative-case-proof.md',
  '02-no-claim-boundary.md',
  '16-validation-commands.log',
]);
const PROOF_EVIDENCE_PATHS = Object.freeze([DURABLE_PROOF_MANIFEST]);

const moduleDirectory = dirname(fileURLToPath(import.meta.url));
const defaultRepoRoot = resolve(moduleDirectory, '..', '..');

export async function loadWp01RouteInputs(repoRoot = defaultRepoRoot) {
  const [graph, codeMap] = await Promise.all([
    readJson(join(repoRoot, 'docs', 'engineering-graph', 'graph.json')),
    readJson(join(repoRoot, 'docs', 'engineering-graph', 'code-map.json')),
  ]);
  return { graph, codeMap };
}

export function validateWp01RouteBoundary(graph, codeMap) {
  const workpack = graph.nodes?.find((candidate) => candidate.id === WORKPACK_ID);
  assertCondition(workpack, `generated graph is missing ${WORKPACK_ID}`);
  assertEqual(workpack.kind, 'workpack', 'WP01 graph kind');
  assertEqual(workpack.parent, PLAN_ID, 'WP01 owning plan');
  assertExactArray(workpack.dependsOn, [], 'WP01 dependencies');
  assertEqual(workpack.state, 'validation', 'WP01 state');
  assertEqual(workpack.lifecycleState, 'validation', 'WP01 lifecycle state');
  assertEqual(
    workpack.metadata?.statusText,
    'validation — route-only proof complete; implementation contract open',
    'WP01 status text'
  );
  assertEqual(workpack.metadata?.needsReview, false, 'WP01 dependency review state');

  const completion = workpack.completion;
  assertCondition(completion, 'WP01 completion contract is missing');
  assertExactArray(
    completion.required,
    ['implementation', 'tests', 'proof', 'checklist'],
    'WP01 completion requirements'
  );
  assertExactObjectKeys(completion.reviewed, ['tests', 'proof', 'checklist'], 'WP01 reviewed completion evidence');
  for (const requirement of ['tests', 'proof', 'checklist']) {
    assertEqual(completion.reviewed[requirement], true, `WP01 reviewed ${requirement} evidence`);
  }
  assertExactArray(completion.references?.implementation, [WORKPACK_PATH], 'WP01 open implementation route');
  assertExactArray(completion.references?.tests, [FOCUSED_TEST_PATH], 'WP01 test evidence');
  assertExactArray(completion.references?.proof, PROOF_EVIDENCE_PATHS, 'WP01 retained proof evidence');
  assertExactArray(completion.references?.checklist, [CHECKLIST_PATH], 'WP01 checklist evidence');
  assertExactArray(completion.expected?.proof, [EXPECTED_PROOF_ROOT], 'WP01 expected proof root');

  const codeRoute = codeMap.workpacks?.[WORKPACK_ID];
  assertCondition(codeRoute, `code map is missing ${WORKPACK_ID}`);
  assertEqual(codeRoute.planSlug, 'child-agent-runtime-distribution-plan', 'WP01 code-map plan');
  assertEqual(codeRoute.codeExpectation, 'no-code-required', 'WP01 production code expectation');
  assertExactArray(codeRoute.roots, [], 'WP01 production roots');
  rejectProductOrParentReferences(workpack, codeRoute);

  return Object.freeze({
    workpackId: workpack.id,
    planId: workpack.parent,
    state: workpack.state,
    lifecycleState: workpack.lifecycleState,
    statusText: workpack.metadata.statusText,
    expectedProofRoot: completion.expected.proof[0],
    retainedProofEvidence: [...completion.references.proof],
    productionCodeExpectation: codeRoute.codeExpectation,
  });
}

export function renderWp01ProofArtifacts({ checkedAt, commit, commandResults, route }) {
  const allPassed = commandResults.every((result) => result.exitCode === 0);
  const validationResult = allPassed ? 'pass' : 'fail';
  const scopeSummary =
    `# WP01 Child Agent Scope And Route Boundary\n\n` +
    `- checked_at: ${checkedAt}\n` +
    `- commit: ${commit}\n` +
    `- graph_workpack: ${route.workpackId}\n` +
    `- owning_plan: ${route.planId}\n` +
    `- graph_state: ${route.state}\n` +
    `- lifecycle_state: ${route.lifecycleState}\n` +
    `- status: ${route.statusText}\n` +
    `- production_code_expectation: ${route.productionCodeExpectation}\n` +
    `- focused_validation: ${validationResult}\n\n` +
    `This workpack proves route ownership only. It does not implement or certify a child runtime or package.\n`;
  const negativeCaseProof =
    `# WP01 Negative-Case Proof\n\n` +
    `The focused behavioral test loads the generated engineering graph and its reviewed code map. It rejects:\n\n` +
    `- ownership by the parent-client distribution plan;\n` +
    `- runtime, platform, package, or release roots on this route-only workpack;\n` +
    `- promotion beyond the reviewed route-only validation state;\n` +
    `- missing, incomplete, or misrouted retained completion proof; and\n` +
    `- an expected proof root other than \`${route.expectedProofRoot}/\`.\n\n` +
    `Focused test result: **${validationResult}**.\n`;
  const noClaimBoundary =
    `# WP01 No-Claim Boundary\n\n` +
    `This proof does not claim package build, install, trusted startup, authenticated ingress, external health, ` +
    `runtime readiness, respawn, uninstall or revocation, setup trust completion, signing, store readiness, ` +
    `device-owner or supervision authority, CI readiness, PR readiness, or release readiness.\n\n` +
    `Parent-client distribution remains owned by \`parent-client-runtime-distribution-plan\`. ` +
    `Child package and runtime work remains in its platform/runtime workpacks.\n`;

  return new Map([
    ['00-scope-summary.md', scopeSummary],
    ['01-negative-case-proof.md', negativeCaseProof],
    ['02-no-claim-boundary.md', noClaimBoundary],
    ['16-validation-commands.log', renderCommandLog(commandResults)],
  ]);
}

export async function writeWp01ProofArtifacts(repoRoot, artifacts) {
  assertExactArray([...artifacts.keys()], PROOF_FILES, 'WP01 proof artifact names');
  const outputDirectory = join(repoRoot, ...EXPECTED_PROOF_ROOT.split('/'));
  await rm(outputDirectory, { recursive: true, force: true });
  await mkdir(outputDirectory, { recursive: true });
  await Promise.all([...artifacts].map(([name, contents]) => writeFile(join(outputDirectory, name), contents, 'utf8')));
  return relative(repoRoot, outputDirectory).replaceAll('\\', '/');
}

async function main() {
  const repoRoot = process.cwd();
  const commandResults = [];
  commandResults.push(await runCommand(process.execPath, ['scripts/engineering-graph.mjs', 'validate'], repoRoot));
  commandResults.push(await runCommand(process.execPath, ['--test', FOCUSED_TEST_PATH], repoRoot));
  const { graph, codeMap } = await loadWp01RouteInputs(repoRoot);
  const route = validateWp01RouteBoundary(graph, codeMap);
  const commit = await gitHead(repoRoot);
  const artifacts = renderWp01ProofArtifacts({
    checkedAt: new Date().toISOString(),
    commit,
    commandResults,
    route,
  });
  await writeWp01ProofArtifacts(repoRoot, artifacts);
  const failed = commandResults.find((result) => result.exitCode !== 0);
  if (failed) throw new Error(`${failed.command} exited with ${failed.exitCode}`);
}

function rejectProductOrParentReferences(workpack, codeRoute) {
  const serialized = JSON.stringify({ workpack, codeRoute }).toLowerCase();
  for (const forbidden of [
    'parent-client-runtime-distribution-plan',
    'crates/child-runtime',
    'scripts/release/',
    'platforms/android/',
    'platforms/ios/',
    'platforms/macos/',
  ]) {
    assertCondition(!serialized.includes(forbidden), `WP01 route must not contain product reference ${forbidden}`);
  }
}

function renderCommandLog(commandResults) {
  return commandResults
    .map(
      (result) =>
        `command: ${result.command}\n` +
        `exit: ${result.exitCode}\n` +
        `result: ${result.exitCode === 0 ? 'pass' : 'fail'}\n` +
        `artifact: n/a\n` +
        `notes: ${result.summary}\n`
    )
    .join('\n');
}

function runCommand(command, args, cwd) {
  const commandLine = [command, ...args].join(' ');
  return new Promise((resolveCommand, rejectCommand) => {
    const chunks = [];
    const child = spawn(command, args, { cwd, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.stderr.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('error', rejectCommand);
    child.once('exit', (code) => {
      const output = chunks.join('').trim();
      resolveCommand({
        command: commandLine,
        exitCode: code ?? 1,
        summary: output.split(/\r?\n/).filter(Boolean).at(-1) ?? 'no command output',
      });
    });
  });
}

async function gitHead(repoRoot) {
  const result = await runCommand('git', ['rev-parse', 'HEAD'], repoRoot);
  if (result.exitCode !== 0) throw new Error('git rev-parse HEAD failed');
  return result.summary;
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

function assertExactArray(actual, expected, label) {
  assertCondition(Array.isArray(actual), `${label} must be an array`);
  assertEqual(JSON.stringify(actual), JSON.stringify(expected), label);
}

function assertExactObjectKeys(actual, expected, label) {
  assertCondition(actual && typeof actual === 'object' && !Array.isArray(actual), `${label} must be an object`);
  assertExactArray(Object.keys(actual).sort(), [...expected].sort(), label);
}

function assertEqual(actual, expected, label) {
  assertCondition(actual === expected, `${label}: expected ${expected}, received ${actual}`);
}

function assertCondition(condition, message) {
  if (!condition) throw new Error(message);
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  await main();
}
