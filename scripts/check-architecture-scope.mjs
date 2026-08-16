import { spawnSync } from 'node:child_process';
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';

export const repoRoot = process.cwd();
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

export function toPosix(value) {
  return value.replace(/\\/gu, '/');
}

export function repoAbsolutePath(filePath) {
  return path.isAbsolute(filePath) ? filePath : path.join(repoRoot, filePath);
}

export function repoRelativePath(filePath) {
  return toPosix(path.relative(repoRoot, repoAbsolutePath(filePath)));
}

export function readRepoFile(relativePath) {
  return readFileSync(repoAbsolutePath(relativePath), 'utf8');
}

export function lineNumberAt(text, matchIndex) {
  return text.slice(0, matchIndex).split(/\r?\n/u).length;
}

export function isIgnoredPath(relativePath) {
  return relativePath.split('/').some((segment) => ignoredSegments.has(segment));
}

export function isWithinRoots(relativePath, roots) {
  return roots.some((root) => relativePath === root || relativePath.startsWith(`${root}/`));
}

function formatUsage(scriptName, usageLines) {
  return [
    `Usage: ${scriptName}`,
    ...usageLines.map((line) => `  ${line}`),
    `  ${scriptName} --files <path> [more paths]`,
  ].join('\n');
}

function runGitOrThrow(args, failureLabel) {
  const result = spawnSync('git', args, {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    throw new Error(`${failureLabel}: ${result.stderr?.trim() || 'git command failed'}`);
  }

  return result.stdout.trim();
}

export function parseScopeArgs(rawArgs, { scriptName, usageLines }) {
  let all = false;
  let base = null;
  let head = null;
  const files = [];

  for (let index = 0; index < rawArgs.length; index += 1) {
    const arg = rawArgs[index];

    if (arg === '--all') {
      all = true;
      continue;
    }

    if (arg === '--base') {
      base = rawArgs[index + 1] ?? null;
      index += 1;
      continue;
    }

    if (arg === '--head') {
      head = rawArgs[index + 1] ?? null;
      index += 1;
      continue;
    }

    if (arg === '--files') {
      files.push(...rawArgs.slice(index + 1));
      break;
    }

    files.push(arg);
  }

  if (all) {
    if (base !== null || head !== null || files.length > 0) {
      throw new Error(formatUsage(scriptName, usageLines));
    }
    return { mode: 'all' };
  }

  if (base !== null || head !== null) {
    if (base === null || head === null) {
      throw new Error(formatUsage(scriptName, usageLines));
    }
    return { mode: 'diff', base, head };
  }

  if (files.length > 0) {
    return { mode: 'files', files };
  }

  const envBase = process.env.OCENTRA_ARCHITECTURE_BASE ?? null;
  const envHead = process.env.OCENTRA_ARCHITECTURE_HEAD ?? null;
  if (envBase !== null && envHead !== null) {
    return { mode: 'diff', base: envBase, head: envHead };
  }

  return {
    mode: 'skip',
    reason: `${scriptName} skipped: provide --files, --base/--head, or --all. CI may set OCENTRA_ARCHITECTURE_BASE/HEAD automatically.`,
  };
}

function listTrackedFiles(roots) {
  const output = runGitOrThrow(['ls-files', '--', ...roots], 'failed to list tracked files');
  return output === '' ? [] : output.split(/\r?\n/u).filter(Boolean);
}

function listDiffFiles(base, head, roots) {
  const output = runGitOrThrow(
    ['diff', '--name-only', '--diff-filter=ACMR', base, head, '--', ...roots],
    'failed to list diff files'
  );
  return output === '' ? [] : output.split(/\r?\n/u).filter(Boolean);
}

function walkExplicitPath(absolutePath, collectedFiles) {
  if (!existsSync(absolutePath)) {
    return;
  }

  const stats = statSync(absolutePath);
  if (stats.isDirectory()) {
    for (const entry of readdirSync(absolutePath)) {
      walkExplicitPath(path.join(absolutePath, entry), collectedFiles);
    }
    return;
  }

  if (stats.isFile()) {
    collectedFiles.push(repoRelativePath(absolutePath));
  }
}

function expandExplicitFiles(files) {
  const collectedFiles = [];
  for (const file of files) {
    walkExplicitPath(repoAbsolutePath(file), collectedFiles);
  }
  return collectedFiles;
}

export function resolveScopedFiles(rawArgs, { scriptName, usageLines, roots, acceptPath }) {
  const scope = parseScopeArgs(rawArgs, { scriptName, usageLines });
  if (scope.mode === 'skip') {
    return { ...scope, files: [] };
  }

  const candidates =
    scope.mode === 'all'
      ? listTrackedFiles(roots)
      : scope.mode === 'diff'
        ? listDiffFiles(scope.base, scope.head, roots)
        : expandExplicitFiles(scope.files);

  const uniqueFiles = [...new Set(candidates.map((file) => toPosix(file)).filter(Boolean))];
  const files = uniqueFiles.filter(
    (file) =>
      existsSync(repoAbsolutePath(file)) && !isIgnoredPath(file) && isWithinRoots(file, roots) && acceptPath(file)
  );

  return { ...scope, files };
}

export function runNodeOrThrow(scriptPath, args) {
  const result = spawnSync(process.execPath, [scriptPath, ...args], {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}

export function runCommandOrThrow(command, args) {
  const result = spawnSync(command, args, {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}
