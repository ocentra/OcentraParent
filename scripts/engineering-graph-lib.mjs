import { lstatSync, realpathSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { readFile, readdir, writeFile } from 'node:fs/promises';
import path from 'node:path';

/** Checked-in engineering graph and override schema version. */
export const GRAPH_SCHEMA_VERSION = 2;
export const GRAPH_PATH = 'docs/engineering-graph/graph.json';
export const OVERRIDES_PATH = 'docs/engineering-graph/overrides.json';
export const CODE_MAP_PATH = 'docs/engineering-graph/code-map.json';
const WORKPACK_CODE_EXPECTATIONS = new Set(['code-and-tests', 'tests-only', 'no-code-required']);
const CODE_MAP_SCHEMA_VERSIONS = new Set([1, 2]);
const PLANNED_IMPLEMENTATION_EXPECTATIONS = new Set(['code-and-tests']);
const PLANNED_TEST_EXPECTATIONS = new Set(['code-and-tests', 'tests-only']);
const WORKSPACE_TARGET_KINDS = new Set(['lib', 'bin']);
const IMPLEMENTATION_GATE = 'reviewed-implementation';
const IMPLEMENTATION_GATE_VALUES = new Set([IMPLEMENTATION_GATE]);
const COMPLETION_EVIDENCE_FIELDS = new Set(['id', 'reason', 'evidence']);
const OVERRIDE_TOP_LEVEL_FIELDS = new Set([
  'schemaVersion',
  'edges',
  'workpackReviews',
  'stateOverrides',
  'proofOverrides',
  'completionEvidenceOverrides',
  'ambiguities',
]);
const OVERRIDE_FIELDS = Object.freeze({
  workpackReviews: new Set(['id', 'hardDependencies', 'evidence', 'reason']),
  edges: new Set(['from', 'to', 'kind', 'confidence', 'evidence', 'reason', 'implementationGate']),
  stateOverrides: new Set(['id', 'state', 'reason', 'statusText', 'needsReview', 'evidence']),
  proofOverrides: new Set(['id', 'proof', 'reason', 'evidence', 'satisfiesExpected']),
  completionEvidenceOverrides: new Set([
    'id',
    'reason',
    'evidence',
    'implementation',
    'tests',
    'proof',
    'checklist',
    'adr',
  ]),
  ambiguities: new Set(['scope', 'reason', 'nextAction']),
});

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
const NON_EXECUTABLE_CODE_DIRECTORIES = new Set([
  ...IGNORED_CODE_DIRECTORIES,
  '.agents',
  '.codebase-memory',
  '.codeql-local',
  '.codex',
  '.codex-artifacts',
  '.codex-logs',
  '.codex-tmp',
  '.enforce',
  '.generated',
  '.github',
  '.hub',
  '.idea',
  '.ledger',
  '.logs',
  '.tmp',
  '.turbo',
  '.vscode',
  'docs',
  'generated',
  'ocentra-ledger',
  'output',
  'outputs',
  'proof',
  'proofs',
  'tmp',
  'vendor',
]);
const SUPPORT_ONLY_CODE_DIRECTORIES = new Set([
  'benchmark',
  'benchmarks',
  'benches',
  'demo',
  'demos',
  'example',
  'examples',
  'fixture',
  'fixtures',
  'mock',
  'mocks',
  'sample',
  'samples',
  'stub',
  'stubs',
  'test-data',
  'testdata',
]);
const workspaceMetadataCache = new Map();

const NODE_KINDS = new Set(['goal', 'plan', 'workpack']);
const STATES = new Set(['planned', 'blocked', 'ready', 'active', 'validation', 'done', 'failed', 'paused']);
const EDGE_KINDS = new Set(['contains', 'depends_on']);
const GRAPH_EDGE_FIELDS = new Set(['from', 'to', 'kind', 'confidence', 'evidence', 'reason', 'implementationGate']);

export function normalizeRepoPath(value) {
  return value.replace(/\\/gu, '/');
}

function isRepoRelativePath(value) {
  if (typeof value !== 'string' || value.trim().length === 0) return false;
  const normalized = normalizeRepoPath(value.trim());
  if (normalized === '.' || normalized.startsWith('/') || path.posix.isAbsolute(normalized)) return false;
  if (/^[A-Za-z]:/u.test(normalized)) return false;
  return !normalized.split('/').some((part) => part === '..' || part.length === 0);
}

function isPathInside(rootPath, candidatePath) {
  const relative = path.relative(rootPath, candidatePath);
  return relative === '' || (!relative.startsWith(`..${path.sep}`) && relative !== '..' && !path.isAbsolute(relative));
}

function repoPathStatus(root, relativePath) {
  if (!isRepoRelativePath(relativePath)) {
    return {
      exists: false,
      regularFile: false,
      reason: 'path must be repository-relative and cannot traverse outside the repository',
    };
  }
  let rootPath;
  try {
    rootPath = realpathSync(root);
  } catch {
    return { exists: false, regularFile: false, reason: 'repository root does not exist' };
  }
  const normalized = normalizeRepoPath(relativePath.trim());
  const candidate = path.resolve(rootPath, normalized);
  if (!isPathInside(rootPath, candidate)) {
    return { exists: false, regularFile: false, reason: 'path resolves outside the repository' };
  }
  let current = rootPath;
  const parts = normalized.split('/');
  for (const [index, part] of parts.entries()) {
    current = path.join(current, part);
    let entryStat;
    try {
      entryStat = lstatSync(current);
    } catch (error) {
      if (error?.code === 'ENOENT') {
        return { exists: false, regularFile: false, reason: `missing path ${relativePath}` };
      }
      return { exists: false, regularFile: false, reason: `path cannot be inspected ${relativePath}` };
    }
    if (entryStat.isSymbolicLink()) {
      return { exists: false, regularFile: false, reason: `symbolic-link path is not accepted ${relativePath}` };
    }
    let resolvedEntry;
    try {
      resolvedEntry = realpathSync(current);
    } catch {
      return { exists: false, regularFile: false, reason: `path cannot be inspected ${relativePath}` };
    }
    if (!isPathInside(rootPath, resolvedEntry)) {
      return { exists: false, regularFile: false, reason: 'path resolves outside the repository' };
    }
    if (index < parts.length - 1 && !entryStat.isDirectory()) {
      return { exists: false, regularFile: false, reason: `path parent is not a directory ${relativePath}` };
    }
  }
  let resolved;
  let stat;
  try {
    resolved = realpathSync(candidate);
    stat = lstatSync(candidate);
  } catch {
    return { exists: false, regularFile: false, reason: `path cannot be inspected ${relativePath}` };
  }
  if (!isPathInside(rootPath, resolved)) {
    return { exists: false, regularFile: false, reason: 'path resolves outside the repository' };
  }
  return {
    exists: true,
    regularFile: stat.isFile(),
    reason: stat.isFile() ? null : `path is not a regular file ${relativePath}`,
  };
}

function executablePathProblem(
  reference,
  requirement,
  { requireNormalized = false, requireProductionPath = false } = {}
) {
  if (!['implementation', 'tests'].includes(requirement)) return null;
  if (!isRepoRelativePath(reference)) {
    return `path must be repository-relative and cannot traverse outside the repository: ${reference}`;
  }
  const trimmed = reference.trim();
  const normalized = normalizeRepoPath(trimmed);
  if (
    requireNormalized &&
    (reference !== trimmed || normalized !== trimmed || path.posix.normalize(normalized) !== normalized)
  ) {
    return `path must use normalized repository-relative form: ${reference}`;
  }
  const extension = path.extname(normalized).toLowerCase();
  if (!CODE_EXTENSIONS.has(extension)) return `unsupported executable evidence path ${reference}`;
  const lower = normalized.toLowerCase();
  const segments = lower.split('/');
  if (requireProductionPath && segments.some((segment) => NON_EXECUTABLE_CODE_DIRECTORIES.has(segment))) {
    return `non-executable or generated path is not accepted ${reference}`;
  }
  const basename = path.posix.basename(lower);
  if (
    requireProductionPath &&
    (segments.some((segment) => SUPPORT_ONLY_CODE_DIRECTORIES.has(segment)) ||
      /(?:^|[._-])(?:example|fixture|mock|sample|stub)s?(?:[._-]|$)/u.test(basename) ||
      /(?:^|[._-])generated(?:[._-]|$)/u.test(basename))
  ) {
    return `support-only or generated path is not accepted ${reference}`;
  }
  if (requirement === 'implementation' && isTestPath(normalized)) {
    return `test path is not production implementation ${reference}`;
  }
  if (requirement === 'tests' && !isTestPath(normalized)) {
    return `planned test path is not test-classified ${reference}`;
  }
  return null;
}

function plannedExecutablePathProblem(root, reference, requirement) {
  const pathProblem = executablePathProblem(reference, requirement, {
    requireNormalized: true,
    requireProductionPath: true,
  });
  if (pathProblem) return pathProblem;
  const status = repoPathStatus(root, reference);
  if (!status.exists) return status.reason.startsWith('missing path ') ? null : status.reason;
  if (!status.regularFile) return status.reason;
  return null;
}

function executableEvidenceProblem(root, _node, requirement, reference) {
  const pathProblem = executablePathProblem(reference, requirement);
  if (pathProblem) return pathProblem;
  const status = repoPathStatus(root, reference);
  if (!status.exists) return status.reason;
  if (!status.regularFile) return status.reason;
  return null;
}

function assertPlannedExecutableRoots(
  root,
  codeMapPath,
  workpackId,
  entry,
  { field, requirement, allowedExpectations }
) {
  const values = entry[field];
  if (values === undefined) return;
  const codeExpectation = entry.codeExpectation ?? 'code-and-tests';
  if (!allowedExpectations.has(codeExpectation)) {
    throw new Error(
      `${codeMapPath} workpack ${workpackId} ${field} is not allowed with codeExpectation ${codeExpectation}`
    );
  }
  if (!Array.isArray(values) || values.length === 0) {
    throw new Error(`${codeMapPath} workpack ${workpackId} ${field} must be a non-empty array when present`);
  }
  for (const reference of values) {
    const problem = plannedExecutablePathProblem(root, reference, requirement);
    if (problem) throw new Error(`${codeMapPath} workpack ${workpackId} ${field}: ${problem}`);
  }
  const normalizedValues = values.map(normalizeRepoPath);
  if (new Set(normalizedValues).size !== normalizedValues.length) {
    throw new Error(`${codeMapPath} workpack ${workpackId} ${field} must not contain duplicates`);
  }
  const normalizedRoots = new Set(entry.roots.map(normalizeRepoPath));
  if (field !== 'expectedTestRoots' && normalizedValues.some((reference) => !normalizedRoots.has(reference))) {
    throw new Error(`${codeMapPath} workpack ${workpackId} ${field} must be a subset of roots`);
  }
}

function workspaceRequirementSchemaErrors(requirements, { owner = 'workspaceRequirements', roots = [] } = {}) {
  const errors = [];
  if (!requirements || typeof requirements !== 'object' || Array.isArray(requirements)) {
    return [`${owner} must be an object`];
  }
  if (!isRepoRelativePath(requirements.rootManifest)) {
    errors.push(`${owner}.rootManifest must be repository-relative`);
  }
  const normalizedRoots = new Set(roots.map(normalizeRepoPath));
  if (normalizedRoots.size > 0 && !normalizedRoots.has(normalizeRepoPath(requirements.rootManifest ?? ''))) {
    errors.push(`${owner}.rootManifest must be included in roots`);
  }
  if (!Array.isArray(requirements.packages) || requirements.packages.length === 0) {
    errors.push(`${owner}.packages must be a non-empty array`);
    return errors;
  }
  const manifests = new Set();
  const packageNames = new Set();
  for (const [index, packageRequirement] of requirements.packages.entries()) {
    const label = `${owner}.packages[${index}]`;
    if (!packageRequirement || typeof packageRequirement !== 'object' || Array.isArray(packageRequirement)) {
      errors.push(`${label} must be an object`);
      continue;
    }
    if (!isRepoRelativePath(packageRequirement.manifest)) {
      errors.push(`${label}.manifest must be repository-relative`);
    } else {
      const manifest = normalizeRepoPath(packageRequirement.manifest);
      if (manifests.has(manifest)) errors.push(`${owner}.packages must not duplicate manifest ${manifest}`);
      manifests.add(manifest);
      if (normalizedRoots.size > 0 && !normalizedRoots.has(manifest)) {
        errors.push(`${label}.manifest must be included in roots`);
      }
    }
    if (typeof packageRequirement.package !== 'string' || packageRequirement.package.trim().length === 0) {
      errors.push(`${label}.package must be a non-empty package name`);
    } else if (packageNames.has(packageRequirement.package)) {
      errors.push(`${owner}.packages must not duplicate package ${packageRequirement.package}`);
    } else {
      packageNames.add(packageRequirement.package);
    }
    if (packageRequirement.activeMember !== true) {
      errors.push(`${label}.activeMember must be true when active workspace membership is required`);
    }
    if (!Array.isArray(packageRequirement.requiredTargets) || packageRequirement.requiredTargets.length === 0) {
      errors.push(`${label}.requiredTargets must be a non-empty array`);
      continue;
    }
    const targets = new Set();
    for (const [targetIndex, target] of packageRequirement.requiredTargets.entries()) {
      const targetLabel = `${label}.requiredTargets[${targetIndex}]`;
      if (!target || typeof target !== 'object' || Array.isArray(target)) {
        errors.push(`${targetLabel} must be an object`);
        continue;
      }
      if (!WORKSPACE_TARGET_KINDS.has(target.kind)) {
        errors.push(`${targetLabel}.kind must be lib or bin`);
      }
      if (!isRepoRelativePath(target.path)) {
        errors.push(`${targetLabel}.path must be repository-relative`);
      } else {
        const targetPath = normalizeRepoPath(target.path);
        const targetKey = `${target.kind}:${targetPath}`;
        if (targets.has(targetKey)) errors.push(`${label}.requiredTargets must not contain duplicate ${targetKey}`);
        targets.add(targetKey);
        if (normalizedRoots.size > 0 && !normalizedRoots.has(targetPath)) {
          errors.push(`${targetLabel}.path must be included in roots`);
        }
      }
    }
  }
  return errors;
}

function comparableAbsolutePath(value) {
  return normalizeRepoPath(path.resolve(value)).toLowerCase();
}

function cargoMetadataForWorkspace(root, requirements) {
  const manifest = normalizeRepoPath(requirements.rootManifest);
  const cacheKey = `${comparableAbsolutePath(root)}::${manifest}`;
  if (workspaceMetadataCache.has(cacheKey)) return workspaceMetadataCache.get(cacheKey);
  const rootStatus = repoPathStatus(root, manifest);
  if (!rootStatus.exists || !rootStatus.regularFile) {
    const result = { error: `root manifest is unavailable ${manifest}` };
    workspaceMetadataCache.set(cacheKey, result);
    return result;
  }
  try {
    const output = execFileSync(
      'cargo',
      ['metadata', '--no-deps', '--format-version', '1', '--manifest-path', path.resolve(root, manifest)],
      {
        cwd: root,
        encoding: 'utf8',
        maxBuffer: 16 * 1024 * 1024,
        stdio: ['ignore', 'pipe', 'pipe'],
      }
    );
    const metadata = JSON.parse(output);
    const result = {
      packages: Array.isArray(metadata.packages) ? metadata.packages : [],
      workspaceMembers: new Set(Array.isArray(metadata.workspace_members) ? metadata.workspace_members : []),
    };
    workspaceMetadataCache.set(cacheKey, result);
    return result;
  } catch (error) {
    const detail = String(error?.stderr ?? error?.message ?? error)
      .replace(/\s+/gu, ' ')
      .trim()
      .slice(0, 240);
    const result = { error: `cargo metadata --no-deps failed for ${manifest}: ${detail}` };
    workspaceMetadataCache.set(cacheKey, result);
    return result;
  }
}

function workspaceRequirementGaps(root, source) {
  const requirements =
    source?.completion?.workspaceRequirements ??
    source?.metadata?.workspaceRequirements ??
    (source?.rootManifest !== undefined && source?.packages !== undefined ? source : null);
  if (!requirements) return [];
  const gaps = [];
  const rootManifest = normalizeRepoPath(requirements.rootManifest ?? '');
  const rootStatus = repoPathStatus(root, rootManifest);
  if (!rootStatus.exists || !rootStatus.regularFile) {
    gaps.push(`workspace: missing root manifest ${rootManifest}`);
    return gaps;
  }
  const metadata = cargoMetadataForWorkspace(root, requirements);
  if (metadata.error) {
    gaps.push(`workspace: ${metadata.error}`);
    return gaps;
  }
  for (const packageRequirement of requirements.packages ?? []) {
    const manifest = normalizeRepoPath(packageRequirement.manifest);
    const packageStatus = repoPathStatus(root, manifest);
    if (!packageStatus.exists || !packageStatus.regularFile) {
      gaps.push(`workspace: missing package manifest ${manifest}`);
    }
    const packageMetadata = metadata.packages.find(
      (candidate) =>
        comparableAbsolutePath(candidate.manifest_path) === comparableAbsolutePath(path.join(root, manifest))
    );
    if (!packageMetadata) {
      gaps.push(`workspace: package ${packageRequirement.package} is not registered by cargo metadata (${manifest})`);
      for (const target of packageRequirement.requiredTargets ?? []) {
        gaps.push(
          `workspace: required ${target.kind} target ${target.path} cannot be confirmed because package ${packageRequirement.package} is absent`
        );
      }
      continue;
    }
    if (packageRequirement.activeMember === true && !metadata.workspaceMembers.has(packageMetadata.id)) {
      gaps.push(`workspace: package ${packageRequirement.package} is not an active workspace member (${manifest})`);
    }
    for (const target of packageRequirement.requiredTargets ?? []) {
      const targetStatus = repoPathStatus(root, target.path);
      if (!targetStatus.exists || !targetStatus.regularFile) {
        gaps.push(`workspace: missing ${target.kind} target path ${target.path}`);
        continue;
      }
      const targetMetadata = (packageMetadata.targets ?? []).some(
        (candidate) =>
          Array.isArray(candidate.kind) &&
          candidate.kind.includes(target.kind) &&
          comparableAbsolutePath(candidate.src_path) === comparableAbsolutePath(path.join(root, target.path))
      );
      if (!targetMetadata) {
        gaps.push(
          `workspace: package ${packageRequirement.package} lacks required ${target.kind} target ${target.path}`
        );
      }
    }
  }
  return gaps;
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
  if (!CODE_MAP_SCHEMA_VERSIONS.has(map?.schemaVersion) || !map?.plans || typeof map.plans !== 'object') {
    throw new Error(`${codeMapPath} must declare schemaVersion 1 or 2 and a plans object`);
  }
  if (
    map.workpacks !== undefined &&
    (!map.workpacks || typeof map.workpacks !== 'object' || Array.isArray(map.workpacks))
  ) {
    throw new Error(`${codeMapPath} workpacks must be an object when present`);
  }
  for (const [planSlug, roots] of Object.entries(map.plans)) {
    if (!Array.isArray(roots) || roots.some((rootPath) => !isRepoRelativePath(rootPath))) {
      throw new Error(`${codeMapPath} plan ${planSlug} must declare only repository-relative roots`);
    }
    if (new Set(roots.map(normalizeRepoPath)).size !== roots.length) {
      throw new Error(`${codeMapPath} plan ${planSlug} must not contain duplicate roots`);
    }
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
    if (entry.roots.some((rootPath) => !isRepoRelativePath(rootPath))) {
      throw new Error(`${codeMapPath} workpack ${workpackId} must declare only repository-relative roots`);
    }
    if (new Set(entry.roots.map(normalizeRepoPath)).size !== entry.roots.length) {
      throw new Error(`${codeMapPath} workpack ${workpackId} must not contain duplicate roots`);
    }
    assertPlannedExecutableRoots(root, codeMapPath, workpackId, entry, {
      field: 'plannedImplementationRoots',
      requirement: 'implementation',
      allowedExpectations: PLANNED_IMPLEMENTATION_EXPECTATIONS,
    });
    assertPlannedExecutableRoots(root, codeMapPath, workpackId, entry, {
      field: 'expectedTestRoots',
      requirement: 'tests',
      allowedExpectations: PLANNED_TEST_EXPECTATIONS,
    });
    if (entry.workspaceRequirements !== undefined) {
      const errors = workspaceRequirementSchemaErrors(entry.workspaceRequirements, {
        owner: `${codeMapPath} workpack ${workpackId}.workspaceRequirements`,
        roots: entry.roots,
      });
      if (errors.length > 0) throw new Error(errors.join('; '));
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

function workpackCodeExpectationSatisfied(
  codeExpectation,
  implementationFiles,
  testFiles,
  missingExpectedTestRoots = [],
  workspaceGaps = []
) {
  if (missingExpectedTestRoots.length > 0 || workspaceGaps.length > 0) return false;
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
    const expectedTestRoots = [...new Set((entry.expectedTestRoots ?? []).map(normalizeRepoPath))];
    const missingExpectedTestRoots = expectedTestRoots.filter((relativePath) => !pathExistsSync(root, relativePath));
    const workspaceRequirements = entry.workspaceRequirements ?? null;
    const workspaceGaps = workspaceRequirements ? workspaceRequirementGaps(root, workspaceRequirements) : [];
    const uniqueRoots = [...new Set(entry.roots.map(normalizeRepoPath))];
    const missingRoots = uniqueRoots.filter((relativePath) => !pathExistsSync(root, relativePath));
    // Deferred tooling-test cases: missing expected roots remain visible and unsatisfied;
    // existing expected roots inside declared roots scan once; existing expected roots
    // outside declared roots are scanned and counted as test evidence.
    const existingExpectedTestRoots = expectedTestRoots.filter(
      (relativePath) => !missingExpectedTestRoots.includes(relativePath)
    );
    const scanRoots = [...new Set([...uniqueRoots, ...existingExpectedTestRoots])];
    const files = [];
    for (const relativePath of scanRoots.filter((candidate) => !missingRoots.includes(candidate))) {
      files.push(...(await walkCodeFiles(root, relativePath)));
    }
    const uniqueFiles = [...new Set(files)].sort();
    const testFiles = uniqueFiles.filter(isTestPath);
    const implementationFiles = uniqueFiles.filter((file) => !isTestPath(file));
    workpacks.push({
      workpackId,
      planSlug,
      codeExpectation,
      codeExpectationSatisfied: workpackCodeExpectationSatisfied(
        codeExpectation,
        implementationFiles,
        testFiles,
        missingExpectedTestRoots,
        workspaceGaps
      ),
      roots: uniqueRoots,
      missingRoots,
      expectedTestRoots,
      missingExpectedTestRoots,
      workspaceRequirements,
      workspaceRequirementGaps: workspaceGaps,
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
  const graph = await loadAuthoritativeGraph(root);
  const validation = validateGraph(graph, { root });
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
        const implementationAuthorization = deriveImplementationAuthorization(graph, workpack, {
          root,
          states,
          workpackMapping: workpackInventory,
        });
        return {
          id: workpack.id,
          title: workpack.title,
          state: states.get(workpack.id),
          storedState: workpack.state,
          dependsOn,
          blockers,
          unlocks: relatedNodes(graph, workpack.id, 'dependents'),
          implementationAuthorization,
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
                workspaceRequirements: workpackInventory.workspaceRequirements,
                workspaceRequirementGaps: workpackInventory.workspaceRequirementGaps,
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
    schemaVersion: GRAPH_SCHEMA_VERSION,
    authority: {
      graphState: 'derived dependency, lifecycle, and completion-contract state',
      codeTestTopology: 'live files under reviewed plan-to-runtime roots',
      implementationAuthorization:
        'separate opt-in source-edit authorization; it never promotes normal READY, validation, or DONE',
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
        workspaceRequirementGaps: typeof topology === 'string' ? [] : (topology.workspaceRequirementGaps ?? []),
        dependsOn: workpack.dependsOn,
        blockers: workpack.blockers,
        unlocks: workpack.unlocks,
        implementationAuthorization: workpack.implementationAuthorization?.status ?? 'blocked',
        implementationBlockers: workpack.implementationAuthorization?.blockers ?? [
          { kind: 'internal', reason: 'implementation authorization was not derived' },
        ],
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

function overrideFieldErrors(value, allowedFields, label) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return [`${label} must be an object`];
  return Object.keys(value)
    .filter((field) => !allowedFields.has(field))
    .map((field) => `${label} has unsupported field ${field}`);
}

function requiredString(value, label) {
  return typeof value === 'string' && value.trim().length > 0 ? [] : [`${label} must be a non-empty string`];
}

function requiredStringArray(value, label, { existing = false, unique = true, root = process.cwd() } = {}) {
  const errors = [];
  if (!Array.isArray(value) || value.length === 0) return [`${label} must be a non-empty array`];
  const seen = new Set();
  for (const reference of value) {
    if (typeof reference !== 'string' || reference.trim().length === 0) {
      errors.push(`${label} entries must be non-empty strings`);
      continue;
    }
    if (unique && seen.has(reference)) errors.push(`${label} must not contain duplicates`);
    seen.add(reference);
    if (existing && !pathExistsSync(root, reference)) errors.push(`${label} references missing path ${reference}`);
  }
  return errors;
}

function assertOverrideShape(parsed, overridesPath) {
  const errors = [];
  if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) {
    throw new Error(`${overridesPath} must contain an object`);
  }
  if (parsed.schemaVersion !== GRAPH_SCHEMA_VERSION) {
    errors.push(`schemaVersion must be ${GRAPH_SCHEMA_VERSION}`);
  }
  for (const field of Object.keys(parsed)) {
    if (!OVERRIDE_TOP_LEVEL_FIELDS.has(field)) errors.push(`unsupported top-level field ${field}`);
  }
  for (const [field, allowedFields] of Object.entries(OVERRIDE_FIELDS)) {
    if (parsed[field] === undefined) continue;
    if (!Array.isArray(parsed[field])) {
      errors.push(`${field} must be an array when provided`);
      continue;
    }
    parsed[field].forEach((entry, index) => {
      errors.push(...overrideFieldErrors(entry, allowedFields, `${field}[${index}]`));
    });
  }
  if (errors.length > 0) throw new Error(`${overridesPath} is invalid: ${errors.join('; ')}`);
}

async function readOverrides(root, overridesPath) {
  const text = await readText(root, overridesPath);
  if (!text) {
    return {
      edges: [],
      workpackReviews: [],
      ambiguities: [],
      stateOverrides: [],
      proofOverrides: [],
      completionEvidenceOverrides: [],
    };
  }
  const parsed = JSON.parse(text);
  assertOverrideShape(parsed, overridesPath);
  return {
    edges: parsed.edges ?? [],
    workpackReviews: parsed.workpackReviews ?? [],
    ambiguities: parsed.ambiguities ?? [],
    stateOverrides: parsed.stateOverrides ?? [],
    proofOverrides: parsed.proofOverrides ?? [],
    completionEvidenceOverrides: parsed.completionEvidenceOverrides ?? [],
  };
}

function reviewedCompletionEvidence(override, node, repoRoot) {
  const rejectionReasons = [];
  const evidence = Array.isArray(override?.evidence) ? override.evidence : [];
  if (typeof override?.reason !== 'string' || override.reason.trim().length === 0) {
    rejectionReasons.push('reason is required');
  }
  if (evidence.length === 0) rejectionReasons.push('evidence must contain existing review paths');
  if (new Set(evidence).size !== evidence.length) rejectionReasons.push('evidence paths must not contain duplicates');
  for (const reference of evidence) {
    if (typeof reference !== 'string' || reference.trim().length === 0) {
      rejectionReasons.push('evidence entries must be non-empty strings');
    } else {
      const status = repoPathStatus(repoRoot, reference);
      if (!status.exists)
        rejectionReasons.push(
          status.reason.startsWith('missing path') ? `missing evidence ${reference}` : status.reason
        );
    }
  }

  const requirements = node?.completion?.required ?? [];
  const requirementSet = new Set(requirements);
  const unknownFields = Object.keys(override ?? {}).filter(
    (field) => !COMPLETION_EVIDENCE_FIELDS.has(field) && !requirementSet.has(field)
  );
  rejectionReasons.push(...unknownFields.map((field) => `unsupported completion evidence field ${field}`));

  const reviewed = {};
  const references = {};
  for (const requirement of requirements) {
    if (!Object.hasOwn(override ?? {}, requirement)) continue;
    const requirementReferences = override[requirement];
    if (!Array.isArray(requirementReferences) || requirementReferences.length === 0) {
      rejectionReasons.push(`${requirement} must be a non-empty array when provided`);
      continue;
    }
    const malformed = requirementReferences.filter(
      (reference) => typeof reference !== 'string' || reference.trim().length === 0
    );
    if (malformed.length > 0) rejectionReasons.push(`${requirement} references must be non-empty strings`);
    const validReferences = requirementReferences.filter(
      (reference) => typeof reference === 'string' && reference.trim().length > 0
    );
    if (new Set(validReferences).size !== validReferences.length) {
      rejectionReasons.push(`${requirement} references must not contain duplicates`);
    }
    for (const reference of validReferences) {
      const status = repoPathStatus(repoRoot, reference);
      if (!status.exists) {
        rejectionReasons.push(
          status.reason.startsWith('missing path')
            ? `${requirement}: missing reference ${reference}`
            : `${requirement}: ${status.reason}`
        );
      } else if (isPlanningDocumentEvidence(node, requirement, reference)) {
        rejectionReasons.push(`${requirement}: planning document is not executable evidence ${reference}`);
      } else {
        const problem = executableEvidenceProblem(repoRoot, node, requirement, reference);
        if (problem) rejectionReasons.push(`${requirement}: ${problem}`);
      }
    }
    reviewed[requirement] = true;
    references[requirement] = validReferences;
  }
  if (Object.keys(reviewed).length === 0) {
    rejectionReasons.push('at least one completion requirement must have reviewed evidence');
  }
  return { rejectionReasons, evidence, reviewed, references };
}

function overrideSemanticErrors(overrides, nodeById, repoRoot) {
  const errors = [];
  const workpack = (id, label) => {
    if (typeof id !== 'string' || id.trim().length === 0) {
      errors.push(`${label}.id must be a non-empty string`);
      return null;
    }
    const node = nodeById.get(id);
    if (!node) errors.push(`${label} references unknown graph node ${id}`);
    else if (node.kind !== 'workpack') errors.push(`${label} must reference a workpack node ${id}`);
    return node?.kind === 'workpack' ? node : null;
  };

  const edgeIds = new Set();
  overrides.edges.forEach((edge, index) => {
    const label = `edges[${index}]`;
    const edgeKey = `${edge.from ?? '<missing>'}->${edge.to ?? '<missing>'}:${edge.kind ?? '<missing>'}`;
    if (edgeIds.has(edgeKey)) errors.push(`${label} duplicates edge ${edgeKey}`);
    edgeIds.add(edgeKey);
    const from = workpack(edge.from, `${label}.from`);
    const to = workpack(edge.to, `${label}.to`);
    if (edge.kind !== 'depends_on') errors.push(`${label}.kind must be depends_on`);
    if (edge.confidence !== 'reviewed') errors.push(`${label}.confidence must be reviewed`);
    errors.push(...requiredString(edge.reason, `${label}.reason`));
    errors.push(...requiredStringArray(edge.evidence, `${label}.evidence`, { existing: true, root: repoRoot }));
    if (from && to && from.id === to.id) errors.push(`${label} must not self-depend`);
    if (edge.implementationGate !== undefined && !IMPLEMENTATION_GATE_VALUES.has(edge.implementationGate)) {
      errors.push(`${label}.implementationGate is unsupported`);
    }
  });

  const reviewedWorkpackIds = new Set();
  overrides.workpackReviews.forEach((review, index) => {
    const label = `workpackReviews[${index}]`;
    if (reviewedWorkpackIds.has(review.id)) errors.push(`${label} duplicates workpack review ${review.id}`);
    reviewedWorkpackIds.add(review.id);
    const node = workpack(review.id, label);
    if (!Array.isArray(review.hardDependencies)) {
      errors.push(`${label}.hardDependencies must be an explicit array`);
    } else {
      const dependencyIds = new Set();
      for (const dependency of review.hardDependencies) {
        if (typeof dependency !== 'string' || dependency.trim().length === 0) {
          errors.push(`${label}.hardDependencies entries must be non-empty strings`);
          continue;
        }
        if (dependencyIds.has(dependency)) errors.push(`${label}.hardDependencies must not contain duplicates`);
        dependencyIds.add(dependency);
        const dependencyNode = nodeById.get(dependency);
        if (!dependencyNode) errors.push(`${label}.hardDependencies references unknown graph node ${dependency}`);
        else if (dependencyNode.kind !== 'workpack') {
          errors.push(`${label}.hardDependencies target ${dependency} must be a workpack node`);
        }
        if (dependency === review.id) errors.push(`${label}.hardDependencies must not self-depend`);
      }
      if (node) {
        const actualReviewedDependencies = overrides.edges
          .filter(
            (edge) =>
              edge.kind === 'depends_on' &&
              edge.from === review.id &&
              edge.confidence === 'reviewed' &&
              Array.isArray(edge.evidence) &&
              edge.evidence.length > 0 &&
              edge.evidence.every((reference) => pathExistsSync(repoRoot, reference))
          )
          .map((edge) => edge.to)
          .sort();
        const requestedDependencies = [...review.hardDependencies].sort();
        if (JSON.stringify(actualReviewedDependencies) !== JSON.stringify(requestedDependencies)) {
          errors.push(
            `${label}.hardDependencies ${JSON.stringify(requestedDependencies)} do not match reviewed depends_on edges ${JSON.stringify(actualReviewedDependencies)}`
          );
        }
      }
    }
    errors.push(...requiredString(review.reason, `${label}.reason`));
    errors.push(...requiredStringArray(review.evidence, `${label}.evidence`, { existing: true, root: repoRoot }));
  });

  const stateIds = new Set();
  overrides.stateOverrides.forEach((override, index) => {
    const label = `stateOverrides[${index}]`;
    if (stateIds.has(override.id)) errors.push(`${label} duplicates state override ${override.id}`);
    stateIds.add(override.id);
    workpack(override.id, label);
    if (override.state !== 'validation') errors.push(`${label}.state must be validation`);
    errors.push(...requiredString(override.reason, `${label}.reason`));
    if (override.statusText !== undefined) errors.push(...requiredString(override.statusText, `${label}.statusText`));
    if (override.needsReview !== undefined && typeof override.needsReview !== 'boolean') {
      errors.push(`${label}.needsReview must be boolean`);
    }
    errors.push(...requiredStringArray(override.evidence, `${label}.evidence`, { existing: true, root: repoRoot }));
  });

  const proofIds = new Set();
  overrides.proofOverrides.forEach((override, index) => {
    const label = `proofOverrides[${index}]`;
    if (proofIds.has(override.id)) errors.push(`${label} duplicates proof override ${override.id}`);
    proofIds.add(override.id);
    workpack(override.id, label);
    errors.push(...requiredString(override.reason, `${label}.reason`));
    errors.push(...requiredStringArray(override.proof, `${label}.proof`, { existing: true, root: repoRoot }));
    errors.push(...requiredStringArray(override.evidence, `${label}.evidence`, { existing: true, root: repoRoot }));
    if (override.satisfiesExpected !== undefined && typeof override.satisfiesExpected !== 'boolean') {
      errors.push(`${label}.satisfiesExpected must be boolean`);
    }
  });

  const completionIds = new Set();
  overrides.completionEvidenceOverrides.forEach((override, index) => {
    const label = `completionEvidenceOverrides[${index}]`;
    if (completionIds.has(override.id)) errors.push(`${label} duplicates completion evidence override ${override.id}`);
    completionIds.add(override.id);
    const node = workpack(override.id, label);
    if (!node) return;
    const review = reviewedCompletionEvidence(override, node, repoRoot);
    errors.push(...review.rejectionReasons.map((reason) => `${label}: ${reason}`));
  });

  const ambiguityScopes = new Set();
  overrides.ambiguities.forEach((ambiguity, index) => {
    const label = `ambiguities[${index}]`;
    if (ambiguityScopes.has(ambiguity.scope)) errors.push(`${label} duplicates ambiguity ${ambiguity.scope}`);
    ambiguityScopes.add(ambiguity.scope);
    errors.push(...requiredString(ambiguity.scope, `${label}.scope`));
    errors.push(...requiredString(ambiguity.reason, `${label}.reason`));
    errors.push(...requiredString(ambiguity.nextAction, `${label}.nextAction`));
  });

  return errors;
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
  const codeMap = (await readText(repoRoot, CODE_MAP_PATH)) ? await loadCodeMap(repoRoot) : { workpacks: {} };
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
  const nodeById = new Map(nodes.map((node) => [node.id, node]));
  for (const [workpackId, mapping] of Object.entries(codeMap.workpacks ?? {})) {
    if (
      !Array.isArray(mapping.plannedImplementationRoots) &&
      !Array.isArray(mapping.expectedTestRoots) &&
      mapping.workspaceRequirements === undefined
    )
      continue;
    const node = nodeById.get(workpackId);
    if (!node || node.kind !== 'workpack') {
      throw new Error(`${CODE_MAP_PATH} planned executable owner is not an imported workpack: ${workpackId}`);
    }
    const plannedImplementationRoots = (mapping.plannedImplementationRoots ?? []).map(normalizeRepoPath);
    const expectedTestRoots = (mapping.expectedTestRoots ?? []).map(normalizeRepoPath);
    const references = { ...node.completion.references };
    if (plannedImplementationRoots.length > 0) references.implementation = [];
    const expected = { ...(node.completion.expected ?? {}) };
    if (plannedImplementationRoots.length > 0) expected.implementation = plannedImplementationRoots;
    if (expectedTestRoots.length > 0) expected.tests = expectedTestRoots;
    node.completion = {
      ...node.completion,
      references,
      expected,
    };
    node.metadata = {
      ...node.metadata,
      plannedSourceExpectation: {
        source: CODE_MAP_PATH,
        roots: plannedImplementationRoots,
        testRoots: expectedTestRoots,
      },
    };
    if (mapping.workspaceRequirements !== undefined) {
      node.completion = {
        ...node.completion,
        workspaceRequirements: mapping.workspaceRequirements,
      };
    }
  }
  const overrideErrors = overrideSemanticErrors(overrides, nodeById, repoRoot);
  if (overrideErrors.length > 0) {
    throw new Error(`${overridesPath} is invalid: ${overrideErrors.join('; ')}`);
  }
  for (const edge of overrides.edges) {
    edges.push(edge);
  }
  // A workpack review is deliberately narrower than a completion override.
  // It authorizes dependency/readiness derivation for one exact workpack only;
  // it never promotes code, tests, proof, or a stale DONE label.
  for (const review of overrides.workpackReviews) {
    const requestedSorted = [...review.hardDependencies].sort();
    const node = nodeById.get(review.id);
    node.metadata = {
      ...node.metadata,
      dependencyConfidence: 'reviewed',
      needsReview: false,
      dependencyReview: {
        hardDependencies: requestedSorted,
        evidence: review.evidence,
        reason: review.reason,
      },
    };
  }
  for (const override of overrides.stateOverrides) {
    const node = nodeById.get(override.id);
    node.metadata = {
      ...node.metadata,
      stateOverride: {
        reason: override.reason,
        evidence: override.evidence,
      },
    };
    node.state = override.state;
    node.lifecycleState = override.state;
    node.metadata = {
      ...node.metadata,
      ...(override.statusText ? { statusText: override.statusText } : {}),
      needsReview: override.needsReview === true || node.metadata?.dependencyReviewRejected ? true : false,
    };
  }
  for (const override of overrides.proofOverrides) {
    const node = nodeById.get(override.id);
    node.completion.references = {
      ...node.completion.references,
      proof: override.proof,
    };
    node.metadata = {
      ...node.metadata,
      proofOverride: {
        reason: override.reason,
        evidence: override.evidence,
        satisfiesExpected: override.satisfiesExpected === true,
      },
    };
  }
  const reviewedCompletionIds = new Set();
  for (const override of overrides.completionEvidenceOverrides) {
    const node = nodeById.get(override.id);
    if (reviewedCompletionIds.has(override.id))
      throw new Error(`${overridesPath} has duplicate completion evidence override ${override.id}`);
    reviewedCompletionIds.add(override.id);
    const review = reviewedCompletionEvidence(override, node, repoRoot);
    node.completion = {
      ...node.completion,
      reviewed: { ...(node.completion.reviewed ?? {}), ...review.reviewed },
      references: { ...(node.completion.references ?? {}), ...review.references },
    };
    node.metadata = {
      ...node.metadata,
      completionEvidenceOverride: {
        reason: override.reason,
        evidence: review.evidence,
        requirements: Object.keys(review.reviewed).sort(),
      },
    };
  }
  // A documented DONE status is only a starting hint.  Recompute it against
  // the completion contract before writing the repo-owned graph so stale
  // checklist prose cannot leave a false DONE node behind.
  for (const node of nodes) {
    if (node.kind !== 'workpack' || node.state !== 'done') continue;
    // Bootstrap persists source-derived graph state. Generated proof output is
    // live validation evidence and must not change the checked-in graph when
    // an ignored artifact happens to exist in one checkout.
    const gaps = completionGaps(repoRoot, node, { includeExpectedArtifacts: false });
    if (gaps.length === 0) continue;
    node.state = 'validation';
    node.lifecycleState = 'validation';
    node.metadata = {
      ...node.metadata,
      completionGaps: gaps,
      sourceStatusText: node.metadata.statusText,
      statusText: `Validation — completion contract gaps: ${gaps.join('; ')}`,
      // A rejected workpack dependency review remains a migration blocker;
      // completion-contract demotion must not accidentally clear it.
      needsReview: node.metadata?.dependencyReviewRejected ? true : false,
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
      implementationPhase:
        'opt-in source-edit authorization only; normal READY, tests, proof, service activation, PR readiness, and DONE are unchanged',
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
  return (graph.edges ?? []).filter((edge) => edge && typeof edge === 'object' && edge.kind === 'depends_on');
}

function nodeMap(graph) {
  return new Map((graph.nodes ?? []).filter((node) => node && typeof node === 'object').map((node) => [node.id, node]));
}

function completionRequirementGaps(
  root,
  node,
  requirement,
  { includeExpectedArtifacts = true, requireDeclared = false } = {}
) {
  const declared = node.completion?.required?.includes(requirement) === true;
  if (!declared && requireDeclared) {
    return [`${requirement}: completion requirement is not declared`];
  }
  if (!node.completion) return [];
  const gaps = [];
  const references = node.completion.references?.[requirement] ?? [];
  const expected = node.completion.expected?.[requirement] ?? [];
  if (declared && node.completion.reviewed?.[requirement] !== true) {
    gaps.push(`${requirement}: reviewed evidence is not recorded`);
  }
  if (declared && references.length === 0 && expected.length === 0) {
    gaps.push(`${requirement}: no reference or expected artifact is declared`);
    return gaps;
  }
  for (const reference of references) {
    const exists = pathExistsSync(root, reference);
    if (!exists) {
      gaps.push(`${requirement}: missing reference ${reference}`);
      continue;
    }
    if (node.completion.reviewed?.[requirement] === true) {
      if (isPlanningDocumentEvidence(node, requirement, reference)) {
        gaps.push(`${requirement}: planning document is not executable evidence ${reference}`);
      } else {
        const problem = executableEvidenceProblem(root, node, requirement, reference);
        if (problem) gaps.push(`${requirement}: ${problem}`);
      }
    }
  }
  if (includeExpectedArtifacts) {
    for (const reference of expected) {
      if (durableProofSatisfiesExpected(root, node, requirement)) continue;
      if (['implementation', 'tests'].includes(requirement)) {
        const problem = executableEvidenceProblem(root, node, requirement, reference);
        if (problem?.startsWith('missing path ')) gaps.push(`${requirement}: missing expected artifact ${reference}`);
        else if (problem) gaps.push(`${requirement}: invalid expected artifact ${reference}: ${problem}`);
      } else if (!pathExistsSync(root, reference)) gaps.push(`${requirement}: missing expected artifact ${reference}`);
    }
  }
  return gaps;
}

export function completionGaps(root, node, { includeExpectedArtifacts = true } = {}) {
  const gaps = [];
  if (node.completion) {
    const requirements = new Set([
      ...(node.completion.required ?? []),
      ...Object.keys(node.completion.references ?? {}),
      ...Object.keys(node.completion.expected ?? {}),
    ]);
    gaps.push(
      ...[...requirements].flatMap((requirement) =>
        completionRequirementGaps(root, node, requirement, { includeExpectedArtifacts })
      )
    );
  }
  gaps.push(...workspaceRequirementGaps(root, node));
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
  return normalized.toLowerCase().startsWith(`docs/plans/${planSlug.toLowerCase()}/`);
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

function implementationDependencyBlocker(graph, edge, states, root) {
  const dependency = nodeMap(graph).get(edge.to);
  if (!dependency) {
    return { kind: 'dependency', id: edge.to, gate: edge.implementationGate ?? 'done', state: 'missing' };
  }
  if (edge.implementationGate === IMPLEMENTATION_GATE) {
    const gaps = completionRequirementGaps(root, dependency, 'implementation', { requireDeclared: true });
    gaps.push(...workspaceRequirementGaps(root, dependency));
    if (
      gaps.length === 0 &&
      !dependency.metadata?.completionEvidenceOverride?.requirements?.includes('implementation')
    ) {
      gaps.push('implementation: hardened reviewed completion evidence is not recorded');
    }
    if (gaps.length === 0) return null;
    return {
      kind: 'dependency',
      id: dependency.id,
      gate: IMPLEMENTATION_GATE,
      state: states.get(dependency.id) ?? 'unknown',
      gaps,
    };
  }
  const state = states.get(dependency.id) ?? 'unknown';
  return state === 'done' ? null : { kind: 'dependency', id: dependency.id, gate: 'done', state, gaps: [] };
}

function deriveImplementationAuthorization(
  graph,
  node,
  { root = process.cwd(), states = deriveStates(graph, { root }), workpackMapping = null } = {}
) {
  if (!node || node.kind !== 'workpack') {
    return {
      phase: 'implementation',
      status: 'not-applicable',
      authorized: false,
      blockers: [{ kind: 'node', reason: 'implementation authorization requires a workpack node' }],
    };
  }
  const codeExpectation = workpackMapping?.codeExpectation ?? 'code-and-tests';
  if (!workpackMapping || codeExpectation !== 'code-and-tests') {
    return {
      phase: 'implementation',
      status: workpackMapping ? 'not-applicable' : 'blocked',
      authorized: false,
      blockers: [
        workpackMapping
          ? { kind: 'ownership', reason: `workpack expects ${codeExpectation}, not implementation source` }
          : { kind: 'ownership', reason: 'reviewed workpack code ownership is not mapped' },
      ],
    };
  }
  if ((node.lifecycleState ?? node.state) === 'done') {
    return { phase: 'implementation', status: 'complete', authorized: false, blockers: [] };
  }
  const implementationGaps = completionRequirementGaps(root, node, 'implementation', { requireDeclared: true });
  const workspaceGaps = workspaceRequirementGaps(root, node);
  if (implementationGaps.length === 0 && workspaceGaps.length === 0) {
    return { phase: 'implementation', status: 'complete', authorized: false, blockers: [] };
  }

  const blockers = [];
  if (node.metadata?.needsReview) {
    blockers.push({ kind: 'migration-review', reason: 'workpack dependency/readiness review is incomplete' });
  }
  const lifecycleState = node.lifecycleState ?? node.state;
  if (['blocked', 'failed', 'paused'].includes(lifecycleState)) {
    blockers.push({ kind: 'lifecycle', state: lifecycleState, reason: 'workpack lifecycle does not authorize edits' });
  }
  for (const edge of dependencyEdges(graph).filter((candidate) => candidate.from === node.id)) {
    const blocker = implementationDependencyBlocker(graph, edge, states, root);
    if (blocker) blockers.push(blocker);
  }
  return {
    phase: 'implementation',
    status: blockers.length === 0 ? 'authorized' : 'blocked',
    authorized: blockers.length === 0,
    blockers,
    gaps: [...implementationGaps, ...workspaceGaps],
  };
}

function authorizeImplementationPhase(graph, node, options = {}) {
  const validation = validateGraph(graph, {
    root: options.root ?? process.cwd(),
    // A direct phase authorization may intentionally inspect a READY row
    // whose normal dependency derivation is blocked. The phase gate must
    // validate graph shape and reviewed edge authority without treating that
    // expected phase distinction as source drift; async query paths compare
    // against regenerated source via loadAuthoritativeGraph.
    allowStoredStateDrift: true,
  });
  if (!validation.ok)
    throw new Error(`Cannot authorize implementation from an invalid graph: ${validation.errors.join('; ')}`);
  return deriveImplementationAuthorization(graph, node, options);
}

async function nextImplementationWork(graph, { root = process.cwd(), scope } = {}) {
  const authoritative = await loadAuthoritativeGraph(root, graph);
  const states = deriveStates(authoritative, { root });
  const inventory = await buildCodeInventory({ root, scope });
  const inventoryByWorkpack = new Map(inventory.workpacks.map((workpack) => [workpack.workpackId, workpack]));
  const rows = scopeNodes(authoritative, scope)
    .filter((node) => node.kind === 'workpack')
    .map((node) => ({
      node,
      authorization: deriveImplementationAuthorization(authoritative, node, {
        root,
        states,
        workpackMapping: inventoryByWorkpack.get(node.id) ?? null,
      }),
    }));
  const authorized = rows.filter((row) => row.authorization.authorized);
  return {
    scope: scope ?? 'GOAL-ocentra-parent',
    phase: 'implementation',
    authorized,
    rows,
    recommendation:
      authorized.length > 0
        ? 'Edit only the authorized implementation scope; normal READY, tests, proof, PR readiness, and DONE remain blocked.'
        : 'No implementation-only workpack is authorized; inspect phase blockers without bypassing normal READY.',
  };
}

async function explainImplementationAuthorization(graph, nodeId, { root = process.cwd() } = {}) {
  const authoritative = await loadAuthoritativeGraph(root, graph);
  const node = nodeMap(authoritative).get(nodeId);
  if (!node) {
    return {
      node: null,
      phase: 'implementation',
      status: 'blocked',
      authorized: false,
      blockers: [{ kind: 'node', reason: `unknown node ${nodeId}` }],
    };
  }
  const inventory = await buildCodeInventory({ root });
  const workpackMapping = inventory.workpacks.find((workpack) => workpack.workpackId === node.id) ?? null;
  const authorization = deriveImplementationAuthorization(authoritative, node, {
    root,
    workpackMapping,
  });
  return { node, ...authorization };
}

/** Narrow source-edit phase API; it never mutates normal READY or DONE state. */
export const implementationPhase = Object.freeze({
  authorize: authorizeImplementationPhase,
  explain: explainImplementationAuthorization,
  next: nextImplementationWork,
});

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
  function visitDependency(id, pathStack) {
    if (visiting.has(id)) {
      const start = pathStack.indexOf(id);
      cycles.push([...pathStack.slice(start), id]);
      return;
    }
    if (visited.has(id)) return;
    visiting.add(id);
    for (const dependency of adjacency.get(id) ?? []) visitDependency(dependency, [...pathStack, id]);
    visiting.delete(id);
    visited.add(id);
  }
  for (const node of graph.nodes) visitDependency(node.id, []);
  return cycles;
}

export function validateGraph(graph, { root = process.cwd(), allowStoredStateDrift = false } = {}) {
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
    if (!node || typeof node !== 'object' || Array.isArray(node)) {
      errors.push('node entries must be objects');
      continue;
    }
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
    const workspaceRequirements = node.completion?.workspaceRequirements ?? node.metadata?.workspaceRequirements;
    if (workspaceRequirements !== undefined) {
      const workspaceErrors = workspaceRequirementSchemaErrors(workspaceRequirements, {
        owner: `${node.id} completion.workspaceRequirements`,
      });
      errors.push(...workspaceErrors);
    }
    if (node.dependsOn !== undefined && !Array.isArray(node.dependsOn)) {
      errors.push(`${node.id} dependsOn must be an array`);
    }
    const declaredDependencies = new Set();
    for (const dependency of Array.isArray(node.dependsOn) ? node.dependsOn : []) {
      if (declaredDependencies.has(dependency)) errors.push(`${node.id} contains duplicate dependency ${dependency}`);
      declaredDependencies.add(dependency);
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
      if (!Array.isArray(references)) {
        errors.push(`${node.id} ${requirement} references must be an array`);
        continue;
      }
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
        if (!Array.isArray(references)) {
          errors.push(`${node.id} ${requirement} expected artifacts must be an array`);
          continue;
        }
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
    for (const reason of node.metadata?.completionEvidenceOverrideRejected ?? []) {
      errors.push(`${node.id} completion evidence override rejected: ${reason}`);
    }
  }
  const edgeIds = new Set();
  for (const edge of graph.edges) {
    if (!edge || typeof edge !== 'object' || Array.isArray(edge)) {
      errors.push('edge entries must be objects');
      continue;
    }
    for (const field of Object.keys(edge)) {
      if (!GRAPH_EDGE_FIELDS.has(field)) errors.push(`edge ${edge.from} -> ${edge.to} has unsupported field ${field}`);
    }
    const edgeId = `${edge.from ?? '<missing>'}->${edge.to ?? '<missing>'}:${edge.kind ?? '<missing>'}`;
    if (edgeIds.has(edgeId)) errors.push(`duplicate edge: ${edgeId}`);
    edgeIds.add(edgeId);
    if (!map.has(edge.from)) errors.push(`edge references missing from node ${edge.from}`);
    if (!map.has(edge.to)) errors.push(`edge references missing to node ${edge.to}`);
    if (!edge.kind) errors.push(`edge ${edge.from} -> ${edge.to} is missing kind`);
    else if (!EDGE_KINDS.has(edge.kind))
      errors.push(`edge ${edge.from} -> ${edge.to} has unsupported kind ${edge.kind}`);
    if (edge.kind !== 'depends_on' && edge.implementationGate !== undefined) {
      errors.push(`${edge.from} -> ${edge.to} implementationGate is only valid on depends_on edges`);
    }
    if (edge.kind === 'depends_on') {
      const dependent = map.get(edge.from);
      if (dependent && !(dependent.dependsOn ?? []).includes(edge.to)) {
        errors.push(`${edge.from} has depends_on edge ${edge.to} without a matching dependsOn entry`);
      }
      if (edge.confidence !== 'reviewed') {
        errors.push(`${edge.from} -> ${edge.to} depends_on edge must have confidence=reviewed`);
      }
      if (typeof edge.reason !== 'string' || edge.reason.trim().length === 0) {
        errors.push(`${edge.from} -> ${edge.to} depends_on edge requires a reviewed reason`);
      }
      const evidence = Array.isArray(edge.evidence) ? edge.evidence : [];
      if (evidence.length === 0) errors.push(`${edge.from} -> ${edge.to} depends_on edge is missing evidence`);
      if (new Set(evidence).size !== evidence.length) {
        errors.push(`${edge.from} -> ${edge.to} depends_on edge evidence must not contain duplicates`);
      }
      for (const reference of evidence) {
        if (!pathExistsSync(root, reference)) {
          errors.push(`${edge.from} -> ${edge.to} dependency evidence is missing: ${reference}`);
        }
      }
      if (edge.implementationGate !== undefined && !IMPLEMENTATION_GATE_VALUES.has(edge.implementationGate)) {
        errors.push(`${edge.from} -> ${edge.to} has unsupported implementationGate ${String(edge.implementationGate)}`);
      }
      if (edge.implementationGate === IMPLEMENTATION_GATE) {
        if (map.get(edge.from)?.kind !== 'workpack') {
          errors.push(`${edge.from} -> ${edge.to} implementationGate dependent must be a workpack`);
        }
        if (map.get(edge.to)?.kind !== 'workpack') {
          errors.push(`${edge.from} -> ${edge.to} implementationGate target must be a workpack`);
        }
        if (typeof edge.reason !== 'string' || edge.reason.trim().length === 0) {
          errors.push(`${edge.from} -> ${edge.to} implementationGate requires a reviewed reason`);
        }
      }
    }
  }
  if (errors.length > 0) return { ok: false, errors, warnings };
  for (const cycle of detectCycles(graph)) errors.push(`dependency cycle: ${cycle.join(' -> ')}`);
  const states = deriveStates(graph, { root });
  for (const node of graph.nodes) {
    if (!allowStoredStateDrift && node.state === 'ready' && states.get(node.id) === 'blocked') {
      errors.push(`${node.id} is stored ready but derived blocked`);
    }
  }
  return { ok: errors.length === 0, errors, warnings };
}

function pathExistsSync(root, relativePath) {
  return repoPathStatus(root, relativePath).exists;
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

async function loadAuthoritativeGraph(root, candidate = null) {
  const checkedIn = candidate ?? (await loadGraph(root));
  const validation = validateGraph(checkedIn, { root });
  if (!validation.ok) {
    throw new Error(`Cannot query an invalid graph: ${validation.errors.join('; ')}`);
  }
  const generated = await buildBootstrapGraph({ root });
  const generatedValidation = validateGraph(generated, { root });
  if (!generatedValidation.ok) {
    throw new Error(`Cannot query an invalid generated graph: ${generatedValidation.errors.join('; ')}`);
  }
  const drift = graphSourceDrift(checkedIn, generated);
  if (drift.length > 0) throw new Error(drift.join('; '));
  return checkedIn;
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
