import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const repoRoot = process.cwd();
const checkedRoot = join(repoRoot, 'vendor', 'ocentra-parent-core-ui', 'AppPages', 'ParentPortal');
const sourceExtension = /\.(?:ts|tsx)$/u;
const forbiddenPattern = /from\s+['"]@ocentra-parent\/portal-assets\//u;
const findings = [];
const scriptName = 'node scripts/check-vendor-portal-asset-imports.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];

function toPosix(path) {
  return path.split('\\').join('/');
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

function collectFullFiles() {
  const files = [];
  walk(checkedRoot, files);
  return files;
}

function walk(path, files) {
  if (!existsSync(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (!stats.isFile() || !sourceExtension.test(path)) {
    return;
  }

  files.push(path);
}

function collectScopedFiles(rawArgs) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['vendor/ocentra-parent-core-ui/AppPages/ParentPortal'],
    acceptPath: (filePath) => sourceExtension.test(filePath),
  });

  if (scope.mode === 'skip') {
    return [];
  }

  return scope.files.map((filePath) => repoAbsolutePath(filePath));
}

function main(rawArgs = process.argv.slice(2)) {
  findings.length = 0;

  for (const file of rawArgs.length === 0 ? collectFullFiles() : collectScopedFiles(rawArgs)) {
    inspectFile(file);
  }

  if (findings.length > 0) {
    console.error('ParentPortal vendor page source must import portal assets through local shims, not app aliases.');
    for (const finding of findings) {
      console.error(`${finding.path}:${finding.line} ${finding.text}`);
    }
    process.exit(1);
  }

  console.log('No ParentPortal vendor page imports use portal asset aliases.');
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
