import assert from 'node:assert/strict';
import { mkdtemp, readFile, readdir, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { after, describe, it } from 'node:test';
import {
  CHILD_IOS_CANONICAL_PROOF_ROOT,
  CHILD_IOS_LEGACY_PROOF_PATH,
  classifyIosXctestResult,
  writeChildIosProofOutputs,
} from '../../scripts/test/child-ios-entitlement-capability-proof-artifacts.mjs';

const temporaryRoots = [];

after(async () => {
  await Promise.all(temporaryRoots.map((root) => rm(root, { recursive: true, force: true })));
});

describe('child iOS entitlement capability proof runner', () => {
  it('records the real Node harness host skip as blocked evidence', () => {
    const outcome = classifyIosXctestResult({
      command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
      exitCode: 0,
      output:
        'ok 1 - child iOS capability status UI runtime # SKIP platform-unavailable/manual-required: iOS XCTest UI runtime requires macOS with Xcode and an iOS Simulator\n',
      platform: 'win32',
    });

    assert.deepEqual(outcome, {
      command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
      exitCode: 0,
      platform: 'win32',
      status: 'skipped',
      reason:
        'platform-unavailable/manual-required: iOS XCTest UI runtime requires macOS with Xcode and an iOS Simulator',
    });
  });

  it('distinguishes a real XCTest pass from a command failure', () => {
    assert.deepEqual(
      classifyIosXctestResult({
        command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
        exitCode: 0,
        output: 'ok 1 - child iOS capability status UI runtime\n',
        platform: 'darwin',
      }),
      {
        command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
        exitCode: 0,
        platform: 'darwin',
        status: 'passed',
        reason: 'The real child iOS XCTest command completed successfully on this host.',
      }
    );
    assert.deepEqual(
      classifyIosXctestResult({
        command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
        exitCode: 1,
        output: 'xcodebuild test exited with status 65\n',
        platform: 'darwin',
      }),
      {
        command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
        exitCode: 1,
        platform: 'darwin',
        status: 'failed',
        reason: 'xcodebuild test exited with status 65',
      }
    );
  });

  it('writes exactly the four canonical artifacts and retains the legacy JSON contract', async () => {
    const root = await mkdtemp(join(tmpdir(), 'ocentra-child-ios-proof-runner-'));
    temporaryRoots.push(root);
    const proof = childIosProofFixture();

    await writeChildIosProofOutputs(root, proof);

    const canonicalDirectory = join(root, ...CHILD_IOS_CANONICAL_PROOF_ROOT.split('/'));
    assert.deepEqual((await readdir(canonicalDirectory)).sort(), [
      '00-scope-summary.md',
      '01-negative-case-proof.md',
      '02-no-claim-boundary.md',
      '16-validation-commands.log',
    ]);
    const noClaim = await readFile(join(canonicalDirectory, '02-no-claim-boundary.md'), 'utf8');
    assert.match(noClaim, /does not claim Apple signing/);
    assert.match(noClaim, /physical-device launch/);
    const legacy = JSON.parse(await readFile(join(root, ...CHILD_IOS_LEGACY_PROOF_PATH.split('/')), 'utf8'));
    assert.deepEqual(legacy.xctestOutcome, proof.xctestOutcome);
    assert.equal(legacy.evidence.canonicalProofRoot, CHILD_IOS_CANONICAL_PROOF_ROOT);
  });
});

function childIosProofFixture() {
  return {
    schemaVersion: 1,
    checkedAt: '2026-09-02T00:00:00.000Z',
    commit: '0123456789abcdef',
    proofMode: 'child-ios-entitlement-capability-proof',
    commandResults: [
      {
        command: 'cargo test -p ocentra-schema --test contract child_ios_entitlement_capability',
        exitCode: 0,
        result: 'pass',
        artifact: 'n/a',
        notes: 'Rust child iOS capability contract test passed.',
      },
      {
        command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
        exitCode: 0,
        result: 'blocked',
        artifact: CHILD_IOS_LEGACY_PROOF_PATH,
        notes: 'platform-unavailable/manual-required',
      },
    ],
    evidence: {
      output: CHILD_IOS_LEGACY_PROOF_PATH,
      canonicalProofRoot: CHILD_IOS_CANONICAL_PROOF_ROOT,
    },
    runtimeReadModel: { bundleId: 'ca.ocentra.child.agent' },
    xctestOutcome: {
      command: 'node --test platforms/ios/tests/child_capability_identity.test.mjs',
      exitCode: 0,
      platform: 'win32',
      status: 'skipped',
      reason: 'platform-unavailable/manual-required',
    },
  };
}
