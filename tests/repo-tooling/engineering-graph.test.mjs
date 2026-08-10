import assert from 'node:assert/strict';
import { mkdtemp, mkdir, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  GRAPH_SCHEMA_VERSION,
  buildCodeInventory,
  buildBootstrapGraph,
  buildProgressReport,
  classifyWorkpackStatus,
  deriveStates,
  completionGaps,
  explainBlocked,
  flattenProgressReport,
  graphSourceDrift,
  loadGraph,
  nextWork,
  parseWorkpackRows,
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

test('workpack status is read from the declared Status or State column', () => {
  const statusFirst = `| Status | Workpack | Proof root |\n| --- | --- | --- |\n| Open | [one](workpacks/01-one.md) | docs/proof/one |`;
  const statusLast = `| Workpack | Purpose | Status |\n| --- | --- | --- |\n| [two](workpacks/02-two.md) | scope | Complete |`;

  assert.equal(parseWorkpackRows(statusFirst)[0].statusText, 'Open');
  assert.equal(parseWorkpackRows(statusLast)[0].statusText, 'Complete');
  assert.equal(classifyWorkpackStatus('Open'), 'planned');
  assert.equal(classifyWorkpackStatus('Ready'), 'ready');
  assert.equal(classifyWorkpackStatus('historical'), 'validation');
  assert.equal(classifyWorkpackStatus('Incomplete; not merged'), 'planned');
});

test('missing workpack paths fail validation instead of authorizing phantom work', () => {
  const value = graph([workpack('MISSING-PATH', 'ready', { path: 'workpacks/does-not-exist.md' })]);
  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('workpack path is missing')));
});

test('graph why explains lifecycle-only blockers', () => {
  const value = graph([
    workpack('LIFECYCLE-BLOCKED', 'blocked', {
      metadata: { needsReview: false, statusText: 'Waiting for human security decision' },
    }),
  ]);
  const explanation = explainBlocked(value, 'LIFECYCLE-BLOCKED', { root: repoRoot });
  assert.equal(explanation.state, 'blocked');
  assert.ok(explanation.reasons.some((reason) => reason.includes('human security decision')));
});

test('state overrides require an evidenced validation slice', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-overrides-'));
  await mkdir(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Example](workpacks/01-example.md) | Open |\n'
  );
  await writeFile(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', '01-example.md'), '# Example\n');
  await writeFile(
    path.join(root, 'docs', 'engineering-graph', 'overrides.json'),
    JSON.stringify({
      schemaVersion: 1,
      edges: [],
      stateOverrides: [{ id: 'WP-example-plan-01-example', state: 'validation', reason: 'missing evidence' }],
    })
  );

  const imported = await buildBootstrapGraph({ root });
  const node = imported.nodes.find((candidate) => candidate.id === 'WP-example-plan-01-example');
  assert.equal(node.state, 'planned');
  assert.ok(node.metadata.needsReview);
  assert.ok(node.metadata.stateOverrideRejected.includes('evidence is required'));
});

test('bootstrap reports workpack files that are not indexed instead of hiding them', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-unindexed-'));
  await mkdir(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks'), { recursive: true });
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Example](workpacks/01-example.md) | Open |\n'
  );
  await writeFile(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', '01-example.md'), '# Example\n');
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', 'legacy-proposal.md'),
    '# Legacy proposal\n'
  );

  const imported = await buildBootstrapGraph({ root });
  assert.deepEqual(imported.migration.unindexedWorkpackArtifacts, [
    {
      planId: 'PLAN-example-plan',
      indexPath: 'docs/plans/example-plan/WORKPACK_INDEX.md',
      paths: ['workpacks/legacy-proposal.md'],
    },
  ]);
  assert.ok(
    imported.migration.ambiguities.some(
      (item) =>
        item.scope === 'PLAN-example-plan:unindexed-workpack-files' &&
        item.unindexedWorkpackFiles.includes('workpacks/legacy-proposal.md')
    )
  );
});

test('graph source drift is actionable', () => {
  const value = graph([workpack('A', 'planned')]);
  assert.deepEqual(graphSourceDrift(value, value), []);
  assert.match(
    graphSourceDrift(value, graph([workpack('A', 'planned'), workpack('B', 'planned')]))[0],
    /graph:bootstrap/u
  );
});

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

test('next work exposes unblocked validation when no READY work is authorized', () => {
  const value = graph([
    workpack('VALIDATION', 'validation'),
    workpack('BLOCKED', 'blocked', { metadata: { needsReview: false, statusText: 'waiting on review' } }),
  ]);

  const queue = nextWork(value, { root: repoRoot });
  assert.deepEqual(queue.authorized, []);
  assert.deepEqual(
    queue.validationQueue.map((node) => node.id),
    ['VALIDATION']
  );
  assert.match(queue.recommendation, /No READY workpack is authorized/u);
});

test('flattened matrix preserves plan, topology, dependency, and completion gaps', () => {
  const rows = flattenProgressReport({
    plans: [
      {
        id: 'PLAN-example',
        title: 'Example',
        state: 'active',
        workpacks: {
          rows: [
            {
              id: 'WP-example-01',
              title: 'One',
              state: 'validation',
              storedState: 'active',
              dependsOn: ['WP-example-00'],
              blockers: [{ id: 'WP-example-00', state: 'validation' }],
              unlocks: ['WP-example-02'],
              completionContract: { gaps: ['tests: missing'] },
              codeTestTopology: { state: 'code-and-tests', implementationFiles: 2, testFiles: 1 },
            },
          ],
        },
      },
    ],
  });

  assert.deepEqual(rows, [
    {
      planId: 'PLAN-example',
      planTitle: 'Example',
      planState: 'active',
      workpackId: 'WP-example-01',
      workpackTitle: 'One',
      state: 'validation',
      storedState: 'active',
      codeState: 'code-and-tests',
      implementationFiles: 2,
      testFiles: 1,
      dependsOn: ['WP-example-00'],
      blockers: [{ id: 'WP-example-00', state: 'validation' }],
      unlocks: ['WP-example-02'],
      completionGapCount: 1,
      completionGaps: ['tests: missing'],
    },
  ]);
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

test('dependency declarations and edge kinds must agree', () => {
  const value = graph(
    [workpack('A', 'ready', { dependsOn: ['B'] }), workpack('B', 'done')],
    [{ from: 'A', to: 'B', kind: 'depends-on' }]
  );

  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('unsupported kind')));
  assert.ok(report.errors.some((error) => error.includes('matching depends_on')));
});

test('reviewed dependency edges require existing evidence', () => {
  const value = graph(
    [workpack('A', 'ready', { dependsOn: ['B'] }), workpack('B', 'done')],
    [{ from: 'A', to: 'B', kind: 'depends_on', confidence: 'reviewed' }]
  );
  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('depends_on edge is missing evidence')));
});

test('failed workpacks propagate to their plan state', () => {
  const value = graph([
    { id: 'PLAN-test', kind: 'plan', title: 'Test', path: 'AGENTS.md', parent: 'GOAL-test', state: 'active' },
    { ...workpack('FAILED', 'failed'), parent: 'PLAN-test' },
  ]);
  value.nodes.push({ id: 'GOAL-test', kind: 'goal', title: 'Test', path: 'AGENTS.md', parent: null, state: 'active' });
  assert.equal(deriveStates(value, { root: repoRoot }).get('PLAN-test'), 'failed');
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

test('completion evidence cannot reuse planning documents as executable implementation or tests', () => {
  const value = graph([
    workpack('WP-app-plan-01-contract-boundary-and-effect-schemas', 'done', {
      metadata: { needsReview: false, planSlug: 'app-plan' },
      completion: {
        required: ['implementation', 'tests'],
        reviewed: { implementation: true, tests: true },
        references: {
          implementation: ['docs/plans/app-plan/workpacks/01-contract-boundary-and-effect-schemas.md'],
          tests: ['docs/plans/app-plan/TEST_PROOF_EXPECTATIONS.md'],
        },
      },
    }),
  ]);

  assert.deepEqual(completionGaps(repoRoot, value.nodes[0]), [
    'implementation: planning document is not executable evidence docs/plans/app-plan/workpacks/01-contract-boundary-and-effect-schemas.md',
    'tests: planning document is not executable evidence docs/plans/app-plan/TEST_PROOF_EXPECTATIONS.md',
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
    'implementation: reviewed evidence is not recorded',
    'proof: reviewed evidence is not recorded',
    'proof: missing expected artifact output/does-not-exist',
  ]);
  const report = validateGraph(value, { root: repoRoot });
  assert.equal(report.ok, false);
  assert.ok(report.errors.some((error) => error.includes('missing expected artifact')));
});

test('explicit durable proof may satisfy a missing generated proof expectation', () => {
  const value = graph([
    workpack('DURABLE-PROOF', 'done', {
      metadata: {
        needsReview: false,
        proofOverride: { satisfiesExpected: true },
      },
      completion: {
        required: ['proof'],
        reviewed: { proof: true },
        references: { proof: ['AGENTS.md'] },
        expected: { proof: ['output/portable-proof-bundle'] },
      },
    }),
  ]);

  assert.deepEqual(completionGaps(repoRoot, value.nodes[0]), []);
  assert.equal(deriveStates(value, { root: repoRoot }).get('DURABLE-PROOF'), 'done');
  assert.equal(validateGraph(value, { root: repoRoot }).ok, true);
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
    reviewedWorkpackMaps: 0,
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

  const eventingWp06 = value.nodes.find((node) => node.id === 'WP-eventing-plan-06-journal-replay-and-lineage');
  assert.equal(eventingWp06.state, 'done');
  assert.deepEqual(completionGaps(repoRoot, eventingWp06), []);
  const eventingReport = await buildProgressReport({ root: repoRoot });
  const eventingWp06Report = eventingReport.plans
    .find((plan) => plan.id === 'PLAN-eventing-plan')
    .workpacks.rows.find((workpack) => workpack.id === eventingWp06.id);
  assert.equal(eventingWp06Report.codeTestTopology.scope, 'reviewed-workpack-roots');
  assert.ok(eventingWp06Report.codeTestTopology.implementationPaths.some((file) => file.endsWith('src/journal.rs')));
  assert.ok(eventingWp06Report.codeTestTopology.testPaths.some((file) => file.includes('tests/journal_replay/')));

  const enforcementWp11 = value.nodes.find(
    (node) => node.id === 'WP-v0-8-enforcement-control-plan-11-audit-journal-events'
  );
  assert.notEqual(enforcementWp11.state, 'done');

  const eventingHistorical = value.nodes.find(
    (node) => node.id === 'WP-eventing-plan-04-queue-idempotency-dead-letter'
  );
  assert.equal(eventingHistorical.state, 'validation');

  const networkRouting = value.nodes.find((node) => node.id === 'WP-network-plan-08-control-catalog-reference-routing');
  assert.equal(networkRouting.state, 'validation');
  assert.match(networkRouting.metadata.statusText, /^Validation/);
  assert.deepEqual(networkRouting.completion.references.proof, []);
  assert.deepEqual(networkRouting.completion.expected.proof, [
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
  assert.ok(globalSummary.blocked.length > 0);
});

test('progress report joins derived workpack state with reviewed plan topology', async () => {
  const report = await buildProgressReport({ root: repoRoot });
  assert.equal(report.scope, 'GOAL-ocentra-parent');
  assert.equal(report.totals.plans, 23);
  assert.ok(report.totals.workpacks >= 500);
  assert.equal(report.validation.ok, true);
  assert.ok(report.totals.implementationFiles > 0);
  assert.ok(report.totals.testFiles > 0);

  const policy = report.plans.find((plan) => plan.id === 'PLAN-policy-control-plane-plan');
  assert.ok(policy);
  assert.equal(policy.codeTestTopology.scope, 'reviewed-plan-roots');
  assert.ok(policy.codeTestTopology.implementationFiles > 0);
  assert.ok(policy.codeTestTopology.testFiles > 0);
  assert.ok(
    policy.workpacks.rows.every(
      (workpack) =>
        workpack.codeTestTopology === 'unknown-workpack-ownership' ||
        workpack.codeTestTopology.scope === 'reviewed-workpack-roots'
    )
  );

  const blocked = policy.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-policy-control-plane-plan-05-ask-parent-overrides'
  );
  assert.equal(blocked.state, 'blocked');
});

test('reviewed workpack code maps stay exact while unmapped rows remain unknown', async () => {
  const report = await buildProgressReport({ root: repoRoot });
  const app = report.plans.find((plan) => plan.id === 'PLAN-app-plan');
  const mapped = app.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-app-plan-01-contract-boundary-and-effect-schemas'
  );
  assert.equal(mapped.codeTestTopology.scope, 'reviewed-workpack-roots');
  assert.ok(mapped.codeTestTopology.implementationFiles > 0);
  assert.ok(mapped.codeTestTopology.testFiles > 0);
  assert.ok(mapped.codeTestTopology.implementationPaths.some((file) => file.endsWith('runtime_decision.rs')));
  const unmapped = app.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-app-plan-21-windows-owned-process-terminate-time-limit-proof'
  );
  assert.equal(unmapped.codeTestTopology, 'unknown-workpack-ownership');
});

test('root goal scope includes the full reviewed code map', async () => {
  const unscoped = await buildCodeInventory({ root: repoRoot });
  const rootScoped = await buildCodeInventory({ root: repoRoot, scope: 'GOAL-ocentra-parent' });
  assert.equal(rootScoped.totals.plans, unscoped.totals.plans);
  assert.equal(rootScoped.totals.implementationFiles, unscoped.totals.implementationFiles);
  assert.equal(rootScoped.totals.testFiles, unscoped.totals.testFiles);
});

test('repository bootstrap imports numeric-table workpacks from their existing files', async () => {
  const graph = await loadGraph(repoRoot);
  const lanWorkpacks = graph.nodes.filter((node) => node.id.startsWith('WP-lan-plan-'));
  assert.equal(lanWorkpacks.length, 25);
  assert.ok(lanWorkpacks.every((workpack) => workpack.metadata.sourceFormat === 'numeric-table-row'));
  assert.ok(lanWorkpacks.some((workpack) => workpack.path.endsWith('25-rollout-checklist-and-pr-gate.md')));
  assert.equal(
    lanWorkpacks.find((workpack) => workpack.id === 'WP-lan-plan-18-signed-child-hello-heartbeat').state,
    'validation'
  );
  assert.equal(
    lanWorkpacks.find((workpack) => workpack.id === 'WP-lan-plan-22-current-state-and-gap-map').state,
    'validation'
  );
});
