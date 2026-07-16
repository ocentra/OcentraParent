import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';

const packageRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const packageManifest = JSON.parse(fs.readFileSync(path.join(packageRoot, 'package.json'), 'utf8')) as {
  readonly exports: Record<string, unknown>;
};

it('package exports keep runtime modules and publish logging-owned type surfaces', async () => {
  expect('./app-log/types' in packageManifest.exports).toBe(true);
  expect('./test-log/types' in packageManifest.exports).toBe(true);
  expect('./test-log/ndjsonBrands' in packageManifest.exports).toBe(true);
  expect('./core/stackTrace' in packageManifest.exports).toBe(true);
  expect('./test-log/ndjsonLogFileWriter' in packageManifest.exports).toBe(true);
  expect('./app-log/createAppLogStorage' in packageManifest.exports).toBe(false);
  expect('./transport/bridgeTransport' in packageManifest.exports).toBe(true);

  const stackTrace = await import('../../src/core/stackTrace');
  const writer = await import('../../src/test-log/ndjsonLogFileWriter');
  const transport = await import('../../src/transport/bridgeTransport');

  expect(typeof stackTrace.getStackTrace).toBe('function');
  expect(typeof writer.writeLogEntry).toBe('function');
  expect(typeof transport.BridgeTransport).toBe('function');
});
