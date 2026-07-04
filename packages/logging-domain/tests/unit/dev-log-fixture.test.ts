import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

import { DevLogEntrySchema } from '../../src/logging-contracts';

const thisDir = path.dirname(fileURLToPath(import.meta.url));
const packageFixturePath = path.resolve(thisDir, '../../fixtures/dev-log-entry.json');
const rustFixturePath = path.resolve(thisDir, '../../../../crates/logging-core/tests/fixtures/dev-log-entry.json');

describe('dev log fixture parity', () => {
  it('parses the package fixture through the TypeScript schema', () => {
    const payload = JSON.parse(readFileSync(packageFixturePath, 'utf8'));
    const entry = DevLogEntrySchema.parse(payload);
    expect(entry.source).toBe('agent-service');
    expect(entry.level).toBe('info');
  });

  it('parses the Rust fixture through the TypeScript schema', () => {
    const payload = JSON.parse(readFileSync(rustFixturePath, 'utf8'));
    const entry = DevLogEntrySchema.parse(payload);
    expect(entry.message).toBe('Agent service dev runtime started.');
    expect(entry.fields).toEqual({});
  });
});
