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
const scriptName = 'node scripts/check-no-validation-bypass.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const patterns = [
  { regex: /\beslint-disable(?:-next-line)?\b/u, message: 'ESLint disable directives are forbidden.' },
  { regex: /@ts-(?:ignore|nocheck|expect-error)\b/u, message: 'TypeScript suppression directives are forbidden.' },
  { regex: /\bprettier-ignore\b/u, message: 'Prettier ignore directives are forbidden.' },
  { regex: /#\s*!\s*\[\s*allow\s*\(/u, message: 'crate-level Rust allow attributes are forbidden.' },
  { regex: /#\s*!\s*\[\s*expect\s*\(/u, message: 'crate-level Rust expect attributes are forbidden.' },
  { regex: /#\s*\[\s*allow\s*\(/u, message: 'Rust allow attributes are forbidden.' },
  { regex: /#\s*\[\s*expect\s*\(/u, message: 'Rust expect attributes are forbidden.' },
  { regex: /cfg_attr\s*\([^)]*allow\s*\(/u, message: 'cfg_attr allow directives are forbidden.' },
  { regex: /rustfmt::skip/u, message: 'rustfmt skip directives are forbidden.' },
  { regex: /clippy::(?:allow|expect)/u, message: 'Clippy suppression directives are forbidden.' },
];

function isSupportedPath(filePath) {
  return supportedExtensions.has(path.extname(filePath));
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
    roots: ['apps', 'packages', 'crates', 'scripts', 'tools'],
    acceptPath: isSupportedPath,
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const findings = scope.files.flatMap((filePath) => collectFindings(filePath));
  if (findings.length > 0) {
    console.error('Validation bypass guard failed. Inline suppression and bypass directives are forbidden.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Validation bypass guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
