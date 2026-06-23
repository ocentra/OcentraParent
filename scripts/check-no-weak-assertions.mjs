import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { readRepoFile, repoRelativePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const scriptPath = repoRelativePath(fileURLToPath(import.meta.url));
const allowedPaths = new Set([scriptPath]);
const supportedExtensions = new Set(['.js', '.jsx', '.ts', '.tsx', '.mjs', '.mts', '.cjs', '.cts', '.rs']);
const scriptName = 'node scripts/check-no-weak-assertions.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const patterns = [
  {
    regex: /\.toBeDefined\s*\(/u,
    message: 'toBeDefined() is too weak for architecture-gated tests.',
  },
  {
    regex: /\.toBeTruthy\s*\(/u,
    message: 'toBeTruthy() is too weak for architecture-gated tests.',
  },
  {
    regex: /\.toBeFalsy\s*\(/u,
    message: 'toBeFalsy() is too weak for architecture-gated tests.',
  },
  {
    regex: /\.not\.toThrow\s*\(/u,
    message: 'not.toThrow() is too weak for architecture-gated tests.',
  },
  {
    regex: /\.toMatchObject\s*\(\s*\{\s*\}\s*\)/u,
    message: 'empty toMatchObject({}) assertions are forbidden.',
  },
  {
    regex: /expect\.anything\s*\(\s*\)/u,
    message: 'expect.anything() is too weak for architecture-gated tests.',
  },
  {
    regex: /expect\.any\s*\(\s*(?:String|Number)\s*\)/u,
    message: 'expect.any(String|Number) is too weak for architecture-gated tests.',
  },
  {
    regex: /assert!\(\s*[\w.]+\s*\.is_some\(\)\s*\)/u,
    message: 'assert!(value.is_some()) is too weak for architecture-gated tests.',
  },
  {
    regex: /assert!\(\s*[\w.]+\s*\.is_ok\(\)\s*\)/u,
    message: 'assert!(result.is_ok()) is too weak for architecture-gated tests.',
  },
  {
    regex: /assert!\(\s*[\w.]+\s*\.is_err\(\)\s*\)/u,
    message: 'assert!(result.is_err()) is too weak for architecture-gated tests.',
  },
  {
    regex: /assert!\(\s*[^)]+\.len\(\)\s*>\s*0\s*\)/u,
    message: 'length > 0 assertions are too weak for architecture-gated tests.',
  },
  {
    regex: /assert!\(\s*!\s*[^)]+\.is_empty\(\)\s*\)/u,
    message: '!is_empty() assertions are too weak for architecture-gated tests.',
  },
  {
    regex: /assert!\(\s*[^)]+\.contains\([^)]+\)\s*\)/u,
    message: 'contains() assertions are too weak for architecture-gated tests.',
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
    if (line.includes('matches!(')) {
      return;
    }
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
    console.error('Weak assertion guard failed. Tests must assert concrete behavior.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Weak assertion guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
