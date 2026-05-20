import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();

function readRepoFile(path) {
  return readFileSync(join(repoRoot, path), 'utf8');
}

test('production release workflow publishes only from production branch', () => {
  const workflow = readRepoFile('.github/workflows/release.yml');

  assert.match(workflow, /branches:\s+- production/u);
  assert.match(workflow, /release-decision:/u);
  assert.match(workflow, /node scripts\/release\/decide-production-release\.mjs/u);
  assert.match(workflow, /if: needs\.release-decision\.outputs\.release-required == 'true'/u);
  assert.match(workflow, /Build signed Windows MSI package/u);
  assert.match(workflow, /Check production release secrets/u);
  assert.match(workflow, /OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64/u);
  assert.match(workflow, /scripts\/smoke\/windows-msi-smoke\.ps1/u);
});

test('package preview workflow builds every scaffolded platform', () => {
  const workflow = readRepoFile('.github/workflows/package-preview.yml');

  for (const jobName of ['windows-msi', 'linux-deb', 'macos-pkg', 'android-apk', 'ios-simulator']) {
    assert.match(workflow, new RegExp(`${jobName}:`, 'u'));
  }
  assert.match(workflow, /OCENTRA_PARENT_ALLOW_EPHEMERAL_UPDATE_KEY: 'true'/u);
  assert.match(workflow, /scripts\/release\/linux\/build-agent-package\.sh/u);
  assert.match(workflow, /scripts\/release\/macos\/build-agent-package\.sh/u);
  assert.match(workflow, /scripts\/release\/android\/build-agent-package\.mjs/u);
  assert.match(workflow, /scripts\/release\/ios\/build-simulator-app\.sh/u);
  assert.match(workflow, /scripts\/smoke\/windows-msi-smoke\.ps1/u);
  assert.match(workflow, /scripts\/smoke\/linux-deb-smoke\.sh/u);
  assert.match(workflow, /scripts\/smoke\/macos-pkg-smoke\.sh/u);
  assert.match(workflow, /scripts\/smoke\/android-apk-smoke\.sh/u);
  assert.match(workflow, /scripts\/smoke\/ios-simulator-smoke\.sh/u);
  assert.match(workflow, /reactivecircus\/android-emulator-runner@v2/u);
});

test('dependency policy workflow audits dependencies and writes SBOM metadata', () => {
  const workflow = readRepoFile('.github/workflows/dependency-policy.yml');
  const packageJson = readRepoFile('package.json');

  assert.match(workflow, /cargo install cargo-audit --locked/u);
  assert.match(workflow, /npm run security:deps/u);
  assert.match(workflow, /npm run security:sbom/u);
  assert.match(workflow, /target\/security\/\*\.json/u);
  assert.match(readRepoFile('scripts/security/write-sbom.mjs'), /--sbom-format=cyclonedx/u);
  assert.match(packageJson, /"security:deps": "node scripts\/security\/check-dependency-policy\.mjs"/u);
  assert.match(packageJson, /"security:sbom": "node scripts\/security\/write-sbom\.mjs"/u);
});

test('toolchains are pinned for Rust and Android packaging', () => {
  const rustToolchain = readRepoFile('rust-toolchain.toml');
  const setupCi = readRepoFile('.github/actions/setup-ci/action.yml');
  const androidBuilder = readRepoFile('scripts/release/android/build-agent-package.mjs');
  const gradleWrapper = readRepoFile('platforms/android/agent/gradle/wrapper/gradle-wrapper.properties');

  assert.match(rustToolchain, /channel = "1\.90\.0"/u);
  assert.match(setupCi, /rust-toolchain\.toml/u);
  assert.match(androidBuilder, /gradlew\.bat assembleDebug/u);
  assert.match(androidBuilder, /\.\/gradlew/u);
  assert.match(gradleWrapper, /gradle-8\.12\.1-bin\.zip/u);
});

test('Linux and macOS packages install real service managers', () => {
  const linuxUnit = readRepoFile('scripts/release/linux/ocentra-parent-agent.service');
  const macLaunchd = readRepoFile('scripts/release/macos/ca.ocentra.parent.agent.plist');

  assert.match(linuxUnit, /ExecStart=\/opt\/ocentra\/ocentra-parent-agent\/bin\/ocentra-parent-agent-service/u);
  assert.match(linuxUnit, /WantedBy=multi-user\.target/u);
  assert.match(macLaunchd, /ca\.ocentra\.parent\.agent/u);
  assert.match(macLaunchd, /\/Library\/Ocentra\/Ocentra Parent Agent\/bin\/ocentra-parent-agent-service/u);
});

test('mobile platform projects define real installable app targets', () => {
  const androidManifest = readRepoFile('platforms/android/agent/app/src/main/AndroidManifest.xml');
  const iosProject = readRepoFile('platforms/ios/OcentraParentAgent.xcodeproj/project.pbxproj');

  assert.match(androidManifest, /android\.intent\.action\.MAIN/u);
  assert.match(androidManifest, /OcentraParentAgentService/u);
  assert.match(androidManifest, /foregroundServiceType="dataSync"/u);
  assert.match(iosProject, /productType = "com\.apple\.product-type\.application"/u);
  assert.match(iosProject, /PRODUCT_BUNDLE_IDENTIFIER = ca\.ocentra\.parent\.agent/u);
});
