import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  readRepoFile,
  repoRelativePath,
  resolveScopedFiles,
} from './check-architecture-scope.mjs';

const scriptPath = repoRelativePath(fileURLToPath(import.meta.url));
const allowedPaths = new Set([scriptPath]);
const supportedExtensions = new Set(['.js', '.jsx', '.ts', '.tsx', '.mjs', '.mts', '.cjs', '.cts', '.rs']);
const scriptName = 'node scripts/check-no-skipped-focused-tests.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const patterns = [
  {
    regex: /\b(?:describe|it|test)\.(?:only|skip|todo)\b/u,
    message: 'focused, skipped, and todo JavaScript tests are forbidden.',
  },
  {
    regex: /\btest\.(?:fixme|skip|only)\b/u,
    message: 'Playwright focused or skipped tests are forbidden.',
  },
  {
    regex: /#\s*\[\s*ignore\s*\]/u,
    message: 'Rust #[ignore] tests are forbidden.',
  },
];

function isTestPath(filePath) {
  const extension = path.extname(filePath);
  if (!supportedExtensions.has(extension)) {
    return false;
  }
  return (
    filePath.startsWith('scripts/test/') ||
    filePath.includes('/tests/') ||
    filePath.includes('.test.') ||
    filePath.includes('.spec.') ||
    filePath.endsWith('_tests.rs')
  );
}

function collectFindings(filePath) {
  if (allowedPaths.has(filePath)) {
    return [];
  }
  const findings = [];
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  lines.forEach((line, index) => {
    for (const pattern of patterns) {
      if (pattern.regex.test(line)) {
        findings.push(`${filePath}:${index + 1} ${pattern.message}`);
      }
    }
  });
  return findings;
}

export function main(rawArgs = process.argv.slice(2)) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['apps', 'packages', 'crates', 'scripts'],
    acceptPath: isTestPath,
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const findings = scope.files.flatMap((filePath) => collectFindings(filePath));
  if (findings.length > 0) {
    console.error('Skipped/focused test guard failed. Hidden, focused, or todo tests are forbidden.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Skipped/focused test guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
