#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const WorkpackId = 'WP-app-game-plan-197-app-game-linux-docker-host-preflight';
const PlanId = 'PLAN-app-game-plan';
const WorkpackPath = 'docs/plans/app-game-plan/workpacks/197-app-game-linux-docker-host-preflight.md';
const ExpectedProofRoot = 'output/app-game-plan-proof/197-app-game-linux-docker-host-preflight';
const ExpectedTestRoots = Object.freeze([
  'crates/agent-service/tests/unit/app_game_linux_docker_host_preflight.rs',
  'crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_parser_tests.rs',
  'crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_path_security_tests.rs',
  'crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_cleanup_tests.rs',
  'crates/agent-service/tests/unit/app_game_platform_probe_cache_tests.rs',
  'crates/agent-service/tests/unit/app_game_platform_proof_status_route_rejection_tests.rs',
]);
const NoClaim =
  'visibility and redacted-count proof only; no container policy, adapter dispatch, enforcement, provider delivery, or child delivery';

export const Wp197ProofArtifactNames = Object.freeze([
  '00-scope-summary.json',
  '01-negative-case-proof.md',
  '02-no-claim-boundary.md',
  '16-validation-commands.log',
]);

export function validateWp197RouteBoundary(graph, codeMap) {
  const workpack = graph.nodes?.find((candidate) => candidate.id === WorkpackId);
  requireCondition(workpack, `generated graph is missing ${WorkpackId}`);
  requireEqual(workpack.kind, 'workpack', 'WP197 graph kind');
  requireEqual(workpack.parent, PlanId, 'WP197 owning plan');
  requireEqual(workpack.path, WorkpackPath, 'WP197 workpack path');
  requireExactArray(workpack.dependsOn, [], 'WP197 dependencies');
  requireEqual(workpack.state, 'done', 'WP197 source state');
  requireEqual(workpack.lifecycleState, 'done', 'WP197 lifecycle state');
  requireEqual(workpack.metadata?.needsReview, false, 'WP197 dependency review state');

  const completion = workpack.completion;
  requireCondition(completion, 'WP197 completion contract is missing');
  requireExactArray(
    completion.required,
    ['implementation', 'tests', 'proof', 'checklist'],
    'WP197 completion requirements'
  );
  requireEqual(completion.reviewed?.implementation, true, 'WP197 reviewed implementation');
  requireExactArray(completion.expected?.proof, [ExpectedProofRoot], 'WP197 expected proof root');
  requireExactArray(completion.expected?.tests, ExpectedTestRoots, 'WP197 expected tests');

  const codeRoute = codeMap.workpacks?.[WorkpackId];
  requireCondition(codeRoute, `code map is missing ${WorkpackId}`);
  requireEqual(codeRoute.planSlug, 'app-game-plan', 'WP197 code-map plan');
  requireExactArray(codeRoute.expectedTestRoots, ExpectedTestRoots, 'WP197 code-map test roots');
  for (const root of ExpectedTestRoots) {
    requireCondition(codeRoute.roots.includes(root), `WP197 code map is missing real test root ${root}`);
  }

  return Object.freeze({
    workpackId: workpack.id,
    planId: workpack.parent,
    state: workpack.state,
    lifecycleState: workpack.lifecycleState,
    expectedProofRoot: completion.expected.proof[0],
    expectedTestRoots: [...completion.expected.tests],
  });
}

export function renderWp197ProofArtifacts({ checkedAt, commit, commandResults, route }) {
  const passed = commandResults.every((result) => result.exitCode === 0);
  const status = passed ? 'passed' : 'failed';
  const scopeSummary = {
    schemaVersion: 1,
    planId: route.planId,
    workpackId: route.workpackId,
    checkedAt,
    commit,
    status,
    graphState: route.state,
    lifecycleState: route.lifecycleState,
    expectedProofRoot: route.expectedProofRoot,
    expectedTestRoots: route.expectedTestRoots,
    dockerEvidence: Object.freeze({
      cli: 'typed-readiness-only',
      daemon: 'typed-readiness-only',
      contexts: 'redacted-count-only',
      images: 'redacted-count-only',
      containers: 'redacted-count-only',
    }),
    noClaim: NoClaim,
  };
  const negativeProof =
    `# WP197 negative-case proof\n\n` +
    `Result: **${status}**.\n\n` +
    `The focused Rust targets exercise unavailable probes, cleanup-owner degradation, failed or malformed markers, ` +
    `invalid UTF-8, over-bound counts, parent traversal, untrusted or missing probe roots, cache unavailable state, ` +
    `and route rejection. Ready fixtures assert that only bounded context, image, and container counts cross the ` +
    `parent-safe proof boundary.\n`;
  const noClaimBoundary =
    `# WP197 no-claim boundary\n\n` +
    `This proof stores no Docker context names, image names, container identifiers, executable paths, raw daemon ` +
    `diagnostics, or private target details. It does not claim container policy execution, adapter dispatch, ` +
    `platform enforcement, provider delivery, child-device delivery, rollback, or audit authority.\n`;

  return new Map([
    ['00-scope-summary.json', `${JSON.stringify(scopeSummary, null, 2)}\n`],
    ['01-negative-case-proof.md', negativeProof],
    ['02-no-claim-boundary.md', noClaimBoundary],
    ['16-validation-commands.log', renderValidationLog(commandResults)],
  ]);
}

export function writeWp197ProofArtifacts(outputDirectory, artifacts, rawLogs = new Map()) {
  requireExactArray([...artifacts.keys()], Wp197ProofArtifactNames, 'WP197 proof artifact names');
  fs.rmSync(outputDirectory, { force: true, recursive: true });
  fs.mkdirSync(path.join(outputDirectory, 'raw'), { recursive: true });
  for (const [name, contents] of artifacts) {
    fs.writeFileSync(path.join(outputDirectory, name), contents, 'utf8');
  }
  for (const [name, contents] of rawLogs) {
    fs.writeFileSync(path.join(outputDirectory, 'raw', name), contents, 'utf8');
  }
}

export function runWp197Proof({ repoRoot, outputDirectory, cargoTargetDirectory = null }) {
  const route = validateWp197RouteBoundary(
    readJson(path.join(repoRoot, 'docs/engineering-graph/graph.json')),
    readJson(path.join(repoRoot, 'docs/engineering-graph/code-map.json'))
  );
  const environment = {
    ...process.env,
    ...(cargoTargetDirectory == null ? {} : { CARGO_TARGET_DIR: cargoTargetDirectory }),
  };
  const commandSpecs = proofCommandSpecs();
  const commandResults = commandSpecs.map((spec, index) => runProofCommand(repoRoot, environment, spec, index));
  const commitResult = spawnSync('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, encoding: 'utf8' });
  requireEqual(commitResult.status, 0, 'git rev-parse HEAD');
  const artifacts = renderWp197ProofArtifacts({
    checkedAt: new Date().toISOString(),
    commit: commitResult.stdout.trim(),
    commandResults,
    route,
  });
  const rawLogs = new Map(commandResults.map((result) => [result.rawArtifact, result.rawOutput]));
  writeWp197ProofArtifacts(outputDirectory, artifacts, rawLogs);
  return commandResults.every((result) => result.exitCode === 0) ? 0 : 1;
}

function proofCommandSpecs() {
  const architectureInvocation = resolveNpmInvocation([
    'run',
    '--silent',
    'lint:architecture',
    '--',
    '--files',
    'crates/agent-protocol/src/app_game_platform_proof_status.rs',
    'crates/agent-service/src/activity_api/app_game_linux_docker_host_preflight.rs',
    'crates/agent-service/tests/unit/app_game_linux_preflight.rs',
    'docs/plans/app-game-plan',
  ]);
  return [
    {
      id: 'graph-validate',
      command: process.execPath,
      args: ['scripts/engineering-graph.mjs', 'validate'],
      display: 'npm run graph:validate',
      owner: 'engineering-graph',
    },
    {
      id: 'protocol-contract',
      command: 'cargo',
      args: [
        'test',
        '-q',
        '-p',
        'ocentra-parent-agent-protocol',
        '--test',
        'contract',
        'app_game_platform_proof_status',
        '--',
        '--test-threads=1',
      ],
      display:
        'cargo test -q -p ocentra-parent-agent-protocol --test contract app_game_platform_proof_status -- --test-threads=1',
      owner: 'agent-protocol',
    },
    {
      id: 'service-preflight',
      command: 'cargo',
      args: [
        'test',
        '-q',
        '-p',
        'ocentra-parent-agent-service',
        '--test',
        'app_game_linux_preflight',
        '--',
        '--test-threads=1',
      ],
      display: 'cargo test -q -p ocentra-parent-agent-service --test app_game_linux_preflight -- --test-threads=1',
      owner: 'agent-service',
    },
    {
      id: 'architecture',
      command: architectureInvocation.command,
      args: architectureInvocation.args,
      display: 'npm run lint:architecture -- --files <WP197 protocol/service/tests/docs>',
      owner: 'app-game-plan',
    },
  ];
}

function runProofCommand(repoRoot, environment, spec, index) {
  const startedAt = Date.now();
  const result = spawnSync(spec.command, spec.args, {
    cwd: repoRoot,
    encoding: 'utf8',
    env: environment,
    maxBuffer: 64 * 1024 * 1024,
    windowsHide: true,
  });
  const exitCode = result.status ?? 1;
  const rawOutput = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  const summary = rawOutput
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter(Boolean)
    .at(-1);
  return Object.freeze({
    id: spec.id,
    command: spec.display,
    owner: spec.owner,
    exitCode,
    durationMs: Date.now() - startedAt,
    status: exitCode === 0 ? 'passed' : 'failed',
    rawArtifact: `${String(index + 1).padStart(2, '0')}-${spec.id}.log`,
    rawOutput,
    summary: summary ?? (result.error == null ? 'no command output' : 'command could not be started'),
  });
}

function renderValidationLog(results) {
  return `${results
    .map(
      (result) =>
        `workpack: ${WorkpackId}\n` +
        `command_id: ${result.id}\n` +
        `owner: ${result.owner}\n` +
        `command: ${result.command}\n` +
        `exit: ${result.exitCode}\n` +
        `result: ${result.status}\n` +
        `duration_ms: ${result.durationMs}\n` +
        `artifact: raw/${result.rawArtifact}\n` +
        `diagnostics: ${result.summary}\n` +
        `no_claim: ${NoClaim}`
    )
    .join('\n\n')}\n`;
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function npmExecutable() {
  return process.platform === 'win32' ? 'npm.cmd' : 'npm';
}

export function resolveNpmInvocation(args, npmExecPath = process.env.npm_execpath) {
  return typeof npmExecPath === 'string' && npmExecPath.length > 0
    ? { command: process.execPath, args: [npmExecPath, ...args] }
    : { command: npmExecutable(), args };
}

function optionValue(name) {
  const prefix = `--${name}=`;
  return process.argv
    .slice(2)
    .find((entry) => entry.startsWith(prefix))
    ?.slice(prefix.length);
}

function requireExactArray(actual, expected, label) {
  requireCondition(Array.isArray(actual), `${label} must be an array`);
  requireEqual(JSON.stringify(actual), JSON.stringify(expected), label);
}

function requireEqual(actual, expected, label) {
  requireCondition(actual === expected, `${label}: expected ${String(expected)}, received ${String(actual)}`);
}

function requireCondition(condition, message) {
  if (!condition) throw new Error(message);
}

const invokedPath = process.argv[1] == null ? null : path.resolve(process.argv[1]);
if (invokedPath === fileURLToPath(import.meta.url)) {
  const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
  const outputDirectory = path.resolve(optionValue('output-dir') ?? path.join(repoRoot, ExpectedProofRoot));
  const cargoTargetDirectory = optionValue('cargo-target-dir') ?? null;
  const exitCode = runWp197Proof({ repoRoot, outputDirectory, cargoTargetDirectory });
  process.stdout.write(`${path.relative(repoRoot, outputDirectory).replaceAll('\\', '/')}\n`);
  process.exitCode = exitCode;
}
