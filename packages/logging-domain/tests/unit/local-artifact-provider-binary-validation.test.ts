import crypto from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { afterEach, expect, it } from 'vitest';
import { validateProviderBinary } from '../../src/local-artifact-provider-binary-validation';
import { readStableJsonFile } from '../../src/local-artifact-provider-manifest';

const roots: string[] = [];

function privateFile(name: string, content: Buffer | string): { readonly root: string; readonly filePath: string } {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'ocentra-provider-identity-'));
  const filePath = path.join(root, name);
  fs.writeFileSync(filePath, content);
  roots.push(root);
  return { root, filePath };
}

afterEach(() => {
  for (const root of roots.splice(0, roots.length)) {
    fs.rmSync(root, { force: true, recursive: true });
  }
});

it('binds provider validation to one private opened binary', () => {
  const content = Buffer.from('provider-binary', 'utf8');
  const fixture = privateFile('provider.exe', content);
  const sha256 = crypto.createHash('sha256').update(content).digest('hex');

  expect(validateProviderBinary(fixture.root, 'provider.exe', sha256)).toEqual({
    path: fs.realpathSync.native(fixture.filePath),
    sha256,
  });

  fs.linkSync(fixture.filePath, path.join(fixture.root, 'provider-copy.exe'));
  expect(validateProviderBinary(fixture.root, 'provider.exe', sha256)).toBeNull();
});

it('reads metadata only through one bounded private opened file', () => {
  const fixture = privateFile('provider.json', '{"schemaVersion":1}');

  expect(readStableJsonFile(fixture.filePath, 1024)).toEqual({ schemaVersion: 1 });

  fs.linkSync(fixture.filePath, path.join(fixture.root, 'provider-copy.json'));
  expect(() => readStableJsonFile(fixture.filePath, 1024)).toThrow(
    'provider metadata file is not a bounded private regular file'
  );
});
