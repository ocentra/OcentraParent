import fs from 'node:fs';
import type { PersistedMutation } from './local-artifact-transaction-codec';
import { assertLocalArtifactIdentity } from './local-artifact-path';

export function assertMissingMutationTarget(backupPath: string, mutation: PersistedMutation): void {
  if (mutation.targetIdentity == null || mutation.targetKind == null) {
    if (fs.existsSync(backupPath)) {
      throw new Error('local artifact transaction has an unexpected backup');
    }
    return;
  }
  if (fs.existsSync(backupPath)) {
    assertLocalArtifactIdentity(backupPath, mutation.targetKind, mutation.targetIdentity);
    return;
  }
  if (mutation.kind !== 'remove') {
    throw new Error('local artifact replacement lost both its target and backup');
  }
}
