import assert from 'node:assert/strict';
import { DatabaseSync } from 'node:sqlite';
import { describe, it } from 'node:test';
import type { D1Database, D1PreparedStatement } from '@cloudflare/workers-types';
import {
  createAccountIdentityStore,
  type AccountIdentityProvider,
  type AccountIdentityStatus,
} from '../../src/storage/account-identity-store.js';

type SQLiteParameter = string | number;

class SQLiteD1PreparedStatement {
  private values: SQLiteParameter[] = [];

  public constructor(private readonly statement: ReturnType<DatabaseSync['prepare']>) {}

  public bind(...values: SQLiteParameter[]): SQLiteD1PreparedStatement {
    this.values = values;
    return this;
  }

  public async first<T>(): Promise<T | null> {
    const row = (this.statement.get as (...parameters: SQLiteParameter[]) => unknown)(...this.values);
    return (row as T | undefined) ?? null;
  }

  public async run(): Promise<unknown> {
    (this.statement.run as (...parameters: SQLiteParameter[]) => unknown)(...this.values);
    return { success: true, meta: {} };
  }
}

class SQLiteD1Harness {
  private readonly database = new DatabaseSync(':memory:');

  public constructor() {
    this.database.exec(`
      CREATE TABLE ocentra_account_identities (
        account_id TEXT NOT NULL,
        provider TEXT NOT NULL CHECK (provider IN ('authjs', 'firebase')),
        provider_subject TEXT NOT NULL,
        status TEXT NOT NULL CHECK (status IN ('active', 'revoked')),
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        PRIMARY KEY (provider, provider_subject)
      );
      CREATE INDEX idx_ocentra_account_identities_account_id
        ON ocentra_account_identities (account_id);
    `);
  }

  public readonly d1 = {
    exec: async (query: string): Promise<void> => {
      this.database.exec(query);
    },
    prepare: (query: string): D1PreparedStatement =>
      new SQLiteD1PreparedStatement(this.database.prepare(query)) as unknown as D1PreparedStatement,
  } as unknown as D1Database;

  public close(): void {
    this.database.close();
  }
}

function input(
  overrides: Partial<{
    accountId: string;
    provider: AccountIdentityProvider;
    providerSubject: string;
    status: AccountIdentityStatus;
    nowMs: number;
  }> = {}
) {
  return {
    accountId: 'account-001',
    provider: 'firebase' as const,
    providerSubject: 'firebase-subject-001',
    status: 'active' as const,
    nowMs: 1_700_000_000_000,
    ...overrides,
  };
}

describe('account identity D1 persistence adapter', () => {
  it('persists a minimal provider mapping and updates the same account idempotently', async () => {
    const harness = new SQLiteD1Harness();
    try {
      const store = createAccountIdentityStore(harness.d1);
      const created = await store.upsert(input());
      assert.deepEqual(created, {
        status: 'persisted',
        record: {
          accountId: 'account-001',
          provider: 'firebase',
          providerSubject: 'firebase-subject-001',
          status: 'active',
          createdAt: 1_700_000_000_000,
          updatedAt: 1_700_000_000_000,
        },
      });

      const revoked = await store.upsert(input({ status: 'revoked', nowMs: 1_700_000_000_500 }));
      assert.deepEqual(revoked, {
        status: 'persisted',
        record: {
          accountId: 'account-001',
          provider: 'firebase',
          providerSubject: 'firebase-subject-001',
          status: 'revoked',
          createdAt: 1_700_000_000_000,
          updatedAt: 1_700_000_000_500,
        },
      });
      assert.deepEqual(await store.get('firebase', 'firebase-subject-001'), {
        status: 'found',
        record: revoked.record,
      });

      assert.deepEqual(
        await store.upsert(
          input({ provider: 'authjs', providerSubject: 'authjs-subject-001', nowMs: 1_700_000_001_000 })
        ),
        {
          status: 'persisted',
          record: {
            accountId: 'account-001',
            provider: 'authjs',
            providerSubject: 'authjs-subject-001',
            status: 'active',
            createdAt: 1_700_000_001_000,
            updatedAt: 1_700_000_001_000,
          },
        }
      );
    } finally {
      harness.close();
    }
  });

  it('rejects a provider subject already linked to another account without changing the original row', async () => {
    const harness = new SQLiteD1Harness();
    try {
      const store = createAccountIdentityStore(harness.d1);
      await store.upsert(input());
      assert.deepEqual(
        await store.upsert(input({ accountId: 'account-002', status: 'active', nowMs: 1_700_000_001_000 })),
        { status: 'conflict', reason: 'provider-subject-already-linked' }
      );
      assert.deepEqual(await store.get('firebase', 'firebase-subject-001'), {
        status: 'found',
        record: {
          accountId: 'account-001',
          provider: 'firebase',
          providerSubject: 'firebase-subject-001',
          status: 'active',
          createdAt: 1_700_000_000_000,
          updatedAt: 1_700_000_000_000,
        },
      });
    } finally {
      harness.close();
    }
  });

  it('returns manual-required when the production binding is absent', async () => {
    const store = createAccountIdentityStore(undefined);
    assert.deepEqual(await store.upsert(input()), {
      status: 'manual-required',
      reason: 'account-identity-d1-binding-missing',
    });
    assert.deepEqual(await store.get('firebase', 'firebase-subject-001'), {
      status: 'manual-required',
      reason: 'account-identity-d1-binding-missing',
    });
  });

  it('rejects unsupported providers and malformed identity inputs before persistence', async () => {
    const harness = new SQLiteD1Harness();
    try {
      const store = createAccountIdentityStore(harness.d1);
      assert.deepEqual(await store.upsert(input({ provider: 'unsupported' as AccountIdentityProvider })), {
        status: 'invalid-input',
        reason: 'provider-unsupported',
      });
      assert.deepEqual(await store.upsert(input({ providerSubject: '   ' })), {
        status: 'invalid-input',
        reason: 'provider-subject-invalid',
      });
      assert.deepEqual(await store.upsert(input({ nowMs: -1 })), {
        status: 'invalid-input',
        reason: 'timestamp-invalid',
      });
      assert.deepEqual(await store.get('unsupported' as AccountIdentityProvider, 'subject'), {
        status: 'invalid-input',
        reason: 'provider-unsupported',
      });
      assert.deepEqual(await store.get('firebase', '\u0000subject'), {
        status: 'invalid-input',
        reason: 'provider-subject-invalid',
      });
    } finally {
      harness.close();
    }
  });
});
