import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { readRepoFile, resolveScopedFiles } from './check-architecture-scope.mjs';

const scriptName = 'node scripts/check-rust-string-boundaries.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const ownerPathPatterns = [
  /^crates\/agent-protocol\/src\//u,
  /^crates\/ocentra-eventing\/src\//u,
  /^crates\/ocentra-evidence\/src\//u,
  /^crates\/ocentra-network-evidence\/src\//u,
  /^crates\/agent-updater\/src\//u,
  /^crates\/logging-core\/src\//u,
  /^crates\/schema\/src\//u,
  /^crates\/[^/]+-(?:core|adapter|runtime)\/src\//u,
];
const lineAllowPatterns = [
  /env!\(/u,
  /#\[tokio::main/u,
  /#\[serde/u,
  /serde\(/u,
  /cfg\(/u,
  /#\[path\s*=/u,
  /panic!\(/u,
  /format!\(/u,
  /^\s*(?:pub\s+)?const\s+[A-Z0-9_]+\s*:\s*&str\s*=\s*"/u,
];
const cfgTestAttributePattern = /^\s*#\[cfg\(test\)\]/u;
const stackedAttributePattern = /^\s*#\[/u;
const testScopedPathPattern = /(^|\/)[^/]*_tests?\/|_tests?\.rs$|_test_support\.rs$|_test_fixtures?\.rs$|\/tests?\//u;

function isGuardedRustSource(filePath) {
  return (
    path.extname(filePath) === '.rs' &&
    filePath.startsWith('crates/') &&
    filePath.includes('/src/') &&
    !testScopedPathPattern.test(filePath) &&
    !ownerPathPatterns.some((pattern) => pattern.test(filePath))
  );
}

function collectFindings(filePath) {
  const findings = [];
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  let pendingCfgTestScope = false;
  let skippedCfgTestBlockDepth = 0;
  lines.forEach((line, index) => {
    if (skippedCfgTestBlockDepth > 0) {
      skippedCfgTestBlockDepth += braceDelta(line);
      return;
    }

    if (cfgTestAttributePattern.test(line)) {
      pendingCfgTestScope = true;
      return;
    }

    if (pendingCfgTestScope) {
      if (stackedAttributePattern.test(line)) {
        return;
      }
      const delta = braceDelta(line);
      if (delta > 0) {
        skippedCfgTestBlockDepth = delta;
      }
      pendingCfgTestScope = false;
      return;
    }

    if (lineAllowPatterns.some((pattern) => pattern.test(line))) {
      return;
    }
    if (/"(?:[^"\\]|\\.)*"/u.test(line)) {
      findings.push(`${filePath}:${index + 1} runtime Rust source cannot contain inline string literals.`);
    }
  });
  return findings;
}

function braceDelta(line) {
  return (line.match(/\{/gu) ?? []).length - (line.match(/\}/gu) ?? []).length;
}

export function main(rawArgs = process.argv.slice(2)) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['crates'],
    acceptPath: isGuardedRustSource,
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const findings = scope.files.flatMap((filePath) => collectFindings(filePath));
  if (findings.length > 0) {
    console.error(
      'Rust string boundary guard failed. Runtime/core Rust source cannot introduce inline strings outside explicit owner crates.'
    );
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Rust string boundary guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
