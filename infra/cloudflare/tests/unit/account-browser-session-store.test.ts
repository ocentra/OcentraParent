import assert from 'node:assert/strict';
import { DatabaseSync } from 'node:sqlite';
import { readFileSync } from 'node:fs';
import { describe, it } from 'node:test';
import type { D1Database, D1PreparedStatement } from '@cloudflare/workers-types';
import {
  createAccountIdentityAuthorityStore,
  type VerifiedAccountIdentityAuthorityCapability,
} from '../../src/storage/account-identity-authority-store.js';
import { createBrowserSessionStore } from '../../src/storage/account-browser-session-store.js';
import {
  isDigest,
  newSessionId,
  parseSessionCookie,
  sha256Hex,
} from '../../src/storage/account-browser-session-codec.js';

type SQLiteParameter = string | number | null;

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

  public async all<T>(): Promise<{ results: T[] }> {
    const rows = (this.statement.all as (...parameters: SQLiteParameter[]) => unknown[])(...this.values);
    return { results: rows as T[] };
  }

  public async run(): Promise<unknown> {
    const result = (this.statement.run as (...parameters: SQLiteParameter[]) => { changes?: number | bigint })(
      ...this.values
    );
    return { success: true, meta: { changes: Number(result.changes ?? 0) } };
  }
}

class SQLiteD1Harness {
  public readonly database = new DatabaseSync(':memory:');

  public readonly d1 = {
    exec: async (query: string): Promise<void> => {
      this.database.exec(query);
    },
    prepare: (query: string): D1PreparedStatement =>
      new SQLiteD1PreparedStatement(this.database.prepare(query)) as unknown as D1PreparedStatement,
    batch: async (statements: readonly D1PreparedStatement[]): Promise<unknown[]> => {
      const results: unknown[] = [];
      for (const statement of statements) {
        results.push(await (statement as unknown as SQLiteD1PreparedStatement).run());
      }
      return results;
    },
  } as unknown as D1Database;

  public close(): void {
    this.database.close();
  }
}

const MIGRATIONS = [
  '0001_account_identity_authority.sql',
  '0002_account_identity_current_authority.sql',
  '0005_account_browser_session_custody.sql',
  '0006_account_browser_session_refresh_custody.sql',
  '0007_account_browser_session_custody_hardening.sql',
] as const;

const AUTHORITY_INSERT_SQL = `
INSERT INTO ocentra_account_identity_current_authority (
  provider, provider_subject, mapping_status, account_id, household_id, member_id, role,
  account_state, membership_state, device_id, device_trust_state, session_freshness_state,
  session_id, session_generation, session_expires_at, authority_generation,
  child_profile_id, child_device_id, pairing_id, installation_id, selected_route_id,
  pairing_state, install_state, selected_route, lifecycle_state, revocation_state,
  support_receipt_id, support_provider_subject, support_account_id, support_member_id,
  support_household_id, support_device_id, support_child_profile_id, support_child_device_id,
  support_scope, support_issuer, support_issued_at, support_expires_at,
  support_revocation_state, support_audit_identity, created_at, updated_at
) VALUES (${Array.from({ length: 42 }, () => '?').join(', ')})
`;

interface StoreFixture {
  readonly harness: SQLiteD1Harness;
  readonly authoritySessionId: string;
  readonly providerSubject: string;
  readonly authority: VerifiedAccountIdentityAuthorityCapability;
}

async function createFixture(): Promise<StoreFixture> {
  const harness = new SQLiteD1Harness();
  try {
    for (const migration of MIGRATIONS) {
      harness.database.exec(
        readFileSync(new URL(`../../migrations/account-identity/${migration}`, import.meta.url), 'utf8')
      );
    }

    const providerSubject = 'firebase-browser-session-test';
    const authoritySessionId = newSessionId();
    const now = Date.now();
    const sessionExpiresAt = new Date(now + 60 * 60 * 1000).toISOString();
    const values: SQLiteParameter[] = [
      'firebase',
      providerSubject,
      'active',
      'account-browser-session-test',
      'household-browser-session-test',
      'member-browser-session-test',
      'parent-owner',
      'active',
      'active',
      'device-browser-session-test',
      'trusted',
      'fresh',
      authoritySessionId,
      1,
      sessionExpiresAt,
      1,
      'child-browser-session-test',
      'child-device-browser-session-test',
      'pairing-browser-session-test',
      'installation-browser-session-test',
      'route-browser-session-test',
      'paired',
      'installed',
      'local',
      'active',
      'active',
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      null,
      now - 1_000,
      now,
    ];
    (harness.database.prepare(AUTHORITY_INSERT_SQL).run as (...parameters: SQLiteParameter[]) => unknown)(...values);

    const authorityResult = await createAccountIdentityAuthorityStore(harness.d1).readCurrentAuthority(
      'firebase',
      providerSubject
    );
    assert.equal(authorityResult.status, 'trusted');
    if (authorityResult.status !== 'trusted') {
      throw new Error('trusted authority fixture was not accepted by the Account store');
    }

    return { harness, authoritySessionId, providerSubject, authority: authorityResult.capability };
  } catch (error) {
    harness.close();
    throw error;
  }
}

describe('account browser session custody store', () => {
  it('creates digest-only credentials and performs one-time refresh rotation', async () => {
    const fixture = await createFixture();
    try {
      const store = createBrowserSessionStore(fixture.harness.d1);
      const created = await store.create(fixture.authority, 'store-create');
      assert.equal(created.status, 'accepted');
      if (created.status !== 'accepted' || created.identity === null) return;

      const sessionCookie = parseSessionCookie(created.secrets.sessionToken);
      const refreshCookie = parseSessionCookie(created.secrets.refreshToken);
      assert.notEqual(sessionCookie, null);
      assert.notEqual(refreshCookie, null);
      assert.equal(sessionCookie?.sessionId, created.identity.sessionId);
      assert.equal(refreshCookie?.sessionId, created.identity.sessionId);
      assert.notEqual(created.secrets.sessionToken, created.secrets.refreshToken);
      assert.notEqual(created.secrets.refreshToken, created.secrets.csrfToken);

      const stored = (
        fixture.harness.database.prepare(
          'SELECT session_token_digest, refresh_token_digest, csrf_token_digest FROM ocentra_account_browser_sessions WHERE session_id = ?'
        ).get as (...parameters: SQLiteParameter[]) => Record<string, unknown> | undefined
      )(created.identity.sessionId);
      assert.equal(stored?.session_token_digest, await sha256Hex(created.secrets.sessionToken));
      assert.equal(stored?.refresh_token_digest, await sha256Hex(created.secrets.refreshToken));
      assert.equal(stored?.csrf_token_digest, await sha256Hex(created.secrets.csrfToken));
      assert.equal(stored?.session_token_digest === created.secrets.sessionToken, false);
      assert.equal(isDigest(String(stored?.session_token_digest)), true);

      assert.equal((await store.read(created.secrets.sessionToken)).status, 'active');
      assert.equal(await store.verifyCsrf(created.secrets.sessionToken, created.secrets.csrfToken), true);
      assert.equal(await store.verifyCsrf(created.secrets.sessionToken, 'wrong-csrf-token'), false);

      const expiredAccessAt = new Date(Date.parse(created.identity.issuedAt) + 1).toISOString();
      const restoreAccessAt = new Date(Date.now() + 30 * 60 * 1_000).toISOString();
      const accessExpiry = fixture.harness.database.prepare(
        'UPDATE ocentra_account_browser_sessions SET access_expires_at = ? WHERE session_id = ?'
      );
      (accessExpiry.run as (...parameters: SQLiteParameter[]) => unknown)(expiredAccessAt, created.identity.sessionId);
      assert.deepEqual(await store.read(created.secrets.sessionToken), { status: 'rejected', reason: 'expired' });
      (accessExpiry.run as (...parameters: SQLiteParameter[]) => unknown)(restoreAccessAt, created.identity.sessionId);

      const rotated = await store.rotate(created.secrets.refreshToken, 'store-refresh');
      assert.equal(rotated.status, 'accepted');
      if (rotated.status !== 'accepted' || rotated.identity === null) return;
      assert.notEqual(rotated.secrets.refreshToken, created.secrets.refreshToken);
      assert.equal((await store.readRefresh(rotated.secrets.refreshToken)).status, 'active');
      assert.equal(await store.verifyRefreshCsrf(rotated.secrets.refreshToken, rotated.secrets.csrfToken), true);

      const loggedOut = await store.logoutRefresh(rotated.secrets.refreshToken, 'store-logout');
      assert.equal(loggedOut.status, 'accepted');
      assert.deepEqual(await store.readRefresh(rotated.secrets.refreshToken), {
        status: 'rejected',
        reason: 'revoked',
      });

      const replay = await createBrowserSessionStore(fixture.harness.d1).rotate(
        created.secrets.refreshToken,
        'store-replay'
      );
      assert.deepEqual(replay, { status: 'rejected', reason: 'replay' });
    } finally {
      fixture.harness.close();
    }
  });

  it('revokes the complete refresh family and revalidates current Account authority', async () => {
    const fixture = await createFixture();
    try {
      const store = createBrowserSessionStore(fixture.harness.d1);
      const created = await store.create(fixture.authority, 'store-revoke');
      assert.equal(created.status, 'accepted');
      if (created.status !== 'accepted' || created.identity === null) return;

      const revoked = await store.revokeAll(fixture.authority, 'store-global-revoke');
      assert.equal(revoked.status, 'accepted');
      assert.equal((await store.read(created.secrets.sessionToken)).status, 'rejected');
      assert.equal((await store.readRefresh(created.secrets.refreshToken)).status, 'rejected');

      const staleAuthority = fixture.harness.database.prepare(
        "UPDATE ocentra_account_identity_current_authority SET device_trust_state = 'revoked' WHERE provider = ? AND provider_subject = ?"
      );
      (staleAuthority.run as (...parameters: SQLiteParameter[]) => unknown)('firebase', fixture.providerSubject);
      const second = await store.create(fixture.authority, 'store-stale-authority');
      assert.deepEqual(second, { status: 'rejected', reason: 'invalid' });
    } finally {
      fixture.harness.close();
    }
  });

  it('fails closed for absent custody binding, malformed credentials, and missing schema', async () => {
    const missingBinding = createBrowserSessionStore(undefined);
    assert.deepEqual(await missingBinding.read('not-a-session-cookie'), {
      status: 'manual-required',
      reason: 'binding-missing',
    });

    const harness = new SQLiteD1Harness();
    try {
      const store = createBrowserSessionStore(harness.d1);
      assert.deepEqual(await store.read('malformed'), { status: 'manual-required', reason: 'schema-missing' });
      assert.deepEqual(await store.read(null), { status: 'manual-required', reason: 'schema-missing' });
    } finally {
      harness.close();
    }
  });
});
