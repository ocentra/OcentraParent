import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  GRAPH_SCHEMA_VERSION,
  buildCodeInventory,
  deriveStates,
  completionGaps,
  loadGraph,
  planId,
  relatedNodes,
  scopeNodes,
  summarizeGraph,
  validateGraph,
} from '../../scripts/engineering-graph-lib.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

function workpack(id, state, extra = {}) {
  return {
    id,
    kind: 'workpack',
    title: id,
    path: 'AGENTS.md',
    parent: null,
    dependsOn: [],
    state,
    lifecycleState: state,
    metadata: { needsReview: false },
    ...extra,
  };
}

function graph(nodes, edges = []) {
  return { schemaVersion: GRAPH_SCHEMA_VERSION, nodes, edges };
}

test('dependency resolution derives READY after a DONE dependency', () => {
  const value = graph(
    [workpack('A', 'done'), workpack('B', 'ready', { dependsOn: ['A'] })],
    [{ from: 'B', to: 'A', kind: 'depends_on' }]
  );

  const states = deriveStates(value, { root: repoRoot });
  assert.equal(states.get('A'), 'done');
  assert.equal(states.get('B'), 'ready');
});

test('blocking and unlock are recomputed when a dependency changes', () => {
  const value = graph(
    [workpack('A', 'active'), workpack('B', 'ready', { dependsOn: ['A'] })],
    [{ from: 'B', to: 'A', kind: 'depends_on' }]
  );

  assert.equal(deriveStates(value, { root: repoRoot }).get('B'), 'blocked');
  value.nodes.find((node) => node.id === 'A').state = 'done';
  value.nodes.find((node) => node.id === 'A').lifecycleState = 'done';
  assert.equal(deriveStates(value, { root: repoRoot }).get('B'), 'ready');
});

test('multiple dependencies remain blocked until every dependency is DONE', () => {
  const value = graph(
    [workpack('A', 'done'), workpack('B', 'ready'), workpack('C', 'ready', { dependsOn: ['A', 'B'] })],
    [
      { from: 'C', to: 'A', kind: 'depends_on' },
      { from: 'C', to: 'B', kind: 'depends_on' },
    ]
  );

  assert.equal(deriveStates(value, { root: repoRoot }).get('C'), 'blocked');
  const dependency = value.nodes.find((node) => node.id === 'B');
  dependency.state = 'done';
  dependency.lifecycleState = 'done';
  assert.equal(deriveStates(value, { root: repoRoot }).get('C'), 'ready');
});

test('independent READY workpacks are returned as a parallel set', () => {
  const value = graph([workpack('A', 'ready'), workpack('B', 'ready')]);
  const summary = summarizeGraph(value);
  assert.deepEqual(
    summary.ready.map((node) => node.id),
    ['A', 'B']
  );
});

test('dependency cycles fail validation', () => {
  const value = graph(
    [
      workpack('A', 'ready', { dependsOn: ['B'] }),
      workpack('B', 'ready', { dependsOn: ['C'] }),
      workpack('C', 'ready', { dependsOn: ['A'] }),
    ],
    [
      { from: 'A', to: 'B', kind: 'depends_on' },
      { from: 'B', to: 'C', kind: 'depends_on' },
      { from: 'C', to: 'A', kind: 'depends_on' },
    ]
  );

  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('dependency cycle')));
});

test('missing dependency references fail validation', () => {
  const value = graph(
    [workpack('B', 'ready', { dependsOn: ['MISSING'] })],
    [{ from: 'B', to: 'MISSING', kind: 'depends_on' }]
  );

  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('MISSING')));
});

test('DONE requires every completion-contract reference', () => {
  const value = graph([
    workpack('DONE-BUT-INCOMPLETE', 'done', {
      completion: {
        required: ['implementation', 'tests', 'proof', 'checklist'],
        references: {
          implementation: ['AGENTS.md'],
          tests: [],
          proof: [],
          checklist: ['AGENTS.md'],
        },
      },
    }),
  ]);

  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('completion contract')));
});

test('missing expected artifacts demote stale DONE to validation', () => {
  const value = graph([
    workpack('STALE-DONE', 'done', {
      completion: {
        required: ['implementation', 'proof'],
        references: { implementation: ['AGENTS.md'], proof: ['AGENTS.md'] },
        expected: { proof: ['output/does-not-exist'] },
      },
    }),
  ]);

  assert.equal(deriveStates(value, { root: repoRoot }).get('STALE-DONE'), 'validation');
  assert.deepEqual(completionGaps(repoRoot, value.nodes[0]), [
    'proof: missing expected artifact output/does-not-exist',
  ]);
  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('missing expected artifact')));
});

test('code inventory reports implementation and test topology without claiming acceptance', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-'));
  await mkdir(path.join(root, 'crates', 'example', 'src'), { recursive: true });
  await mkdir(path.join(root, 'crates', 'example', 'tests'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await writeFile(path.join(root, 'crates', 'example', 'src', 'lib.rs'), 'pub fn value() {}\n');
  await writeFile(path.join(root, 'crates', 'example', 'tests', 'unit.rs'), '#[test] fn value() {}\n');
  await writeFile(
    path.join(root, 'docs', 'engineering-graph', 'code-map.json'),
    JSON.stringify({
      schemaVersion: 1,
      authority: 'test map',
      plans: { 'example-plan': ['crates/example'] },
    })
  );

  const inventory = await buildCodeInventory({ root });
  assert.deepEqual(inventory.totals, {
    plans: 1,
    codeFiles: 2,
    implementationFiles: 1,
    testFiles: 1,
  });
  assert.equal(inventory.plans[0].state, 'code-and-tests');
  assert.deepEqual(inventory.plans[0].missingRoots, []);
  assert.deepEqual(inventory.plans[0].testPaths, ['crates/example/tests/unit.rs']);
});

test('repository bootstrap is queryable and keeps plan scope isolated', async () => {
  const value = await loadGraph(repoRoot);
  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, true, report.errors.join('; '));

  assert.equal(value.nodes.filter((node) => node.kind === 'plan').length, 23);
  assert.ok(value.nodes.filter((node) => node.kind === 'workpack').length >= 500);

  const appContract = value.nodes.find((node) => node.id === 'WP-app-plan-01-contract-boundary-and-effect-schemas');
  assert.equal(appContract.state, 'validation');
  assert.match(appContract.metadata.statusText, /^Validation/);
  assert.deepEqual(appContract.completion.expected.proof, [
    'output/app-plan-proof/01-contract-boundary-and-effect-schemas',
  ]);

  const remoteSibling = value.nodes.find((node) => node.id === 'WP-remote-access-plan-02-live-screen-relay');
  assert.deepEqual(remoteSibling.completion.references.proof, []);

  const eventingProofOverride = value.nodes.find(
    (node) => node.id === 'WP-eventing-plan-11-type-safety-and-ownership-hardening'
  );
  assert.deepEqual(eventingProofOverride.completion.references.proof, ['docs/proof/eventing-plan']);

  const networkRouting = value.nodes.find((node) => node.id === 'WP-network-plan-08-control-catalog-reference-routing');
  assert.equal(networkRouting.state, 'validation');
  assert.match(networkRouting.metadata.statusText, /^Validation/);
  assert.deepEqual(networkRouting.completion.references.proof, [
    'output/network-plan-proof/08-control-catalog-reference-routing',
  ]);

  const blockedId = 'WP-policy-control-plane-plan-05-ask-parent-overrides';
  const dependencyId = 'WP-policy-control-plane-plan-04-delivery-ack-audit';
  assert.equal(deriveStates(value, { root: repoRoot }).get(blockedId), 'blocked');
  assert.deepEqual(relatedNodes(value, blockedId, 'deps'), [dependencyId]);

  const scoped = scopeNodes(value, planId('app-plan'));
  assert.ok(scoped.some((node) => node.id === planId('app-plan')));
  assert.ok(scoped.every((node) => node.parent === planId('app-plan') || node.id === planId('app-plan')));
  assert.ok(!scoped.some((node) => node.id.startsWith('WP-browser-plan-')));

  const globalSummary = summarizeGraph(value, undefined, { root: repoRoot });
  assert.equal(globalSummary.ready.length, 0);
});
