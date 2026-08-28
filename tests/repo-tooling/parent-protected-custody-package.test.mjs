import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import { test } from 'node:test';

const repoRoot = process.cwd();
const releaseRoot = join(repoRoot, 'scripts', 'release', 'windows');
const packageRoot = join(releaseRoot, 'parent-protected-custody');
const packageTestRoot = join(packageRoot, 'tests');

function readRepoFile(relativePath) {
  return readFileSync(join(repoRoot, relativePath), 'utf8');
}

function readPackageFile(name) {
  return readFileSync(join(packageRoot, name), 'utf8');
}

test('Parent WP12 includes real source-level contract suites for every required boundary', () => {
  const expectedSuites = [
    'package-contract.Tests.ps1',
    'package-publication.Tests.ps1',
    'package-journal.Tests.ps1',
  ];

  for (const suite of expectedSuites) {
    const path = join(packageTestRoot, suite);
    assert.equal(existsSync(path), true, `missing real PowerShell suite ${suite}`);
    const source = readFileSync(path, 'utf8');
    assert.match(source, /#requires -Version 7\.2/u);
    assert.match(source, /Set-StrictMode -Version Latest/u);
    assert.match(source, /Write-Output ['"]PASS:/u);
    assert.doesNotMatch(source, /\b(?:mock|fake|stub|noop|skip|todo)\b/i);
  }

  const packageTest = readRepoFile('tests/repo-tooling/parent-protected-custody-package.test.mjs');
  assert.match(packageTest, /node:assert\/strict/u);
});

test('Parent WP12 WiX source fixes the exact MSI identity, custom action, service, and content boundary', () => {
  const wixSource = readRepoFile('scripts/release/windows/parent-protected-custody.wxs');
  const msiContract = readPackageFile('msi-contract.ps1');
  const msiValidation = readPackageFile('msi-validation.ps1');

  assert.match(wixSource, /<Package\s+[\s\S]*?Name="Ocentra Parent Protected Capability Custody"/u);
  assert.match(wixSource, /Manufacturer="Ocentra"/u);
  assert.match(wixSource, /UpgradeCode="A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C"/u);
  assert.match(wixSource, /Scope="perMachine"/u);
  assert.match(wixSource, /InstallerVersion="500"/u);
  assert.match(wixSource, /Id="RunProtectedProvisioner"[\s\S]*?FileRef="ProtectedProvisionerFile"[\s\S]*?ExeCommand=""/u);
  assert.match(wixSource, /Action="RunProtectedProvisioner"[\s\S]*?Before="StartServices"[\s\S]*?Condition='NOT REMOVE~="ALL"'/u);
  assert.match(wixSource, /Name="OcentraProtectedCapabilityCustodyBroker"[\s\S]*?Account="LocalSystem"[\s\S]*?Start="auto"[\s\S]*?ErrorControl="critical"/u);
  assert.match(wixSource, /Name="ocentra-protected-capability-custody-provisioner\.exe"/u);
  assert.match(wixSource, /Name="package-boundary"[\s\S]*?Value="parent-protected-custody-v1"/u);
  assert.doesNotMatch(wixSource, /authValue\s*=/iu);

  assert.match(msiContract, /RunProtectedProvisioner', '11282', 'ProtectedProvisionerFile'/u);
  assert.match(msiContract, /RunProtectedProvisioner', 'NOT REMOVE~="ALL"', '5899'/u);
  assert.match(msiValidation, /ProtectedProvisionerFile = @\('ProtectedProvisioner'/u);
  assert.match(msiValidation, /UpgradeCode = '\{A1BA5AA2-F5DB-4B97-9889-4BB4DBF52B3C\}'/u);
});

test('Parent WP12 build source binds the fixed production closure and pinned WiX provenance', () => {
  const buildSource = readPackageFile('build-package.ps1');
  const extensionSource = readPackageFile('wix-extension.ps1');
  const wrapperSource = readRepoFile('scripts/release/windows/build-parent-protected-custody-package.ps1');

  for (const requiredSource of [
    'parent-protected-custody.wxs',
    'package-inputs.ps1',
    'package-path-safety.ps1',
    'package-publication.ps1',
    'wix-extension.ps1',
    'msi-contract.ps1',
  ]) {
    assert.match(buildSource, new RegExp(requiredSource.replaceAll('.', '\\.'), 'u'));
  }
  assert.match(buildSource, /'--manifest-path'/u);
  assert.match(buildSource, /'--locked'/u);
  assert.match(buildSource, /ocentra-protected-capability-custody-broker/u);
  assert.match(buildSource, /ocentra-protected-capability-custody-provisioner/u);
  assert.match(buildSource, /WixToolset\.Util\.wixext/u);
  assert.match(buildSource, /ExpectedUtilBinaryHash/u);
  assert.match(extensionSource, /requiredVersion = '6\.0\.2'/u);
  assert.match(extensionSource, /WixToolset\.Util\.wixext\/\$requiredVersion/u);
  assert.match(extensionSource, /global-packages/u);
  assert.match(extensionSource, /Get-WixPayloadFileHashes/u);
  assert.match(extensionSource, /Assert-WixToolchainProvenanceSnapshot/u);
  assert.match(wrapperSource, /parent-protected-custody\\build-package\.ps1/u);
  assert.doesNotMatch(buildSource, /authValue\s*=/iu);
});

test('Parent WP12 publication source enforces semantic manifest binding, journal v4 recovery, and exclusive lock proof', () => {
  const publicationSource = readPackageFile('package-publication.ps1');
  const inputSource = readPackageFile('package-inputs.ps1');
  const pathSource = readPackageFile('package-path-safety.ps1');

  assert.match(publicationSource, /PackagePublicationJournalSchema = 4/u);
  assert.match(publicationSource, /PackagePublicationJournalFields = @\(/u);
  assert.match(publicationSource, /Get-PackageManifestSemanticHash/u);
  assert.match(publicationSource, /root fields are not the exact protected-custody contract/u);
  assert.match(publicationSource, /declared semanticSha256 does not match/u);
  assert.match(publicationSource, /Assert-PackagePublicationJournalChain/u);
  assert.match(publicationSource, /previousRecordHash/u);
  assert.match(publicationSource, /Recover-PackagePublication/u);
  assert.match(publicationSource, /preserving all bytes and refusing recovery/u);
  assert.match(publicationSource, /\[System\.IO\.FileShare\]::None/u);
  assert.match(publicationSource, /HResult -band 0xffff\) -ne 32/u);
  assert.match(publicationSource, /non-sharing IOException/u);
  assert.match(inputSource, /\[System\.IO\.FileMode\]::CreateNew/u);
  assert.match(inputSource, /\[System\.IO\.FileShare\]::None/u);
  assert.match(pathSource, /Assert-NoPackageReparseChain/u);
  assert.match(pathSource, /refusing physical path escape/u);
});

test('Parent WP12 test source keeps package lifecycle separate from protected authority', () => {
  const wixSource = readRepoFile('scripts/release/windows/parent-protected-custody.wxs');
  const buildSource = readPackageFile('build-package.ps1');
  const contractSuite = readFileSync(join(packageTestRoot, 'package-contract.Tests.ps1'), 'utf8');
  const journalSuite = readFileSync(join(packageTestRoot, 'package-journal.Tests.ps1'), 'utf8');

  assert.match(wixSource, /manual-required/iu);
  assert.match(wixSource, /never invokes deprovisioning implicitly/iu);
  assert.match(buildSource, /zero-argument owner-approved provisioner/iu);
  assert.match(buildSource, /External WP02 owner ceremony/iu);
  assert.match(buildSource, /manual-required external WP02 owner path/iu);
  assert.doesNotMatch(wixSource, /authValue\s*=/iu);
  assert.doesNotMatch(buildSource, /authValue\s*=/iu);
  assert.match(contractSuite, /Get-RegistryId/u);
  assert.match(contractSuite, /LocalSystem/u);
  assert.match(journalSuite, /Recover-PackagePublication/u);
  assert.match(journalSuite, /recovery did not promote/u);
});
