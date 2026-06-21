import { describe, expect, it } from 'vitest';
import { BrowserChannel, BrowserCustodyLabel, BrowserFamily } from '@ocentra-parent/schema-domain/browser-values';
import { BrowserEvidenceSchemaVersion } from '@ocentra-parent/schema-domain/browser-schemas';
import {
  BrowserManagedProfileLifecycleState,
  BrowserManagedProfileStoreEntrySchema,
} from '@ocentra-parent/schema-domain/browser-managed-profile-store';

describe('browser managed profile store contracts', () => {
  it('accepts redacted managed profile store entries for portal DTOs', () => {
    const parsed = BrowserManagedProfileStoreEntrySchema.safeParse(profileStoreEntry());

    expect(parsed.success).toBe(true);
    if (parsed.success) {
      expect(parsed.data.profilePathRef).toBe('managed-profile-redacted');
      expect(parsed.data.profileRootRef).toBe('managed-profile-root-redacted');
      expect(parsed.data.lifecycleState).toBe('ready');
      expect(parsed.data.deletedAt).toBeNull();
    }
  });

  it('rejects raw profile path refs before portal consumption', () => {
    const parsed = BrowserManagedProfileStoreEntrySchema.safeParse({
      ...profileStoreEntry(),
      profilePathRef: 'C:\\Users\\kid\\AppData\\Local\\Google\\Chrome\\User Data\\Default',
    });

    expect(parsed.success).toBe(false);
  });

  it('rejects inconsistent missing and deleted profile lifecycle states', () => {
    const missingWithoutTimestamp = BrowserManagedProfileStoreEntrySchema.safeParse({
      ...profileStoreEntry(),
      lifecycleState: BrowserManagedProfileLifecycleState.Missing,
      missingSince: null,
      repairReason: 'managed-profile-dir-missing',
    });
    const deletedWithoutDeletedAt = BrowserManagedProfileStoreEntrySchema.safeParse({
      ...profileStoreEntry(),
      lifecycleState: BrowserManagedProfileLifecycleState.Deleted,
      deletedAt: null,
      repairReason: 'managed-profile-deleted',
    });

    expect(missingWithoutTimestamp.success).toBe(false);
    expect(deletedWithoutDeletedAt.success).toBe(false);
  });
});

function profileStoreEntry() {
  return {
    schemaVersion: BrowserEvidenceSchemaVersion,
    profileId: 'managed-browser-profile-dev',
    profilePathRef: 'managed-profile-redacted',
    profileRootRef: 'managed-profile-root-redacted',
    profileScopeId: 'managed-profile-scope-dev',
    deviceId: 'local-dev-agent',
    browserFamily: BrowserFamily.Chrome,
    browserChannel: BrowserChannel.Stable,
    lifecycleState: BrowserManagedProfileLifecycleState.Ready,
    custodyLabel: BrowserCustodyLabel.ChildDeviceLocal,
    policyRevision: 'browser-policy-revision-dev',
    createdAt: '2026-05-21T03:29:50Z',
    updatedAt: '2026-05-21T03:30:00Z',
    missingSince: null,
    repairedAt: null,
    deletedAt: null,
    repairReason: null,
  };
}
