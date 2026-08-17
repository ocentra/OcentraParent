import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtemp, mkdir, readFile, writeFile } from 'node:fs/promises';
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
  completionGaps,
  deriveStates,
  explainBlocked,
  flattenProgressReport,
  graphSourceDrift,
  implementationPhase,
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
const graphCli = path.join(repoRoot, 'scripts', 'engineering-graph.mjs');

function runGraphCli(root, args) {
  const result = spawnSync(process.execPath, [graphCli, ...args, '--root', root], {
    cwd: root,
    encoding: 'utf8',
  });
  return {
    status: result.status,
    stdout: result.stdout ?? '',
    stderr: result.stderr ?? '',
  };
}

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

function reviewedDependencyEdge(from, to, extra = {}) {
  return {
    from,
    to,
    kind: 'depends_on',
    confidence: 'reviewed',
    evidence: ['AGENTS.md'],
    reason: 'Reviewed dependency fixture.',
    ...extra,
  };
}

function reviewedImplementationMetadata(extra = {}) {
  return {
    needsReview: false,
    completionEvidenceOverride: {
      reason: 'Reviewed implementation evidence fixture.',
      evidence: ['AGENTS.md'],
      requirements: ['implementation'],
    },
    ...extra,
  };
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
      schemaVersion: GRAPH_SCHEMA_VERSION,
      edges: [],
      stateOverrides: [{ id: 'WP-example-plan-01-example', state: 'validation', reason: 'missing evidence' }],
    })
  );

  await assert.rejects(() => buildBootstrapGraph({ root }), /stateOverrides\[0\]\.evidence must be a non-empty array/u);
});

test('partial reviewed implementation evidence never marks unfinished tests and proof DONE', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-partial-evidence-'));
  await mkdir(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await mkdir(path.join(root, 'src'), { recursive: true });
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Example](workpacks/01-example.md) | Complete |\n'
  );
  await writeFile(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', '01-example.md'), '# Example\n');
  await writeFile(path.join(root, 'src', 'runtime.mjs'), 'export const runtime = true;\n');
  await writeFile(
    path.join(root, 'docs', 'engineering-graph', 'overrides.json'),
    JSON.stringify({
      schemaVersion: GRAPH_SCHEMA_VERSION,
      edges: [],
      completionEvidenceOverrides: [
        {
          id: 'WP-example-plan-01-example',
          implementation: ['src/runtime.mjs'],
          reason: 'Reviewed production implementation only; validation remains open.',
          evidence: ['docs/plans/example-plan/workpacks/01-example.md'],
        },
      ],
    })
  );

  const imported = await buildBootstrapGraph({ root });
  const node = imported.nodes.find((candidate) => candidate.id === 'WP-example-plan-01-example');
  assert.equal(node.state, 'validation');
  assert.equal(node.completion.reviewed.implementation, true);
  assert.deepEqual(node.completion.references.implementation, ['src/runtime.mjs']);
  assert.deepEqual(node.metadata.completionEvidenceOverride.requirements, ['implementation']);
  assert.ok(completionGaps(root, node).some((gap) => gap.startsWith('tests:')));
  assert.ok(completionGaps(root, node).some((gap) => gap.startsWith('proof:')));
});

test('reviewed implementation evidence rejects missing review context and planning-only source', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-invalid-evidence-'));
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
      schemaVersion: GRAPH_SCHEMA_VERSION,
      edges: [],
      completionEvidenceOverrides: [
        {
          id: 'WP-example-plan-01-example',
          implementation: ['docs/plans/example-plan/workpacks/01-example.md'],
          evidence: [],
        },
      ],
    })
  );

  await assert.rejects(() => buildBootstrapGraph({ root }), /completionEvidenceOverrides\[0\]: reason is required/u);
});

test('override schema rejects unknown, malformed, duplicate, and unknown-ID records', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-strict-overrides-'));
  await mkdir(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Example](workpacks/01-example.md) | Open |\n'
  );
  await writeFile(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', '01-example.md'), '# Example\n');
  await writeFile(path.join(root, 'review.md'), '# Reviewed override evidence\n');
  await mkdir(path.join(root, 'src'), { recursive: true });
  await writeFile(path.join(root, 'src', 'runtime.mjs'), 'export const runtime = true;\n');

  const cases = [
    [{ schemaVersion: 1 }, /schemaVersion must be 2/u],
    [{ schemaVersion: GRAPH_SCHEMA_VERSION, unknown: true }, /unsupported top-level field unknown/u],
    [{ schemaVersion: GRAPH_SCHEMA_VERSION, edges: {} }, /edges must be an array/u],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        edges: [
          {
            from: 'WP-example-plan-01-example',
            to: 'WP-example-plan-01-example',
            kind: 'depends_on',
            confidence: 'reviewed',
            evidence: ['review.md'],
            reason: 'unknown edge field',
            unsupported: true,
          },
        ],
      },
      /edges\[0\] has unsupported field unsupported/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        edges: [
          {
            from: 'WP-example-plan-01-example',
            to: 'WP-example-plan-01-example',
            kind: 'depends_on',
            confidence: 'reviewed',
            evidence: ['AGENTS.md'],
            reason: 'invalid self edge',
          },
          {
            from: 'WP-example-plan-01-example',
            to: 'WP-example-plan-01-example',
            kind: 'depends_on',
            confidence: 'reviewed',
            evidence: ['AGENTS.md'],
            reason: 'duplicate self edge',
          },
        ],
      },
      /duplicates edge/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        workpackReviews: [{ id: 'WP-unknown', hardDependencies: [], evidence: ['AGENTS.md'], reason: 'unknown' }],
      },
      /references unknown graph node WP-unknown/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        stateOverrides: [{ id: 'WP-unknown', state: 'validation', reason: 'unknown', evidence: ['AGENTS.md'] }],
      },
      /stateOverrides\[0\] references unknown graph node WP-unknown/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        workpackReviews: [
          { id: 'WP-example-plan-01-example', hardDependencies: [], evidence: ['review.md'], reason: 'first' },
          { id: 'WP-example-plan-01-example', hardDependencies: [], evidence: ['review.md'], reason: 'duplicate' },
        ],
      },
      /workpackReviews\[1\] duplicates workpack review/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        stateOverrides: [
          { id: 'WP-example-plan-01-example', state: 'validation', reason: 'first', evidence: ['review.md'] },
          { id: 'WP-example-plan-01-example', state: 'validation', reason: 'duplicate', evidence: ['review.md'] },
        ],
      },
      /stateOverrides\[1\] duplicates state override/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        proofOverrides: [
          { id: 'WP-example-plan-01-example', proof: ['review.md'], reason: 'first', evidence: ['review.md'] },
          { id: 'WP-example-plan-01-example', proof: ['review.md'], reason: 'duplicate', evidence: ['review.md'] },
        ],
      },
      /proofOverrides\[1\] duplicates proof override/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        completionEvidenceOverrides: [
          {
            id: 'WP-example-plan-01-example',
            implementation: ['src/runtime.mjs'],
            reason: 'first',
            evidence: ['review.md'],
          },
          {
            id: 'WP-example-plan-01-example',
            implementation: ['src/runtime.mjs'],
            reason: 'duplicate',
            evidence: ['review.md'],
          },
        ],
      },
      /completionEvidenceOverrides\[1\] duplicates completion evidence override/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        ambiguities: [
          { scope: 'example', reason: 'first', nextAction: 'review' },
          { scope: 'example', reason: 'duplicate', nextAction: 'review' },
        ],
      },
      /ambiguities\[1\] duplicates ambiguity/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        workpackReviews: [
          { id: 'WP-example-plan-01-example', hardDependencies: {}, evidence: ['review.md'], reason: 'invalid' },
        ],
      },
      /workpackReviews\[0\]\.hardDependencies must be an explicit array/u,
    ],
    [
      {
        schemaVersion: GRAPH_SCHEMA_VERSION,
        proofOverrides: [
          {
            id: 'WP-example-plan-01-example',
            proof: [],
            reason: 'empty proof',
            evidence: ['AGENTS.md'],
          },
        ],
      },
      /proofOverrides\[0\]\.proof must be a non-empty array/u,
    ],
  ];
  for (const [overrides, expected] of cases) {
    await writeFile(path.join(root, 'docs', 'engineering-graph', 'overrides.json'), JSON.stringify(overrides));
    await assert.rejects(() => buildBootstrapGraph({ root }), expected);
  }
});

test('implementation evidence rejects directories and traversal paths', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-evidence-paths-'));
  await mkdir(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await mkdir(path.join(root, 'src'), { recursive: true });
  await writeFile(path.join(root, 'docs', 'evidence.md'), '# Documentation evidence\n');
  await writeFile(path.join(root, 'src', 'runtime.test.mjs'), 'export const testRuntime = true;\n');
  await writeFile(path.join(root, 'src', 'runtime.txt'), 'not executable source\n');
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Example](workpacks/01-example.md) | Open |\n'
  );
  await writeFile(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', '01-example.md'), '# Example\n');
  const base = {
    schemaVersion: GRAPH_SCHEMA_VERSION,
    completionEvidenceOverrides: [
      {
        id: 'WP-example-plan-01-example',
        reason: 'Reviewed source evidence.',
        evidence: ['docs/plans/example-plan/workpacks/01-example.md'],
      },
    ],
  };
  for (const [reference, expected] of [
    ['src', /implementation: path is not a regular file src/u],
    ['../outside.rs', /implementation: path must be repository-relative/u],
    ['src/runtime.test.mjs', /implementation: test path is not production implementation/u],
    ['docs/evidence.md', /implementation: documentation is not executable evidence/u],
    ['src/runtime.txt', /implementation: unsupported executable evidence path/u],
  ]) {
    await writeFile(
      path.join(root, 'docs', 'engineering-graph', 'overrides.json'),
      JSON.stringify({
        ...base,
        completionEvidenceOverrides: [{ ...base.completionEvidenceOverrides[0], implementation: [reference] }],
      })
    );
    await assert.rejects(() => buildBootstrapGraph({ root }), expected);
  }
});

test('CLI phase, why, report, and matrix reject unsupported flags and stale graphs', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-cli-'));
  await mkdir(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await mkdir(path.join(root, 'src'), { recursive: true });
  await writeFile(
    path.join(root, 'docs', 'plans', 'example-plan', 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Example](workpacks/01-example.md) | Ready |\n'
  );
  await writeFile(path.join(root, 'docs', 'plans', 'example-plan', 'workpacks', '01-example.md'), '# Example\n');
  await writeFile(path.join(root, 'src', 'runtime.mjs'), 'export const runtime = true;\n');
  await writeFile(
    path.join(root, 'docs', 'engineering-graph', 'code-map.json'),
    JSON.stringify({
      schemaVersion: 1,
      plans: { 'example-plan': ['src'] },
      workpacks: {
        'WP-example-plan-01-example': {
          planSlug: 'example-plan',
          codeExpectation: 'code-and-tests',
          roots: ['src'],
        },
      },
    })
  );
  const generated = await buildBootstrapGraph({ root });
  await writeFile(path.join(root, 'docs', 'engineering-graph', 'graph.json'), JSON.stringify(generated, null, 2));
  const id = 'WP-example-plan-01-example';

  const next = runGraphCli(root, ['next', '--phase', 'implementation']);
  assert.equal(next.status, 0);
  assert.match(next.stdout, new RegExp(`${id} \\[IMPLEMENTATION-ONLY\\]`));

  const why = runGraphCli(root, ['why', id, '--phase', 'implementation']);
  assert.equal(why.status, 0);
  assert.match(why.stdout, /implementation phase is authorized/u);

  const report = runGraphCli(root, ['report', '--json']);
  assert.equal(report.status, 0);
  assert.equal(JSON.parse(report.stdout).schemaVersion, GRAPH_SCHEMA_VERSION);

  const matrix = runGraphCli(root, ['matrix', '--json']);
  assert.equal(matrix.status, 0);
  assert.equal(JSON.parse(matrix.stdout).schemaVersion, GRAPH_SCHEMA_VERSION);

  for (const args of [
    ['status', '--phase', 'implementation'],
    ['bootstrap', '--phase', 'implementation'],
    ['validate', '--phase', 'implementation'],
    ['next', '--phase'],
    ['next', '--phase', 'unsupported'],
    ['next', '--root'],
  ]) {
    const rejected = runGraphCli(root, args);
    assert.notEqual(rejected.status, 0, args.join(' '));
  }

  const stale = JSON.parse(await readFile(path.join(root, 'docs', 'engineering-graph', 'graph.json'), 'utf8'));
  stale.nodes.find((node) => node.id === id).metadata.statusText = 'hand-edited graph';
  await writeFile(path.join(root, 'docs', 'engineering-graph', 'graph.json'), JSON.stringify(stale, null, 2));
  const staleNext = runGraphCli(root, ['next', '--phase', 'implementation']);
  assert.notEqual(staleNext.status, 0);
  assert.match(staleNext.stderr, /checked-in graph differs/u);
  const staleWhy = runGraphCli(root, ['why', id, '--phase', 'implementation']);
  assert.notEqual(staleWhy.status, 0);
  assert.match(staleWhy.stderr, /checked-in graph differs/u);
  for (const command of ['report', 'matrix']) {
    const staleQuery = runGraphCli(root, [command, '--json']);
    assert.notEqual(staleQuery.status, 0, command);
    assert.match(staleQuery.stderr, /checked-in graph differs/u, command);
  }
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

test('reviewed implementation gates authorize source edits without changing normal READY', () => {
  const dependency = workpack('A', 'validation', {
    metadata: reviewedImplementationMetadata(),
    completion: {
      required: ['implementation', 'tests', 'proof', 'checklist'],
      reviewed: { implementation: true },
      references: {
        implementation: ['scripts/engineering-graph-lib.mjs'],
        tests: [],
        proof: [],
        checklist: [],
      },
    },
  });
  const dependent = workpack('B', 'ready', { dependsOn: ['A'] });
  const value = graph(
    [dependency, dependent],
    [reviewedDependencyEdge('B', 'A', { implementationGate: 'reviewed-implementation' })]
  );

  const states = deriveStates(value, { root: repoRoot });
  assert.equal(states.get('B'), 'blocked');
  assert.ok(completionGaps(repoRoot, dependency).some((gap) => gap.startsWith('tests:')));
  assert.deepEqual(
    implementationPhase.authorize(value, dependent, {
      root: repoRoot,
      states,
      workpackMapping: { codeExpectation: 'code-and-tests', roots: ['scripts'] },
    }),
    { phase: 'implementation', status: 'authorized', authorized: true, blockers: [] }
  );
});

test('ungated dependencies still require DONE for implementation-only authorization', () => {
  const dependency = workpack('A', 'validation', {
    completion: {
      required: ['implementation'],
      reviewed: { implementation: true },
      references: { implementation: ['scripts/engineering-graph-lib.mjs'] },
    },
  });
  const dependent = workpack('B', 'ready', { dependsOn: ['A'] });
  const value = graph([dependency, dependent], [reviewedDependencyEdge('B', 'A')]);
  const states = deriveStates(value, { root: repoRoot });

  assert.deepEqual(
    implementationPhase.authorize(value, dependent, {
      root: repoRoot,
      states,
      workpackMapping: { codeExpectation: 'code-and-tests', roots: ['scripts'] },
    }).blockers,
    [{ kind: 'dependency', id: 'A', gate: 'done', state: 'validation', gaps: [] }]
  );
});

test('source paths without reviewed implementation evidence do not satisfy a phase gate', () => {
  const dependency = workpack('A', 'validation', {
    completion: {
      required: ['implementation'],
      reviewed: {},
      references: { implementation: ['scripts/engineering-graph-lib.mjs'] },
    },
  });
  const dependent = workpack('B', 'ready', { dependsOn: ['A'] });
  const value = graph(
    [dependency, dependent],
    [reviewedDependencyEdge('B', 'A', { implementationGate: 'reviewed-implementation' })]
  );
  const authorization = implementationPhase.authorize(value, dependent, {
    root: repoRoot,
    workpackMapping: { codeExpectation: 'code-and-tests', roots: ['scripts'] },
  });

  assert.equal(authorization.authorized, false);
  assert.deepEqual(authorization.blockers[0].gaps, ['implementation: reviewed evidence is not recorded']);
});

test('implementation authorization fails closed for unknown ownership and unreviewed workpacks', () => {
  const value = graph([workpack('A', 'ready')]);
  const node = value.nodes[0];

  const unknownOwnership = implementationPhase.authorize(value, node, { root: repoRoot });
  assert.equal(unknownOwnership.status, 'blocked');
  assert.deepEqual(unknownOwnership.blockers, [
    { kind: 'ownership', reason: 'reviewed workpack code ownership is not mapped' },
  ]);

  node.metadata.needsReview = true;
  const unreviewed = implementationPhase.authorize(value, node, {
    root: repoRoot,
    workpackMapping: { codeExpectation: 'code-and-tests', roots: ['scripts'] },
  });
  assert.equal(unreviewed.status, 'blocked');
  assert.deepEqual(unreviewed.blockers, [
    { kind: 'migration-review', reason: 'workpack dependency/readiness review is incomplete' },
  ]);
});

test('mixed implementation gates require every dependency threshold', () => {
  const implementationReviewed = workpack('A', 'validation', {
    metadata: reviewedImplementationMetadata(),
    completion: {
      required: ['implementation'],
      reviewed: { implementation: true },
      references: { implementation: ['scripts/engineering-graph-lib.mjs'] },
    },
  });
  const completionRequired = workpack('B', 'done');
  const dependent = workpack('C', 'ready', { dependsOn: ['A', 'B'] });
  const value = graph(
    [implementationReviewed, completionRequired, dependent],
    [
      reviewedDependencyEdge('C', 'A', { implementationGate: 'reviewed-implementation' }),
      reviewedDependencyEdge('C', 'B'),
    ]
  );
  const mapping = { codeExpectation: 'code-and-tests', roots: ['scripts'] };

  assert.equal(
    implementationPhase.authorize(value, dependent, { root: repoRoot, workpackMapping: mapping }).authorized,
    true
  );
  completionRequired.state = 'validation';
  completionRequired.lifecycleState = 'validation';
  const blocked = implementationPhase.authorize(value, dependent, { root: repoRoot, workpackMapping: mapping });
  assert.equal(blocked.authorized, false);
  assert.deepEqual(
    blocked.blockers.map((blocker) => blocker.id),
    ['B']
  );
});

test('implementation queue requires reviewed workpack ownership and exposes only phase-authorized rows', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-implementation-phase-'));
  const planRoot = path.join(root, 'docs', 'plans', 'example-plan');
  const dependencyId = 'WP-example-plan-01-dependency';
  const dependentId = 'WP-example-plan-02-dependent';
  await mkdir(path.join(planRoot, 'workpacks'), { recursive: true });
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  await mkdir(path.join(root, 'target-source'), { recursive: true });
  await writeFile(
    path.join(planRoot, 'WORKPACK_INDEX.md'),
    '# Example plan\n\n| Workpack | Status |\n| --- | --- |\n| [01 Dependency](workpacks/01-dependency.md) | Validation |\n| [02 Dependent](workpacks/02-dependent.md) | Ready |\n'
  );
  await writeFile(path.join(planRoot, 'workpacks', '01-dependency.md'), '# Dependency\n');
  await writeFile(path.join(planRoot, 'workpacks', '02-dependent.md'), '# Dependent\n');
  await writeFile(path.join(root, 'dependency.mjs'), 'export const dependency = true;\n');
  await writeFile(path.join(root, 'review.md'), '# Reviewed dependency evidence\n');
  await writeFile(
    path.join(root, 'docs', 'engineering-graph', 'code-map.json'),
    JSON.stringify({
      schemaVersion: 1,
      plans: { 'example-plan': ['target-source'] },
      workpacks: {
        [dependentId]: {
          planSlug: 'example-plan',
          codeExpectation: 'code-and-tests',
          roots: ['target-source'],
        },
      },
    })
  );
  await writeFile(
    path.join(root, 'docs', 'engineering-graph', 'overrides.json'),
    JSON.stringify({
      schemaVersion: GRAPH_SCHEMA_VERSION,
      edges: [
        reviewedDependencyEdge(dependentId, dependencyId, {
          evidence: ['review.md'],
          implementationGate: 'reviewed-implementation',
        }),
      ],
      completionEvidenceOverrides: [
        {
          id: dependencyId,
          implementation: ['dependency.mjs'],
          reason: 'Reviewed dependency implementation source.',
          evidence: ['review.md'],
        },
      ],
    })
  );
  const value = await buildBootstrapGraph({ root });
  await writeFile(path.join(root, 'docs', 'engineering-graph', 'graph.json'), JSON.stringify(value, null, 2));
  const dependency = value.nodes.find((node) => node.id === dependencyId);
  const dependent = value.nodes.find((node) => node.id === dependentId);

  const queue = await implementationPhase.next(value, { root });
  assert.deepEqual(
    queue.authorized.map((row) => row.node.id),
    [dependentId]
  );
  assert.equal(queue.rows.find((row) => row.node.id === dependencyId).authorization.status, 'blocked');
  assert.match(queue.recommendation, /normal READY, tests, proof, PR readiness, and DONE remain blocked/u);

  dependency.completion.reviewed = {};
  const authorization = implementationPhase.authorize(value, dependent, {
    root,
    workpackMapping: { codeExpectation: 'code-and-tests', roots: ['target-source'] },
  });
  assert.equal(authorization.status, 'blocked');
  assert.deepEqual(authorization.blockers[0].gaps, ['implementation: reviewed evidence is not recorded']);
});

test('reviewed flags without hardened completion evidence do not satisfy a phase gate', () => {
  const dependency = workpack('A', 'validation', {
    completion: {
      required: ['implementation'],
      reviewed: { implementation: true },
      references: { implementation: ['scripts/engineering-graph-lib.mjs'] },
    },
  });
  const dependent = workpack('B', 'ready', { dependsOn: ['A'] });
  const value = graph(
    [dependency, dependent],
    [reviewedDependencyEdge('B', 'A', { implementationGate: 'reviewed-implementation' })]
  );

  const authorization = implementationPhase.authorize(value, dependent, {
    root: repoRoot,
    workpackMapping: { codeExpectation: 'code-and-tests', roots: ['scripts'] },
  });
  assert.deepEqual(authorization.blockers[0].gaps, [
    'implementation: hardened reviewed completion evidence is not recorded',
  ]);
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
              implementationAuthorization: {
                status: 'blocked',
                blockers: [
                  {
                    kind: 'dependency',
                    id: 'WP-example-00',
                    gate: 'reviewed-implementation',
                    state: 'validation',
                    gaps: ['implementation: reviewed evidence is not recorded'],
                  },
                ],
              },
              completionContract: { gaps: ['tests: missing'] },
              codeTestTopology: {
                state: 'code-and-tests',
                codeExpectation: 'code-and-tests',
                codeExpectationSatisfied: true,
                implementationFiles: 2,
                testFiles: 1,
              },
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
      codeExpectation: 'code-and-tests',
      codeExpectationSatisfied: true,
      implementationFiles: 2,
      testFiles: 1,
      dependsOn: ['WP-example-00'],
      blockers: [{ id: 'WP-example-00', state: 'validation' }],
      unlocks: ['WP-example-02'],
      implementationAuthorization: 'blocked',
      implementationBlockers: [
        {
          kind: 'dependency',
          id: 'WP-example-00',
          gate: 'reviewed-implementation',
          state: 'validation',
          gaps: ['implementation: reviewed evidence is not recorded'],
        },
      ],
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
    [reviewedDependencyEdge('A', 'B'), reviewedDependencyEdge('B', 'C'), reviewedDependencyEdge('C', 'A')]
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

test('implementation gates reject unsupported values and require a reviewed reason', () => {
  const unsupported = graph(
    [workpack('A', 'ready', { dependsOn: ['B'] }), workpack('B', 'done')],
    [
      {
        from: 'A',
        to: 'B',
        kind: 'depends_on',
        confidence: 'reviewed',
        evidence: ['AGENTS.md'],
        reason: 'reviewed edge',
        implementationGate: 'source-exists',
      },
    ]
  );
  const missingReason = graph(
    [workpack('A', 'ready', { dependsOn: ['B'] }), workpack('B', 'done')],
    [
      {
        from: 'A',
        to: 'B',
        kind: 'depends_on',
        confidence: 'reviewed',
        evidence: ['AGENTS.md'],
        implementationGate: 'reviewed-implementation',
      },
    ]
  );

  assert.ok(
    validateGraph(unsupported, { root: repoRoot }).errors.some((error) =>
      error.includes('unsupported implementationGate source-exists')
    )
  );
  assert.ok(
    validateGraph(missingReason, { root: repoRoot }).errors.some((error) =>
      error.includes('implementationGate requires a reviewed reason')
    )
  );
});

test('direct implementation authorization rejects unreviewed dependency authority', () => {
  const dependency = workpack('A', 'done');
  const dependent = workpack('B', 'ready', { dependsOn: ['A'] });
  const value = graph(
    [dependency, dependent],
    [
      {
        from: 'B',
        to: 'A',
        kind: 'depends_on',
        confidence: 'unreviewed',
        evidence: [],
        reason: '',
      },
    ]
  );

  assert.throws(
    () =>
      implementationPhase.authorize(value, dependent, {
        root: repoRoot,
        workpackMapping: { codeExpectation: 'code-and-tests', roots: ['scripts'] },
      }),
    /confidence=reviewed|requires a reviewed reason|missing evidence/u
  );
});

test('planning-only and missing implementation references block phase authorization', () => {
  const dependency = workpack('A', 'validation', {
    metadata: { needsReview: false, planSlug: 'app-plan' },
    completion: {
      required: ['implementation'],
      reviewed: { implementation: true },
      references: {
        implementation: ['docs/plans/app-plan/workpacks/01-contract-boundary-and-effect-schemas.md'],
      },
    },
  });
  const dependent = workpack('B', 'ready', { dependsOn: ['A'] });
  const value = graph(
    [dependency, dependent],
    [reviewedDependencyEdge('B', 'A', { implementationGate: 'reviewed-implementation' })]
  );
  const mapping = { codeExpectation: 'code-and-tests', roots: ['scripts'] };

  const planningOnly = implementationPhase.authorize(value, dependent, {
    root: repoRoot,
    workpackMapping: mapping,
  });
  assert.deepEqual(planningOnly.blockers[0].gaps, [
    'implementation: planning document is not executable evidence docs/plans/app-plan/workpacks/01-contract-boundary-and-effect-schemas.md',
  ]);

  dependency.completion.references.implementation = ['missing/implementation.mjs'];
  const missing = implementationPhase.authorize(value, dependent, {
    root: repoRoot,
    workpackMapping: mapping,
  });
  assert.deepEqual(missing.blockers[0].gaps, ['implementation: missing reference missing/implementation.mjs']);
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
      workpacks: {
        'WP-example-plan-01-planning-boundary': {
          planSlug: 'example-plan',
          codeExpectation: 'no-code-required',
          roots: [],
        },
        'WP-example-plan-02-test-audit': {
          planSlug: 'example-plan',
          codeExpectation: 'tests-only',
          roots: ['crates/example/tests/unit.rs'],
        },
        'WP-example-plan-03-runtime': {
          planSlug: 'example-plan',
          roots: ['crates/example'],
        },
      },
    })
  );

  const inventory = await buildCodeInventory({ root });
  assert.deepEqual(inventory.totals, {
    plans: 1,
    codeFiles: 2,
    implementationFiles: 1,
    testFiles: 1,
    reviewedWorkpackMaps: 3,
  });
  assert.equal(inventory.plans[0].state, 'code-and-tests');
  assert.deepEqual(inventory.plans[0].missingRoots, []);
  assert.deepEqual(inventory.plans[0].testPaths, ['crates/example/tests/unit.rs']);
  const planning = inventory.workpacks.find((workpack) => workpack.workpackId.endsWith('planning-boundary'));
  assert.equal(planning.state, 'no-source');
  assert.equal(planning.codeExpectation, 'no-code-required');
  assert.equal(planning.codeExpectationSatisfied, true);
  const testAudit = inventory.workpacks.find((workpack) => workpack.workpackId.endsWith('test-audit'));
  assert.equal(testAudit.state, 'tests-only');
  assert.equal(testAudit.codeExpectationSatisfied, true);
  const runtime = inventory.workpacks.find((workpack) => workpack.workpackId.endsWith('runtime'));
  assert.equal(runtime.state, 'code-and-tests');
  assert.equal(runtime.codeExpectation, 'code-and-tests');
  assert.equal(runtime.codeExpectationSatisfied, true);
});

test('code inventory rejects unknown expectations and empty required roots', async () => {
  const root = await mkdtemp(path.join(os.tmpdir(), 'ocentra-engineering-graph-invalid-map-'));
  await mkdir(path.join(root, 'docs', 'engineering-graph'), { recursive: true });
  const codeMapPath = path.join(root, 'docs', 'engineering-graph', 'code-map.json');
  await writeFile(
    codeMapPath,
    JSON.stringify({
      schemaVersion: 1,
      authority: 'test map',
      plans: { 'example-plan': [] },
      workpacks: {
        'WP-example-plan-01-invalid': {
          planSlug: 'example-plan',
          codeExpectation: 'docs-maybe',
          roots: [],
        },
      },
    })
  );
  await assert.rejects(() => buildCodeInventory({ root }), /codeExpectation must be/);

  await writeFile(
    codeMapPath,
    JSON.stringify({
      schemaVersion: 1,
      authority: 'test map',
      plans: { 'example-plan': [] },
      workpacks: {
        'WP-example-plan-02-missing-roots': {
          planSlug: 'example-plan',
          codeExpectation: 'tests-only',
          roots: [],
        },
      },
    })
  );
  await assert.rejects(() => buildCodeInventory({ root }), /must declare non-empty roots/);
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
  assert.deepEqual(
    globalSummary.ready.map((node) => node.id),
    [
      'WP-device-trust-bootstrap-plan-01-device-trust-source-of-truth',
      'WP-eventing-plan-11-type-safety-and-ownership-hardening',
    ]
  );
  assert.ok(globalSummary.blocked.length > 0);
});

test('progress report joins derived workpack state with reviewed plan topology', async () => {
  const report = await buildProgressReport({ root: repoRoot });
  assert.equal(report.schemaVersion, 2);
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
  assert.equal(blocked.implementationAuthorization.status, 'blocked');
  assert.ok(blocked.implementationAuthorization.blockers.some((blocker) => blocker.gate === 'done'));
});

test('reviewed app workpack code maps stay exact after the full plan audit', async () => {
  const report = await buildProgressReport({ root: repoRoot });
  const app = report.plans.find((plan) => plan.id === 'PLAN-app-plan');
  assert.equal(app.workpacks.rows.length, 95);
  assert.ok(app.workpacks.rows.every((workpack) => workpack.codeTestTopology.scope === 'reviewed-workpack-roots'));
  const mapped = app.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-app-plan-01-contract-boundary-and-effect-schemas'
  );
  assert.equal(mapped.codeTestTopology.scope, 'reviewed-workpack-roots');
  assert.ok(mapped.codeTestTopology.implementationFiles > 0);
  assert.ok(mapped.codeTestTopology.testFiles > 0);
  assert.ok(mapped.codeTestTopology.implementationPaths.some((file) => file.endsWith('runtime_decision.rs')));
  const ownedProcessLimit = app.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-app-plan-21-windows-owned-process-terminate-time-limit-proof'
  );
  assert.equal(ownedProcessLimit.codeTestTopology.scope, 'reviewed-workpack-roots');
  assert.ok(
    ownedProcessLimit.codeTestTopology.implementationPaths.some((file) =>
      file.endsWith('enforcement_app_time_limit.rs')
    )
  );
  assert.ok(ownedProcessLimit.codeTestTopology.testFiles > 0);
});

test('reviewed app game workpack maps cover the full imported plan', async () => {
  const report = await buildProgressReport({ root: repoRoot });
  const appGame = report.plans.find((plan) => plan.id === 'PLAN-app-game-plan');
  assert.equal(appGame.workpacks.rows.length, 220);
  assert.ok(appGame.workpacks.rows.every((workpack) => workpack.codeTestTopology.scope === 'reviewed-workpack-roots'));

  const windowsInventory = appGame.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-app-game-plan-06-windows-installed-inventory-adapter'
  );
  assert.ok(windowsInventory.codeTestTopology.implementationFiles > 0);
  assert.ok(windowsInventory.codeTestTopology.testFiles > 0);
  assert.ok(
    windowsInventory.codeTestTopology.implementationPaths.some((file) =>
      file.endsWith('app_game_windows_inventory_source.rs')
    )
  );

  const physicalProof = appGame.workpacks.rows.find(
    (workpack) => workpack.id === 'WP-app-game-plan-181-app-game-android-physical-device-proof'
  );
  assert.equal(physicalProof.codeTestTopology.codeExpectation, 'no-code-required');
  assert.equal(physicalProof.codeTestTopology.implementationFiles, 0);
  assert.equal(physicalProof.codeTestTopology.testFiles, 0);
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
  assert.equal(lanWorkpacks.length, 26);
  assert.ok(lanWorkpacks.every((workpack) => workpack.metadata.sourceFormat === 'numeric-table-row'));
  assert.ok(lanWorkpacks.some((workpack) => workpack.path.endsWith('25-rollout-checklist-and-pr-gate.md')));
  assert.ok(
    lanWorkpacks.some((workpack) =>
      workpack.path.endsWith('26-signed-child-beacon-ingress-and-household-mesh-authority-handoff.md')
    )
  );
  assert.equal(
    lanWorkpacks.find((workpack) => workpack.id === 'WP-lan-plan-18-signed-child-hello-heartbeat').state,
    'validation'
  );
  assert.equal(
    lanWorkpacks.find((workpack) => workpack.id === 'WP-lan-plan-22-current-state-and-gap-map').state,
    'validation'
  );
});
