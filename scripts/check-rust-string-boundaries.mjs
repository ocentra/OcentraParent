import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const sourceRoots = ['crates/agent-core/src', 'crates/agent-service/src'];
const ignoredPathParts = new Set(['ocentra-ledger', 'target']);
const findings = [];

function toPosix(path) {
  return path.split('\\').join('/');
}

function shouldIgnorePath(path) {
  return toPosix(relative(repoRoot, path))
    .split('/')
    .some((part) => ignoredPathParts.has(part));
}

function walk(path, files) {
  if (!existsSync(path) || shouldIgnorePath(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stats.isFile() && path.endsWith('.rs')) {
    files.push(path);
  }
}

function inspectFile(path) {
  const relativePath = toPosix(relative(repoRoot, path));
  const lines = readFileSync(path, 'utf8').split(/\r?\n/u);

  lines.forEach((line, index) => {
    if (line.includes('env!(') || line.includes('#[tokio::main')) {
      return;
    }
    if (/"(?:[^"\\]|\\.)*"/u.test(line)) {
      findings.push({ path: relativePath, line: index + 1, text: line.trim() });
    }
  });
}

const files = [];
for (const root of sourceRoots) {
  walk(join(repoRoot, root), files);
}

for (const file of files) {
  inspectFile(file);
}

if (findings.length > 0) {
  console.error(
    'Rust service/core source cannot contain inline string literals. Move runtime values into agent-protocol constants.'
  );
  for (const finding of findings) {
    console.error(`${finding.path}:${finding.line} ${finding.text}`);
  }
  process.exit(1);
}

console.log(`No inline Rust service/core string literals found across ${files.length} checked files.`);
