import { decodeApiPath } from '../types/brands';

export const ApiVersion = {
  V1: 'v1',
} as const;

export const ApiPathPrefix = decodeApiPath(`/api/${ApiVersion.V1}`);
