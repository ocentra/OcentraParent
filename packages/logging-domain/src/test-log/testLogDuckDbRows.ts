import { rowToGeneratedStats, rowToGeneratedStoredLog } from '../duckdb-log-query';
import { type StoredTestLogLine, type TestLogStats } from './types';

export interface StoredLogRow {
  readonly scope: string;
  readonly run_id: string;
  readonly run_type: string;
  readonly suite_type: string | null;
  readonly test_name: string;
  readonly log_timestamp: number;
  readonly level: string;
  readonly source: string | null;
  readonly context: string | null;
  readonly message: string;
  readonly data: string | null;
  readonly file: string | null;
  readonly file_path: string | null;
  readonly line: number | null;
  readonly column_value: number | null;
  readonly correlation_id: string | null;
  readonly tags: string | null;
  readonly stack: string | null;
  readonly origin: string | null;
  readonly environment: string | null;
}

export interface StatsRow {
  readonly total_logs: number;
  readonly error_logs: number;
  readonly warn_logs: number;
  readonly distinct_runs: number;
  readonly distinct_tests: number;
  readonly newest_timestamp: number | null;
}

export function testLogRow(row: StoredLogRow): StoredTestLogLine {
  return rowToGeneratedStoredLog(row);
}

export function testLogStats(row: StatsRow | undefined): TestLogStats {
  return rowToGeneratedStats(row);
}
