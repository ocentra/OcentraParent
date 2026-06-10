import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const sourceRoots = ['apps', 'packages', 'scripts'];
const standaloneFiles = ['package.json', 'eslint.config.js'];
const ignoredPathParts = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'ocentra-ledger', 'target']);
const textExtensions = new Set(['.cjs', '.js', '.json', '.jsx', '.mjs', '.ts', '.tsx']);
const forbiddenSourcePatterns = [
  { label: 'direct zod import', pattern: /from\s+['"]zod['"]|require\(\s*['"]zod['"]\s*\)/u },
  { label: 'Zod resolver', pattern: /\bzodResolver\b/u },
  {
    label: 'Zod public type/API',
    pattern: /\bZod(?:Error|Issue|Type|Schema|Object|String|Number|Boolean|Array|Record|Union)\b/u,
  },
  { label: 'stale schema/zod path', pattern: /schema\/zod|schema\\zod/u },
];
const forbiddenDependencyNames = new Set(['zod', 'zod-to-json-schema', 'zod-validation-error']);
const findings = [];

function toPosix(path) {
  return path.split('\\').join('/');
}

function shouldIgnorePath(path) {
  return toPosix(relative(repoRoot, path))
    .split('/')
    .some((part) => ignoredPathParts.has(part));
}

function extensionOf(path) {
  const index = path.lastIndexOf('.');
  return index === -1 ? '' : path.slice(index);
}

function walk(path, files) {
  if (!existsSync(path) || shouldIgnorePath(path)) {
    return;
  }

  const stat = statSync(path);
  if (stat.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stat.isFile() && textExtensions.has(extensionOf(path))) {
    files.push(path);
  }
}

function lineNumberFor(text, index) {
  return text.slice(0, index).split(/\r?\n/u).length;
}

function inspectSourceFile(path) {
  const relativePath = toPosix(relative(repoRoot, path));
  if (relativePath === 'scripts/check-no-zod-source.mjs') {
    return;
  }

  const text = readFileSync(path, 'utf8');
  for (const rule of forbiddenSourcePatterns) {
    const match = rule.pattern.exec(text);
    if (match) {
      findings.push({ path: relativePath, line: lineNumberFor(text, match.index), reason: rule.label });
    }
  }
}

function inspectPackageManifest(path) {
  const parsed = JSON.parse(readFileSync(path, 'utf8'));
  for (const section of ['dependencies', 'devDependencies', 'peerDependencies', 'optionalDependencies']) {
    const dependencies = parsed[section];
    if (dependencies == null || typeof dependencies !== 'object') {
      continue;
    }
    for (const name of Object.keys(dependencies)) {
      if (forbiddenDependencyNames.has(name)) {
        findings.push({
          path: toPosix(relative(repoRoot, path)),
          line: 1,
          reason: `direct ${name} dependency in ${section}`,
        });
      }
    }
  }
}

const files = [];
for (const root of sourceRoots) {
  walk(join(repoRoot, root), files);
}
for (const file of standaloneFiles) {
  const path = join(repoRoot, file);
  if (existsSync(path)) {
    files.push(path);
  }
}

for (const file of files) {
  if (file.endsWith('package.json')) {
    inspectPackageManifest(file);
  }
  inspectSourceFile(file);
}

if (findings.length > 0) {
  console.error('Direct Zod usage is not allowed. Use Effect Schema through domain-owned schemas.');
  for (const finding of findings) {
    console.error(`${finding.path}:${finding.line} ${finding.reason}`);
  }
  process.exit(1);
}

console.log(`No direct Zod source usage found across ${files.length} checked files.`);
