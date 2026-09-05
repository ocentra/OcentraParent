import { type LocalArtifactProviderErrorCode } from './local-artifact-provider-error';

export const LocalArtifactProviderProtocolVersion = 1;
export const MaximumProviderFrameBytes = 96 * 1024 * 1024;
export const MaximumProviderAppendBytes = 1024 * 1024;
export const MaximumProviderReplaceBytes = 64 * 1024 * 1024;
export const MaximumProviderReadBytes = 64 * 1024 * 1024;
export const MaximumProviderTransactionMutations = 256;
export const MaximumProviderRelativePathBytes = 4_096;

export interface LocalArtifactProviderIdentity {
  readonly device: string;
  readonly inode: string;
}

export interface LocalArtifactProviderStat {
  readonly size: number;
  readonly modified_ms: number;
  readonly is_directory: boolean;
  readonly identity: LocalArtifactProviderIdentity;
}

export interface LocalArtifactProviderSnapshot {
  readonly content: Buffer;
  readonly stat: LocalArtifactProviderStat;
}

export interface LocalArtifactProviderEntry {
  readonly name: string;
  readonly is_directory: boolean;
}

export type LocalArtifactProviderTransactionMutation =
  | {
      readonly kind: 'replace';
      readonly relative_path: string;
      readonly payload_base64: string;
    }
  | { readonly kind: 'remove'; readonly relative_path: string }
  | { readonly kind: 'removeTree'; readonly relative_path: string };

export type LocalArtifactProviderOperation =
  | { readonly kind: 'beginLease' }
  | { readonly kind: 'endLease'; readonly lease_id: string }
  | { readonly kind: 'recover' }
  | { readonly kind: 'ensureDirectory'; readonly relative_path: string }
  | { readonly kind: 'syncDirectory'; readonly relative_path: string }
  | { readonly kind: 'stat'; readonly relative_path: string }
  | {
      readonly kind: 'readSnapshot';
      readonly relative_path: string;
      readonly maximum_bytes: number;
    }
  | {
      readonly kind: 'append';
      readonly relative_path: string;
      readonly payload_base64: string;
    }
  | {
      readonly kind: 'replace';
      readonly relative_path: string;
      readonly payload_base64: string;
    }
  | { readonly kind: 'remove'; readonly relative_path: string }
  | { readonly kind: 'list'; readonly relative_path: string }
  | { readonly kind: 'removeTree'; readonly relative_path: string }
  | {
      readonly kind: 'applyTransaction';
      readonly mutations: readonly LocalArtifactProviderTransactionMutation[];
    }
  | { readonly kind: 'shutdown' };

export interface LocalArtifactProviderRequest {
  readonly protocol_version: number;
  readonly request_id: string;
  readonly nonce: string;
  readonly lease_id: string | null;
  readonly operation: LocalArtifactProviderOperation;
}

export interface LocalArtifactProviderReady {
  readonly protocolVersion: number;
  readonly providerInstanceId: string;
  readonly binarySha256: string;
  readonly rootIdentity: LocalArtifactProviderIdentity;
}

export type LocalArtifactProviderResponse =
  | {
      readonly protocolVersion: number;
      readonly requestId: string;
      readonly operation: string;
      readonly nonce: string;
      readonly ok: true;
      readonly result: unknown;
    }
  | {
      readonly protocolVersion: number;
      readonly requestId: string;
      readonly operation: string;
      readonly nonce: string;
      readonly ok: false;
      readonly error: {
        readonly code: LocalArtifactProviderErrorCode;
        readonly message: string;
      };
    };

export function operationName(operation: LocalArtifactProviderOperation): string {
  return operation.kind;
}
