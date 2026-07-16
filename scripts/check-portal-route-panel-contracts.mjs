import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

import { repoAbsolutePath, resolveScopedFiles } from './check-architecture-scope.mjs';

const routePanelDir = join(process.cwd(), 'apps', 'portal', 'src');
const routePanelFilePattern = /RoutePanel\.tsx$/u;
const routeDecisionPattern =
  /\broute\s*(?:={2,3}|!={1,2})\s*PortalRoute\.|\bPortalRoute\.\w+\s*(?:={2,3}|!={1,2})\s*route\b/u;
const findings = [];
const scriptName = 'node scripts/check-portal-route-panel-contracts.mjs';
const usageLines = ['--all', '--base <sha> --head <sha>'];

function collectFullFiles() {
  const files = [];
  for (const entry of readdirSync(routePanelDir)) {
    if (!routePanelFilePattern.test(entry)) {
      continue;
    }
    files.push(join(routePanelDir, entry));
  }
  return files;
}

function collectScopedFiles(rawArgs) {
  const scope = resolveScopedFiles(rawArgs, {
    scriptName,
    usageLines,
    roots: ['apps/portal/src'],
    acceptPath: (filePath) => routePanelFilePattern.test(filePath),
  });

  if (scope.mode === 'skip') {
    return [];
  }

  return scope.files.map((filePath) => repoAbsolutePath(filePath));
}

function inspectFiles(files) {
  findings.length = 0;

  for (const path of files) {
    const entry = path.split(/[\\/]/u).pop() ?? path;
    const text = readFileSync(path, 'utf8');
    if (routeDecisionPattern.test(text)) {
      findings.push(`apps/portal/src/${entry}: route panel must use portal-domain route predicate contracts`);
    }
  }
}

function main(rawArgs = process.argv.slice(2)) {
  inspectFiles(rawArgs.length === 0 ? collectFullFiles() : collectScopedFiles(rawArgs));

  if (findings.length > 0) {
    console.error('Portal route panels must not own product route decisions.');
    console.error('Move route ownership to packages/portal-domain/src/routes.ts and import predicate helpers.');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log('Portal route panel contract check passed.');
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  main();
}
