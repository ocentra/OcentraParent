import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();
const packageTestRoot = join(repoRoot, 'scripts', 'release', 'windows', 'parent-protected-custody', 'tests');

const packageSuites = ['package-contract.Tests.ps1', 'package-publication.Tests.ps1', 'package-journal.Tests.ps1'];

test('Parent WP12 executes the real Windows package contract and lifecycle suites', (context) => {
  if (process.platform !== 'win32') {
    context.skip('the WP12 package and PowerShell runtime are Windows-only');
    return;
  }

  for (const suite of packageSuites) {
    const suitePath = join(packageTestRoot, suite);
    assert.equal(existsSync(suitePath), true, `missing WP12 PowerShell suite ${suite}`);

    const result = spawnSync('pwsh', ['-NoLogo', '-NoProfile', '-NonInteractive', '-File', suitePath], {
      cwd: repoRoot,
      encoding: 'utf8',
      windowsHide: true,
    });

    assert.equal(
      result.error,
      undefined,
      `pwsh could not execute ${suite}: ${result.error?.message ?? '<unknown error>'}`
    );
    assert.equal(
      result.status,
      0,
      `${suite} failed with exit code ${result.status}.\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
    assert.match(result.stdout, /^PASS:/mu, `${suite} did not report its behavioral pass result`);
  }
});
