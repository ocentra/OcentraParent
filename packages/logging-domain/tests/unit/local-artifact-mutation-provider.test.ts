import { expect, it } from 'vitest';
import { LocalArtifactProviderError } from '../../src/local-artifact-provider-error';
import { LocalArtifactProviderProtocolVersion } from '../../src/local-artifact-provider-protocol';
import { parseProviderEntries, parseProviderSnapshot } from '../../src/local-artifact-provider-protocol-collections';
import { parseProviderReady, parseProviderResponse } from '../../src/local-artifact-provider-protocol-parse';
import { parseProviderStat } from '../../src/local-artifact-provider-protocol-results';

const validIdentity = { device: '17', inode: '42' };
const validReady = {
  protocol_version: LocalArtifactProviderProtocolVersion,
  provider_instance_id: 'b'.repeat(64),
  binary_sha256: 'a'.repeat(64),
  root_identity: validIdentity,
};
const expectedResponse = {
  requestId: 'request-1',
  operation: 'stat',
  nonce: 'nonce-1',
};
const validSuccessResponse = {
  protocol_version: LocalArtifactProviderProtocolVersion,
  request_id: expectedResponse.requestId,
  operation: expectedResponse.operation,
  nonce: expectedResponse.nonce,
  ok: true,
  result: { size: 0 },
};
const validFailedResponse = {
  protocol_version: LocalArtifactProviderProtocolVersion,
  request_id: expectedResponse.requestId,
  operation: expectedResponse.operation,
  nonce: expectedResponse.nonce,
  ok: false,
  error: { code: 'not-found', message: 'artifact is absent' },
};
const validStat = {
  size: 5,
  modified_ms: 123,
  is_directory: false,
  identity: validIdentity,
};
const validSnapshot = {
  content_base64: Buffer.from('hello', 'utf8').toString('base64'),
  stat: validStat,
};

function expectProtocolFrameFailure(operation: () => unknown): void {
  expect(operation).toThrowError(LocalArtifactProviderError);
}

it('parses an exact provider Ready frame', () => {
  expect(parseProviderReady(validReady)).toEqual({
    protocolVersion: LocalArtifactProviderProtocolVersion,
    providerInstanceId: 'b'.repeat(64),
    binarySha256: 'a'.repeat(64),
    rootIdentity: validIdentity,
  });
  expectProtocolFrameFailure(() => parseProviderReady({ ...validReady, provider_instance_id: 'B'.repeat(64) }));
  expectProtocolFrameFailure(() => parseProviderReady({ ...validReady, provider_instance_id: 'b'.repeat(31) }));
});

it('preserves full canonical Windows volume and file identities', () => {
  const identity = {
    device: ((1n << 64n) - 1n).toString(),
    inode: ((1n << 128n) - 1n).toString(),
  };
  expect(parseProviderReady({ ...validReady, root_identity: identity }).rootIdentity).toEqual(identity);
  expectProtocolFrameFailure(() =>
    parseProviderReady({
      ...validReady,
      root_identity: { ...identity, device: (1n << 64n).toString() },
    })
  );
  expectProtocolFrameFailure(() =>
    parseProviderReady({
      ...validReady,
      root_identity: { ...identity, inode: (1n << 128n).toString() },
    })
  );
  expectProtocolFrameFailure(() =>
    parseProviderReady({ ...validReady, root_identity: { device: '017', inode: '42' } })
  );
});

it('parses exact successful and failed provider responses', () => {
  expect(parseProviderResponse(validSuccessResponse, expectedResponse)).toEqual({
    protocolVersion: LocalArtifactProviderProtocolVersion,
    requestId: expectedResponse.requestId,
    operation: expectedResponse.operation,
    nonce: expectedResponse.nonce,
    ok: true,
    result: { size: 0 },
  });
  expect(parseProviderResponse(validFailedResponse, expectedResponse)).toEqual({
    protocolVersion: LocalArtifactProviderProtocolVersion,
    requestId: expectedResponse.requestId,
    operation: expectedResponse.operation,
    nonce: expectedResponse.nonce,
    ok: false,
    error: { code: 'not-found', message: 'artifact is absent' },
  });
});

it('parses exact stat, snapshot, and entry results', () => {
  expect(parseProviderStat(validStat)).toEqual(validStat);

  const snapshot = parseProviderSnapshot(validSnapshot, 5);
  expect(snapshot?.content.toString('utf8')).toBe('hello');
  expect(snapshot?.stat).toEqual(validStat);

  expect(
    parseProviderEntries([
      { name: 'scope', is_directory: true },
      { name: 'run.ndjson', is_directory: false },
    ])
  ).toEqual([
    { name: 'scope', is_directory: true },
    { name: 'run.ndjson', is_directory: false },
  ]);
});

it('rejects unknown fields at every protocol boundary', () => {
  expectProtocolFrameFailure(() => parseProviderReady({ ...validReady, unexpected: true }));
  expectProtocolFrameFailure(() =>
    parseProviderResponse({ ...validSuccessResponse, unexpected: true }, expectedResponse)
  );
  expectProtocolFrameFailure(() => parseProviderStat({ ...validStat, unexpected: true }));
  expectProtocolFrameFailure(() => parseProviderSnapshot({ ...validSnapshot, unexpected: true }, 5));
  expectProtocolFrameFailure(() => parseProviderEntries([{ name: 'entry', is_directory: false, unexpected: true }]));
});

it('rejects contradictory result and error fields', () => {
  expectProtocolFrameFailure(() =>
    parseProviderResponse(
      { ...validSuccessResponse, error: { code: 'io', message: 'unexpected failure' } },
      expectedResponse
    )
  );
  expectProtocolFrameFailure(() =>
    parseProviderResponse({ ...validSuccessResponse, result: undefined }, expectedResponse)
  );
  expectProtocolFrameFailure(() =>
    parseProviderResponse({ ...validFailedResponse, result: { removed: true } }, expectedResponse)
  );
  expectProtocolFrameFailure(() =>
    parseProviderResponse(
      {
        protocol_version: LocalArtifactProviderProtocolVersion,
        request_id: expectedResponse.requestId,
        operation: expectedResponse.operation,
        nonce: expectedResponse.nonce,
        ok: false,
      },
      expectedResponse
    )
  );
});

it('rejects a response whose authority echo does not match the request', () => {
  expectProtocolFrameFailure(() =>
    parseProviderResponse({ ...validSuccessResponse, request_id: 'other-request' }, expectedResponse)
  );
  expectProtocolFrameFailure(() =>
    parseProviderResponse({ ...validSuccessResponse, operation: 'replace' }, expectedResponse)
  );
  expectProtocolFrameFailure(() =>
    parseProviderResponse({ ...validSuccessResponse, nonce: 'other-nonce' }, expectedResponse)
  );
});

it('rejects negative and unsafe stat sizes', () => {
  expectProtocolFrameFailure(() => parseProviderStat({ ...validStat, size: -1 }));
  expectProtocolFrameFailure(() => parseProviderStat({ ...validStat, size: Number.MAX_SAFE_INTEGER + 1 }));
  expectProtocolFrameFailure(() => parseProviderStat({ ...validStat, modified_ms: Number.MAX_SAFE_INTEGER + 1 }));
});

it('rejects directory snapshots and noncanonical base64', () => {
  expectProtocolFrameFailure(() =>
    parseProviderSnapshot(
      {
        ...validSnapshot,
        stat: { ...validStat, is_directory: true },
      },
      5
    )
  );
  expectProtocolFrameFailure(() => parseProviderSnapshot({ ...validSnapshot, content_base64: 'aGVsbG8' }, 5));
});

it('rejects a snapshot larger than the request bound', () => {
  expectProtocolFrameFailure(() => parseProviderSnapshot(validSnapshot, 4));
});

it('rejects unsafe provider entry names', () => {
  for (const name of ['', '.', '..', 'nested/name', 'nested\\name', 'nul\0name', 'x'.repeat(256)]) {
    expectProtocolFrameFailure(() => parseProviderEntries([{ name, is_directory: false }]));
  }
});
