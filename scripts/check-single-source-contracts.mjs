import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';

const repoRoot = process.cwd();
const configPath = 'scripts/check-single-source-contracts.json';
const sourceExtension = /\.(?:rs|ts|tsx|mjs|cjs|js|json|md|yml|yaml)$/u;
const ignoredSegments = new Set([
  '.git',
  '.hub',
  '.turbo',
  'coverage',
  'dist',
  'node_modules',
  'ocentra-ledger',
  'output',
  'target',
  'test-results',
]);
const findings = [];

const config = JSON.parse(readFileSync(join(repoRoot, configPath), 'utf8'));

function toPosix(path) {
  return path.split(sep).join('/');
}

function relativePath(path) {
  return toPosix(relative(repoRoot, path));
}

function shouldSkip(path, allowedPaths) {
  const pathText = relativePath(path);
  if (allowedPaths.has(pathText)) {
    return true;
  }
  return pathText.split('/').some((segment) => ignoredSegments.has(segment));
}

function walk(path, contract, guardedValues) {
  if (!existsSync(path) || shouldSkip(path, contract.allowedPaths)) {
    return;
  }
  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), contract, guardedValues);
    }
    return;
  }
  if (!stats.isFile() || !sourceExtension.test(path)) {
    return;
  }
  inspectFile(path, contract, guardedValues);
}

function inspectFile(path, contract, guardedValues) {
  const pathText = relativePath(path);
  const text = readFileSync(path, 'utf8');
  for (const value of guardedValues) {
    if (text.includes(value.text)) {
      findings.push(
        `${pathText}: copied ${contract.name}.${value.name} ${value.text}; import or derive from ${contract.ownerPath}`
      );
    }
  }
}

function valueAtPath(source, jsonPath) {
  let value = source;
  for (const segment of jsonPath.split('.')) {
    if (value === null || typeof value !== 'object' || !(segment in value)) {
      throw new Error(`${jsonPath} is missing`);
    }
    value = value[segment];
  }
  return value;
}

function loadContract(rawContract) {
  const ownerPath = rawContract.ownerPath;
  const source = JSON.parse(readFileSync(join(repoRoot, ownerPath), 'utf8'));
  const values = rawContract.values.map(({ name, jsonPath }) => {
    const text = valueAtPath(source, jsonPath);
    if (typeof text !== 'string' || text.length === 0) {
      throw new Error(`${ownerPath}: ${name} at ${jsonPath} must be a non-empty string`);
    }
    return { name, text };
  });
  return {
    ...rawContract,
    allowedPaths: new Set([ownerPath, ...(rawContract.allowedPaths ?? [])]),
    values,
  };
}

for (const rawContract of config.contracts ?? []) {
  const contract = loadContract(rawContract);
  for (const root of contract.scanRoots) {
    walk(join(repoRoot, root), contract, contract.values);
  }
}

if (findings.length > 0) {
  console.error('Single-source contract values must not be copied across repo source.');
  console.error(`Declare owned values in ${configPath}; import or derive them from the owner contract.`);
  for (const finding of findings) {
    console.error(finding);
  }
  process.exit(1);
}

console.log(`Single-source contract check passed for ${(config.contracts ?? []).length} declared contract(s).`);
