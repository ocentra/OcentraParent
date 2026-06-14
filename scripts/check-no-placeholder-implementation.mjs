import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  readRepoFile,
  repoRelativePath,
  resolveScopedFiles,
} from './check-architecture-scope.mjs';

const scriptPath = repoRelativePath(fileURLToPath(import.meta.url));
const allowedScriptPattern = /^scripts\/check-[^/]+\.mjs$/u;
const supportedExtensions = new Set(['.js', '.jsx', '.ts', '.tsx', '.mjs', '.mts', '.cjs', '.cts', '.rs']);
const scriptName = 'node scripts/check-no-placeholder-implementation.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const commentPatterns = [
  { regex: /\bTODO\b/u, message: 'TODO markers are forbidden in production source.' },
  { regex: /\bFIXME\b/u, message: 'FIXME markers are forbidden in production source.' },
  { regex: /\bTBD\b/u, message: 'TBD markers are forbidden in production source.' },
  { regex: /\bplaceholder\b/iu, message: 'placeholder markers are forbidden in production source.' },
  { regex: /\bstub\b/iu, message: 'stub markers are forbidden in production source.' },
  { regex: /\bfake\b/iu, message: 'fake markers are forbidden in production source.' },
  { regex: /\btemporary\b/iu, message: 'temporary markers are forbidden in production source.' },
  { regex: /\bfor now\b/iu, message: 'for now markers are forbidden in production source.' },
  { regex: /\bscaffold[- ]only\b/iu, message: 'scaffold-only markers are forbidden in production source.' },
];
const directPatterns = [
  { regex: /throw new Error\(\s*['"`]not implemented['"`]\s*\)/u, message: 'not implemented throws are forbidden.' },
  { regex: /return null as any/u, message: 'return null as any is forbidden.' },
  { regex: /return \{\s*\} as any/u, message: 'return {} as any is forbidden.' },
  { regex: /\btodo!\s*\(\s*\)/u, message: 'todo!() is forbidden in production source.' },
  { regex: /\bunimplemented!\s*\(\s*\)/u, message: 'unimplemented!() is forbidden in production source.' },
  { regex: /panic!\(\s*['"`]not implemented['"`]\s*\)/u, message: 'panic!(\"not implemented\") is forbidden.' },
  { regex: /\bdbg!\s*\(/u, message: 'dbg!() is forbidden in production source.' },
  { regex: /\bprintln!\s*\(/u, message: 'println!() is forbidden in production source.' },
  { regex: /\beprintln!\s*\(/u, message: 'eprintln!() is forbidden in production source.' },
  { regex: /\bunreachable!\s*\(/u, message: 'unreachable!() is forbidden without a deliberate owner exception.' },
];

function isProductionSource(filePath) {
  const extension = path.extname(filePath);
  if (!supportedExtensions.has(extension)) {
    return false;
  }
  if (filePath.includes('/tests/') || filePath.startsWith('scripts/test/')) {
    return false;
  }
  if (allowedScriptPattern.test(filePath) || filePath === scriptPath) {
    return false;
  }
  return (
    filePath.startsWith('apps/') ||
    filePath.startsWith('packages/') ||
    filePath.startsWith('crates/') ||
    filePath.startsWith('scripts/')
  );
}

function isCommentLine(line) {
  return /^\s*(?:\/\/|\/\*|\*|#(?!\[|!\[))/u.test(line);
}

function collectFindings(filePath) {
  const findings = [];
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  lines.forEach((line, index) => {
    for (const pattern of directPatterns) {
      if (pattern.regex.test(line)) {
        findings.push(`${filePath}:${index + 1} ${pattern.message}`);
      }
    }
    if (!isCommentLine(line)) {
      return;
    }
    for (const pattern of commentPatterns) {
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
    acceptPath: isProductionSource,
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const findings = scope.files.flatMap((filePath) => collectFindings(filePath));
  if (findings.length > 0) {
    console.error('Placeholder implementation guard failed. Production source must not claim work that is not implemented.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Placeholder implementation guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
