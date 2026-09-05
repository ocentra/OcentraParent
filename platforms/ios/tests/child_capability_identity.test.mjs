import { spawn } from 'node:child_process';
import test from 'node:test';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../../..');
const project = resolve(repositoryRoot, 'platforms/ios/OcentraChildAgent.xcodeproj');
const scheme = 'OcentraChildAgent';
const destination = process.env.OCENTRA_IOS_TEST_DESTINATION ?? 'platform=iOS Simulator';
const unavailableReason =
  'platform-unavailable/manual-required: iOS XCTest UI runtime requires macOS with Xcode and an iOS Simulator';

if (process.platform !== 'darwin') {
  test('child iOS capability status UI runtime', { skip: unavailableReason }, () => {});
} else {
  test('child iOS capability status UI runtime', async () => {
    await runXcodebuild([
      '-project',
      project,
      '-scheme',
      scheme,
      '-destination',
      destination,
      '-only-testing:OcentraChildAgentUITests/ChildCapabilityIdentityUITests/testCapabilityStatusReportsObservedFailClosedRuntimeStates',
      'test',
    ]);
  });
}

function runXcodebuild(args) {
  return new Promise((resolve, reject) => {
    const child = spawn('xcodebuild', args, { cwd: repositoryRoot, stdio: 'inherit' });
    child.once('error', reject);
    child.once('exit', (code, signal) => {
      if (code === 0) {
        resolve();
        return;
      }

      reject(new Error(`xcodebuild test exited with ${signal ?? `status ${code}`}`));
    });
  });
}
