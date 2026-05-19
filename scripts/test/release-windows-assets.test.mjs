import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();
const releaseRoot = join(repoRoot, 'scripts', 'release', 'windows');

function readReleaseFile(name) {
  return readFileSync(join(releaseRoot, name), 'utf8');
}

test('Windows service wrapper config uses the Ocentra Parent service identity', () => {
  const config = readReleaseFile('OcentraParentAgentService.xml');

  assert.match(config, /<id>OcentraParentAgent<\/id>/u);
  assert.match(config, /<name>Ocentra Parent Agent<\/name>/u);
  assert.match(config, /ocentra-parent-agent-service\.exe/u);
});

test('Windows installer pins the service wrapper download by hash', () => {
  const installer = readReleaseFile('install-service.ps1');

  assert.match(installer, /winsw\/winsw\/releases\/download\/v2\.12\.0\/WinSW-x64\.exe/u);
  assert.match(installer, /05B82D46AD331CC16BDC00DE5C6332C1EF818DF8CEEFCD49C726553209B3A0DA/u);
});

test('Windows release package builder emits bootstrap, manifest, and checksum assets', () => {
  const builder = readReleaseFile('build-agent-package.ps1');

  assert.match(builder, /latest-windows\.json/u);
  assert.match(builder, /install-ocentra-parent-agent-windows\.ps1/u);
  assert.match(builder, /\.sha256/u);
});
