import crypto from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { stageLocalArtifactProvider } from '../../scripts/stage-local-artifact-provider-library.mjs';

interface StagingFixture {
  readonly root: string;
  readonly sourceBinary: string;
  readonly providerDirectory: string;
  readonly manifestPath: string;
  readonly content: Buffer;
}

function createFixture(): StagingFixture {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'ocentra-provider-stage-'));
  const sourceDirectory = path.join(root, 'target', 'release');
  const providerDirectory = path.join(root, 'package', 'dist', 'local-artifact-provider');
  fs.mkdirSync(sourceDirectory, { recursive: true });
  const sourceBinary = path.join(sourceDirectory, 'ocentra-logging-local-artifact-provider.exe');
  const content = Buffer.from('real provider binary fixture\n', 'utf8');
  fs.writeFileSync(sourceBinary, content);
  fs.linkSync(sourceBinary, path.join(sourceDirectory, 'provider-deps-copy.exe'));
  return {
    root,
    sourceBinary,
    providerDirectory,
    manifestPath: path.join(root, 'package', 'dist', 'local-artifact-provider.manifest.json'),
    content,
  };
}

describe('local artifact provider staging', () => {
  const roots: string[] = [];

  afterEach(() => {
    for (const root of roots.splice(0, roots.length)) {
      fs.rmSync(root, { force: true, recursive: true });
    }
  });

  it('stages a Cargo hard-linked source as one private content-addressed file', () => {
    const fixture = createFixture();
    roots.push(fixture.root);
    expect(fs.lstatSync(fixture.sourceBinary).nlink).toBe(2);

    const first = stageLocalArtifactProvider({
      sourceBinary: fixture.sourceBinary,
      providerDirectory: fixture.providerDirectory,
      manifestPath: fixture.manifestPath,
      packageName: '@ocentra-parent/logging-domain',
      packageVersion: '0.1.1',
    });
    const expectedDigest = crypto.createHash('sha256').update(fixture.content).digest('hex');
    expect(first.binarySha256).toBe(expectedDigest);
    expect(path.basename(first.outputBinary)).toBe(`ocentra-logging-local-artifact-provider-${expectedDigest}.exe`);
    expect(fs.readFileSync(first.outputBinary)).toEqual(fixture.content);
    expect(fs.lstatSync(first.outputBinary).nlink).toBe(1);
    expect(JSON.parse(fs.readFileSync(fixture.manifestPath, 'utf8'))).toEqual(first.manifest);
    expect(fs.lstatSync(fixture.manifestPath).nlink).toBe(1);

    const second = stageLocalArtifactProvider({
      sourceBinary: fixture.sourceBinary,
      providerDirectory: fixture.providerDirectory,
      manifestPath: fixture.manifestPath,
      packageName: '@ocentra-parent/logging-domain',
      packageVersion: '0.1.1',
    });
    expect(second).toEqual(first);
    expect(fs.readdirSync(fixture.providerDirectory)).toEqual([path.basename(first.outputBinary)]);
  });

  it('rejects tampered and hard-linked staged binaries', () => {
    const tamperedFixture = createFixture();
    roots.push(tamperedFixture.root);
    const tampered = stageLocalArtifactProvider({
      sourceBinary: tamperedFixture.sourceBinary,
      providerDirectory: tamperedFixture.providerDirectory,
      manifestPath: tamperedFixture.manifestPath,
      packageName: '@ocentra-parent/logging-domain',
      packageVersion: '0.1.1',
    });
    fs.writeFileSync(tampered.outputBinary, 'tampered');
    expect(() =>
      stageLocalArtifactProvider({
        sourceBinary: tamperedFixture.sourceBinary,
        providerDirectory: tamperedFixture.providerDirectory,
        manifestPath: tamperedFixture.manifestPath,
        packageName: '@ocentra-parent/logging-domain',
        packageVersion: '0.1.1',
      })
    ).toThrow('digest does not match');

    const linkedFixture = createFixture();
    roots.push(linkedFixture.root);
    const linked = stageLocalArtifactProvider({
      sourceBinary: linkedFixture.sourceBinary,
      providerDirectory: linkedFixture.providerDirectory,
      manifestPath: linkedFixture.manifestPath,
      packageName: '@ocentra-parent/logging-domain',
      packageVersion: '0.1.1',
    });
    fs.linkSync(linked.outputBinary, path.join(linkedFixture.root, 'unexpected-provider-link.exe'));
    expect(() =>
      stageLocalArtifactProvider({
        sourceBinary: linkedFixture.sourceBinary,
        providerDirectory: linkedFixture.providerDirectory,
        manifestPath: linkedFixture.manifestPath,
        packageName: '@ocentra-parent/logging-domain',
        packageVersion: '0.1.1',
      })
    ).toThrow('not a private regular file');
  });
});
