/* generated from crates/logging-core/src/local_ndjson_log.rs */

import type { AppLogEntry, AppLogQuery } from './app-log/types';

export function matchesGeneratedAppLogQuery(entry: AppLogEntry, query?: AppLogQuery): boolean {
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
