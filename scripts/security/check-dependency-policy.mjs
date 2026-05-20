import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const allowedExternalLicenses = new Set([
  '0BSD',
  'Apache-2.0',
  'BSD-2-Clause',
  'BSD-3-Clause',
  'BlueOak-1.0.0',
  'ISC',
  'MIT',
  'MPL-2.0',
  'Python-2.0',
]);

run('npm', ['audit', '--audit-level=high']);
checkNodeLicensePolicy();
run('cargo', ['audit', '--deny', 'warnings']);

console.log('Dependency security and license policy passed.');

function checkNodeLicensePolicy() {
  const lock = JSON.parse(readFileSync('package-lock.json', 'utf8'));
  const findings = [];

  for (const [path, packageEntry] of Object.entries(lock.packages ?? {})) {
    if (!path.includes('node_modules')) {
      continue;
    }

    const packageName = path.split('node_modules/').at(-1);
    if (packageName?.startsWith('@ocentra-parent/')) {
      continue;
    }

    const license = packageEntry.license;
    if (typeof license !== 'string' || !allowedExternalLicenses.has(license)) {
      findings.push(`${path}: ${license ?? 'MISSING'}`);
    }
  }

  if (findings.length > 0) {
    console.error('Disallowed or missing external npm package licenses:');
    for (const finding of findings) {
      console.error(finding);
    }
    process.exit(1);
  }

  console.log(
    `Node dependency license policy passed for ${Object.keys(lock.packages ?? {}).length} package lock entries.`
  );
}

function run(command, args) {
  const result = spawnSync(command, args, {
    encoding: 'utf8',
    shell: process.platform === 'win32',
    stdio: 'inherit',
  });

  if (result.error) {
    throw result.error;
  }

  if ((result.status ?? 1) !== 0) {
    process.exit(result.status ?? 1);
  }
}
