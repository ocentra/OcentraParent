import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { extname, join } from 'node:path';

const outputDir = join(process.cwd(), 'target', 'repo-size');
const fileListPath = join(outputDir, 'tracked-files.txt');
const jsonPath = join(outputDir, 'repo-size.json');
const summaryPath = join(outputDir, 'repo-size-summary.md');
const summaryOnly = process.argv.includes('--summary-only');

const generatedPathParts = new Set([
  'node_modules',
  'target',
  'dist',
  'coverage',
  '.turbo',
  'output',
  'test-results',
  'playwright-report',
  'gen',
  'generated',
  '.generated',
  '.wix',
  '.codeql-local',
]);

const docsExtensions = new Set(['.md', '.mdc', '.adoc', '.rst', '.txt']);
const codeExtensions = new Set([
  '.ts',
  '.tsx',
  '.js',
  '.jsx',
  '.mjs',
  '.cjs',
  '.mts',
  '.cts',
  '.rs',
  '.swift',
  '.kt',
  '.kts',
  '.java',
  '.cs',
  '.cpp',
  '.c',
  '.h',
  '.hpp',
  '.py',
  '.rb',
  '.go',
  '.php',
  '.scala',
  '.sh',
  '.ps1',
  '.bat',
  '.cmd',
]);
const configExtensions = new Set([
  '.json',
  '.jsonc',
  '.toml',
  '.yaml',
  '.yml',
  '.xml',
  '.lock',
  '.ini',
  '.cfg',
  '.conf',
]);
const generatedExtensions = new Set(['.min.js', '.min.css']);
const binaryExtensions = new Set([
  '.png',
  '.jpg',
  '.jpeg',
  '.gif',
  '.webp',
  '.ico',
  '.pdf',
  '.zip',
  '.gz',
  '.tar',
  '.woff',
  '.woff2',
  '.ttf',
  '.otf',
  '.mp3',
  '.mp4',
  '.mov',
  '.avi',
  '.wasm',
  '.exe',
  '.dll',
  '.so',
  '.dylib',
  '.bin',
  '.db',
  '.sqlite',
  '.duckdb',
]);
const orderedCategories = ['Code', 'Tests', 'Docs', 'Config / CI / Scripts', 'Generated / ignored', 'Other'];

function normalizePath(filePath) {
  return filePath.replaceAll('\\', '/');
}

function isGeneratedLikePath(filePath) {
  const normalized = normalizePath(filePath);
  const parts = normalized.split('/');
  return parts.some((part) => generatedPathParts.has(part));
}

function isGeneratedLikeFile(filePath) {
  const normalized = normalizePath(filePath);
  return generatedExtensions.has(extname(normalized)) || normalized.includes('.generated.');
}

function categoryFor(filePath) {
  const normalized = normalizePath(filePath);
  const extension = extname(normalized);

  if (isGeneratedLikePath(normalized) || isGeneratedLikeFile(normalized)) {
    return 'Generated / ignored';
  }

  if (
    normalized.includes('/tests/') ||
    normalized.endsWith('.test.ts') ||
    normalized.endsWith('.test.tsx') ||
    normalized.endsWith('.test.js') ||
    normalized.endsWith('.test.mjs') ||
    normalized.endsWith('.spec.ts') ||
    normalized.endsWith('.spec.tsx') ||
    normalized.endsWith('.spec.js') ||
    normalized.endsWith('.spec.mjs')
  ) {
    return 'Tests';
  }

  if (normalized.startsWith('docs/') || docsExtensions.has(extension)) {
    return 'Docs';
  }

  if (
    normalized.startsWith('.github/') ||
    normalized.startsWith('scripts/') ||
    normalized.endsWith('package.json') ||
    normalized.endsWith('Cargo.toml') ||
    normalized.endsWith('Cargo.lock') ||
    normalized.endsWith('turbo.json') ||
    normalized.endsWith('tsconfig.json') ||
    normalized.endsWith('.gitignore') ||
    normalized.endsWith('.prettierignore') ||
    normalized.endsWith('.prettierrc') ||
    normalized.endsWith('.editorconfig') ||
    configExtensions.has(extension)
  ) {
    return 'Config / CI / Scripts';
  }

  if (codeExtensions.has(extension)) {
    return 'Code';
  }

  return 'Other';
}

function formatNumber(value) {
  return new Intl.NumberFormat('en-US').format(value);
}

function normalizeNewlines(text) {
  return text.replace(/\r\n/gu, '\n');
}

function fileLinesFromText(text) {
  if (text.length === 0) {
    return [];
  }

  const normalized = normalizeNewlines(text);
  const lines = normalized.split('\n');
  if (lines.at(-1) === '') {
    lines.pop();
  }
  return lines;
}

function isLikelyBinary(filePath, fileContents) {
  return binaryExtensions.has(extname(filePath)) || fileContents.includes('\u0000');
}

function hasErrorCode(error, code) {
  return typeof error === 'object' && error !== null && 'code' in error && error.code === code;
}

function readTrackedFile(absolutePath) {
  try {
    return {
      kind: 'file',
      contents: readFileSync(absolutePath, 'utf8'),
    };
  } catch (error) {
    if (hasErrorCode(error, 'ENOENT')) {
      return { kind: 'missing' };
    }
    if (!hasErrorCode(error, 'EISDIR') && !hasErrorCode(error, 'EPERM') && !hasErrorCode(error, 'EACCES')) {
      throw error;
    }
  }

  try {
    const stats = statSync(absolutePath);
    if (stats.isDirectory()) {
      return { kind: 'directory' };
    }
  } catch (error) {
    if (hasErrorCode(error, 'ENOENT')) {
      return { kind: 'missing' };
    }
    throw error;
  }

  throw new Error(`unable to read tracked file ${normalizePath(absolutePath)}`);
}

function measureTrackedFile(filePath) {
  const category = categoryFor(filePath);
  const absolutePath = join(process.cwd(), filePath);
  const trackedFile = readTrackedFile(absolutePath);
  if (trackedFile.kind === 'missing') {
    return {
      category,
      total: 0,
      blank: 0,
      loc: 0,
      skippedBinary: false,
      skippedDirectory: false,
      skippedMissing: true,
    };
  }

  if (trackedFile.kind === 'directory') {
    return {
      category,
      total: 0,
      blank: 0,
      loc: 0,
      skippedBinary: false,
      skippedDirectory: true,
      skippedMissing: false,
    };
  }

  const contents = trackedFile.contents;

  if (isLikelyBinary(filePath, contents)) {
    return {
      category,
      total: 0,
      blank: 0,
      loc: 0,
      skippedBinary: true,
      skippedDirectory: false,
      skippedMissing: false,
    };
  }

  const lines = fileLinesFromText(contents);
  const total = lines.length;
  const blank = lines.filter((line) => line.trim().length === 0).length;

  return {
    category,
    total,
    blank,
    loc: total - blank,
    skippedBinary: false,
    skippedDirectory: false,
    skippedMissing: false,
  };
}

function trackedFilesFromGit() {
  return execFileSync('git', ['ls-files'], { encoding: 'utf8' })
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter(Boolean);
}

function emptyBucket() {
  return {
    files: 0,
    total: 0,
    blank: 0,
    loc: 0,
  };
}

const trackedFiles = trackedFilesFromGit();
const buckets = new Map(orderedCategories.map((category) => [category, emptyBucket()]));
const perFileEntries = [];
let skippedBinaryFiles = 0;
let skippedDirectoryEntries = 0;
let skippedMissingFiles = 0;

for (const filePath of trackedFiles) {
  const measurement = measureTrackedFile(filePath);
  perFileEntries.push({
    path: filePath,
    ...measurement,
  });

  if (measurement.skippedMissing) {
    skippedMissingFiles += 1;
    continue;
  }

  if (measurement.skippedDirectory) {
    skippedDirectoryEntries += 1;
    continue;
  }

  const bucket = buckets.get(measurement.category) ?? emptyBucket();

  bucket.files += 1;
  bucket.total += measurement.total;
  bucket.blank += measurement.blank;
  bucket.loc += measurement.loc;

  buckets.set(measurement.category, bucket);

  if (measurement.skippedBinary) {
    skippedBinaryFiles += 1;
  }
}

const totals = [...buckets.values()].reduce(
  (sum, bucket) => ({
    files: sum.files + bucket.files,
    total: sum.total + bucket.total,
    blank: sum.blank + bucket.blank,
    loc: sum.loc + bucket.loc,
  }),
  emptyBucket()
);

const summaryRows = orderedCategories
  .map((category) => {
    const bucket = buckets.get(category);
    if (!bucket || bucket.files === 0) {
      return null;
    }

    return `| ${category} | ${formatNumber(bucket.files)} | ${formatNumber(bucket.loc)} | ${formatNumber(bucket.blank)} | ${formatNumber(bucket.total)} |`;
  })
  .filter(Boolean);

const summaryLines = [
  '# Repository Size Summary',
  '',
  'Counted files: Git-tracked files only.',
  '',
  '| Category | Files | LOC (non-blank) | Blank lines | Total lines |',
  '| --- | ---: | ---: | ---: | ---: |',
];

summaryLines.push(...summaryRows);

summaryLines.push(
  `| **Total** | **${formatNumber(totals.files)}** | **${formatNumber(totals.loc)}** | **${formatNumber(totals.blank)}** | **${formatNumber(totals.total)}** |`
);

if (skippedBinaryFiles > 0) {
  summaryLines.push('', `Skipped binary/non-text tracked files: ${formatNumber(skippedBinaryFiles)}`);
}

if (skippedDirectoryEntries > 0) {
  summaryLines.push(`Skipped tracked directory/gitlink entries: ${formatNumber(skippedDirectoryEntries)}`);
}

if (skippedMissingFiles > 0) {
  summaryLines.push(`Skipped tracked paths missing from the working tree: ${formatNumber(skippedMissingFiles)}`);
}

if (!summaryOnly) {
  mkdirSync(outputDir, { recursive: true });
  writeFileSync(fileListPath, `${trackedFiles.join('\n')}\n`, 'utf8');
  summaryLines.push(
    '',
    `Tracked file list: \`${normalizePath(fileListPath)}\``,
    `Raw JSON: \`${normalizePath(jsonPath)}\``,
    `Markdown summary: \`${normalizePath(summaryPath)}\``
  );
  writeFileSync(
    jsonPath,
    `${JSON.stringify(
      {
        trackedFiles: trackedFiles.length,
        skippedBinaryFiles,
        skippedDirectoryEntries,
        skippedMissingFiles,
        totals,
        categories: orderedCategories
          .map((category) => {
            const bucket = buckets.get(category);
            if (!bucket || bucket.files === 0) {
              return null;
            }
            return { category, ...bucket };
          })
          .filter(Boolean),
        files: perFileEntries,
      },
      null,
      2
    )}\n`,
    'utf8'
  );
  writeFileSync(summaryPath, `${summaryLines.join('\n')}\n`, 'utf8');
}

console.log(summaryLines.join('\n'));
