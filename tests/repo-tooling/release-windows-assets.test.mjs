import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();
const releaseRoot = join(repoRoot, 'scripts', 'release', 'windows');

function readReleaseFile(name) {
  return readFileSync(join(releaseRoot, name), 'utf8');
}

test('Windows service wrapper config uses the Ocentra Child Agent service identity', () => {
  const config = readReleaseFile('OcentraParentAgentService.xml');

  assert.match(config, /<id>OcentraChildAgent<\/id>/u);
  assert.match(config, /<name>Ocentra Child Agent<\/name>/u);
  assert.match(config, /ocentra-child-agent-service\.exe/u);
});

test('Windows updater wrapper config uses a separate signed updater identity', () => {
  const config = readReleaseFile('OcentraParentUpdaterService.xml');

  assert.match(config, /<id>OcentraChildUpdater<\/id>/u);
  assert.match(config, /<name>Ocentra Child Updater<\/name>/u);
  assert.match(config, /ocentra-child-agent-updater\.exe/u);
  assert.match(config, /run-loop/u);
});

test('Windows service wrapper configs use WinSW v2 compatible logging', () => {
  const configs = [
    readReleaseFile('OcentraParentAgentService.xml'),
    readReleaseFile('OcentraParentUpdaterService.xml'),
  ];

  for (const config of configs) {
    assert.match(config, /<log mode="roll"><\/log>/u);
    assert.doesNotMatch(config, /roll-by-size-time/u);
    assert.doesNotMatch(config, /sizeThreshold/u);
    assert.doesNotMatch(config, /keepFiles/u);
  }
});

test('Windows MSI definition installs the Ocentra Child Agent service identity', () => {
  const installer = readReleaseFile('OcentraParentAgent.wxs');

  assert.match(installer, /UpgradeCode="0143F5A1-4C10-4C0F-97BE-55EDAF5012BB"/u);
  assert.match(installer, /Name="OcentraChildAgent"/u);
  assert.match(installer, /DisplayName="Ocentra Child Agent"/u);
  assert.match(installer, /Name="OcentraChildUpdater"/u);
  assert.match(installer, /DisplayName="Ocentra Child Updater"/u);
  assert.match(installer, /FirstFailureActionType="restart"/u);
});

test('Windows release package builder emits MSI, bootstrap, manifest, and checksum assets', () => {
  const builder = readReleaseFile('build-agent-package.ps1');

  assert.match(builder, /ocentra-child-agent-windows-x64-v\$Version\.msi/u);
  assert.match(builder, /OCENTRA_CHILD_UPDATE_SIGNING_KEY_BASE64/u);
  assert.match(builder, /OCENTRA_CHILD_ALLOW_EPHEMERAL_UPDATE_KEY/u);
  assert.match(builder, /AllowEphemeralSigningKey/u);
  assert.match(builder, /sign-manifest/u);
  assert.match(builder, /WixToolset\.Util\.wixext\/6\.0\.2/u);
  assert.match(builder, /\$extensionList = @\(dotnet wix extension list\)/u);
  assert.match(builder, /\$extensionList\.Count -eq 0/u);
  assert.match(builder, /winsw\/winsw\/releases\/download\/v\$WinSwVersion\/\$WinSwAssetName/u);
  assert.match(builder, /05B82D46AD331CC16BDC00DE5C6332C1EF818DF8CEEFCD49C726553209B3A0DA/u);
  assert.match(builder, /latest-windows\.json/u);
  assert.match(builder, /ocentra-child-agent-windows-x64-latest\.msi/u);
  assert.match(builder, /install-ocentra-child-agent-windows\.ps1/u);
  assert.match(builder, /\.sha256/u);
});

test('Windows latest installer consumes MSI release assets', () => {
  const installer = readReleaseFile('install-latest-windows.ps1');

  assert.match(installer, /Release manifest signature verification failed/u);
  assert.match(installer, /manifest\.payload\.installer\.type -ne 'msi'/u);
  assert.match(installer, /msiexec\.exe/u);
  assert.match(installer, /\/passive/u);
  assert.match(installer, /\/qn/u);
});

test('Windows GitHub release publisher uploads stable latest MSI assets', () => {
  const publisher = readReleaseFile('create-github-release.ps1');

  assert.match(publisher, /ocentra-child-agent-windows-x64-latest\.msi/u);
  assert.match(publisher, /LatestArtifactPath/u);
  assert.match(publisher, /LatestChecksumPath/u);
  assert.match(publisher, /gh release create \$Tag .* \$LatestArtifactPath \$LatestChecksumPath/u);
});

test('Windows package lifecycle proof harness emits local proof states without automatic reboot', () => {
  const harness = readReleaseFile('package-lifecycle-proof.mjs');
  const host = readReleaseFile('package-lifecycle-host.mjs');
  const runner = readReleaseFile('package-lifecycle-runner.mjs');

  assert.match(harness, /test-results['"], ['"]windows-package-lifecycle-proof/u);
  assert.match(harness, /windows-package-lifecycle-decision/u);
  assert.match(runner, /requires-elevated-shell/u);
  assert.match(runner, /admin-required/u);
  assert.match(host, /\/qn', '\/norestart/u);
  assert.match(host, /DEFAULT_HEALTH_URL = 'http:\/\/127\.0\.0\.1:4477\/health'/u);
  assert.doesNotMatch(host, /Restart-Computer/u);
  assert.doesNotMatch(host, /shutdown\.exe/u);
});
