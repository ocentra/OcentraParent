import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import test from 'node:test';

import {
  GRAPH_SCHEMA_VERSION,
  deriveStates,
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

test('repository bootstrap is queryable and keeps plan scope isolated', async () => {
  const value = await loadGraph(repoRoot);
  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, true, report.errors.join('; '));

  assert.equal(value.nodes.filter((node) => node.kind === 'plan').length, 23);
  assert.ok(value.nodes.filter((node) => node.kind === 'workpack').length >= 500);

  const blockedId = 'WP-policy-control-plane-plan-05-ask-parent-overrides';
  const dependencyId = 'WP-policy-control-plane-plan-04-delivery-ack-audit';
  assert.equal(deriveStates(value, { root: repoRoot }).get(blockedId), 'blocked');
  assert.deepEqual(relatedNodes(value, blockedId, 'deps'), [dependencyId]);

  const scoped = scopeNodes(value, planId('app-plan'));
  assert.ok(scoped.some((node) => node.id === planId('app-plan')));
  assert.ok(scoped.every((node) => node.parent === planId('app-plan') || node.id === planId('app-plan')));
  assert.ok(!scoped.some((node) => node.id.startsWith('WP-browser-plan-')));
});
