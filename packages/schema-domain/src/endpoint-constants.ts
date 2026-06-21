import { decodeApiPath, decodeHeaderName } from './endpoint-brands';

export const ApiVersion = {
  V1: 'v1',
} as const;

export const ApiPathPrefix = decodeApiPath(`/api/${ApiVersion.V1}`);

export const HttpMethod = {
  Get: 'GET',
  Post: 'POST',
  Put: 'PUT',
  Delete: 'DELETE',
  Options: 'OPTIONS',
} as const;

export type HttpMethod = (typeof HttpMethod)[keyof typeof HttpMethod];

export const HttpHeader = {
  Authorization: decodeHeaderName('Authorization'),
  ContentType: decodeHeaderName('Content-Type'),
  Origin: decodeHeaderName('Origin'),
} as const;

export const HttpContentType = {
  ApplicationJson: 'application/json',
} as const;

export const HttpStatus = {
  Ok: 200,
  Created: 201,
  NoContent: 204,
  BadRequest: 400,
  Unauthorized: 401,
  Forbidden: 403,
  NotFound: 404,
  InternalServerError: 500,
} as const;
