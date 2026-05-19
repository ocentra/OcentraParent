import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { extname, join } from 'node:path';

const targetRoot = process.argv[2] ?? 'dist';
const sourceExtensions = new Set(['.js']);
const importPattern = /\b(from\s+['"]|import\s*\(\s*['"])(\.[^'"]+?)(['"]\s*\)?)/g;

function walk(path, files) {
  if (!existsSync(path)) {
    return;
  }

  const stat = statSync(path);
  if (stat.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stat.isFile() && sourceExtensions.has(extname(path))) {
    files.push(path);
  }
}

function needsExtension(specifier) {
  return !/\.[cm]?js$/u.test(specifier) && !specifier.endsWith('.json');
}

function fixFile(path) {
  const before = readFileSync(path, 'utf8');
  const after = before.replace(importPattern, (match, prefix, specifier, suffix) => {
    if (!needsExtension(specifier)) {
      return match;
    }
    return `${prefix}${specifier}.js${suffix}`;
  });

  if (after !== before) {
    writeFileSync(path, after);
  }
}

const files = [];
walk(targetRoot, files);
for (const file of files) {
  fixFile(file);
}
