import { readdirSync, readFileSync } from 'node:fs';
import { dirname, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, it } from 'vitest';
import { AgentLogSnapshotSchema, DevLogEntrySchema } from '../../src/logging-contracts';
import {
  GeneratedDevLogMessage as DevLogMessage,
  GeneratedLogSource as LogSource,
} from '../../src/generated-logging-contracts';
import { LoggingDomainPackage } from '../../src/package-info';

const TestDirectory = dirname(fileURLToPath(import.meta.url));
const PackageRoot = resolve(TestDirectory, '..', '..');
const SchemaDomainPackageSpecifier = `@ocentra-parent/${'schema-domain'}`;
const SourceFileExtensions = ['.mjs', '.mts', '.ts', '.tsx'] as const;
const IgnoredDirectories = new Set(['.turbo', 'coverage', 'dist', 'node_modules']);

it('LoggingDomainPackage: identifies the operational logging boundary', () => {
  expect(LoggingDomainPackage.Boundary).toBe('operational-logging-contracts');
});

it('logging-domain: does not import schema-domain contract authority', () => {
  const directImportFiles = listSourceFiles(PackageRoot)
    .filter((filePath) => readFileSync(filePath, 'utf8').includes(SchemaDomainPackageSpecifier))
    .map((filePath) => relative(PackageRoot, filePath).replaceAll('\\', '/'))
    .sort();

  expect(directImportFiles).toEqual([]);
});

it('AgentLogSnapshotSchema: accepts the Rust localhost log snapshot contract', () => {
  const parsed = AgentLogSnapshotSchema.parse({
    schemaVersion: 1,
    agent: {
      deviceId: 'local-dev',
      hostname: 'devbox',
      platform: 'windows',
      serviceVersion: '0.1.0',
    },
    entries: [
      {
        schemaVersion: 1,
        id: 'dev-startup',
        timestamp: '2026-05-19T00:00:00Z',
        level: 'info',
        source: 'agent-service',
        message: 'Agent service localhost API is reachable.',
        fields: {
          captureEnabled: false,
          pid: 1000,
          mode: 'dev',
          remoteSync: null,
        },
      },
    ],
  });

  expect(parsed.entries[0]?.source).toBe('agent-service');
});

it('AgentLogSnapshotSchema: rejects unknown log levels', () => {
  const parsed = AgentLogSnapshotSchema.safeParse({
    schemaVersion: 1,
    agent: {
      deviceId: 'local-dev',
      hostname: 'devbox',
      platform: 'windows',
      serviceVersion: '0.1.0',
    },
    entries: [
      {
        schemaVersion: 1,
        id: 'bad-level',
        timestamp: '2026-05-19T00:00:00Z',
        level: 'notice',
        source: 'agent-service',
        message: 'This level is not part of the contract.',
        fields: {},
      },
    ],
  });

  expect(parsed.success).toBe(false);
});

it('DevLogEntrySchema: accepts local dev NDJSON entries', () => {
  const parsed = DevLogEntrySchema.parse({
    schemaVersion: 1,
    id: 'portal-log-1',
    timestamp: '2026-05-20T00:00:00Z',
    level: 'info',
    source: LogSource.Portal,
    message: DevLogMessage.PortalStarted,
    fields: {
      agentWebSocketUrl: 'ws://127.0.0.1:4477/api/dev/ws',
    },
  });

  expect(parsed.source).toBe('portal');
});

function listSourceFiles(directory: string): string[] {
  const entries = readdirSync(directory, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    if (IgnoredDirectories.has(entry.name)) {
      continue;
    }
    const entryPath = resolve(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...listSourceFiles(entryPath));
      continue;
    }
    if (SourceFileExtensions.some((extension) => entry.name.endsWith(extension))) {
      files.push(entryPath);
    }
  }

  return files;
}
