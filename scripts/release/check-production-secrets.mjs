const requiredNowByRelease = {
  windows: ['OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64'],
};

const futureProductionSecrets = [
  'OCENTRA_PARENT_WINDOWS_AUTHENTICODE_CERT_BASE64',
  'OCENTRA_PARENT_WINDOWS_AUTHENTICODE_PASSWORD',
  'OCENTRA_PARENT_MACOS_DEVELOPER_ID_CERT_BASE64',
  'OCENTRA_PARENT_MACOS_DEVELOPER_ID_PASSWORD',
  'OCENTRA_PARENT_APPLE_API_KEY_ID',
  'OCENTRA_PARENT_APPLE_API_ISSUER_ID',
  'OCENTRA_PARENT_APPLE_API_PRIVATE_KEY_BASE64',
  'OCENTRA_PARENT_ANDROID_RELEASE_KEYSTORE_BASE64',
  'OCENTRA_PARENT_ANDROID_RELEASE_KEYSTORE_PASSWORD',
  'OCENTRA_PARENT_ANDROID_RELEASE_KEY_ALIAS',
  'OCENTRA_PARENT_ANDROID_RELEASE_KEY_PASSWORD',
];

const release = readOption('--release') ?? 'windows';
const requiredNow = requiredNowByRelease[release];

if (!requiredNow) {
  throw new Error(`Unknown release secret set: ${release}`);
}

const missing = requiredNow.filter((name) => !process.env[name]);
if (missing.length > 0) {
  console.error(`Missing required ${release} production secret(s): ${missing.join(', ')}`);
  process.exit(1);
}

console.log(`Required ${release} production secrets are present: ${requiredNow.join(', ')}`);
console.log(`Future production signing/notarization/store secrets are declared: ${futureProductionSecrets.join(', ')}`);

function readOption(name) {
  const index = process.argv.indexOf(name);
  if (index === -1) {
    return null;
  }
  return process.argv[index + 1] ?? null;
}
