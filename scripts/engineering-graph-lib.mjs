import { existsSync } from 'node:fs';
import { readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';

export const GRAPH_SCHEMA_VERSION = 1;
export const GRAPH_PATH = 'docs/engineering-graph/graph.json';
export const OVERRIDES_PATH = 'docs/engineering-graph/overrides.json';
export const CODE_MAP_PATH = 'docs/engineering-graph/code-map.json';
const WORKPACK_CODE_EXPECTATIONS = new Set(['code-and-tests', 'tests-only', 'no-code-required']);

const CODE_EXTENSIONS = new Set([
  '.c',
  '.cc',
  '.cpp',
  '.cs',
  '.dart',
  '.go',
  '.java',
  '.js',
  '.jsx',
  '.kt',
  '.kts',
  '.mjs',
  '.rs',
  '.swift',
  '.ts',
  '.tsx',
]);
const IGNORED_CODE_DIRECTORIES = new Set([
  '.git',
  '.next',
  'build',
  'coverage',
  'dist',
  'node_modules',
  'target',
  'test-results',
]);

const NODE_KINDS = new Set(['goal', 'plan', 'workpack']);
const STATES = new Set(['planned', 'blocked', 'ready', 'active', 'validation', 'done', 'failed', 'paused']);
const EDGE_KINDS = new Set(['contains', 'depends_on']);

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

export async function loadCodeMap(root, codeMapPath = CODE_MAP_PATH) {
  const text = await readText(root, codeMapPath);
  if (!text) throw new Error(`Code map is missing: ${codeMapPath}`);
  const map = JSON.parse(text);
  if (map?.schemaVersion !== 1 || !map?.plans || typeof map.plans !== 'object') {
    throw new Error(`${codeMapPath} must declare schemaVersion 1 and a plans object`);
  }
  if (
    map.workpacks !== undefined &&
    (!map.workpacks || typeof map.workpacks !== 'object' || Array.isArray(map.workpacks))
  ) {
    throw new Error(`${codeMapPath} workpacks must be an object when present`);
  }
  for (const [workpackId, entry] of Object.entries(map.workpacks ?? {})) {
    if (!entry || typeof entry !== 'object') {
      throw new Error(`${codeMapPath} workpack ${workpackId} must be an object`);
    }
    const codeExpectation = entry.codeExpectation ?? 'code-and-tests';
    if (!WORKPACK_CODE_EXPECTATIONS.has(codeExpectation)) {
      throw new Error(
        `${codeMapPath} workpack ${workpackId} codeExpectation must be code-and-tests, tests-only, or no-code-required`
      );
    }
    if (!Array.isArray(entry.roots)) {
      throw new Error(`${codeMapPath} workpack ${workpackId} must declare a roots array`);
    }
    if (codeExpectation !== 'no-code-required' && entry.roots.length === 0) {
      throw new Error(
        `${codeMapPath} workpack ${workpackId} must declare non-empty roots unless codeExpectation is no-code-required`
      );
    }
  }
  return map;
}

function isTestPath(relativePath) {
  const normalized = normalizeRepoPath(relativePath).toLowerCase();
  const basename = path.basename(normalized);
  return (
    normalized.split('/').some((part) => ['test', 'tests', '__tests__', 'spec', 'specs'].includes(part)) ||
    /(?:\.test|\.spec|_test)(?:\.[^.]+)+$/.test(basename) ||
    /^(?:test[-_]|spec[-_])/.test(basename)
  );
}

function workpackCodeExpectationSatisfied(codeExpectation, implementationFiles, testFiles) {
  if (codeExpectation === 'no-code-required') {
    return implementationFiles.length === 0 && testFiles.length === 0;
  }
  if (codeExpectation === 'tests-only') {
    return implementationFiles.length === 0 && testFiles.length > 0;
  }
  return implementationFiles.length > 0 && testFiles.length > 0;
}

function codeTopologyState(implementationFiles, testFiles) {
  if (implementationFiles.length === 0 && testFiles.length === 0) return 'no-source';
  if (implementationFiles.length === 0) return 'tests-only';
  if (testFiles.length === 0) return 'source-only';
  return 'code-and-tests';
}

async function walkCodeFiles(root, relativeDirectory) {
  const files = [];
  async function visit(relativePath) {
    const absolutePath = path.join(root, relativePath);
    let entries;
    try {
      entries = await readdir(absolutePath, { withFileTypes: true });
    } catch (error) {
      if (error?.code === 'ENOENT') return;
      if (error?.code === 'ENOTDIR') {
        if (CODE_EXTENSIONS.has(path.extname(relativePath).toLowerCase())) files.push(normalizeRepoPath(relativePath));
        return;
      }
      throw error;
    }
    for (const entry of entries) {
      if (entry.isDirectory() && IGNORED_CODE_DIRECTORIES.has(entry.name)) continue;
      const child = normalizeRepoPath(path.join(relativePath, entry.name));
      if (entry.isDirectory()) {
        await visit(child);
        continue;
      }
      if (entry.isFile() && CODE_EXTENSIONS.has(path.extname(entry.name).toLowerCase())) {
        files.push(child);
      }
    }
  }
  await visit(normalizeRepoPath(relativeDirectory));
  return files;
}

export async function buildCodeInventory({ root = process.cwd(), codeMapPath = CODE_MAP_PATH, scope } = {}) {
  const codeMap = await loadCodeMap(root, codeMapPath);
  const rootScope = !scope || scope === 'GOAL-ocentra-parent';
  const selected = Object.entries(codeMap.plans).filter(
    ([planSlug]) => rootScope || planId(planSlug) === scope || planSlug === scope
  );
  const plans = [];
  const allCodeFiles = new Set();
  const allImplementationFiles = new Set();
  const allTestFiles = new Set();
  for (const [planSlug, roots] of selected) {
    const uniqueRoots = [...new Set(roots.map(normalizeRepoPath))];
    const missingRoots = uniqueRoots.filter((relativePath) => !pathExistsSync(root, relativePath));
    const files = [];
    for (const relativePath of uniqueRoots.filter((candidate) => !missingRoots.includes(candidate))) {
      files.push(...(await walkCodeFiles(root, relativePath)));
    }
    const uniqueFiles = [...new Set(files)].sort();
    const testFiles = uniqueFiles.filter(isTestPath);
    const implementationFiles = uniqueFiles.filter((file) => !isTestPath(file));
    for (const file of uniqueFiles) allCodeFiles.add(file);
    for (const file of implementationFiles) allImplementationFiles.add(file);
    for (const file of testFiles) allTestFiles.add(file);
    const state = codeTopologyState(implementationFiles, testFiles);
    plans.push({
      planId: planId(planSlug),
      planSlug,
      roots: uniqueRoots,
      missingRoots,
      state,
      codeFiles: uniqueFiles.length,
      implementationFiles: implementationFiles.length,
      testFiles: testFiles.length,
      implementationPaths: implementationFiles,
      testPaths: testFiles,
    });
  }
  const workpacks = [];
  for (const [workpackId, entry] of Object.entries(codeMap.workpacks ?? {})) {
    const planSlug = entry.planSlug ?? null;
    const codeExpectation = entry.codeExpectation ?? 'code-and-tests';
    if (!rootScope && planSlug && planId(planSlug) !== scope && planSlug !== scope) continue;
    if (!rootScope && !planSlug && !workpackId.startsWith(`WP-${String(scope).replace(/^PLAN-/u, '')}-`)) continue;
    const uniqueRoots = [...new Set(entry.roots.map(normalizeRepoPath))];
    const missingRoots = uniqueRoots.filter((relativePath) => !pathExistsSync(root, relativePath));
    const files = [];
    for (const relativePath of uniqueRoots.filter((candidate) => !missingRoots.includes(candidate))) {
      files.push(...(await walkCodeFiles(root, relativePath)));
    }
    const uniqueFiles = [...new Set(files)].sort();
    const testFiles = uniqueFiles.filter(isTestPath);
    const implementationFiles = uniqueFiles.filter((file) => !isTestPath(file));
    workpacks.push({
      workpackId,
      planSlug,
      codeExpectation,
      codeExpectationSatisfied: workpackCodeExpectationSatisfied(codeExpectation, implementationFiles, testFiles),
      roots: uniqueRoots,
      missingRoots,
      state: codeTopologyState(implementationFiles, testFiles),
      codeFiles: uniqueFiles.length,
      implementationFiles: implementationFiles.length,
      testFiles: testFiles.length,
      implementationPaths: implementationFiles,
      testPaths: testFiles,
    });
  }
  return {
    schemaVersion: codeMap.schemaVersion,
    authority: codeMap.authority,
    codeMapPath,
    plans,
    workpacks,
    totals: {
      plans: plans.length,
      codeFiles: allCodeFiles.size,
      implementationFiles: allImplementationFiles.size,
      testFiles: allTestFiles.size,
      reviewedWorkpackMaps: workpacks.length,
    },
  };
}

function stateCounts(nodes, states) {
  const counts = Object.fromEntries([...STATES].map((state) => [state, 0]));
  for (const node of nodes) counts[states.get(node.id)] += 1;
  return counts;
}

/**
 * Join the graph's derived workpack state with the reviewed plan-to-runtime
 * code/test inventory.  The two sources intentionally remain separate:
 * code/test counts prove that files exist under an owned plan root, while the
 * graph state proves dependency and completion-contract state.  This report
 * is the machine-readable answer to "where are we?" without turning file
 * counts into a completion claim.
 */
export async function buildProgressReport({ root = process.cwd(), scope } = {}) {
  const graph = await loadGraph(root);
  const validation = validateGraph(graph, { root });
  if (!validation.ok) {
    throw new Error(`Cannot build progress report from an invalid graph: ${validation.errors.join('; ')}`);
  }
  const states = deriveStates(graph, { root });
  const scoped = scopeNodes(graph, scope);
  const plans = scoped.filter((node) => node.kind === 'plan');
  const inventory = await buildCodeInventory({ root, scope });
  const inventoryByPlan = new Map(inventory.plans.map((plan) => [plan.planId, plan]));
  const inventoryByWorkpack = new Map(inventory.workpacks.map((workpack) => [workpack.workpackId, workpack]));
  const workpacksByPlan = new Map();
  for (const plan of plans) workpacksByPlan.set(plan.id, []);
  for (const node of scoped.filter((candidate) => candidate.kind === 'workpack')) {
    workpacksByPlan.get(node.parent)?.push(node);
  }

  const planReports = plans.map((plan) => {
    const workpacks = workpacksByPlan.get(plan.id) ?? [];
    const planInventory = inventoryByPlan.get(plan.id);
    const workpackReports = workpacks
      .sort((left, right) => left.id.localeCompare(right.id))
      .map((workpack) => {
        const gaps = completionGaps(root, workpack);
        const workpackInventory = inventoryByWorkpack.get(workpack.id);
        const dependsOn = relatedNodes(graph, workpack.id, 'deps');
        const blockers = dependsOn
          .filter((dependencyId) => states.get(dependencyId) !== 'done')
          .map((dependencyId) => ({ id: dependencyId, state: states.get(dependencyId) ?? 'unknown' }));
        return {
          id: workpack.id,
          title: workpack.title,
          state: states.get(workpack.id),
          storedState: workpack.state,
          dependsOn,
          blockers,
          unlocks: relatedNodes(graph, workpack.id, 'dependents'),
          completionContract: {
            pathsPresent: gaps.length === 0,
            gaps,
          },
          // A workpack-to-file map is never inferred from prose.  Explicit
          // reviewed entries expose exact code/test topology; every other row
          // stays visibly unknown instead of inheriting a plan-wide count.
          codeTestTopology: workpackInventory
            ? {
                scope: 'reviewed-workpack-roots',
                state: workpackInventory.state,
                codeExpectation: workpackInventory.codeExpectation,
                codeExpectationSatisfied: workpackInventory.codeExpectationSatisfied,
                roots: workpackInventory.roots,
                missingRoots: workpackInventory.missingRoots,
                implementationFiles: workpackInventory.implementationFiles,
                testFiles: workpackInventory.testFiles,
                implementationPaths: workpackInventory.implementationPaths,
                testPaths: workpackInventory.testPaths,
              }
            : 'unknown-workpack-ownership',
        };
      });
    const counts = stateCounts(workpacks, states);
    return {
      id: plan.id,
      slug: plan.metadata?.planSlug,
      title: plan.title,
      state: states.get(plan.id),
      workpacks: {
        total: workpackReports.length,
        counts,
        rows: workpackReports,
      },
      codeTestTopology: {
        scope: 'reviewed-plan-roots',
        state: planInventory?.state ?? 'no-code-map-entry',
        roots: planInventory?.roots ?? [],
        missingRoots: planInventory?.missingRoots ?? [],
        implementationFiles: planInventory?.implementationFiles ?? 0,
        testFiles: planInventory?.testFiles ?? 0,
      },
    };
  });

  const allWorkpacks = scoped.filter((node) => node.kind === 'workpack');
  const unindexedWorkpackArtifacts = graph.migration?.unindexedWorkpackArtifacts ?? [];
  return {
    schemaVersion: 1,
    authority: {
      graphState: 'derived dependency, lifecycle, and completion-contract state',
      codeTestTopology: 'live files under reviewed plan-to-runtime roots',
      acceptance: 'tests, proof, CI, review, and merge remain separate gates',
    },
    scope: scope ?? 'GOAL-ocentra-parent',
    plans: planReports,
    totals: {
      plans: plans.length,
      workpacks: allWorkpacks.length,
      states: stateCounts(allWorkpacks, states),
      implementationFiles: inventory.totals.implementationFiles,
      testFiles: inventory.totals.testFiles,
      reviewedWorkpackMaps: inventory.totals.reviewedWorkpackMaps,
    },
    migration: {
      reviewItems: graph.migration?.ambiguities?.length ?? 0,
      unindexedWorkpackFiles: unindexedWorkpackArtifacts.reduce(
        (total, artifact) => total + (artifact.paths?.length ?? 0),
        0
      ),
      unindexedWorkpackArtifacts,
    },
    validation: {
      ok: validation.ok,
      warnings: validation.warnings,
    },
  };
}

/**
 * Flatten the joined report into the operator matrix used for plan-by-plan
 * status reviews.  This intentionally keeps unknown workpack ownership
 * explicit; plan-root counts are never copied into a workpack row.
 */
export function flattenProgressReport(report) {
  return report.plans.flatMap((plan) =>
    plan.workpacks.rows.map((workpack) => {
      const topology = workpack.codeTestTopology;
      return {
        planId: plan.id,
        planTitle: plan.title,
        planState: plan.state,
        workpackId: workpack.id,
        workpackTitle: workpack.title,
        state: workpack.state,
        storedState: workpack.storedState,
        codeState: typeof topology === 'string' ? topology : topology.state,
        codeExpectation: typeof topology === 'string' ? null : topology.codeExpectation,
        codeExpectationSatisfied: typeof topology === 'string' ? null : topology.codeExpectationSatisfied,
        implementationFiles: typeof topology === 'string' ? null : topology.implementationFiles,
        testFiles: typeof topology === 'string' ? null : topology.testFiles,
        dependsOn: workpack.dependsOn,
        blockers: workpack.blockers,
        unlocks: workpack.unlocks,
        completionGapCount: workpack.completionContract.gaps.length,
        completionGaps: workpack.completionContract.gaps,
      };
    })
  );
}

/**
 * Return legal READY work first.  When the graph authorizes no new work, also
 * expose unblocked active/validation rows that are candidates for evidence or
 * review.  The latter are explicitly not READY authorization.
 */
export function nextWork(graph, { root = process.cwd(), scope } = {}) {
  const states = deriveStates(graph, { root });
  const workpacks = scopeNodes(graph, scope).filter((node) => node.kind === 'workpack');
  const ready = workpacks.filter((node) => states.get(node.id) === 'ready');
  const validationQueue = workpacks
    .filter((node) => ['active', 'validation'].includes(states.get(node.id)))
    .filter((node) => relatedNodes(graph, node.id, 'deps').every((dependencyId) => states.get(dependencyId) === 'done'))
    .sort((left, right) => {
      const rank = (node) => (states.get(node.id) === 'active' ? 0 : 1);
      return rank(left) - rank(right) || left.id.localeCompare(right.id);
    });
  return {
    scope: scope ?? 'GOAL-ocentra-parent',
    authorized: ready,
    validationQueue,
    recommendation:
      ready.length > 0
        ? 'Start only READY workpacks; validation and proof remain part of their completion contract.'
        : validationQueue.length > 0
          ? 'No READY workpack is authorized. Finish the unblocked validation/review queue before starting new work.'
          : 'No READY or unblocked validation work exists; inspect blocked workpacks and their dependency reasons.',
  };
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

export function parseWorkpackRows(indexText, availableWorkpackPaths = []) {
  if (!indexText) return [];
  const statusColumn = tableColumnIndex(indexText, /^(?:status|state|lifecycle)$/iu);
  const rows = [];
  for (const line of indexText.split(/\r?\n/)) {
    const match = line.match(/\[([^\]]+)\]\((workpacks\/[^)]+\.md)\)/i);
    if (!match) continue;
    const cells = line
      .split('|')
      .map((cell) => cell.trim())
      .filter(Boolean);
    const statusText = cleanMarkdownText(cells[statusColumn ?? cells.length - 1] ?? '');
    rows.push({
      title: cleanMarkdownText(match[1]),
      relativePath: normalizeRepoPath(match[2]),
      statusText,
      sourceFormat: 'linked-row',
    });
  }
  if (rows.length > 0 || availableWorkpackPaths.length === 0) return rows;

  // Some plan indexes use a compact ID/state table and keep the authoritative
  // workpack filename in `workpacks/`.  Import that existing format instead of
  // silently treating the plan as if it had no workpacks (LAN currently uses
  // this form for rows 01-25).
  const pathsByNumber = new Map();
  for (const candidate of availableWorkpackPaths) {
    const basename = path.basename(candidate);
    const match = basename.match(/^(\d{1,3})[-_].+\.md$/i);
    if (match) pathsByNumber.set(String(Number(match[1])), normalizeRepoPath(candidate));
  }
  for (const line of indexText.split(/\r?\n/)) {
    const match = line.match(/^\s*\|\s*`?(\d{1,3})`?\s*\|\s*([^|]+)\|/);
    if (!match) continue;
    const relativePath = pathsByNumber.get(String(Number(match[1])));
    if (!relativePath) continue;
    const cells = line
      .split('|')
      .map((cell) => cell.trim())
      .filter(Boolean);
    const stem = path.basename(relativePath).replace(/\.md$/i, '');
    rows.push({
      title: humanize(stem),
      relativePath,
      // In this table format only the state column controls lifecycle.  The
      // current-truth column may legitimately mention a manual-required
      // boundary without making the workpack itself graph-blocked.
      statusText: cleanMarkdownText(cells[statusColumn ?? 1] ?? match[2]),
      sourceFormat: 'numeric-table-row',
    });
  }
  return rows;
}

function tableColumnIndex(indexText, headerPattern) {
  for (const line of indexText.split(/\r?\n/)) {
    if (!line.includes('|')) continue;
    const cells = line
      .split('|')
      .map((cell) => cell.trim())
      .filter(Boolean);
    if (cells.length === 0 || cells.every((cell) => /^:?-{3,}:?$/u.test(cell))) continue;
    const index = cells.findIndex((cell) => headerPattern.test(cell));
    if (index >= 0) return index;
  }
  return null;
}

export function classifyWorkpackStatus(statusText) {
  const value = statusText.toLowerCase();
  if (/\b(?:incomplete|unfinished|unmerged|not\s+(?:done|complete|merged|active))\b/u.test(value)) {
    return 'planned';
  }
  if (value.includes('historical')) return 'validation';
  if (value.includes('blocked') || value.includes('manual-required')) return 'blocked';
  if (value.includes('failed')) return 'failed';
  if (value.includes('paused')) return 'paused';
  if (value.includes('active') || value.includes('in progress') || value.includes('ci-active')) {
    return 'active';
  }
  if (value.includes('checked') || value.includes('validation')) return 'validation';
  if (value.includes('partial')) return 'validation';
  if (value.includes('done') || value.includes('complete') || value.includes('merged')) {
    return 'done';
  }
  if (value.includes('ready')) return 'ready';
  if (value.includes('open') && !value.includes('unknown')) return 'planned';
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
  // Generated output is an expected artifact, not a portable graph reference:
  // output/ is ignored and may exist only in one checkout.  Only retain a
  // durable docs/proof reference in the checked-in graph.
  const durableProof = expectedProofRoot === durableProofRoot ? await firstExisting(root, [durableProofRoot]) : null;
  const adr = await firstExisting(root, [
    normalizeRepoPath(path.join(planRoot, 'adr')),
    normalizeRepoPath(path.join(planRoot, 'adrs')),
  ]);
  const required = ['implementation', 'tests', 'proof', 'checklist'];
  if (adr) required.push('adr');
  const expected = expectedProofRoot !== durableProofRoot ? { proof: [expectedProofRoot] } : {};
  return {
    required,
    reviewed: {},
    references: {
      implementation: [workpackPath],
      tests: tests ? [tests] : [],
      proof: durableProof ? [durableProof] : [],
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
  const workpackDirectory = path.join(root, planRoot, 'workpacks');
  let availableWorkpackPaths = [];
  try {
    availableWorkpackPaths = (await readdir(workpackDirectory, { withFileTypes: true }))
      .filter((entry) => entry.isFile() && entry.name.toLowerCase().endsWith('.md'))
      .map((entry) => normalizeRepoPath(path.join('workpacks', entry.name)));
  } catch (error) {
    if (error?.code !== 'ENOENT') throw error;
  }
  const rows = parseWorkpackRows(indexText, availableWorkpackPaths);
  const indexedPaths = new Set(rows.map((row) => normalizeRepoPath(row.relativePath)));
  const unindexedWorkpackFiles = availableWorkpackPaths.filter(
    (candidate) => !indexedPaths.has(normalizeRepoPath(candidate))
  );
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
        sourceFormat: row.sourceFormat ?? 'linked-row',
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
    unindexedWorkpackFiles,
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
  if (!text) {
    return {
      edges: [],
      ambiguities: [],
      stateOverrides: [],
      proofOverrides: [],
      completionEvidenceOverrides: [],
    };
  }
  const parsed = JSON.parse(text);
  return {
    edges: Array.isArray(parsed.edges) ? parsed.edges : [],
    ambiguities: Array.isArray(parsed.ambiguities) ? parsed.ambiguities : [],
    stateOverrides: Array.isArray(parsed.stateOverrides) ? parsed.stateOverrides : [],
    proofOverrides: Array.isArray(parsed.proofOverrides) ? parsed.proofOverrides : [],
    completionEvidenceOverrides: Array.isArray(parsed.completionEvidenceOverrides)
      ? parsed.completionEvidenceOverrides
      : [],
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
  const unindexedWorkpackArtifacts = [];
  for (const entry of entries) {
    const result = await buildPlan(repoRoot, entry);
    plans.push(result.plan);
    workpacks.push(...result.workpacks);
    ambiguities.push(result.ambiguity);
    if (result.unindexedWorkpackFiles.length > 0) {
      const artifact = {
        planId: result.plan.id,
        indexPath: result.plan.metadata.indexPath,
        paths: result.unindexedWorkpackFiles,
      };
      unindexedWorkpackArtifacts.push(artifact);
      ambiguities.push({
        scope: `${result.plan.id}:unindexed-workpack-files`,
        reason:
          'Markdown files exist under the workpacks directory but are not linked by WORKPACK_INDEX.md; classify them before treating them as graph workpacks.',
        path: result.plan.metadata.indexPath,
        unindexedWorkpackFiles: result.unindexedWorkpackFiles,
      });
    }
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
  for (const edge of overrides.edges) {
    edges.push(edge);
    if (edge.kind !== 'depends_on') continue;
    const evidence = Array.isArray(edge.evidence) ? edge.evidence : [];
    const missingEvidence = evidence.filter((reference) => !pathExistsSync(repoRoot, reference));
    if (edge.confidence !== 'reviewed' || evidence.length === 0 || missingEvidence.length > 0) {
      ambiguities.push({
        scope: `edge:${edge.from}->${edge.to}`,
        reason: 'Reviewed dependency edges require confidence=reviewed and existing evidence paths.',
        evidenceGaps: [
          ...(edge.confidence !== 'reviewed' ? ['confidence must be reviewed'] : []),
          ...(evidence.length === 0 ? ['evidence is missing'] : []),
          ...missingEvidence.map((reference) => `missing evidence ${reference}`),
        ],
      });
    }
  }

  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  for (const override of overrides.stateOverrides) {
    const node = nodeById.get(override.id);
    if (!node || !STATES.has(override.state)) continue;
    const evidence = Array.isArray(override.evidence) ? override.evidence : [];
    const missingEvidence = evidence.filter((reference) => !pathExistsSync(repoRoot, reference));
    const rejectionReasons = [
      ...(override.state !== 'validation' ? ['state overrides may only record validation slices'] : []),
      ...(evidence.length === 0 ? ['evidence is required'] : []),
      ...missingEvidence.map((reference) => `missing evidence ${reference}`),
    ];
    node.metadata = {
      ...node.metadata,
      stateOverride: {
        reason: override.reason ?? 'reviewed state override',
        evidence,
      },
      ...(rejectionReasons.length > 0 ? { stateOverrideRejected: rejectionReasons } : {}),
      ...(rejectionReasons.length > 0 ? { needsReview: true } : {}),
    };
    if (rejectionReasons.length > 0) continue;
    node.state = override.state;
    node.lifecycleState = override.state;
    node.metadata = {
      ...node.metadata,
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
        satisfiesExpected: override.satisfiesExpected === true,
      },
    };
  }
  for (const override of overrides.completionEvidenceOverrides) {
    const node = nodeById.get(override.id);
    if (!node?.completion) continue;
    const reviewed = { ...(node.completion.reviewed ?? {}) };
    const references = { ...(node.completion.references ?? {}) };
    for (const requirement of node.completion.required ?? []) {
      const evidence = Array.isArray(override[requirement]) ? override[requirement] : [];
      if (evidence.length === 0) continue;
      references[requirement] = evidence;
      reviewed[requirement] = true;
    }
    node.completion = { ...node.completion, reviewed, references };
    node.metadata = {
      ...node.metadata,
      completionEvidenceOverride: {
        reason: override.reason ?? 'reviewed completion evidence override',
        evidence: override.evidence ?? [],
      },
    };
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

  // Apply dependency blocking after DONE demotion.  A stale DONE dependency
  // must become validation before its dependents are derived as blocked.
  for (const edge of edges.filter((candidate) => candidate.kind === 'depends_on')) {
    const dependent = nodeById.get(edge.from);
    const dependency = nodeById.get(edge.to);
    if (!dependent || !dependency) continue;
    dependent.dependsOn ??= [];
    if (!dependent.dependsOn.includes(dependency.id)) dependent.dependsOn.push(dependency.id);
    if (dependency.state !== 'done') dependent.state = 'blocked';
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
      unindexedWorkpackArtifacts,
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
    if (required.has(requirement) && node.completion.reviewed?.[requirement] !== true) {
      gaps.push(`${requirement}: reviewed evidence is not recorded`);
    }
    if (required.has(requirement) && references.length === 0 && expected.length === 0) {
      gaps.push(`${requirement}: no reference or expected artifact is declared`);
      continue;
    }
    for (const reference of references) {
      if (!pathExistsSync(root, reference)) gaps.push(`${requirement}: missing reference ${reference}`);
      if (
        node.completion.reviewed?.[requirement] === true &&
        isPlanningDocumentEvidence(node, requirement, reference)
      ) {
        gaps.push(`${requirement}: planning document is not executable evidence ${reference}`);
      }
    }
    for (const reference of expected) {
      if (!pathExistsSync(root, reference) && !durableProofSatisfiesExpected(root, node, requirement)) {
        gaps.push(`${requirement}: missing expected artifact ${reference}`);
      }
    }
  }
  return gaps;
}

function durableProofSatisfiesExpected(root, node, requirement) {
  if (requirement !== 'proof' || node.metadata?.proofOverride?.satisfiesExpected !== true) return false;
  const references = node.completion?.references?.proof ?? [];
  return references.length > 0 && references.every((reference) => pathExistsSync(root, reference));
}

function isPlanningDocumentEvidence(node, requirement, reference) {
  if (!['implementation', 'tests'].includes(requirement)) return false;
  const planSlug = node.metadata?.planSlug;
  if (typeof planSlug !== 'string' || planSlug.length === 0) return false;
  const normalized = normalizeRepoPath(reference);
  return normalized.startsWith(`docs/plans/${planSlug}/`);
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
    else if (childStates.some((state) => state === 'failed')) states.set(node.id, 'failed');
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
      else if (
        !graph.edges.some((edge) => edge.kind === 'depends_on' && edge.from === node.id && edge.to === dependency)
      ) {
        errors.push(`${node.id} declares dependency ${dependency} without a matching depends_on edge`);
      }
    }
    if (!node.path) errors.push(`${node.id} is missing path`);
    else if (!pathExistsSync(root, node.path)) {
      if (node.kind === 'workpack') errors.push(`${node.id} workpack path is missing: ${node.path}`);
      else warnings.push(`${node.id} path is missing: ${node.path}`);
    }
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
          if (!pathExistsSync(root, reference) && !durableProofSatisfiesExpected(root, node, requirement))
            warnings.push(`${node.id} ${requirement} expected artifact is missing: ${reference}`);
        }
      }
    }
    if (node.state === 'done' && !completionSatisfied(root, node)) {
      errors.push(
        `${node.id} is marked done but its completion contract is unsatisfied: ${completionGaps(root, node).join('; ')}`
      );
    }
    for (const reference of node.metadata?.stateOverride?.evidence ?? []) {
      if (!pathExistsSync(root, reference)) errors.push(`${node.id} state override evidence is missing: ${reference}`);
    }
    for (const reason of node.metadata?.stateOverrideRejected ?? []) {
      errors.push(`${node.id} state override rejected: ${reason}`);
    }
    for (const reference of node.metadata?.proofOverride?.evidence ?? []) {
      if (!pathExistsSync(root, reference)) errors.push(`${node.id} proof override evidence is missing: ${reference}`);
    }
    for (const reference of node.metadata?.completionEvidenceOverride?.evidence ?? []) {
      if (!pathExistsSync(root, reference)) errors.push(`${node.id} completion evidence is missing: ${reference}`);
    }
  }
  for (const edge of graph.edges) {
    if (!map.has(edge.from)) errors.push(`edge references missing from node ${edge.from}`);
    if (!map.has(edge.to)) errors.push(`edge references missing to node ${edge.to}`);
    if (!edge.kind) errors.push(`edge ${edge.from} -> ${edge.to} is missing kind`);
    else if (!EDGE_KINDS.has(edge.kind))
      errors.push(`edge ${edge.from} -> ${edge.to} has unsupported kind ${edge.kind}`);
    if (edge.kind === 'depends_on') {
      const dependent = map.get(edge.from);
      if (dependent && !(dependent.dependsOn ?? []).includes(edge.to)) {
        errors.push(`${edge.from} has depends_on edge ${edge.to} without a matching dependsOn entry`);
      }
      if (edge.confidence !== 'reviewed') {
        errors.push(`${edge.from} -> ${edge.to} depends_on edge must have confidence=reviewed`);
      }
      const evidence = Array.isArray(edge.evidence) ? edge.evidence : [];
      if (evidence.length === 0) errors.push(`${edge.from} -> ${edge.to} depends_on edge is missing evidence`);
      for (const reference of evidence) {
        if (!pathExistsSync(root, reference)) {
          errors.push(`${edge.from} -> ${edge.to} dependency evidence is missing: ${reference}`);
        }
      }
    }
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

function canonicalJson(value) {
  if (Array.isArray(value)) return value.map(canonicalJson);
  if (value && typeof value === 'object') {
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalJson(value[key])])
    );
  }
  return value;
}

export function graphSourceDrift(checkedIn, generated) {
  if (JSON.stringify(canonicalJson(checkedIn)) === JSON.stringify(canonicalJson(generated))) return [];
  return [
    `checked-in graph differs from the current plan/workpack source (${checkedIn.nodes?.length ?? 0} nodes vs ${generated.nodes?.length ?? 0}); run npm run graph:bootstrap -- --write`,
  ];
}

export async function writeGraph(root, graphPath, graph) {
  const target = path.join(root, graphPath);
  const source = JSON.stringify(graph, null, 2);
  // Read-only graph queries must remain usable from a fresh checkout where
  // workspace dependencies have not been installed yet.  Keep the formatter
  // on the write-only path so validate/report/status do not fail at import.
  const { default: prettier } = await import('prettier');
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
  if (states.get(node.id) === 'blocked' && reasons.length === 0) {
    const lifecycleReason =
      node.metadata?.statusText ??
      node.metadata?.stateOverride?.reason ??
      node.metadata?.sourceStatusText ??
      node.metadata?.stateOverrideRejected?.join('; ');
    reasons.push(
      lifecycleReason
        ? `lifecycle blocker: ${lifecycleReason}`
        : 'lifecycle blocker: the workpack is marked blocked without a dependency explanation'
    );
  }
  return { node, state: states.get(node.id), reasons };
}

export function relatedNodes(graph, nodeId, direction) {
  const edges = dependencyEdges(graph);
  if (direction === 'deps') return edges.filter((edge) => edge.from === nodeId).map((edge) => edge.to);
  return edges.filter((edge) => edge.to === nodeId).map((edge) => edge.from);
}
