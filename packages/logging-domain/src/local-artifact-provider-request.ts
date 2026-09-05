import path from 'node:path';
import { LocalArtifactProviderError } from './local-artifact-provider-error';
import { currentLocalArtifactLease } from './local-artifact-provider-lease-context';
import { type LocalArtifactProviderOperation } from './local-artifact-provider-protocol';
import { discardProviderSession, startProviderSession } from './local-artifact-provider-registry';
import {
  prepareLocalArtifactProviderRequest,
  sendLocalArtifactProviderRequest,
} from './local-artifact-provider-session';

function operationRequiresLease(operation: LocalArtifactProviderOperation): boolean {
  return ['recover', 'append', 'replace', 'remove', 'removeTree', 'applyTransaction'].includes(operation.kind);
}

function isSessionFailure(error: unknown): error is LocalArtifactProviderError {
  if (!(error instanceof LocalArtifactProviderError)) return false;
  return ['provider-unavailable', 'provider-timeout', 'protocol-frame'].includes(error.code);
}

export function requestProvider(rootDir: string, operation: LocalArtifactProviderOperation): unknown {
  const root = path.resolve(rootDir);
  const prepared = prepareLocalArtifactProviderRequest(operation);
  const session = startProviderSession(root);
  if (
    operationRequiresLease(operation) &&
    (session.leaseId == null || currentLocalArtifactLease(root) !== session.leaseId)
  ) {
    throw new LocalArtifactProviderError('provider-authority', 'provider mutation requires the current root lease');
  }
  try {
    const response = sendLocalArtifactProviderRequest(session, prepared);
    if (!response.ok) {
      throw new LocalArtifactProviderError(response.error.code, response.error.message);
    }
    return response.result;
  } catch (error) {
    if (isSessionFailure(error)) {
      void discardProviderSession(root, session).catch(() => undefined);
    }
    throw error;
  }
}
