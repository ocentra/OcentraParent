export type ReadModelResult<TValue, TReason extends string = string> =
  | {
      readonly ok: true;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly reason: TReason;
    };

export type StatefulReadModelResult<TState, TValue, TReason extends string = string> =
  | {
      readonly ok: true;
      readonly state: TState;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly state: TState;
      readonly reason: TReason;
    };

export type ParsedPayloadResult<TValue, TReason extends string = string> =
  | {
      readonly parseState: 'parsed';
      readonly value: TValue;
    }
  | {
      readonly parseState: 'failed';
      readonly reason: TReason;
    };
