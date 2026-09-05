import assert from 'node:assert/strict';
import { describe, it } from 'node:test';
import {
  loadWp01RouteInputs,
  renderWp01ProofArtifacts,
  validateWp01RouteBoundary,
} from '../../scripts/test/child-agent-scope-and-route-boundary-proof.mjs';

describe('child agent runtime distribution WP01 route boundary', () => {
  it('accepts the generated route-only graph node and reviewed no-code map', async () => {
    const { graph, codeMap } = await loadWp01RouteInputs();
    const route = validateWp01RouteBoundary(graph, codeMap);

    assert.deepEqual(route, {
      workpackId: 'WP-child-agent-runtime-distribution-plan-01-child-agent-scope-and-route-boundary',
      planId: 'PLAN-child-agent-runtime-distribution-plan',
      state: 'validation',
      lifecycleState: 'validation',
      statusText: 'validation — route-only proof complete; implementation contract open',
      expectedProofRoot: 'output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary',
      retainedProofEvidence: [
        'docs/proof/child-agent-runtime-distribution-plan/WP01_CHILD_AGENT_SCOPE_AND_ROUTE_BOUNDARY_PROOF.md',
      ],
      productionCodeExpectation: 'no-code-required',
    });
  });

  it('rejects parent-client ownership in the generated graph contract', async () => {
    const { graph, codeMap } = await loadWp01RouteInputs();
    const mutated = cloneJson(graph);
    const workpack = findWp01(mutated);
    workpack.parent = 'PLAN-parent-client-runtime-distribution-plan';

    assert.throws(
      () => validateWp01RouteBoundary(mutated, codeMap),
      /WP01 owning plan: expected PLAN-child-agent-runtime-distribution-plan/
    );
  });

  it('rejects runtime or package roots and promotion beyond route-only validation', async () => {
    const { graph, codeMap } = await loadWp01RouteInputs();
    const graphWithReadyClaim = cloneJson(graph);
    findWp01(graphWithReadyClaim).state = 'done';
    assert.throws(
      () => validateWp01RouteBoundary(graphWithReadyClaim, codeMap),
      /WP01 state: expected validation, received done/
    );

    const codeMapWithRuntimeRoot = cloneJson(codeMap);
    codeMapWithRuntimeRoot.workpacks[
      'WP-child-agent-runtime-distribution-plan-01-child-agent-scope-and-route-boundary'
    ].roots = ['scripts/release/windows/build-agent-package.ps1'];
    assert.throws(
      () => validateWp01RouteBoundary(graph, codeMapWithRuntimeRoot),
      /WP01 production roots: expected \[\], received \["scripts\/release\/windows\/build-agent-package.ps1"\]/
    );
  });

  it('rejects missing or misrouted retained completion proof after closure', async () => {
    const { graph, codeMap } = await loadWp01RouteInputs();
    const graphWithMissingProof = cloneJson(graph);
    findWp01(graphWithMissingProof).completion.references.proof = [];
    assert.throws(
      () => validateWp01RouteBoundary(graphWithMissingProof, codeMap),
      /WP01 retained proof evidence: expected \[/
    );

    const graphWithWrongExpectedRoot = cloneJson(graph);
    findWp01(graphWithWrongExpectedRoot).completion.expected.proof = [
      'output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package',
    ];
    assert.throws(() => validateWp01RouteBoundary(graphWithWrongExpectedRoot, codeMap), /WP01 expected proof root/);
  });

  it('renders only the four canonical route proof artifacts', async () => {
    const { graph, codeMap } = await loadWp01RouteInputs();
    const route = validateWp01RouteBoundary(graph, codeMap);
    const artifacts = renderWp01ProofArtifacts({
      checkedAt: '2026-09-02T00:00:00.000Z',
      commit: '0123456789abcdef',
      commandResults: [
        { command: 'node scripts/engineering-graph.mjs validate', exitCode: 0, summary: 'graph valid' },
        {
          command: 'node --test tests/repo-tooling/child-agent-runtime-distribution-route.test.mjs',
          exitCode: 0,
          summary: 'tests 5 pass',
        },
      ],
      route,
    });

    assert.deepEqual(
      [...artifacts.keys()],
      ['00-scope-summary.md', '01-negative-case-proof.md', '02-no-claim-boundary.md', '16-validation-commands.log']
    );
    assert.match(artifacts.get('02-no-claim-boundary.md'), /does not claim package build/);
    assert.match(artifacts.get('16-validation-commands.log'), /result: pass/);
  });
});

function findWp01(graph) {
  return graph.nodes.find(
    (candidate) => candidate.id === 'WP-child-agent-runtime-distribution-plan-01-child-agent-scope-and-route-boundary'
  );
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}
