import { isLocalArtifactProviderErrorCode, LocalArtifactProviderError } from './local-artifact-provider-error';
import {
  LocalArtifactProviderProtocolVersion,
  type LocalArtifactProviderReady,
  type LocalArtifactProviderResponse,
} from './local-artifact-provider-protocol';
import {
  boundedProviderString,
  isProviderRecord,
  parseProviderIdentity,
  providerRecordHasOnlyKeys,
} from './local-artifact-provider-protocol-values';

export function parseProviderReady(value: unknown): LocalArtifactProviderReady {
  if (
    !isProviderRecord(value) ||
    !providerRecordHasOnlyKeys(value, ['protocol_version', 'provider_instance_id', 'binary_sha256', 'root_identity']) ||
    value['protocol_version'] !== LocalArtifactProviderProtocolVersion
  ) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider ready frame is invalid');
  }
  const binarySha256 = boundedProviderString(value['binary_sha256'], 64, 'provider binary digest');
  if (!/^[0-9a-f]{64}$/u.test(binarySha256)) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider binary digest is invalid');
  }
  const providerInstanceId = boundedProviderString(value['provider_instance_id'], 128, 'provider instance id');
  if (!/^[0-9a-f]{32,128}$/u.test(providerInstanceId)) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider instance id is invalid');
  }
  return {
    protocolVersion: LocalArtifactProviderProtocolVersion,
    providerInstanceId,
    binarySha256,
    rootIdentity: parseProviderIdentity(value['root_identity']),
  };
}

function validateResponseEnvelope(value: unknown): asserts value is Record<string, unknown> & { readonly ok: boolean } {
  if (
    !isProviderRecord(value) ||
    !providerRecordHasOnlyKeys(value, [
      'protocol_version',
      'request_id',
      'operation',
      'nonce',
      'ok',
      'result',
      'error',
    ]) ||
    value['protocol_version'] !== LocalArtifactProviderProtocolVersion ||
    typeof value['ok'] !== 'boolean'
  ) {
    throw new LocalArtifactProviderError('protocol-frame', 'provider response is invalid');
  }
}

interface ResponseAuthority {
  readonly requestId: string;
  readonly operation: string;
  readonly nonce: string;
}

function parseResponseAuthority(value: Record<string, unknown>, expected: ResponseAuthority): ResponseAuthority {
  const requestId = boundedProviderString(value['request_id'], 128, 'provider response request id');
  const operation = boundedProviderString(value['operation'], 64, 'provider response operation');
  const nonce = boundedProviderString(value['nonce'], 128, 'provider response nonce');
  if (requestId !== expected.requestId || operation !== expected.operation || nonce !== expected.nonce) {
    throw new LocalArtifactProviderError(
      'protocol-frame',
      'provider response authority echo does not match the request'
    );
  }
  return { requestId, operation, nonce };
}

function parseSuccessfulResponse(
  value: Record<string, unknown>,
  authority: ResponseAuthority
): LocalArtifactProviderResponse {
  const hasResult = Object.prototype.hasOwnProperty.call(value, 'result');
  const hasError = Object.prototype.hasOwnProperty.call(value, 'error');
  if (!hasResult || hasError || value['result'] === undefined) {
    throw new LocalArtifactProviderError('protocol-frame', 'successful provider response must contain only a result');
  }
  return {
    protocolVersion: LocalArtifactProviderProtocolVersion,
    ...authority,
    ok: true,
    result: value['result'],
  };
}

function parseFailedResponse(
  value: Record<string, unknown>,
  authority: ResponseAuthority
): LocalArtifactProviderResponse {
  const hasResult = Object.prototype.hasOwnProperty.call(value, 'result');
  const hasError = Object.prototype.hasOwnProperty.call(value, 'error');
  const error = value['error'];
  if (
    hasResult ||
    !hasError ||
    !isProviderRecord(error) ||
    !providerRecordHasOnlyKeys(error, ['code', 'message']) ||
    !isLocalArtifactProviderErrorCode(error['code'])
  ) {
    throw new LocalArtifactProviderError('protocol-frame', 'failed provider response must contain only a typed error');
  }
  return {
    protocolVersion: LocalArtifactProviderProtocolVersion,
    ...authority,
    ok: false,
    error: {
      code: error['code'],
      message: boundedProviderString(error['message'], 2_048, 'provider error message'),
    },
  };
}

export function parseProviderResponse(
  value: unknown,
  expected: { readonly requestId: string; readonly operation: string; readonly nonce: string }
): LocalArtifactProviderResponse {
  validateResponseEnvelope(value);
  const authority = parseResponseAuthority(value, expected);
  return value.ok ? parseSuccessfulResponse(value, authority) : parseFailedResponse(value, authority);
}
