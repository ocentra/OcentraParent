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
const appDelegate = await read('platforms/ios/OcentraChildAgent/AppDelegate.swift');
const status = await read('platforms/ios/OcentraChildAgent/AgentStatusViewController.swift');
const release = await read('scripts/release/ios/build-simulator-app.sh');
const generatedContract = await read('packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts');

test('canonical child identity is wired across project, scheme, plist, and status', () => {
  assert.match(project, /PRODUCT_BUNDLE_IDENTIFIER = ca\.ocentra\.child\.agent;/);
  assert.match(project, /PRODUCT_NAME = OcentraChildAgent;/);
  assert.match(project, /AgentStatusViewController\.swift in Sources/);
  assert.match(scheme, /BuildableName="OcentraChildAgent\.app"/);
  assert.match(scheme, /BlueprintName="OcentraChildAgent"/);
  assert.match(scheme, /ReferencedContainer="container:OcentraChildAgent\.xcodeproj"/);
  assert.match(plist, /<key>CFBundleIdentifier<\/key>/);
  assert.match(plist, /<string>Ocentra Child Agent<\/string>/);
  assert.match(appDelegate, /ChildAgentAppDelegate/);
  assert.match(appDelegate, /AgentStatusViewController\(\)/);
  assert.match(status, /ca\.ocentra\.child\.agent/);
  assert.match(generatedContract, /ca\.ocentra\.child\.agent/);
});

test('release inputs are simulator-only, unsigned, and child-artifact named', () => {
  assert.match(release, /OcentraChildAgent\.xcodeproj/);
  assert.match(release, /-scheme OcentraChildAgent/);
  assert.match(release, /-sdk iphonesimulator/);
  assert.match(release, /CODE_SIGNING_ALLOWED=NO/);
  assert.match(release, /ocentra-child-agent-ios-simulator-v/);
});

test('capability mapping reads real OS state and keeps fail-closed boundaries explicit', () => {
  for (const sourceObservation of [
    /Bundle\.main\.bundleIdentifier/,
    /UIApplication\.shared\.applicationState/,
    /UNUserNotificationCenter\.current\(\)\.getNotificationSettings/,
    /AuthorizationCenter\.shared\.authorizationStatus/,
    /UIBackgroundModes/,
    /UIApplication\.didBecomeActiveNotification/,
  ]) assert.match(status, sourceObservation);

  for (const capability of [
    'family-controls',
    'device-activity',
    'screen-time',
    'network-extension',
    'notifications',
    'background-execution',
    'provisioning',
    'supervision',
    'signing',
    'testflight',
    'device-proof',
  ]) assert.match(status, new RegExp(capability));

  for (const state of [
    'service-mode=capability-only',
    'launch-availability=manual-required',
    'recovery=not-implemented',
    'manual-required',
    'unavailable',
    'not-implemented',
    'not-claimed',
  ]) assert.match(status, new RegExp(state.replace(/[=]/g, '\\=')));

  assert.doesNotMatch(status, /ChildIosEntitlementCapabilityProof\.statusText/);
  assert.doesNotMatch(status, /UILabel/);
});

test('all loaded artifacts reject parent identity reuse', () => {
  for (const artifact of [project, scheme, plist, appDelegate, status, release, generatedContract]) {
    assert.doesNotMatch(artifact, /OcentraParentAgent/);
    assert.doesNotMatch(artifact, /ca\.ocentra\.parent\.agent/);
  }
});

test('iOS source does not declare an unproved background entitlement', () => {
  assert.doesNotMatch(plist, /<key>UIBackgroundModes<\/key>/);
  assert.match(status, /Info\.plist-declaration-missing/);
});
