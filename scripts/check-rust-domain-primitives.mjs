import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { readRepoFile, resolveScopedFiles } from './check-architecture-scope.mjs';

const scriptName = 'node scripts/check-rust-domain-primitives.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];
const suspiciousNamePattern =
  '(?:id|device_id|child_id|family_id|path|key|name|hash|url|route|label|title|status|version|event_type|command_type)';
const suspiciousTypePattern =
  '(?:String|&str|Cow<[^>]*str[^>]*>|Option<String>|Vec<String>|HashMap<String\\s*,[^>]+>)';
const fieldPattern = new RegExp(
  `\\b(?<name>${suspiciousNamePattern})\\s*:\\s*(?<type>${suspiciousTypePattern})`,
  'u'
);
const fnContextPattern = /^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+/u;

function isRustSource(filePath) {
  return path.extname(filePath) === '.rs' && filePath.startsWith('crates/') && filePath.includes('/src/');
}

function collectFindings(filePath) {
  const findings = [];
  const lines = readRepoFile(filePath).split(/\r?\n/u);
  lines.forEach((line, index) => {
    const match = fieldPattern.exec(line);
    if (match === null) {
      return;
    }

    const reason = fnContextPattern.test(line)
      ? 'function signatures must use typed domain newtypes or enums instead of raw primitives.'
      : 'struct and enum fields must use typed domain newtypes or enums instead of raw primitives.';
    findings.push(`${filePath}:${index + 1} ${reason}`);
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
