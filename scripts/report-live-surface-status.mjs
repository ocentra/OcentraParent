#!/usr/bin/env node

import fs from 'node:fs/promises';
import path from 'node:path';
import { execFile as execFileCallback } from 'node:child_process';
import { promisify } from 'node:util';

const repoRoot = process.cwd();
const roots = ['apps', 'crates', 'packages'];
const jsonMode = process.argv.includes('--json');
const execFile = promisify(execFileCallback);

const rustTestIndicators = [
  /#\[(tokio::)?test\]/,
  /assert!?\(/,
  /assert_eq!\(/,
  /assert_ne!\(/,
  /proptest!/,
  /rstest/,
  /insta::/,
];

async function pathExists(targetPath) {
  try {
    await fs.access(targetPath);
    return true;
  } catch {
    return false;
  }
}

async function walkFiles(targetPath) {
  if (!(await pathExists(targetPath))) {
    return [];
  }
  const files = [];
  const stack = [targetPath];

  while (stack.length > 0) {
    const current = stack.pop();
    const entries = await fs.readdir(current, { withFileTypes: true });
    for (const entry of entries) {
      const entryPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        stack.push(entryPath);
      } else if (entry.isFile()) {
        files.push(entryPath);
      }
    }
  }

  return files;
}

async function collectSurfaceDirectories(rootName) {
  const rootPath = path.join(repoRoot, rootName);
  const entries = await fs.readdir(rootPath, { withFileTypes: true });
  return entries
    .filter((entry) => entry.isDirectory())
    .map((entry) => ({
      rootName,
      surfacePath: path.join(rootPath, entry.name),
      surfaceName: entry.name,
      relativePath: path.join(rootName, entry.name),
    }))
    .sort((left, right) => left.relativePath.localeCompare(right.relativePath));
}

async function countRustTests(testRoot) {
  const files = (await walkFiles(testRoot)).filter((filePath) => filePath.endsWith('.rs'));
  let count = 0;
  for (const filePath of files) {
    const content = await fs.readFile(filePath, 'utf8');
    if (rustTestIndicators.some((pattern) => pattern.test(content))) {
      count += 1;
    }
  }
  return count;
}

async function countNodeTests(...rootsToScan) {
  let count = 0;
  for (const rootPath of rootsToScan) {
    const files = await walkFiles(rootPath);
    count += files.filter((filePath) => /\.(test|spec)\.[^.]+$/u.test(path.basename(filePath))).length;
  }
  return count;
}

async function countEmptyScaffolds(...rootsToScan) {
  const trackedFiles = await trackedFileSet();
  let count = 0;
  for (const rootPath of rootsToScan) {
    if (!(await pathExists(rootPath))) {
      continue;
    }
    const directories = await walkDirectories(rootPath);
    for (const directoryPath of directories) {
      const trackedFilesInDirectory = [...trackedFiles].filter((relativePath) =>
        relativePath.startsWith(toRepoRelativePath(directoryPath) + '/')
      );
      if (trackedFilesInDirectory.length === 0) {
        continue;
      }
      const usefulTrackedFiles = trackedFilesInDirectory.filter(
        (relativePath) => path.basename(relativePath) !== '.gitkeep'
      );
      if (usefulTrackedFiles.length === 0) {
        count += 1;
      }
    }
  }
  return count;
}

let trackedFilesPromise;

function toRepoRelativePath(absolutePath) {
  return path.relative(repoRoot, absolutePath).split(path.sep).join('/');
}

async function trackedFileSet() {
  if (!trackedFilesPromise) {
    trackedFilesPromise = execFile('git', ['ls-files'], { cwd: repoRoot }).then(({ stdout }) => {
      return new Set(
        stdout
          .split(/\r?\n/u)
          .map((line) => line.trim())
          .filter(Boolean)
      );
    });
  }
  return trackedFilesPromise;
}

async function walkDirectories(targetPath) {
  if (!(await pathExists(targetPath))) {
    return [];
  }
  const directories = [];
  const stack = [targetPath];

  while (stack.length > 0) {
    const current = stack.pop();
    const entries = await fs.readdir(current, { withFileTypes: true });
    for (const entry of entries) {
      if (!entry.isDirectory()) {
        continue;
      }
      const entryPath = path.join(current, entry.name);
      directories.push(entryPath);
      stack.push(entryPath);
    }
  }

  return directories;
}

async function countInlineRustTests(...rootsToScan) {
  let count = 0;
  for (const rootPath of rootsToScan) {
    if (!(await pathExists(rootPath))) {
      continue;
    }
    const files = (await walkFiles(rootPath)).filter((filePath) => filePath.endsWith('.rs'));
    for (const filePath of files) {
      const content = await fs.readFile(filePath, 'utf8');
      if (content.includes('#[cfg(test)]')) {
        count += 1;
      }
    }
  }
  return count;
}

function classifyStatus({ surfaceName, realTests, emptyScaffolds, inlineSrcTests }) {
  if (surfaceName.endsWith('-generated')) {
    return 'generated-support';
  }
  if (inlineSrcTests > 0) {
    return 'inline-test-debt';
  }
  if (emptyScaffolds > 0) {
    return 'empty-scaffold-debt';
  }
  if (realTests === 0) {
    return 'needs-tests';
  }
  return 'ok';
}

function formatMarkdown(rows) {
  const header = [
    '| Surface | Kind | Real tests | Inline src tests | TS src | TS tests | TS generated | Status | Notes |',
    '| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- |',
  ];
  const lines = rows.map((row) => {
    const notes = row.notes.length > 0 ? row.notes.join(', ') : '-';
    return `| \`${row.surface}\` | ${row.kind} | ${row.realTests} | ${row.inlineSrcTests} | ${row.tsSourceFiles} | ${row.tsTestFiles} | ${row.tsGeneratedFiles} | ${row.status} | ${notes} |`;
  });
  return header.concat(lines).join('\n');
}

function summarize(rows) {
  return {
    totalSurfaces: rows.length,
    ok: rows.filter((row) => row.status === 'ok').length,
    generatedSupport: rows.filter((row) => row.status === 'generated-support').length,
    needsTests: rows.filter((row) => row.status === 'needs-tests').length,
    emptyScaffoldDebt: rows.filter((row) => row.status === 'empty-scaffold-debt').length,
    inlineTestDebt: rows.filter((row) => row.status === 'inline-test-debt').length,
  };
}

async function buildRow(surface) {
  const testsRoot = path.join(surface.surfacePath, 'tests');
  const e2eRoot = path.join(surface.surfacePath, 'e2e');
  const srcRustRoot = path.join(surface.surfacePath, 'src');
  const tauriTestsRoot = path.join(surface.surfacePath, 'src-tauri', 'tests');
  const tauriSrcRoot = path.join(surface.surfacePath, 'src-tauri', 'src');

  const notes = [];
  let realTests = 0;
  let emptyScaffolds = 0;
  let inlineSrcTests = 0;
  let tsSourceFiles = 0;
  let tsTestFiles = 0;
  let tsGeneratedFiles = 0;
  let tsScriptFiles = 0;

  const typeScriptBuckets = await countTypeScriptBuckets(surface.surfacePath);
  tsSourceFiles = typeScriptBuckets.source;
  tsTestFiles = typeScriptBuckets.tests;
  tsGeneratedFiles = typeScriptBuckets.generated;
  tsScriptFiles = typeScriptBuckets.scripts;

  if (surface.rootName === 'crates') {
    realTests += await countRustTests(testsRoot);
    emptyScaffolds += await countEmptyScaffolds(testsRoot);
    inlineSrcTests += await countInlineRustTests(srcRustRoot);
  } else if (surface.rootName === 'packages') {
    realTests += await countNodeTests(testsRoot, e2eRoot);
    emptyScaffolds += await countEmptyScaffolds(testsRoot, e2eRoot);
  } else {
    realTests += await countNodeTests(testsRoot, e2eRoot);
    realTests += await countRustTests(testsRoot);
    emptyScaffolds += await countEmptyScaffolds(testsRoot, e2eRoot);
    inlineSrcTests += await countInlineRustTests(srcRustRoot);
    if (await pathExists(tauriTestsRoot)) {
      realTests += await countRustTests(tauriTestsRoot);
      emptyScaffolds += await countEmptyScaffolds(tauriTestsRoot);
      inlineSrcTests += await countInlineRustTests(tauriSrcRoot);
      notes.push('nested-rust-tests:src-tauri/tests');
    }
  }

  if (surface.surfaceName.endsWith('-generated')) {
    notes.push('generated-support-folder');
  }
  if (tsScriptFiles > 0) {
    notes.push(`ts-scripts:${tsScriptFiles}`);
  }
  if (emptyScaffolds > 0) {
    notes.push(`empty-scaffolds:${emptyScaffolds}`);
  }

  return {
    surface: surface.relativePath.replaceAll(path.sep, '/'),
    kind: surface.rootName.slice(0, -1),
    realTests,
    emptyScaffolds,
    inlineSrcTests,
    tsSourceFiles,
    tsTestFiles,
    tsGeneratedFiles,
    tsScriptFiles,
    status: classifyStatus({
      surfaceName: surface.surfaceName,
      realTests,
      emptyScaffolds,
      inlineSrcTests,
    }),
    notes,
  };
}

async function countTypeScriptBuckets(surfacePath) {
  const files = (await walkFiles(surfacePath)).filter((filePath) => {
    const normalized = filePath.split(path.sep).join('/');
    return (
      /\.(ts|tsx|mts|cts)$/u.test(filePath) &&
      !normalized.includes('/node_modules/') &&
      !normalized.includes('/.turbo/') &&
      !normalized.includes('/target/')
    );
  });

  let source = 0;
  let tests = 0;
  let generated = 0;
  let scripts = 0;

  for (const filePath of files) {
    const normalized = filePath.split(path.sep).join('/');
    if (normalized.includes('/tests/') || normalized.includes('/e2e/')) {
      tests += 1;
      continue;
    }
    if (normalized.includes('/dist/') || normalized.includes('/generated/')) {
      generated += 1;
      continue;
    }
    if (normalized.includes('/scripts/')) {
      scripts += 1;
      continue;
    }
    if (normalized.includes('/src/')) {
      source += 1;
    }
  }

  return { source, tests, generated, scripts };
}

async function main() {
  const surfaces = [];
  for (const rootName of roots) {
    surfaces.push(...(await collectSurfaceDirectories(rootName)));
  }

  const rows = [];
  for (const surface of surfaces) {
    rows.push(await buildRow(surface));
  }

  rows.sort((left, right) => left.surface.localeCompare(right.surface));

  if (jsonMode) {
    console.log(
      JSON.stringify(
        {
          summary: summarize(rows),
          rows,
        },
        null,
        2
      )
    );
    return;
  }

  const summary = summarize(rows);
  console.log(
    [
      `# Live Surface Status`,
      ``,
      `surfaces=${summary.totalSurfaces} ok=${summary.ok} generated_support=${summary.generatedSupport} needs_tests=${summary.needsTests} empty_scaffold_debt=${summary.emptyScaffoldDebt} inline_test_debt=${summary.inlineTestDebt}`,
      ``,
      formatMarkdown(rows),
    ].join('\n')
  );
}

await main();
