import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, sep } from 'node:path';
import { pathToFileURL } from 'node:url';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const repoRoot = process.cwd();
const scannedRoots = ['scripts'];
const ignoredSegments = new Set(['node_modules']);
const ignoredFiles = new Set(['scripts/test/cross-platform-script-commands.test.mjs']);
const windowsOnlyCommandPatterns = [
  {
    label: 'Windows cmd npm invocation',
    pattern: /['"]cmd(?:\.exe)?['"]\s*,\s*\[\s*['"]\/c['"]\s*,\s*['"]npm['"]/u,
  },
];
const scriptName = 'node scripts/check-cross-platform-script-commands.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];

function toPosix(path) {
  return path.split(sep).join('/');
}

function shouldSkip(path) {
  const relativePath = toPosix(relative(repoRoot, path));
  return ignoredFiles.has(relativePath) || relativePath.split('/').some((part) => ignoredSegments.has(part));
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

  if (stats.isFile() && path.endsWith('.mjs')) {
    files.push(path);
  }
}

function hasNearbyWindowsGuard(lines, lineIndex) {
  const start = Math.max(0, lineIndex - 8);
  const nearby = lines.slice(start, lineIndex + 1).join('\n');
  return /process\.platform\s*={2,3}\s*['"]win32['"]|process\.platform\s*!={1,2}\s*['"]win32['"]/u.test(nearby);
}

export function inspectCrossPlatformScriptCommands(relativePath, text) {
  const findings = [];
  const lines = text.split(/\r?\n/u);

  lines.forEach((line, index) => {
    for (const { label, pattern } of windowsOnlyCommandPatterns) {
      if (pattern.test(line) && !hasNearbyWindowsGuard(lines, index)) {
        findings.push({
          path: relativePath,
          line: index + 1,
          reason: `${label} must be behind an explicit process.platform guard or use a cross-platform command helper`,
          text: line.trim(),
        });
      }
    }
  });

  return findings;
}

function collectFindingsForFiles(files, root = repoRoot) {
  const findings = [];
  for (const file of files) {
    const relativePath = toPosix(relative(root, file));
    findings.push(...inspectCrossPlatformScriptCommands(relativePath, readFileSync(file, 'utf8')));
  }
  return findings;
}

export function collectCrossPlatformScriptCommandFindings(root = repoRoot) {
  const files = [];
  for (const scannedRoot of scannedRoots) {
    walk(join(root, scannedRoot), files);
  }

  return { checkedFiles: files.length, findings: collectFindingsForFiles(files, root) };
}

function collectScopedFiles(rawArgs) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: scannedRoots,
    acceptPath: (filePath) => filePath.endsWith('.mjs') && !ignoredFiles.has(filePath),
  });

  if (scope.mode === 'skip') {
    return [];
  }

  return scope.files.map((filePath) => repoAbsolutePath(filePath));
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const rawArgs = process.argv.slice(2);
  const files = rawArgs.length === 0 ? null : collectScopedFiles(rawArgs);
  const { checkedFiles, findings } =
    files === null
      ? collectCrossPlatformScriptCommandFindings()
      : { checkedFiles: files.length, findings: collectFindingsForFiles(files) };

  if (findings.length > 0) {
    console.error('Cross-platform scripts must not hardcode Windows-only npm command invocations.');
    for (const finding of findings) {
      console.error(`${finding.path}:${finding.line} ${finding.reason}: ${finding.text}`);
    }
    process.exit(1);
  }

  console.log(`No unguarded Windows-only npm command invocations found across ${checkedFiles} scripts.`);
}
