import {
  Schema,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from './effect';

const SlashPathString = NonEmptyStringSchema.pipe(
  Schema.filter((value) => value.startsWith('/') || 'Expected a path starting with /')
);

export const ApiPathSchema = SlashPathString.pipe(Schema.brand('ApiPath'));
export type ApiPath = typeof ApiPathSchema.Type;

export const EndpointIdSchema = brandedNonEmptyStringSchema('EndpointId');
export type EndpointId = typeof EndpointIdSchema.Type;

export const PathSegmentSchema = NonEmptyStringSchema.pipe(
  Schema.filter((value) => !value.includes('/') || 'Expected one path segment without /'),
  Schema.brand('PathSegment')
);
export type PathSegment = typeof PathSegmentSchema.Type;

export const QueryParamSchema = brandedNonEmptyStringSchema('QueryParam');
export type QueryParam = typeof QueryParamSchema.Type;

export const HeaderNameSchema = brandedNonEmptyStringSchema('HeaderName');
export type HeaderName = typeof HeaderNameSchema.Type;

export const decodeApiPath = Schema.decodeUnknownSync(ApiPathSchema);
export const decodeEndpointId = Schema.decodeUnknownSync(EndpointIdSchema);
export const decodePathSegment = Schema.decodeUnknownSync(PathSegmentSchema);
export const decodeQueryParam = Schema.decodeUnknownSync(QueryParamSchema);
export const decodeHeaderName = Schema.decodeUnknownSync(HeaderNameSchema);
