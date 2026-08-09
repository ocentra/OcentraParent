import { existsSync } from 'node:fs';
import { readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';
import prettier from 'prettier';

export const GRAPH_SCHEMA_VERSION = 1;
export const GRAPH_PATH = 'docs/engineering-graph/graph.json';
export const OVERRIDES_PATH = 'docs/engineering-graph/overrides.json';

const NODE_KINDS = new Set(['goal', 'plan', 'workpack']);
const STATES = new Set(['planned', 'blocked', 'ready', 'active', 'validation', 'done', 'failed', 'paused']);

export function normalizeRepoPath(value) {
  return value.split(path.sep).join('/');
}

export function stableId(prefix, value) {
  const slug = value
    .replace(/\.md$/i, '')
    .replace(/[^a-zA-Z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .toLowerCase();
  return `${prefix}-${slug}`;
}

export function planId(planSlug) {
  return stableId('PLAN', planSlug);
}

export function workpackId(planSlug, workpackPath) {
  return stableId('WP', `${planSlug}-${path.basename(workpackPath)}`);
}

export function humanize(value) {
  return value
    .replace(/\.md$/i, '')
    .replace(/[-_]+/g, ' ')
    .replace(/\b\w/g, (character) => character.toUpperCase());
}

async function readText(root, relativePath) {
  try {
    return await readFile(path.join(root, relativePath), 'utf8');
  } catch (error) {
    if (error?.code === 'ENOENT') return null;
    throw error;
  }
}

async function pathExists(root, relativePath) {
  try {
    await readFile(path.join(root, relativePath));
    return true;
  } catch (error) {
    if (error?.code !== 'EISDIR' && error?.code !== 'ENOENT') throw error;
    if (error?.code === 'EISDIR') return true;
    try {
      await readdir(path.join(root, relativePath));
      return true;
    } catch (directoryError) {
      if (directoryError?.code === 'ENOENT') return false;
      throw directoryError;
    }
  }
}

function firstHeading(text) {
  return text?.match(/^#\s+(.+)$/m)?.[1]?.trim() ?? null;
}

function cleanMarkdownText(value) {
  return value
    .replace(/<[^>]+>/g, '')
    .replace(/`/g, '')
    .replace(/&mdash;|â€”/g, '—')
    .replace(/&ndash;|â€“/g, '–')
    .trim();
}

export function parseWorkpackRows(indexText) {
  if (!indexText) return [];
  const rows = [];
  for (const line of indexText.split(/\r?\n/)) {
    const match = line.match(/\[([^\]]+)\]\((workpacks\/[^)]+\.md)\)/i);
    if (!match) continue;
    const cells = line
      .split('|')
      .map((cell) => cell.trim())
      .filter(Boolean);
    const statusText = cleanMarkdownText(cells.at(-1) ?? '');
    rows.push({
      title: cleanMarkdownText(match[1]),
      relativePath: normalizeRepoPath(match[2]),
      statusText,
    });
  }
  return rows;
}

export function classifyWorkpackStatus(statusText) {
  const value = statusText.toLowerCase();
  if (value.includes('blocked') || value.includes('manual-required')) return 'blocked';
  if (value.includes('failed')) return 'failed';
  if (value.includes('paused')) return 'paused';
  if (value.includes('active') || value.includes('in progress') || value.includes('ci-active')) {
    return 'active';
  }
  if (value.includes('checked') || value.includes('validation')) return 'validation';
  if (value.includes('done') || value.includes('complete') || value.includes('merged')) {
    return 'done';
  }
  if (value.includes('open') && !value.includes('unknown')) return 'ready';
  return 'planned';
}

function planContextCandidates(planSlug) {
  return [
    path.join('docs', 'plans', planSlug, 'implementation-checklist.md'),
    path.join('docs', 'plans', planSlug, 'CHECKLIST_INDEX.md'),
    path.join('docs', 'plans', planSlug, 'checklist.md'),
    path.join('docs', 'plans', planSlug, 'TEST_PROOF_EXPECTATIONS.md'),
    path.join('docs', 'plans', planSlug, 'test-proof-expectations.md'),
  ].map(normalizeRepoPath);
}

async function firstExisting(root, candidates) {
  for (const candidate of candidates) {
    if (await pathExists(root, candidate)) return candidate;
  }
  return null;
}

function declaredProofRoot(testsText, planSlug, workpackPath) {
  const section = testsText?.match(/^##\s+Proof root\b([\s\S]*?)(?=^##\s)/im)?.[1] ?? '';
  const match = section.match(/((?:output|docs\/proof)\/[A-Za-z0-9._/-]+(?:<[^>\r\n]+>)?\/?)/);
  const fallback = normalizeRepoPath(path.join('docs', 'proof', planSlug));
  const candidate = match?.[1] ?? fallback;
  const workpackStem = path.basename(workpackPath).replace(/\.md$/i, '');
  return normalizeRepoPath(candidate.replace(/<workpack-file-stem>|<workpack-id>/gi, workpackStem).replace(/\/+$/, ''));
}

async function buildCompletionContract(root, planSlug, workpackPath) {
  const planRoot = path.join('docs', 'plans', planSlug);
  const checklist = await firstExisting(root, planContextCandidates(planSlug).slice(0, 3));
  const tests = await firstExisting(root, planContextCandidates(planSlug).slice(3));
  const testsText = tests ? await readText(root, tests) : null;
  const expectedProofRoot = declaredProofRoot(testsText, planSlug, workpackPath);
  const durableProofRoot = normalizeRepoPath(path.join('docs', 'proof', planSlug));
  const proofCandidates = expectedProofRoot !== durableProofRoot ? [expectedProofRoot] : [durableProofRoot];
  const proof = await firstExisting(root, proofCandidates);
  const adr = await firstExisting(root, [
    normalizeRepoPath(path.join(planRoot, 'adr')),
    normalizeRepoPath(path.join(planRoot, 'adrs')),
  ]);
  const required = ['implementation', 'tests', 'proof', 'checklist'];
  if (adr) required.push('adr');
  const expected = expectedProofRoot !== durableProofRoot ? { proof: [expectedProofRoot] } : {};
  return {
    required,
    references: {
      implementation: [workpackPath],
      tests: tests ? [tests] : [],
      proof: proof ? [proof] : [],
      checklist: checklist ? [checklist] : [],
      adr: adr ? [adr] : [],
    },
    ...(Object.keys(expected).length > 0 ? { expected } : {}),
  };
}

async function buildPlan(root, planEntry) {
  const planSlug = planEntry.name;
  const planRoot = normalizeRepoPath(path.join('docs', 'plans', planSlug));
  const indexPath = normalizeRepoPath(path.join(planRoot, 'WORKPACK_INDEX.md'));
  const indexText = await readText(root, indexPath);
  const title =
    firstHeading(indexText) ??
    firstHeading(await readText(root, normalizeRepoPath(path.join(planRoot, 'README.md')))) ??
    humanize(planSlug);
  const rows = parseWorkpackRows(indexText);
  const workpacks = [];
  for (const row of rows) {
    const relativePath = normalizeRepoPath(path.join(planRoot, row.relativePath));
    const id = workpackId(planSlug, row.relativePath);
    const storedState = classifyWorkpackStatus(row.statusText);
    workpacks.push({
      id,
      kind: 'workpack',
      title: row.title,
      path: relativePath,
      parent: planId(planSlug),
      dependsOn: [],
      state: storedState,
      lifecycleState: storedState,
      metadata: {
        planSlug,
        indexPath,
        statusText: row.statusText,
        dependencyConfidence: 'unreviewed',
        needsReview: storedState === 'planned',
      },
      completion: await buildCompletionContract(root, planSlug, relativePath),
    });
  }
  return {
    plan: {
      id: planId(planSlug),
      kind: 'plan',
      title,
      path: planRoot,
      parent: 'GOAL-ocentra-parent',
      dependsOn: [],
      state: workpacks.length ? 'active' : 'planned',
      metadata: {
        planSlug,
        indexPath,
        workpackCount: workpacks.length,
        dependencyConfidence: 'unreviewed',
      },
    },
    workpacks,
    ambiguity:
      rows.length === 0
        ? {
            planId: planId(planSlug),
            reason: 'No workpack rows were parsed from WORKPACK_INDEX.md.',
            path: indexPath,
          }
        : {
            planId: planId(planSlug),
            reason:
              'Workpack dependencies were not promoted from prose; add a reviewed override or explicit dependency metadata.',
            path: indexPath,
            workpackCount: rows.length,
          },
  };
}

async function readOverrides(root, overridesPath) {
  const text = await readText(root, overridesPath);
  if (!text) return { edges: [], ambiguities: [] };
  const parsed = JSON.parse(text);
  return {
    edges: Array.isArray(parsed.edges) ? parsed.edges : [],
    ambiguities: Array.isArray(parsed.ambiguities) ? parsed.ambiguities : [],
    stateOverrides: Array.isArray(parsed.stateOverrides) ? parsed.stateOverrides : [],
    proofOverrides: Array.isArray(parsed.proofOverrides) ? parsed.proofOverrides : [],
  };
}

export async function buildBootstrapGraph({ root, overridesPath = OVERRIDES_PATH } = {}) {
  const repoRoot = root ?? process.cwd();
  const planRoot = path.join(repoRoot, 'docs', 'plans');
  const entries = (await readdir(planRoot, { withFileTypes: true }))
    .filter((entry) => entry.isDirectory())
    .sort((left, right) => left.name.localeCompare(right.name));
  const plans = [];
  const workpacks = [];
  const ambiguities = [];
  for (const entry of entries) {
    const result = await buildPlan(repoRoot, entry);
    plans.push(result.plan);
    workpacks.push(...result.workpacks);
    ambiguities.push(result.ambiguity);
  }

  const overrides = await readOverrides(repoRoot, overridesPath);
  const nodes = [
    {
      id: 'GOAL-ocentra-parent',
      kind: 'goal',
      title: 'Ocentra Parent',
      path: 'AGENTS.md',
      parent: null,
      dependsOn: [],
      state: 'active',
      metadata: { source: 'repo-owned engineering graph' },
    },
    ...plans,
    ...workpacks,
  ];
  const edges = [];
  for (const plan of plans) {
    edges.push({ from: 'GOAL-ocentra-parent', to: plan.id, kind: 'contains', confidence: 'structural' });
  }
  for (const workpack of workpacks) {
    edges.push({ from: workpack.parent, to: workpack.id, kind: 'contains', confidence: 'structural' });
  }
  for (const edge of overrides.edges) edges.push(edge);

  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  for (const override of overrides.stateOverrides) {
    const node = nodeById.get(override.id);
    if (!node || !STATES.has(override.state)) continue;
    node.state = override.state;
    node.lifecycleState = override.state;
    node.metadata = {
      ...node.metadata,
      stateOverride: {
        reason: override.reason ?? 'reviewed state override',
        evidence: override.evidence ?? [],
      },
      ...(override.statusText ? { statusText: override.statusText } : {}),
      needsReview: false,
    };
  }
  for (const override of overrides.proofOverrides) {
    const node = nodeById.get(override.id);
    const proof = Array.isArray(override.proof) ? override.proof : [];
    if (!node?.completion || proof.length === 0) continue;
    node.completion.references = {
      ...node.completion.references,
      proof,
    };
    node.metadata = {
      ...node.metadata,
      proofOverride: {
        reason: override.reason ?? 'reviewed proof reference override',
        evidence: override.evidence ?? [],
      },
    };
  }
  for (const edge of edges.filter((candidate) => candidate.kind === 'depends_on')) {
    const dependent = nodeById.get(edge.from);
    const dependency = nodeById.get(edge.to);
    if (!dependent || !dependency) continue;
    dependent.dependsOn ??= [];
    if (!dependent.dependsOn.includes(dependency.id)) dependent.dependsOn.push(dependency.id);
    if (dependency.state !== 'done') dependent.state = 'blocked';
  }

  // A documented DONE status is only a starting hint.  Recompute it against
  // the completion contract before writing the repo-owned graph so stale
  // checklist prose cannot leave a false DONE node behind.
  for (const node of nodes) {
    if (node.kind !== 'workpack' || node.state !== 'done') continue;
    const gaps = completionGaps(repoRoot, node);
    if (gaps.length === 0) continue;
    node.state = 'validation';
    node.lifecycleState = 'validation';
    node.metadata = {
      ...node.metadata,
      completionGaps: gaps,
      sourceStatusText: node.metadata.statusText,
      statusText: `Validation — completion contract gaps: ${gaps.join('; ')}`,
      needsReview: false,
    };
  }

  return {
    schemaVersion: GRAPH_SCHEMA_VERSION,
    sourceOfTruth: GRAPH_PATH,
    authorityBoundaries: {
      graph: 'dependency, readiness, execution state, and completion validation',
      workpackDocs: 'intent, scope, acceptance expectations, and detailed instructions',
      agents: 'execution protocol and safety rules',
      tests: 'technical validation evidence',
      proof: 'process evidence required by the selected workpack',
      adr: 'architectural decisions where required',
      user: 'unresolved product or scope decisions',
    },
    nodes,
    edges,
    migration: {
      importedPlans: plans.length,
      importedWorkpacks: workpacks.length,
      ambiguities: [...ambiguities, ...overrides.ambiguities],
      dependencyPolicy:
        'Only structural parent edges and reviewed entries in overrides.json are hard graph edges; prose dependencies remain review items.',
    },
  };
}

function dependencyEdges(graph) {
  return graph.edges.filter((edge) => edge.kind === 'depends_on');
}

function nodeMap(graph) {
  return new Map(graph.nodes.map((node) => [node.id, node]));
}

export function completionGaps(root, node) {
  if (!node.completion) return [];
  const gaps = [];
  const required = new Set(node.completion.required ?? []);
  const requirements = new Set([
    ...required,
    ...Object.keys(node.completion.references ?? {}),
    ...Object.keys(node.completion.expected ?? {}),
  ]);
  for (const requirement of requirements) {
    const references = node.completion.references?.[requirement] ?? [];
    const expected = node.completion.expected?.[requirement] ?? [];
    if (required.has(requirement) && references.length === 0 && expected.length === 0) {
      gaps.push(`${requirement}: no reference or expected artifact is declared`);
      continue;
    }
    for (const reference of references) {
      if (!pathExistsSync(root, reference)) gaps.push(`${requirement}: missing reference ${reference}`);
    }
    for (const reference of expected) {
      if (!pathExistsSync(root, reference)) gaps.push(`${requirement}: missing expected artifact ${reference}`);
    }
  }
  return gaps;
}

function completionSatisfied(root, node) {
  return completionGaps(root, node).length === 0;
}

export function deriveNodeState(graph, node, states = new Map(), root = process.cwd(), visiting = new Set()) {
  if (states.has(node.id)) return states.get(node.id);
  if (visiting.has(node.id)) return 'blocked';
  visiting.add(node.id);
  const map = nodeMap(graph);
  const dependencies = dependencyEdges(graph).filter((edge) => edge.from === node.id);
  const unsatisfied = dependencies.filter((edge) => {
    const dependency = map.get(edge.to);
    return !dependency || deriveNodeState(graph, dependency, states, root, visiting) !== 'done';
  });
  let state = node.lifecycleState ?? node.state;
  if (unsatisfied.length > 0) state = 'blocked';
  else if (state === 'done' && !completionSatisfied(root, node)) state = 'validation';
  else if (state === 'planned' && node.metadata?.needsReview) state = 'planned';
  else if (state === 'planned' || state === 'ready') state = 'ready';
  visiting.delete(node.id);
  states.set(node.id, state);
  return state;
}

export function deriveStates(graph, { root = process.cwd() } = {}) {
  const states = new Map();
  for (const node of graph.nodes) deriveNodeState(graph, node, states, root);
  for (const node of graph.nodes.filter((candidate) => candidate.kind === 'plan')) {
    const children = graph.nodes.filter((candidate) => candidate.parent === node.id);
    const childStates = children.map((child) => states.get(child.id));
    if (childStates.length > 0 && childStates.every((state) => state === 'done')) states.set(node.id, 'done');
    else if (childStates.some((state) => state === 'active' || state === 'validation' || state === 'ready')) {
      states.set(node.id, 'active');
    } else if (childStates.some((state) => state === 'blocked')) states.set(node.id, 'blocked');
    else states.set(node.id, 'planned');
  }
  return states;
}

export function descendants(graph, rootId) {
  const result = new Set([rootId]);
  let changed = true;
  while (changed) {
    changed = false;
    for (const node of graph.nodes) {
      if (node.parent && result.has(node.parent) && !result.has(node.id)) {
        result.add(node.id);
        changed = true;
      }
    }
  }
  return result;
}

export function scopeNodes(graph, scope) {
  if (!scope) return graph.nodes;
  const ids = descendants(graph, scope);
  return graph.nodes.filter((node) => ids.has(node.id));
}

export function summarizeGraph(graph, scope, { root = process.cwd() } = {}) {
  const states = deriveStates(graph, { root });
  const scoped = scopeNodes(graph, scope).filter((node) => node.kind === 'workpack');
  const counts = Object.fromEntries([...STATES].map((state) => [state, 0]));
  for (const node of scoped) counts[states.get(node.id)] += 1;
  return {
    scope: scope ?? 'GOAL-ocentra-parent',
    plans: scopeNodes(graph, scope).filter((node) => node.kind === 'plan').length,
    workpacks: scoped.length,
    counts,
    ready: scoped.filter((node) => states.get(node.id) === 'ready'),
    blocked: scoped.filter((node) => states.get(node.id) === 'blocked'),
    active: scoped.filter((node) => ['active', 'validation'].includes(states.get(node.id))),
  };
}

function detectCycles(graph) {
  const adjacency = new Map();
  for (const edge of dependencyEdges(graph)) {
    if (!adjacency.has(edge.from)) adjacency.set(edge.from, []);
    adjacency.get(edge.from).push(edge.to);
  }
  const visiting = new Set();
  const visited = new Set();
  const cycles = [];
  function visit(id, pathStack) {
    if (visiting.has(id)) {
      const start = pathStack.indexOf(id);
      cycles.push([...pathStack.slice(start), id]);
      return;
    }
    if (visited.has(id)) return;
    visiting.add(id);
    for (const dependency of adjacency.get(id) ?? []) visit(dependency, [...pathStack, id]);
    visiting.delete(id);
    visited.add(id);
  }
  for (const node of graph.nodes) visit(node.id, []);
  return cycles;
}

export function validateGraph(graph, { root = process.cwd() } = {}) {
  const errors = [];
  const warnings = [];
  if (graph?.schemaVersion !== GRAPH_SCHEMA_VERSION) {
    errors.push(`schemaVersion must be ${GRAPH_SCHEMA_VERSION}`);
  }
  if (!Array.isArray(graph?.nodes)) errors.push('nodes must be an array');
  if (!Array.isArray(graph?.edges)) errors.push('edges must be an array');
  if (errors.length > 0) return { ok: false, errors, warnings };
  const ids = new Set();
  const map = nodeMap(graph);
  for (const node of graph.nodes) {
    if (ids.has(node.id)) errors.push(`duplicate node id: ${node.id}`);
    ids.add(node.id);
    if (!NODE_KINDS.has(node.kind)) errors.push(`${node.id} has invalid kind ${node.kind}`);
    if (!STATES.has(node.state)) errors.push(`${node.id} has invalid state ${node.state}`);
    if (node.parent && !map.has(node.parent)) errors.push(`${node.id} references missing parent ${node.parent}`);
    if (node.kind === 'goal' && node.parent !== null) errors.push(`${node.id} goal parent must be null`);
    if (node.kind === 'plan' && node.parent && map.get(node.parent)?.kind !== 'goal') {
      errors.push(`${node.id} plan parent must be a goal`);
    }
    if (node.kind === 'workpack' && node.parent && map.get(node.parent)?.kind !== 'plan') {
      errors.push(`${node.id} workpack parent must be a plan`);
    }
    for (const dependency of node.dependsOn ?? []) {
      if (!map.has(dependency)) errors.push(`${node.id} references missing dependency ${dependency}`);
    }
    if (!node.path) errors.push(`${node.id} is missing path`);
    else if (!pathExistsSync(root, node.path)) warnings.push(`${node.id} path is missing: ${node.path}`);
    for (const [requirement, references] of Object.entries(node.completion?.references ?? {})) {
      for (const reference of references) {
        if (!pathExistsSync(root, reference))
          warnings.push(`${node.id} ${requirement} reference is missing: ${reference}`);
      }
    }
    if (node.lifecycleState && !STATES.has(node.lifecycleState)) {
      errors.push(`${node.id} has invalid lifecycleState ${node.lifecycleState}`);
    }
    // Expected artifacts are intentionally allowed to be absent while a node
    // is in validation.  A stale DONE node is an actionable warning/error and
    // is handled by completionGaps below; warning on every open workpack would
    // turn graph validation into an unreadable dump.
    if (node.state === 'done' || node.lifecycleState === 'done') {
      for (const [requirement, references] of Object.entries(node.completion?.expected ?? {})) {
        for (const reference of references) {
          if (!pathExistsSync(root, reference))
            warnings.push(`${node.id} ${requirement} expected artifact is missing: ${reference}`);
        }
      }
    }
    if (node.state === 'done' && !completionSatisfied(root, node)) {
      errors.push(
        `${node.id} is marked done but its completion contract is unsatisfied: ${completionGaps(root, node).join('; ')}`
      );
    }
  }
  for (const edge of graph.edges) {
    if (!map.has(edge.from)) errors.push(`edge references missing from node ${edge.from}`);
    if (!map.has(edge.to)) errors.push(`edge references missing to node ${edge.to}`);
    if (!edge.kind) errors.push(`edge ${edge.from} -> ${edge.to} is missing kind`);
  }
  for (const cycle of detectCycles(graph)) errors.push(`dependency cycle: ${cycle.join(' -> ')}`);
  const states = deriveStates(graph, { root });
  for (const node of graph.nodes) {
    if (node.state === 'ready' && states.get(node.id) === 'blocked') {
      errors.push(`${node.id} is stored ready but derived blocked`);
    }
  }
  return { ok: errors.length === 0, errors, warnings };
}

function pathExistsSync(root, relativePath) {
  return existsSync(path.join(root, relativePath));
}

export async function loadGraph(root, graphPath = GRAPH_PATH) {
  const text = await readFile(path.join(root, graphPath), 'utf8');
  return JSON.parse(text);
}

export async function writeGraph(root, graphPath, graph) {
  const target = path.join(root, graphPath);
  const source = JSON.stringify(graph, null, 2);
  const formatted = await prettier.format(source, { filepath: target, parser: 'json' });
  await writeFile(target, formatted, 'utf8');
}

export function explainBlocked(graph, nodeId, { root = process.cwd() } = {}) {
  const map = nodeMap(graph);
  const node = map.get(nodeId);
  if (!node) return { node: null, reasons: [`unknown node ${nodeId}`] };
  const states = deriveStates(graph, { root });
  const reasons = [];
  for (const edge of dependencyEdges(graph).filter((candidate) => candidate.from === nodeId)) {
    const dependency = map.get(edge.to);
    if (!dependency) reasons.push(`missing dependency ${edge.to}`);
    else if (states.get(dependency.id) !== 'done') reasons.push(`${dependency.id} is ${states.get(dependency.id)}`);
  }
  if (node.metadata?.needsReview) reasons.push('migration/import status needs review before readiness');
  if (states.get(node.id) === 'validation') {
    const gaps = completionGaps(root, node);
    reasons.push(
      gaps.length > 0
        ? `completion contract gaps: ${gaps.join('; ')}`
        : 'completion contract still needs validation evidence'
    );
  }
  return { node, state: states.get(node.id), reasons };
}

export function relatedNodes(graph, nodeId, direction) {
  const edges = dependencyEdges(graph);
  if (direction === 'deps') return edges.filter((edge) => edge.from === nodeId).map((edge) => edge.to);
  return edges.filter((edge) => edge.to === nodeId).map((edge) => edge.from);
}
