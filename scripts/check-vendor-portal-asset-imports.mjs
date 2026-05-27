import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const checkedRoot = join(repoRoot, 'vendor', 'ocentra-parent-core-ui', 'AppPages', 'ParentPortal');
const sourceExtension = /\.(?:ts|tsx)$/u;
const forbiddenPattern = /from\s+['"]@ocentra-parent\/portal-assets\//u;
const findings = [];

function toPosix(path) {
  return path.split('\\').join('/');
}

function walk(path) {
  if (!existsSync(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry));
    }
    return;
  }

  if (!stats.isFile() || !sourceExtension.test(path)) {
    return;
  }

  inspectFile(path);
}

function inspectFile(path) {
  const text = readFileSync(path, 'utf8');
  const lines = text.split(/\r?\n/u);
  lines.forEach((line, index) => {
    if (forbiddenPattern.test(line)) {
      findings.push({
        path: toPosix(relative(repoRoot, path)),
        line: index + 1,
        text: line.trim(),
      });
    }
  });
}

walk(checkedRoot);

if (findings.length > 0) {
  console.error('ParentPortal vendor page source must import portal assets through local shims, not app aliases.');
  for (const finding of findings) {
    console.error(`${finding.path}:${finding.line} ${finding.text}`);
  }
  process.exit(1);
}

console.log('No ParentPortal vendor page imports use portal asset aliases.');
