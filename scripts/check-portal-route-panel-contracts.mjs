import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

const routePanelDir = join(process.cwd(), 'apps', 'portal', 'src');
const routePanelFilePattern = /RoutePanel\.tsx$/u;
const routeDecisionPattern =
  /\broute\s*(?:={2,3}|!={1,2})\s*PortalRoute\.|\bPortalRoute\.\w+\s*(?:={2,3}|!={1,2})\s*route\b/u;
const findings = [];

for (const entry of readdirSync(routePanelDir)) {
  if (!routePanelFilePattern.test(entry)) {
    continue;
  }
  const path = join(routePanelDir, entry);
  const text = readFileSync(path, 'utf8');
  if (routeDecisionPattern.test(text)) {
    findings.push(`apps/portal/src/${entry}: route panel must use portal-domain route predicate contracts`);
  }
}

if (findings.length > 0) {
  console.error('Portal route panels must not own product route decisions.');
  console.error('Move route ownership to packages/portal-domain/src/routes.ts and import predicate helpers.');
  for (const finding of findings) {
    console.error(finding);
  }
  process.exit(1);
}

console.log('Portal route panel contract check passed.');
