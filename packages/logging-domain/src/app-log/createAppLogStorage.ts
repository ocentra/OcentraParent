import fs from 'node:fs';
import crypto from 'node:crypto';
import { AppLogSchemaVersion, type AppLogEntry, type AppLogQuery, type AppLogStats } from '@ocentra-parent/schema-domain/app-log/types';
import {
  appendAppLogEntries,
  listAppLogSessionFiles,
  pruneAppLogSessions,
  readAppLogEntries,
} from './appNdjsonWriter';
import { LogLevel } from '@ocentra-parent/schema-domain/logging-contracts';
import type { TestLogScope } from '@ocentra-parent/schema-domain/test-log/types';

export interface CreateAppLogStorageOptions {
  readonly scope: TestLogScope;
  readonly rootDir?: string;
  readonly sessionId?: string;
  readonly keepNewestSessions?: number;
}

interface AppLogEntryInput {
  readonly timestamp: AppLogEntry['timestamp'];
  readonly level: AppLogEntry['level'];
  readonly source: AppLogEntry['source'];
  readonly context: AppLogEntry['context'];
  readonly message: AppLogEntry['message'];
  readonly data?: AppLogEntry['data'];
  readonly file?: AppLogEntry['file'];
  readonly filePath?: AppLogEntry['filePath'];
  readonly line?: AppLogEntry['line'];
  readonly column?: AppLogEntry['column'];
  readonly correlationId?: AppLogEntry['correlationId'];
  readonly environment?: AppLogEntry['environment'];
}

export interface AppLogStorage {
  readonly sessionId: string;
  storeLog(entry: AppLogEntryInput): void;
  storeLogsBatch(entries: ReadonlyArray<AppLogEntryInput>): void;
  queryLogs(query?: AppLogQuery): Promise<AppLogEntry[]>;
  getStats(): Promise<AppLogStats>;
  clearLogs(): Promise<number>;
  flush(): Promise<void>;
  dispose(): Promise<void>;
}

function makeSessionId(): string {
  return `session-${Date.now()}-${crypto.randomUUID()}`;
}

function normalizeEntry(
  scope: TestLogScope,
  sessionId: string,
  entry: AppLogEntryInput
): AppLogEntry {
  return {
    schemaVersion: AppLogSchemaVersion,
    sessionId,
    scope,
    timestamp: entry.timestamp,
    level: entry.level,
    source: entry.source,
    context: entry.context,
    message: entry.message,
    data: entry.data ?? null,
    file: entry.file ?? null,
    filePath: entry.filePath ?? null,
    line: entry.line ?? null,
    column: entry.column ?? null,
    correlationId: entry.correlationId ?? null,
    environment: entry.environment ?? null,
  };
}

function matchesQuery(entry: AppLogEntry, query?: AppLogQuery): boolean {
  if (query?.level != null && entry.level !== query.level) {
    return false;
  }

  if (query?.search != null && query.search.trim().length > 0) {
    const search = query.search.toLowerCase();
    const haystack = `${entry.message} ${entry.context ?? ''} ${entry.data ?? ''}`.toLowerCase();
    if (!haystack.includes(search)) {
      return false;
    }
  }

  return true;
}

export function createAppLogStorage(options: CreateAppLogStorageOptions): AppLogStorage {
  const scope = options.scope;
  const rootDir = options.rootDir;
  const sessionId = options.sessionId ?? makeSessionId();
  const keepNewestSessions = options.keepNewestSessions ?? 10;
  const pending: AppLogEntry[] = [];

  pruneAppLogSessions(scope, keepNewestSessions, rootDir);

  async function flushPending(): Promise<void> {
    if (pending.length === 0) {
      return;
    }
    appendAppLogEntries(scope, sessionId, pending.splice(0, pending.length), rootDir);
    pruneAppLogSessions(scope, keepNewestSessions, rootDir);
  }

  async function loadEntries(): Promise<AppLogEntry[]> {
    await flushPending();
    return listAppLogSessionFiles(scope, rootDir)
      .flatMap((filePath) => readAppLogEntries(filePath))
      .sort((left, right) => right.timestamp - left.timestamp);
  }

  return {
    sessionId,
    storeLog(entry) {
      pending.push(normalizeEntry(scope, sessionId, entry));
    },
    storeLogsBatch(entries) {
      for (const entry of entries) {
        pending.push(normalizeEntry(scope, sessionId, entry));
      }
    },
    async queryLogs(query) {
      const entries = await loadEntries();
      const filtered = entries.filter((entry) => matchesQuery(entry, query));
      return query?.limit == null ? filtered : filtered.slice(0, query.limit);
    },
    async getStats() {
      const entries = await loadEntries();
      return {
        totalLogs: entries.length,
        sessions: listAppLogSessionFiles(scope, rootDir).length,
        newestTimestamp: entries[0]?.timestamp ?? null,
      };
    },
    async clearLogs() {
      const files = listAppLogSessionFiles(scope, rootDir);
      for (const filePath of files) {
        fs.rmSync(filePath, { force: true });
      }
      pending.length = 0;
      return files.length;
    },
    async flush() {
      await flushPending();
    },
    async dispose() {
      await flushPending();
    },
  };
}

export const AppLogDefaults = {
  Level: LogLevel.Info,
} as const;
