import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { readRepoFile, resolveScopedFiles } from './check-architecture-scope.mjs';

const scriptName = 'node scripts/check-rust-domain-primitives.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const ownerPrefixes = [
  'crates/agent-protocol/src/',
  'crates/ocentra-eventing/src/',
  'crates/ocentra-evidence/src/',
  'crates/ocentra-network-evidence/src/',
  'crates/agent-updater/src/',
  'crates/logging-core/src/',
];
const testScopedPathPattern = /(^|\/)[^/]*_tests?\/|_tests?\.rs$|_test_support\.rs$|_test_fixtures?\.rs$|\/tests?\//u;
const suspiciousTypePattern = '(?:String|&str|Cow<[^>]*str[^>]*>|Option<String>|Vec<String>|HashMap<String\\s*,[^>]+>)';
const publicSerdeStructPattern = /^\s*pub\s+struct\s+\w+/u;
const publicFieldPattern = new RegExp(
  `^\\s*pub\\s+(?<name>[A-Za-z_][A-Za-z0-9_]*)\\s*:\\s*(?<type>${suspiciousTypePattern})`,
  'u'
);
const serializeDerivePattern = /^\s*#\[derive\([^#\]]*\b(?:Serialize|Deserialize)\b[^#\]]*\)\]/u;
const serdeShapeAttributePattern = /^\s*#\[serde\(/u;
const stackedAttributePattern = /^\s*#\[/u;

function normalizedNameTokens(name) {
  return name
    .replace(/([a-z0-9])([A-Z])/gu, '$1_$2')
    .toLowerCase()
    .split(/[^a-z0-9]+/u)
    .filter(Boolean);
}

function isSuspiciousName(name) {
  const tokens = normalizedNameTokens(name);
  const lastToken = tokens.at(-1);
  const secondToLastToken = tokens.at(-2);
  return (
    lastToken === 'id' ||
    lastToken === 'ids' ||
    lastToken === 'ref' ||
    lastToken === 'refs' ||
    (secondToLastToken === 'event' && lastToken === 'type') ||
    (secondToLastToken === 'command' && lastToken === 'type')
  );
}

function isRustSource(filePath) {
  return (
    path.extname(filePath) === '.rs' &&
    filePath.startsWith('crates/') &&
    filePath.includes('/src/') &&
    !testScopedPathPattern.test(filePath) &&
    !ownerPrefixes.some((prefix) => filePath.startsWith(prefix))
  );
}

function braceDelta(line) {
  return (line.match(/\{/gu) ?? []).length - (line.match(/\}/gu) ?? []).length;
}

function collectFindings(filePath) {
  const findings = [];
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  let pendingSerializeDerive = false;
  let pendingSerdeShape = false;
  let trackedSerdeStructDepth = 0;

  lines.forEach((line, index) => {
    if (trackedSerdeStructDepth === 0) {
      if (serializeDerivePattern.test(line)) {
        pendingSerializeDerive = true;
        return;
      }

      if (serdeShapeAttributePattern.test(line)) {
        pendingSerdeShape = true;
        return;
      }

      if (pendingSerializeDerive || pendingSerdeShape) {
        if (stackedAttributePattern.test(line)) {
          return;
        }
        const shouldTrackSerdeStruct =
          pendingSerializeDerive && pendingSerdeShape && publicSerdeStructPattern.test(line);
        pendingSerializeDerive = false;
        pendingSerdeShape = false;
        if (shouldTrackSerdeStruct) {
          trackedSerdeStructDepth = braceDelta(line);
        }
        return;
      }

      return;
    }

    const match = publicFieldPattern.exec(line);
    if (match !== null && isSuspiciousName(match.groups.name)) {
      findings.push(
        `${filePath}:${index + 1} serialized public struct fields outside owner crates must use typed domain newtypes or enums instead of raw primitives.`
      );
    }

    trackedSerdeStructDepth += braceDelta(line);
  });
  return findings;
}

export function main(rawArgs = process.argv.slice(2)) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['crates'],
    acceptPath: isRustSource,
  });

  if (scope.mode === 'skip') {
    console.log(scope.reason);
    return;
  }

  const findings = scope.files.flatMap((filePath) => collectFindings(filePath));
  if (findings.length > 0) {
    console.error('Rust domain primitive guard failed. Raw domain-bearing primitives are forbidden.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(`Rust domain primitive guard passed for ${scope.files.length} file(s).`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
