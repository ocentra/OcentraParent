import assert from 'node:assert/strict';
import test from 'node:test';
import { readFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../../..');
const read = (path) => readFile(resolve(root, path), 'utf8');

const project = await read('platforms/ios/OcentraChildAgent.xcodeproj/project.pbxproj');
const scheme = await read('platforms/ios/OcentraChildAgent.xcodeproj/xcshareddata/xcschemes/OcentraChildAgent.xcscheme');
const plist = await read('platforms/ios/OcentraChildAgent/Info.plist');
const status = await read('platforms/ios/OcentraChildAgent/AgentStatusViewController.swift');
const release = await read('scripts/release/ios/build-simulator-app.sh');

test('canonical child identity is wired across project, scheme, plist, and status', () => {
  assert.match(project, /PRODUCT_BUNDLE_IDENTIFIER = ca\.ocentra\.child\.agent;/);
  assert.match(project, /PRODUCT_NAME = OcentraChildAgent;/);
  assert.match(scheme, /BuildableName="OcentraChildAgent\.app"/);
  assert.match(scheme, /BlueprintName="OcentraChildAgent"/);
  assert.match(plist, /<key>CFBundleIdentifier<\/key>/);
  assert.match(status, /ca\.ocentra\.child\.agent/);
});

test('release inputs are simulator-only, unsigned, and child-artifact named', () => {
  assert.match(release, /OcentraChildAgent\.xcodeproj/);
  assert.match(release, /-scheme OcentraChildAgent/);
  assert.match(release, /-sdk iphonesimulator/);
  assert.match(release, /CODE_SIGNING_ALLOWED=NO/);
  assert.match(release, /ocentra-child-agent-ios-simulator-v/);
});

test('capability-only manual-required and no-claim boundaries remain explicit', () => {
  for (const boundary of [
  'service-mode=capability-only',
  'launch-availability=manual-required',
  'recovery=not-implemented',
  'provisioning=manual-required',
  'supervision=manual-required',
  'signing=manual-required',
  'testflight=manual-required',
  'device-proof=manual-required',
  'daemon=not-claimed',
  'child-agent-parity=not-claimed',
  ]) assert.match(status, new RegExp(boundary.replace(/[=]/g, '\\=')));
});

test('all loaded artifacts reject parent identity reuse', () => {
  for (const artifact of [project, scheme, plist, status, release]) {
    assert.doesNotMatch(artifact, /OcentraParentAgent/);
    assert.doesNotMatch(artifact, /ca\.ocentra\.parent\.agent/);
  }
});
