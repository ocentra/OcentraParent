import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const sourceRoots = ['apps', 'packages', 'crates'];
const sourceExtensions = new Set(['.ts', '.tsx', '.rs']);
const ignoredSegments = new Set(['.git', '.turbo', 'coverage', 'dist', 'node_modules', 'ocentra-ledger', 'target']);
const forbiddenPatterns = [
  { label: 'module mock API', pattern: /\b(?:vi|jest)\.mock\b/iu },
  { label: 'mock function API', pattern: /\b(?:vi|jest)\.fn\b/iu },
  { label: 'spy API', pattern: /\b(?:vi|jest)\.spyOn\b|\bspyOn\b/iu },
  { label: 'test-double package', pattern: /\b(?:sinon|nock|msw)\b/iu },
  { label: 'test-double vocabulary', pattern: /\b(?:mock|fake|stub|spy)\b/iu },
];

function toPosix(path) {
  return path.split(sep).join('/');
}

function extensionOf(path) {
  const match = path.match(/\.[^.]+$/u);
  return match?.[0] ?? '';
}

function shouldSkip(path) {
  const relativePath = toPosix(relative(repoRoot, path));
  return relativePath.split('/').some((part) => ignoredSegments.has(part));
}

function walk(path, files) {
  if (!existsSync(path) || shouldSkip(path)) {
    return;
  }

  const stats = statSync(path);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(path)) {
      walk(join(path, entry), files);
    }
    return;
  }

  if (stats.isFile() && sourceExtensions.has(extensionOf(path))) {
    files.push(path);
  }
}

export function inspectTestDoubleText(relativePath, text) {
  const findings = [];
  const lines = text.split(/\r?\n/u);

  lines.forEach((line, index) => {
    for (const { label, pattern } of forbiddenPatterns) {
      if (pattern.test(line)) {
        findings.push({
          path: relativePath,
          line: index + 1,
          reason: label,
          text: line.trim(),
        });
      }
    }
  });

  return findings;
}

export function collectTestDoubleFindings(root = repoRoot) {
  const files = [];
  for (const sourceRoot of sourceRoots) {
    walk(join(root, sourceRoot), files);
  }

  const findings = [];
  for (const file of files) {
    const relativePath = toPosix(relative(root, file));
    findings.push(...inspectTestDoubleText(relativePath, readFileSync(file, 'utf8')));
  }
  return { checkedFiles: files.length, findings };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const { checkedFiles, findings } = collectTestDoubleFindings();

  if (findings.length > 0) {
    console.error('Test doubles are not allowed. Use real domain contracts, real parsers, and real local services.');
    for (const finding of findings) {
      console.error(`${finding.path}:${finding.line} ${finding.reason}: ${finding.text}`);
    }
    process.exit(1);
  }

  console.log(`No test doubles found across ${checkedFiles} checked source files.`);
}
