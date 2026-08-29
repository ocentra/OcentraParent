import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import { spawnSync } from 'node:child_process';
import { test } from 'node:test';

import { createParentDesktopDevEnv } from '../../scripts/dev/dev-parent-desktop.mjs';
import {
  ParentDevEnv,
  ParentDevNetworkMode,
  resolveParentDevNetworkConfig,
} from '../../scripts/dev/local-dev-config.mjs';

const repoRoot = process.cwd();

function readRepoFile(path) {
  return readFileSync(join(repoRoot, path), 'utf8');
}

function readEnforcerProfileProofScript(relativePath) {
  const candidateRoots = [
    process.env.OCENTRA_ENFORCER_HOME,
    join(repoRoot, 'node_modules', 'ocentra-enforcer'),
    join(repoRoot, '..', 'ocentra-enforcer'),
    'E:\\ocentra-enforcer',
  ].filter(Boolean);
  for (const root of candidateRoots) {
    try {
      return readFileSync(join(root, 'profiles', 'ocentra-parent', 'legacy-scripts', relativePath), 'utf8');
    } catch {
      // Try the next configured Enforcer install path.
    }
  }
  throw new Error(`Unable to find migrated Enforcer proof script ${relativePath}`);
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

test('manual production release requires an explicit production ref', () => {
  const workflow = readRepoFile('.github/workflows/release.yml');

  assert.match(
    workflow,
    /workflow_dispatch:[\s\S]*?inputs:[\s\S]*?publish:[\s\S]*?github\.ref[\s\S]*?refs\/heads\/production/u
  );
  assert.match(
    workflow,
    /github\.event_name[\s\S]*?workflow_dispatch[\s\S]*?inputs\.publish[\s\S]*?github\.ref[\s\S]*?refs\/heads\/production/u
  );
});

test('production release decision rejects non-production refs and accepts production creation', () => {
  const root = mkdtempSync(join(tmpdir(), 'ocentra-release-decision-'));
  const eventPath = join(root, 'event.json');
  const outputPath = join(root, 'output.txt');
  writeFileSync(eventPath, JSON.stringify({ before: '0000000000000000000000000000000000000000' }));

  try {
    const run = (ref) =>
      spawnSync(process.execPath, ['scripts/release/decide-production-release.mjs'], {
        cwd: repoRoot,
        env: {
          ...process.env,
          GITHUB_EVENT_PATH: eventPath,
          GITHUB_OUTPUT: outputPath,
          GITHUB_REF: ref,
        },
        encoding: 'utf8',
      });

    const nonProduction = run('refs/heads/main');
    assert.equal(nonProduction.status, 0, nonProduction.stderr);
    assert.match(nonProduction.stdout, /release_required=false/u);
    assert.match(nonProduction.stdout, /reason=non-production-ref/u);

    const production = run('refs/heads/production');
    assert.equal(production.status, 0, production.stderr);
    assert.match(production.stdout, /release_required=false/u);
    assert.match(production.stdout, /reason=production-branch-created/u);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
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
  assert.match(workflow, /Enable KVM for Android emulator/u);
  assert.match(workflow, /emulator-boot-timeout: 900/u);
  assert.match(workflow, /Upload Windows MSI smoke logs/u);
});

test('package smoke scripts check real uninstall and emit diagnostics', () => {
  const linuxSmoke = readRepoFile('scripts/smoke/linux-deb-smoke.sh');
  const windowsSmoke = readRepoFile('scripts/smoke/windows-msi-smoke.ps1');

  assert.match(linuxSmoke, /\$\{db:Status-Abbrev\}/u);
  assert.match(linuxSmoke, /Agent executable remained after remove/u);
  assert.match(windowsSmoke, /windows-msi-install\.log/u);
  assert.match(windowsSmoke, /\/L\*v/u);
});

test('parent desktop Tauri package keeps built portal and Rust-service runtime boundaries', () => {
  const tauriConfig = JSON.parse(readRepoFile('apps/parent-desktop/src-tauri/tauri.conf.json'));
  const packageJson = JSON.parse(readRepoFile('apps/parent-desktop/package.json'));
  const network = resolveParentDevNetworkConfig(
    {
      [ParentDevEnv.AgentPort]: '4477',
      [ParentDevEnv.PortalPort]: '4478',
      [ParentDevEnv.DevNetworkMode]: ParentDevNetworkMode.Loopback,
    },
    {},
    ['node', 'platform-packaging.test.mjs']
  );
  const desktopEnv = createParentDesktopDevEnv(network, { EXISTING_ENV: 'preserved' });

  assert.equal(tauriConfig.build.frontendDist, '../../portal/dist');
  assert.equal(tauriConfig.build.devUrl, 'http://127.0.0.1:4478');
  assert.equal(tauriConfig.build.beforeDevCommand, 'npm run portal:dev');
  assert.equal(tauriConfig.app.security.csp.includes(':4477'), false);
  assert.equal(packageJson.scripts['tauri:check'], 'cargo check --manifest-path src-tauri/Cargo.toml');
  assert.equal(desktopEnv.EXISTING_ENV, 'preserved');
  assert.equal(desktopEnv[ParentDevEnv.AgentAddress], '127.0.0.1:4477');
  assert.equal(desktopEnv[ParentDevEnv.PortalPort], '4478');
  assert.equal(desktopEnv[ParentDevEnv.PortalAgentWebSocketUrl], 'ws://127.0.0.1:4477/api/dev/ws');
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

  assert.match(linuxUnit, /ExecStart=\/opt\/ocentra\/ocentra-child-agent\/bin\/ocentra-child-agent-service/u);
  assert.match(linuxUnit, /WantedBy=multi-user\.target/u);
  assert.match(macLaunchd, /ca\.ocentra\.child\.agent/u);
  assert.match(macLaunchd, /\/Library\/Ocentra\/Ocentra Child Agent\/bin\/ocentra-child-agent-service/u);
});

test('mobile platform projects define real installable app targets', () => {
  const androidManifest = readRepoFile('platforms/android/parent/app/src/main/AndroidManifest.xml');
  const iosProject = readRepoFile('platforms/ios/OcentraParentMobile.xcodeproj/project.pbxproj');
  const parentMobileSourceProof = readEnforcerProfileProofScript(
    'scripts/test/parent-mobile-package-source-artifact-proof.mjs'
  );

  assert.match(androidManifest, /android\.intent\.action\.MAIN/u);
  assert.match(androidManifest, /android\.intent\.category\.LAUNCHER/u);
  assert.doesNotMatch(androidManifest, /OcentraParentAgentService/u);
  assert.match(iosProject, /productType = "com\.apple\.product-type\.application"/u);
  assert.match(iosProject, /PRODUCT_BUNDLE_IDENTIFIER = ca\.ocentra\.parent\.mobile/u);
  assert.match(parentMobileSourceProof, /child-agent-parity=not-claimed/u);
  assert.match(
    parentMobileSourceProof,
    /parent mobile release scripts and smoke inputs are separate from child agent package scripts/u
  );
});
