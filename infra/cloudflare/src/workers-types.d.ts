declare module '@cloudflare/workers-types' {
  export interface AnalyticsEngineDataset {
    writeDataPoint(data: {
      indexes?: ReadonlyArray<string>;
      blobs?: ReadonlyArray<string>;
      doubles?: ReadonlyArray<number>;
    }): void;
  }
  export interface D1PreparedStatement {
    bind(...values: ReadonlyArray<unknown>): D1PreparedStatement;
    first<T>(): Promise<T | null>;
    all<T>(): Promise<{ results: ReadonlyArray<T>; success: true }>;
    run(): Promise<{ results: ReadonlyArray<never>; success: true; meta: { changes: number } }>;
  }
  export interface D1Database {
    prepare(query: string): D1PreparedStatement;
    batch(
      statements: ReadonlyArray<D1PreparedStatement>
    ): Promise<ReadonlyArray<{ results: ReadonlyArray<unknown>; success: true }>>;
    exec(query: string): Promise<{ count: number; duration: number }>;
  }
  export interface DurableObjectId {}
  export interface DurableObjectStub {
    fetch(request: Request): Promise<Response>;
  }
  export interface DurableObjectNamespace {
    idFromName(name: string): DurableObjectId;
    get(id: DurableObjectId): DurableObjectStub;
  }
  export interface DurableObjectState {}
  export interface KVNamespace {
    get(key: string, type?: 'text' | 'json'): Promise<unknown>;
    put(key: string, value: string): Promise<void>;
  }
  export interface Queue {
    send(message: unknown): Promise<void>;
    sendBatch(messages: ReadonlyArray<{ body: unknown }>): Promise<void>;
  }
  export interface QueueMessage<Body = unknown> {
    body: Body;
    attempts: number;
    ack(): void;
    retry(options?: { delaySeconds?: number }): void;
  }
  export interface MessageBatch<Body = unknown> {
    queue: string;
    messages: ReadonlyArray<QueueMessage<Body>>;
  }
  export interface R2ObjectBody {
    text(): Promise<string>;
    json<T>(): Promise<T>;
  }
  export interface R2Bucket {
    get(key: string): Promise<R2ObjectBody | null>;
    put(key: string, value: string): Promise<void>;
  }
}
