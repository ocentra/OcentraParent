import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, it } from 'node:test';
import { fileURLToPath } from 'node:url';
import {
  Wp197ProofArtifactNames,
  renderWp197ProofArtifacts,
  resolveNpmInvocation,
  validateWp197RouteBoundary,
  writeWp197ProofArtifacts,
} from '../../scripts/test/app-game-linux-docker-host-preflight-proof.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const temporaryDirectories = [];

describe('app/game WP197 Linux Docker host preflight proof runner', () => {
  afterEach(() => {
    for (const directory of temporaryDirectories.splice(0, temporaryDirectories.length)) {
      fs.rmSync(directory, { force: true, recursive: true });
    }
  });

  it('runs npm scripts through the current Node executable when npm provides its CLI path', () => {
    const npmCliPath = path.join('tooling', 'npm-cli.js');

    assert.deepEqual(resolveNpmInvocation(['run', 'lint:architecture'], npmCliPath), {
      command: process.execPath,
      args: [npmCliPath, 'run', 'lint:architecture'],
    });
  });

  it('validates the real generated route and exact six behavioral test roots', () => {
    const graph = readJson('docs/engineering-graph/graph.json');
    const codeMap = readJson('docs/engineering-graph/code-map.json');
    const route = validateWp197RouteBoundary(graph, codeMap);

    assert.equal(route.workpackId, 'WP-app-game-plan-197-app-game-linux-docker-host-preflight');
    assert.equal(route.planId, 'PLAN-app-game-plan');
    assert.equal(route.state, 'done');
    assert.equal(route.lifecycleState, 'done');
    assert.equal(route.expectedTestRoots.length, 6);
    assert.deepEqual(
      route.expectedTestRoots.map((entry) => path.posix.basename(entry)),
      [
        'app_game_linux_docker_host_preflight.rs',
        'app_game_linux_docker_host_preflight_parser_tests.rs',
        'app_game_linux_docker_host_preflight_path_security_tests.rs',
        'app_game_linux_docker_host_preflight_cleanup_tests.rs',
        'app_game_platform_probe_cache_tests.rs',
        'app_game_platform_proof_status_route_rejection_tests.rs',
      ]
    );
  });

  it('rejects a route that redirects retained proof outside the canonical WP197 root', () => {
    const graph = structuredClone(readJson('docs/engineering-graph/graph.json'));
    const workpack = graph.nodes.find(
      (candidate) => candidate.id === 'WP-app-game-plan-197-app-game-linux-docker-host-preflight'
    );
    workpack.completion.expected.proof = ['output/app-game-plan-proof/wrong-workpack'];

    assert.throws(
      () => validateWp197RouteBoundary(graph, readJson('docs/engineering-graph/code-map.json')),
      /WP197 expected proof root/u
    );
  });

  it('writes the bounded proof schema, raw command pointers, and explicit no-claim boundary', () => {
    const route = validateWp197RouteBoundary(
      readJson('docs/engineering-graph/graph.json'),
      readJson('docs/engineering-graph/code-map.json')
    );
    const commandResults = ['graph-validate', 'protocol-contract', 'service-preflight', 'architecture'].map(
      (id, index) => ({
        id,
        command: `bounded command ${index + 1}`,
        owner: index === 2 ? 'agent-service' : 'app-game-plan',
        exitCode: 0,
        durationMs: index + 1,
        status: 'passed',
        rawArtifact: `${String(index + 1).padStart(2, '0')}-${id}.log`,
        rawOutput: `command ${id} passed\n`,
        summary: `command ${id} passed`,
      })
    );
    const artifacts = renderWp197ProofArtifacts({
      checkedAt: '2026-09-02T00:00:00.000Z',
      commit: '0123456789abcdef',
      commandResults,
      route,
    });
    const outputDirectory = fs.mkdtempSync(path.join(os.tmpdir(), 'app-game-wp197-proof-'));
    temporaryDirectories.push(outputDirectory);
    writeWp197ProofArtifacts(
      outputDirectory,
      artifacts,
      new Map(commandResults.map((result) => [result.rawArtifact, result.rawOutput]))
    );

    assert.deepEqual(
      fs
        .readdirSync(outputDirectory, { withFileTypes: true })
        .filter((entry) => entry.isFile())
        .map((entry) => entry.name)
        .sort(),
      [...Wp197ProofArtifactNames].sort()
    );
    assert.deepEqual(fs.readdirSync(path.join(outputDirectory, 'raw')).sort(), [
      '01-graph-validate.log',
      '02-protocol-contract.log',
      '03-service-preflight.log',
      '04-architecture.log',
    ]);
    const summary = JSON.parse(fs.readFileSync(path.join(outputDirectory, '00-scope-summary.json'), 'utf8'));
    assert.equal(summary.status, 'passed');
    assert.equal(summary.dockerEvidence.contexts, 'redacted-count-only');
    assert.equal(summary.dockerEvidence.images, 'redacted-count-only');
    assert.equal(summary.dockerEvidence.containers, 'redacted-count-only');
    assert.match(summary.noClaim, /no container policy/u);
    assert.match(
      fs.readFileSync(path.join(outputDirectory, '02-no-claim-boundary.md'), 'utf8'),
      /stores no Docker context names, image names, container identifiers/u
    );
  });
});

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}
