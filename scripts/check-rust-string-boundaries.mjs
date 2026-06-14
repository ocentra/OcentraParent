import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { readRepoFile, resolveScopedFiles } from './check-architecture-scope.mjs';

const scriptName = 'node scripts/check-rust-string-boundaries.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const ownerPrefixes = [
  'crates/agent-protocol/src/',
  'crates/ocentra-eventing/src/',
  'crates/ocentra-evidence/src/',
  'crates/ocentra-network-evidence/src/',
  'crates/agent-updater/src/',
];
const lineAllowPatterns = [/env!\(/u, /#\[tokio::main/u, /#\[serde/u, /serde\(/u, /cfg\(/u];

function isGuardedRustSource(filePath) {
  return (
    path.extname(filePath) === '.rs' &&
    filePath.startsWith('crates/') &&
    filePath.includes('/src/') &&
    !ownerPrefixes.some((prefix) => filePath.startsWith(prefix))
  );
}

function collectFindings(filePath) {
  const findings = [];
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  lines.forEach((line, index) => {
    if (lineAllowPatterns.some((pattern) => pattern.test(line))) {
      return;
    }
    if (/"(?:[^"\\]|\\.)*"/u.test(line)) {
      findings.push(`${filePath}:${index + 1} runtime Rust source cannot contain inline string literals.`);
    }
  });
  return findings;
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
