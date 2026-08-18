#!/usr/bin/env node

import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  GRAPH_PATH,
  GRAPH_SCHEMA_VERSION,
  buildCodeInventory,
  buildBootstrapGraph,
  buildProgressReport,
  deriveStates,
  explainBlocked,
  flattenProgressReport,
  graphSourceDrift,
  implementationPhase,
  loadGraph,
  nextWork,
  planId,
  relatedNodes,
  scopeNodes,
  summarizeGraph,
  validateGraph,
  writeGraph,
} from './engineering-graph-lib.mjs';

const defaultRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function usage() {
  console.log(`Engineering graph control plane

Usage:
  npm run graph:bootstrap                 Preview imported graph counts
  npm run graph:bootstrap -- --write      Rebuild docs/engineering-graph/graph.json
  npm run graph:status [scope-id]
  npm run graph:code [scope-id]
  npm run graph:report [scope-id] [--json]
  npm run graph:matrix [scope-id] [--state <state>] [--json]
  npm run graph:ready [scope-id]
  npm run graph:parallel [scope-id]
  npm run graph:next [scope-id] [--phase implementation]
  npm run graph:blocked [scope-id]
  npm run graph:inspect <id>
  npm run graph:deps <id>
  npm run graph:dependents <id>
  npm run graph:why <id> [--phase implementation]
  npm run graph:validate

Global option: --root <repo> (defaults to the repository containing this script)
`);
}

function flag(args, name) {
  return args.includes(name);
}

function positionalArg(args) {
  const valueFlags = new Set(['--state', '--limit', '--phase', '--root']);
  for (let index = 0; index < args.length; index += 1) {
    const argument = args[index];
    if (valueFlags.has(argument)) {
      index += 1;
      continue;
    }
    if (!argument.startsWith('--')) return argument;
  }
  return undefined;
}

function option(args, name) {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
}

function nodeMap(graph) {
  return new Map(graph.nodes.map((node) => [node.id, node]));
}

function printNode(node, states) {
  console.log(`${node.id} [${node.kind}]`);
  console.log(`Title: ${node.title}`);
  console.log(`State: ${states.get(node.id)}`);
  if (node.path) console.log(`Path: ${node.path}`);
  if (node.parent) console.log(`Parent: ${node.parent}`);
  if (node.metadata?.statusText) console.log(`Plan status: ${node.metadata.statusText}`);
  if (node.metadata?.completionGaps?.length) {
    console.log('Completion gaps:');
    for (const gap of node.metadata.completionGaps) console.log(`  - ${gap}`);
  }
}

function printList(nodes, states, { limit = Number.POSITIVE_INFINITY } = {}) {
  if (nodes.length === 0) {
    console.log('(none)');
    return;
  }
  for (const node of nodes.slice(0, limit)) {
    console.log(`${node.id} [${states.get(node.id)}] ${node.title}`);
  }
  if (nodes.length > limit) console.log(`... ${nodes.length - limit} more; scope or use a focused command.`);
}

function implementationBlockerText(blocker) {
  if (blocker.kind === 'dependency') {
    const gaps = blocker.gaps?.length ? `: ${blocker.gaps.join('; ')}` : '';
    return `${blocker.id} requires ${blocker.gate}, observed ${blocker.state}${gaps}`;
  }
  return blocker.reason ?? `${blocker.kind} blocker`;
}

function printImplementationRows(rows, { limit = Number.POSITIVE_INFINITY } = {}) {
  if (rows.length === 0) {
    console.log('(none)');
    return;
  }
  for (const row of rows.slice(0, limit)) {
    console.log(`${row.node.id} [IMPLEMENTATION-ONLY] ${row.node.title}`);
  }
  if (rows.length > limit) console.log(`... ${rows.length - limit} more; scope or use a focused command.`);
}

async function run(command, args) {
  if (command === 'help' || command === undefined) {
    usage();
    return;
  }
  const rootRequested = flag(args, '--root');
  const rootOption = option(args, '--root');
  if (rootRequested && (!rootOption || rootOption.startsWith('--'))) {
    console.error('Missing value for --root.');
    process.exitCode = 1;
    return;
  }
  const root = path.resolve(rootOption ?? defaultRoot);
  const phaseRequested = flag(args, '--phase');
  const phase = option(args, '--phase');
  if (phaseRequested && phase === undefined) {
    console.error('Missing value for --phase; supported value: implementation');
    process.exitCode = 1;
    return;
  }
  if (phase !== undefined && phase !== 'implementation') {
    console.error(`Unsupported graph phase: ${phase}`);
    process.exitCode = 1;
    return;
  }
  if (phase !== undefined && !['next', 'why'].includes(command)) {
    console.error(`--phase implementation is supported only by graph:next and graph:why, not ${command}`);
    process.exitCode = 1;
    return;
  }
  if (command === 'bootstrap') {
    const graph = await buildBootstrapGraph({ root });
    const report = validateGraph(graph, { root });
    console.log(`Imported plans: ${graph.migration.importedPlans}`);
    console.log(`Imported workpacks: ${graph.migration.importedWorkpacks}`);
    console.log(`Ambiguities requiring review: ${graph.migration.ambiguities.length}`);
    console.log(
      `Unindexed workpack files requiring review: ${graph.migration.unindexedWorkpackArtifacts?.reduce((total, artifact) => total + artifact.paths.length, 0) ?? 0}`
    );
    if (report.errors.length > 0) {
      for (const error of report.errors) console.error(`ERROR ${error}`);
      process.exitCode = 1;
      return;
    }
    if (flag(args, '--write')) {
      await writeGraph(root, GRAPH_PATH, graph);
      console.log(`Wrote ${GRAPH_PATH}`);
    } else {
      console.log('Preview only; pass --write to update the checked-in graph.');
    }
    return;
  }

  const graph = await loadGraph(root);
  const validation = validateGraph(graph, { root });
  if (!validation.ok && command !== 'validate') {
    for (const error of validation.errors) console.error(`ERROR ${error}`);
    process.exitCode = 1;
    return;
  }
  if (command === 'validate') {
    for (const error of validation.errors) console.error(`ERROR ${error}`);
    for (const warning of validation.warnings) console.warn(`WARN ${warning}`);
    const generated = await buildBootstrapGraph({ root });
    const generatedValidation = validateGraph(generated, { root });
    for (const error of generatedValidation.errors) console.error(`ERROR generated graph: ${error}`);
    for (const warning of generatedValidation.warnings) console.warn(`WARN generated graph: ${warning}`);
    for (const error of graphSourceDrift(graph, generated)) console.error(`ERROR ${error}`);
    if (!validation.ok || !generatedValidation.ok || graphSourceDrift(graph, generated).length > 0) {
      process.exitCode = 1;
      return;
    }
    console.log(`Graph valid: ${graph.nodes.length} nodes, ${graph.edges.length} edges.`);
    console.log(
      `Imported ${graph.migration.importedPlans} plans and ${graph.migration.importedWorkpacks} workpacks; ${graph.migration.ambiguities.length} review items remain.`
    );
    return;
  }

  const scope = positionalArg(args);
  const map = nodeMap(graph);
  if (
    scope &&
    ['status', 'ready', 'next', 'parallel', 'blocked', 'report', 'matrix', 'code'].includes(command) &&
    !map.has(scope) &&
    !(command === 'code' && map.has(planId(scope)))
  ) {
    console.error(`Unknown graph scope: ${scope}`);
    process.exitCode = 1;
    return;
  }

  if (command === 'code') {
    const inventory = await buildCodeInventory({ root, scope });
    console.log(`Code map: ${inventory.codeMapPath}`);
    console.log(`Plans: ${inventory.totals.plans}`);
    console.log(`Implementation files: ${inventory.totals.implementationFiles}`);
    console.log(`Test files: ${inventory.totals.testFiles}`);
    console.log('\nPlan code/test topology:');
    for (const plan of inventory.plans) {
      const missing = plan.missingRoots.length ? ` missing=${plan.missingRoots.join(',')}` : '';
      console.log(
        `${plan.planId} [${plan.state}] implementation=${plan.implementationFiles} tests=${plan.testFiles} roots=${plan.roots.length}${missing}`
      );
    }
    console.log('\nReviewed workpack code/test expectations:');
    for (const workpack of inventory.workpacks) {
      const missing = workpack.missingRoots.length ? ` missing=${workpack.missingRoots.join(',')}` : '';
      const missingExpectedTests = workpack.missingExpectedTestRoots.length
        ? ` missingExpectedTestRoots=${workpack.missingExpectedTestRoots.join(',')}`
        : '';
      console.log(
        `${workpack.workpackId} [${workpack.state}] expectation=${workpack.codeExpectation} ` +
          `satisfied=${workpack.codeExpectationSatisfied} implementation=${workpack.implementationFiles} ` +
          `tests=${workpack.testFiles} roots=${workpack.roots.length}${missing}${missingExpectedTests}`
      );
    }
    console.log('\nCounts are live file topology only; they do not claim acceptance, proof, CI, or merge.');
    return;
  }

  if (command === 'report') {
    const report = await buildProgressReport({ root, scope });
    if (flag(args, '--json')) {
      console.log(JSON.stringify(report, null, 2));
      return;
    }
    console.log(`Scope: ${report.scope}`);
    console.log(`Plans: ${report.totals.plans}`);
    console.log(`Workpacks: ${report.totals.workpacks}`);
    for (const [state, count] of Object.entries(report.totals.states)) {
      console.log(`${state.toUpperCase().padEnd(10)} ${count}`);
    }
    console.log(`Implementation files: ${report.totals.implementationFiles}`);
    console.log(`Test files: ${report.totals.testFiles}`);
    console.log(`Reviewed workpack code maps: ${report.totals.reviewedWorkpackMaps}`);
    console.log(`Migration review items: ${report.migration.reviewItems}`);
    console.log(`Unindexed workpack files: ${report.migration.unindexedWorkpackFiles}`);
    console.log('\nPlan matrix (state is graph-derived; code/test is reviewed plan-root topology):');
    for (const plan of report.plans) {
      const counts = Object.entries(plan.workpacks.counts)
        .filter(([, count]) => count > 0)
        .map(([state, count]) => `${state}=${count}`)
        .join(', ');
      const topology = plan.codeTestTopology;
      console.log(
        `${plan.id} [${plan.state}] workpacks=${plan.workpacks.total} (${counts || 'none'}) ` +
          `implementation=${topology.implementationFiles} tests=${topology.testFiles} ` +
          `code=${topology.state}`
      );
      const exceptions = plan.workpacks.rows.filter((workpack) =>
        ['blocked', 'active', 'validation', 'failed'].includes(workpack.state)
      );
      for (const workpack of exceptions.slice(0, 8)) {
        const gaps = workpack.completionContract.gaps.length ? ` gaps=${workpack.completionContract.gaps.length}` : '';
        const topology =
          typeof workpack.codeTestTopology === 'string'
            ? workpack.codeTestTopology
            : `${workpack.codeTestTopology.state} ${workpack.codeTestTopology.implementationFiles}/${workpack.codeTestTopology.testFiles} ` +
              `expected=${workpack.codeTestTopology.codeExpectation} ` +
              `satisfied=${workpack.codeTestTopology.codeExpectationSatisfied}`;
        console.log(
          `  - ${workpack.id} [${workpack.state}] code=${topology} ` +
            `implementation=${workpack.implementationAuthorization.status}${gaps}`
        );
      }
      if (exceptions.length > 8) console.log(`  - ... ${exceptions.length - 8} more non-planned rows`);
    }
    console.log(
      '\nAuthority: code/test counts are live topology only; they do not claim acceptance, proof, CI, review, or merge.'
    );
    return;
  }

  if (command === 'matrix') {
    const report = await buildProgressReport({ root, scope });
    const rows = flattenProgressReport(report).filter(
      (row) => !option(args, '--state') || row.state === option(args, '--state')
    );
    if (flag(args, '--json')) {
      console.log(
        JSON.stringify(
          {
            schemaVersion: GRAPH_SCHEMA_VERSION,
            scope: report.scope,
            totals: report.totals,
            rows,
          },
          null,
          2
        )
      );
      return;
    }
    console.log(`Scope: ${report.scope}`);
    console.log(`Rows: ${rows.length}${option(args, '--state') ? ` (state=${option(args, '--state')})` : ''}`);
    console.log('\nPlan summary:');
    console.log('PLAN | STATE | WORKPACKS | COUNTS | IMPLEMENTATION | TESTS');
    for (const plan of report.plans) {
      const counts = Object.entries(plan.workpacks.counts)
        .filter(([, count]) => count > 0)
        .map(([state, count]) => `${state}=${count}`)
        .join(',');
      console.log(
        `${plan.id} | ${plan.state} | ${plan.workpacks.total} | ${counts || 'none'} | ` +
          `${plan.codeTestTopology.implementationFiles} | ${plan.codeTestTopology.testFiles}`
      );
    }
    console.log('\nWorkpack matrix:');
    console.log(
      'PLAN | WORKPACK | STATE | IMPLEMENTATION AUTH | IMPLEMENTATION BLOCKERS | CODE/TEST | GAPS | DEPENDS ON | BLOCKERS | UNLOCKS'
    );
    for (const row of rows) {
      const topology =
        row.implementationFiles === null
          ? row.codeState
          : `${row.codeState} ${row.implementationFiles}/${row.testFiles} ` +
            `expected=${row.codeExpectation} satisfied=${row.codeExpectationSatisfied}`;
      const blockerText = row.blockers.map((blocker) => `${blocker.id}[${blocker.state}]`).join(',') || '-';
      const implementationBlockers = row.implementationBlockers.map(implementationBlockerText).join(',') || '-';
      console.log(
        `${row.planId} | ${row.workpackId} | ${row.state} | ${row.implementationAuthorization} | ` +
          `${implementationBlockers} | ${topology} | ${row.completionGapCount} | ` +
          `${row.dependsOn.join(',') || '-'} | ${blockerText} | ${row.unlocks.join(',') || '-'}`
      );
    }
    console.log(
      '\nUnknown code/test ownership is deliberate: add a reviewed code-map workpack entry before using file topology as evidence.'
    );
    return;
  }

  const states = deriveStates(graph, { root });
  if (phase === 'implementation' && command === 'next') {
    const queue = await implementationPhase.next(graph, { root, scope });
    console.log('IMPLEMENTATION-ONLY authorization; normal READY, tests, proof, PR readiness, and DONE are unchanged.');
    printImplementationRows(queue.authorized, {
      limit: Number(option(args, '--limit') ?? Number.POSITIVE_INFINITY),
    });
    console.log(`\n${queue.recommendation}`);
    return;
  }
  if (command === 'status') {
    const summary = summarizeGraph(graph, scope, { root });
    console.log(`Scope: ${summary.scope}`);
    console.log(`Plans: ${summary.plans}`);
    console.log(`Workpacks: ${summary.workpacks}`);
    for (const [state, count] of Object.entries(summary.counts)) {
      console.log(`${state.toUpperCase().padEnd(10)} ${count}`);
    }
    console.log('\nActive / validating:');
    printList(summary.active, states, { limit: 25 });
    console.log('\nReady:');
    printList(summary.ready, states);
    console.log('\nBlocked:');
    printList(summary.blocked, states, { limit: 25 });
    return;
  }
  if (command === 'ready' || command === 'next' || command === 'parallel') {
    if (command === 'next') {
      const queue = nextWork(graph, { root, scope });
      if (queue.authorized.length > 0) {
        console.log('READY workpacks (authorized):');
        printList(queue.authorized, states);
      } else {
        console.log('No READY workpack is authorized.');
        console.log(queue.recommendation);
        console.log('\nUnblocked validation/review queue (not READY authorization):');
        printList(queue.validationQueue, states, { limit: Number(option(args, '--limit') ?? 25) });
      }
      return;
    }
    const ready = scopeNodes(graph, scope).filter(
      (node) => node.kind === 'workpack' && states.get(node.id) === 'ready'
    );
    if (command === 'parallel') console.log(`Parallel-ready workpacks: ${ready.length}`);
    printList(ready, states);
    return;
  }
  if (command === 'blocked') {
    printList(
      scopeNodes(graph, scope).filter((node) => states.get(node.id) === 'blocked'),
      states
    );
    return;
  }
  if (['inspect', 'deps', 'dependents', 'why'].includes(command)) {
    if (!scope || !map.has(scope)) {
      console.error(`Unknown graph node: ${scope ?? '<missing id>'}`);
      process.exitCode = 1;
      return;
    }
    const node = map.get(scope);
    if (command === 'inspect') {
      printNode(node, states);
      if (node.kind === 'workpack') {
        const inventory = await buildCodeInventory({ root, scope: node.parent });
        const topology = inventory.workpacks.find((entry) => entry.workpackId === node.id);
        if (topology) {
          console.log(`Code/test topology: ${topology.state}`);
          console.log(`  Expected topology: ${topology.codeExpectation}`);
          console.log(`  Expectation satisfied: ${topology.codeExpectationSatisfied}`);
          console.log(`  Implementation files: ${topology.implementationFiles}`);
          console.log(`  Test files: ${topology.testFiles}`);
          console.log(`  Roots: ${topology.roots.join(', ')}`);
          if (topology.missingRoots.length > 0) console.log(`  Missing roots: ${topology.missingRoots.join(', ')}`);
          if (topology.missingExpectedTestRoots.length > 0) {
            console.log(`  missingExpectedTestRoots: ${topology.missingExpectedTestRoots.join(', ')}`);
          }
        } else {
          console.log('Code/test topology: unknown-workpack-ownership');
          console.log('  Plan-root counts are available from graph:report; no reviewed workpack map exists yet.');
        }
      }
      console.log(`Depends on: ${relatedNodes(graph, scope, 'deps').join(', ') || 'none'}`);
      console.log(`Unlocks: ${relatedNodes(graph, scope, 'dependents').join(', ') || 'none'}`);
      if (node.completion) {
        console.log('Completion contract:');
        for (const requirement of node.completion.required) {
          const refs = node.completion.references?.[requirement] ?? [];
          const expected = node.completion.expected?.[requirement] ?? [];
          console.log(`  ${requirement}: ${refs.join(', ') || 'missing'}`);
          if (expected.length > 0) {
            console.log(`    expected: ${expected.join(', ')}`);
          }
        }
        const expected = Object.entries(node.completion.expected ?? {});
        if (expected.length > 0) {
          console.log('Expected artifacts:');
          for (const [requirement, refs] of expected) console.log(`  ${requirement}: ${refs.join(', ') || 'missing'}`);
        }
      }
      return;
    }
    if (command === 'why') {
      if (phase === 'implementation') {
        const explanation = await implementationPhase.explain(graph, scope, { root });
        console.log(`${scope} implementation phase is ${explanation.status}.`);
        if (explanation.authorized) {
          console.log(
            '- IMPLEMENTATION-ONLY source edits are authorized; normal READY and completion remain unchanged.'
          );
        } else if (explanation.blockers.length === 0) {
          console.log('- No implementation work remains for this workpack.');
        } else {
          for (const blocker of explanation.blockers) {
            console.log(`- ${implementationBlockerText(blocker)}`);
          }
        }
        return;
      }
      const explanation = explainBlocked(graph, scope, { root });
      console.log(`${scope} is ${explanation.state}.`);
      for (const reason of explanation.reasons) console.log(`- ${reason}`);
      return;
    }
    const ids = relatedNodes(graph, scope, command);
    printList(ids.map((id) => map.get(id)).filter(Boolean), states);
    return;
  }
  console.error(`Unknown graph command: ${command}`);
  usage();
  process.exitCode = 1;
}

run(process.argv[2], process.argv.slice(3)).catch((error) => {
  console.error(error?.stack ?? error);
  process.exitCode = 1;
});
