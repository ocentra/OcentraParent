import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

import { resolveScopedFiles } from './check-architecture-scope.mjs';

const repoRoot = process.cwd();
const findings = [];
const scriptName = 'node scripts/check-required-tests.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];

function toPosix(path) {
  return path.split('\\').join('/');
}

function childDirs(path) {
  if (!existsSync(path)) {
    return [];
  }
  return readdirSync(path, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => join(path, entry.name));
}

function hasFile(path, predicate) {
  if (!existsSync(path)) {
    return false;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    return readdirSync(path).some((entry) => hasFile(join(path, entry), predicate));
  }
  return stats.isFile() && predicate(path);
}

function checkNodeWorkspace(path) {
  const packageJsonPath = join(path, 'package.json');
  const srcPath = join(path, 'src');
  if (!existsSync(packageJsonPath) || !existsSync(srcPath)) {
    return;
  }

  const manifest = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
  const hasTests = hasFile(join(path, 'tests'), (file) => /\.(?:test|spec)\.ts$/u.test(file));
  if (!hasTests) {
    findings.push(`${manifest.name ?? toPosix(relative(repoRoot, path))}: missing tests/*.test.ts`);
  }
}

function checkRustCrate(path) {
  const cargoPath = join(path, 'Cargo.toml');
  if (!existsSync(cargoPath)) {
    return;
  }

  const hasInlineTestModule = hasFile(join(path, 'src'), (file) => {
    if (!file.endsWith('.rs')) {
      return false;
    }
    return readFileSync(file, 'utf8').includes('#[cfg(test)]');
  });
  const hasIntegrationTest = hasFile(join(path, 'tests'), (file) => file.endsWith('.rs'));
  if (!hasInlineTestModule && !hasIntegrationTest) {
    findings.push(`${toPosix(relative(repoRoot, path))}: missing Rust unit or integration tests`);
  }
}

function collectFullTargets() {
  const nodeWorkspaces = [];
  const rustCrates = [];

  for (const workspaceRoot of ['packages', 'apps']) {
    for (const path of childDirs(join(repoRoot, workspaceRoot))) {
      nodeWorkspaces.push(path);
    }
  }

  for (const path of childDirs(join(repoRoot, 'crates'))) {
    rustCrates.push(path);
  }

  return { nodeWorkspaces, rustCrates };
}

function collectScopedTargets(rawArgs) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['apps', 'packages', 'crates'],
    acceptPath: () => true,
  });

  if (scope.mode === 'skip') {
    return { nodeWorkspaces: [], rustCrates: [] };
  }

  const nodeWorkspaces = new Set();
  const rustCrates = new Set();

  for (const filePath of scope.files) {
    const segments = filePath.split('/');
    if (segments.length < 2) {
      continue;
    }

    const workspacePath = join(repoRoot, segments[0], segments[1]);
    if (segments[0] === 'packages' || segments[0] === 'apps') {
      nodeWorkspaces.add(workspacePath);
      continue;
    }

    if (segments[0] === 'crates') {
      rustCrates.add(workspacePath);
    }
  }

  return {
    nodeWorkspaces: [...nodeWorkspaces],
    rustCrates: [...rustCrates],
  };
}

export function main(rawArgs = process.argv.slice(2)) {
  findings.length = 0;
  const { nodeWorkspaces, rustCrates } = rawArgs.length === 0 ? collectFullTargets() : collectScopedTargets(rawArgs);

  for (const path of nodeWorkspaces) {
    checkNodeWorkspace(path);
  }

  for (const path of rustCrates) {
    checkRustCrate(path);
  }

  if (findings.length > 0) {
    console.error('Every source workspace must have tests from the beginning.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  if (rawArgs.length === 0) {
    console.log('Required test scaffold is present for all source workspaces.');
    return;
  }

  console.log(
    `Required test scaffold is present for ${nodeWorkspaces.length} node workspace(s) and ${rustCrates.length} Rust crate(s).`
  );
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
