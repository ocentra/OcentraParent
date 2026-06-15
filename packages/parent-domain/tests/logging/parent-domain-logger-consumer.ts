import { Logger } from '@ocentra-parent/logging-domain/core/logger';
import { getStackTrace, type StackTrace } from '@ocentra-parent/logging-domain/core/stackTrace';

export class ParentDomainLoggerConsumer {
  private readonly log = Logger.instance;

  constructor() {
    this.log.register(import.meta.url);
  }

  private logInfo = (message: string, stackTrace: StackTrace, data?: unknown, enabled = false) => {
    this.log.logInfo(message, stackTrace, data, enabled);
  };

  private logWarn = (message: string, stackTrace: StackTrace, data?: unknown, enabled = false) => {
    this.log.logWarn(message, stackTrace, data, enabled);
  };

  private logError = (message: string, stackTrace: StackTrace, data?: unknown) => {
    this.log.logError(message, stackTrace, data);
  };

  private logDebug = (message: string, stackTrace: StackTrace, data?: unknown, enabled = false) => {
    this.log.logDebug(message, stackTrace, data, enabled);
  };

  emitHelloWorldLogs(): void {
    this.logInfo('parent-domain info log', getStackTrace(), { hello: 'world', level: 'info' }, true);
    this.logWarn('parent-domain warn log', getStackTrace(), { hello: 'world', level: 'warn' }, true);
    this.logError('parent-domain error log', getStackTrace(), { hello: 'world', level: 'error' });
    this.logDebug('parent-domain debug log', getStackTrace(), { hello: 'world', level: 'debug' }, true);
  }
}
