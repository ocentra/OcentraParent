import { spawnSync } from 'node:child_process';
import { mkdir, readFile, rm, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = dirname(dirname(dirname(fileURLToPath(import.meta.url))));
const timestamp = '2026-06-06T10:18:00.000Z';
const testOutputDir = join(repoRoot, 'test-results', 'tracking-android-static-permission-readiness-proof');
const wp08ProofDir = join(repoRoot, 'output', 'tracking-plan-proof', '08-android-foreground-location-adapter');
const wp09ProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '09-android-background-location-and-geofence-adapter'
);
const wp31ProofDir = join(
  repoRoot,
  'output',
  'tracking-plan-proof',
  '31-platform-extension-checklists-and-proof-routing'
);
const manifestPath = join(repoRoot, 'platforms', 'android', 'agent', 'app', 'src', 'main', 'AndroidManifest.xml');
const readinessClassPath = join(
  repoRoot,
  'platforms',
  'android',
  'agent',
  'app',
  'src',
  'main',
  'java',
  'ca',
  'ocentra',
  'parent',
  'agent',
  'TrackingAndroidLocationPermissionReadinessProof.java'
);
const commands = [];
const initialGitStatusShort = gitOutput(['status', '--short']);

await rm(testOutputDir, { recursive: true, force: true });
await mkdir(testOutputDir, { recursive: true });
await mkdir(wp08ProofDir, { recursive: true });
await mkdir(wp09ProofDir, { recursive: true });
await mkdir(wp31ProofDir, { recursive: true });

run('cmd', ['/c', 'gradlew.bat', ':app:assembleDebug', '--console=plain'], {
  cwd: join(repoRoot, 'platforms', 'android', 'agent'),
});

const manifest = await readFile(manifestPath, 'utf8');
const readinessClass = await readFile(readinessClassPath, 'utf8');
const proof = proofPayload(manifest, readinessClass);

assertProof(proof);
await writeJson(join(testOutputDir, 'proof.json'), proof);
await writeJson(join(wp08ProofDir, '18-static-permission-readiness-proof.json'), foregroundProof(proof));
await writeJson(join(wp09ProofDir, '18-static-permission-readiness-proof.json'), backgroundProof(proof));
await writeJson(join(wp31ProofDir, '25-android-static-permission-readiness-proof.json'), proof);
await writeFile(join(wp08ProofDir, '16-validation-commands.log'), validationLog(), 'utf8');
await writeFile(join(wp09ProofDir, '16-validation-commands.log'), validationLog(), 'utf8');
await writeFile(join(wp31ProofDir, '16-validation-commands.log'), validationLog(), 'utf8');
await writeFile(join(wp08ProofDir, '15-manual-platform-proof.md'), manualProof('WP08 foreground location'), 'utf8');
await writeFile(join(wp09ProofDir, '15-manual-platform-proof.md'), manualProof('WP09 background and geofence'), 'utf8');

console.log('tracking-android-static-permission-readiness-proof-ok');
console.log(`evidence=${join('test-results', 'tracking-android-static-permission-readiness-proof', 'proof.json')}`);

function proofPayload(manifestSource, readinessClassSource) {
  const declaredManifestPermissions = [
    'android.permission.FOREGROUND_SERVICE_LOCATION',
    'android.permission.ACCESS_COARSE_LOCATION',
    'android.permission.ACCESS_FINE_LOCATION',
    'android.permission.ACCESS_BACKGROUND_LOCATION',
  ];
  const foregroundServiceTypes = ['dataSync', 'location'];
  const missingManifestPermissions = declaredManifestPermissions.filter(
    (permission) => !manifestSource.includes(`android:name="${permission}"`)
  );
  const missingForegroundServiceTypes = foregroundServiceTypes.filter(
    (serviceType) =>
      !manifestSource.includes(`android:foregroundServiceType="dataSync|location"`) ||
      !manifestSource.includes(serviceType)
  );

  return {
    proofMode: 'tracking-android-static-permission-readiness-proof',
    generatedAt: timestamp,
    branch: gitOutput(['rev-parse', '--abbrev-ref', 'HEAD']),
    baseCommitAtGeneration: gitOutput(['rev-parse', 'HEAD']),
    gitStatusShort: initialGitStatusShort,
    commands,
    manifestPath: 'platforms/android/agent/app/src/main/AndroidManifest.xml',
    readinessClassPath:
      'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/TrackingAndroidLocationPermissionReadinessProof.java',
    declaredManifestPermissions,
    missingManifestPermissions,
    foregroundServiceTypes,
    missingForegroundServiceTypes,
    readinessClassSignals: {
      hasStaticReadinessState: readinessClassSource.includes('manifest-declared-build-proof'),
      hasManualRuntimeState: readinessClassSource.includes('manual-runtime-required'),
      hasForegroundLocationGap: readinessClassSource.includes('foreground-location-sample'),
      hasGeofenceRuntimeGap: readinessClassSource.includes('geofence-transition-runtime'),
    },
    staticReadinessClaimed: missingManifestPermissions.length === 0 && missingForegroundServiceTypes.length === 0,
    runtimeClaims: {
      foregroundPermissionGrantClaimed: false,
      foregroundLocationSampleClaimed: false,
      backgroundPermissionGrantClaimed: false,
      backgroundLocationRuntimeClaimed: false,
      geofenceTransitionRuntimeClaimed: false,
      physicalDeviceProofClaimed: false,
      productReadyAndroidTrackingClaimed: false,
    },
    proofPaths: {
      foreground:
        'output/tracking-plan-proof/08-android-foreground-location-adapter/18-static-permission-readiness-proof.json',
      background:
        'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/18-static-permission-readiness-proof.json',
      platformRouting:
        'output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/25-android-static-permission-readiness-proof.json',
      evidence: 'test-results/tracking-android-static-permission-readiness-proof/proof.json',
    },
  };
}

function foregroundProof(proof) {
  return {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    manifestPath: proof.manifestPath,
    declaredManifestPermissions: proof.declaredManifestPermissions.filter(
      (permission) => permission.includes('FOREGROUND') || permission.includes('COARSE') || permission.includes('FINE')
    ),
    foregroundServiceTypes: proof.foregroundServiceTypes,
    staticReadinessClaimed: proof.staticReadinessClaimed,
    runtimeClaims: proof.runtimeClaims,
  };
}

function backgroundProof(proof) {
  return {
    proofMode: proof.proofMode,
    generatedAt: proof.generatedAt,
    manifestPath: proof.manifestPath,
    declaredManifestPermissions: proof.declaredManifestPermissions.filter((permission) =>
      permission.includes('BACKGROUND')
    ),
    foregroundServiceTypes: proof.foregroundServiceTypes,
    staticReadinessClaimed: proof.staticReadinessClaimed,
    runtimeClaims: proof.runtimeClaims,
  };
}

function assertProof(proof) {
  if (!proof.staticReadinessClaimed) {
    throw new Error(`Android static readiness proof missing declarations: ${JSON.stringify(proof)}`);
  }
  if (Object.values(proof.readinessClassSignals).some((value) => value !== true)) {
    throw new Error(
      `Android readiness class is missing no-claim signals: ${JSON.stringify(proof.readinessClassSignals)}`
    );
  }
  if (Object.values(proof.runtimeClaims).some((value) => value !== false)) {
    throw new Error(`Android static proof overclaimed runtime behavior: ${JSON.stringify(proof.runtimeClaims)}`);
  }
}

function manualProof(scope) {
  return [
    `# ${scope} Manual Platform Proof`,
    '',
    'Static Android manifest readiness is present after the debug APK build, but runtime proof is still manual-required.',
    '',
    '- Foreground permission grant: not claimed.',
    '- Foreground location sample: not claimed.',
    '- Background permission grant: not claimed.',
    '- Geofence transition runtime: not claimed.',
    '- Physical-device proof: not claimed.',
    '',
  ].join('\n');
}

function validationLog() {
  return commands
    .map((command) =>
      [`$ ${command.command}`, normalizeOutput(command.stdout), normalizeOutput(command.stderr)]
        .filter((line) => line.length > 0)
        .join('\n')
    )
    .join('\n\n');
}

function normalizeOutput(value) {
  return value
    .replace(/\r\n/g, '\n')
    .replace(/\r/g, '\n')
    .split('\n')
    .map((line) => line.trimEnd())
    .join('\n')
    .trim();
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd ?? repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    command: [command, ...args].join(' '),
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
  });
  if (result.status !== 0) {
    throw new Error(
      `Command failed: ${[command, ...args].join(' ')}\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`
    );
  }
}

function gitOutput(args) {
  const result = spawnSync('git', args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`git ${args.join(' ')} failed: ${result.stderr}`);
  }
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await mkdir(dirname(path), { recursive: true });
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`, 'utf8');
}
