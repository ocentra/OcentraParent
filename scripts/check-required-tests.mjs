import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const findings = [];

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

for (const workspaceRoot of ['packages', 'apps']) {
  for (const path of childDirs(join(repoRoot, workspaceRoot))) {
    checkNodeWorkspace(path);
  }
}

for (const path of childDirs(join(repoRoot, 'crates'))) {
  checkRustCrate(path);
}

if (findings.length > 0) {
  console.error('Every source workspace must have tests from the beginning.');
  for (const finding of findings) {
    console.error(finding);
  }
  process.exit(1);
}

console.log('Required test scaffold is present for all source workspaces.');
