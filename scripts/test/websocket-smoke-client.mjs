export function runAgentEventWebSocketSession({
  wsUrl,
  headers,
  timeoutMs,
  timeoutMessage,
  parseMessage,
  onOpen,
  onEvent,
  errorMessage = 'WebSocket smoke failed',
  closeMessage = 'WebSocket smoke closed before completion',
}) {
  return new Promise((resolve, reject) => {
    const socket = createWebSocket(wsUrl, headers);
    let settled = false;
    const timer = setTimeout(() => fail(new Error(resolveMessage(timeoutMessage))), timeoutMs);

    const complete = (value) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      socket.close();
      resolve(value);
    };

    const fail = (error) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timer);
      socket.close();
      reject(error);
    };

    const controls = {
      complete,
      fail,
      sendJson: (value) => socket.send(JSON.stringify(value)),
      socket,
    };

    socket.addEventListener('open', () => {
      try {
        onOpen(controls);
      } catch (error) {
        fail(error instanceof Error ? error : new Error(String(error)));
      }
    });

    socket.addEventListener('message', (message) => {
      try {
        onEvent(parseMessage(message), controls);
      } catch (error) {
        fail(error instanceof Error ? error : new Error(String(error)));
      }
    });

    socket.addEventListener('close', () => {
      if (!settled) {
        fail(new Error(resolveMessage(closeMessage)));
      }
    });

    socket.addEventListener('error', () => fail(new Error(resolveMessage(errorMessage))));
  });
}

export function sendAgentWebSocketCommand({
  wsUrl,
  headers,
  command,
  events,
  parseMessage,
  timeoutMs,
  timeoutMessage,
  errorMessage,
  closeMessage,
  ignoredEvents = ['agent.connection.ready'],
}) {
  const ignored = new Set(ignoredEvents);
  return runAgentEventWebSocketSession({
    wsUrl,
    headers,
    timeoutMs,
    timeoutMessage,
    errorMessage,
    closeMessage,
    parseMessage,
    onOpen: ({ sendJson }) => sendJson(command),
    onEvent: (parsed, { complete }) => {
      events?.push(parsed.event);
      if (ignored.has(parsed.event)) {
        return;
      }
      complete(parsed);
    },
  });
}

export function withTimeout(promise, timeoutMs, message) {
  let timer;
  return Promise.race([
    promise,
    new Promise((_, reject) => {
      timer = setTimeout(() => reject(new Error(resolveMessage(message))), timeoutMs);
    }),
  ]).finally(() => clearTimeout(timer));
}

function createWebSocket(wsUrl, headers) {
  if (headers === undefined) {
    return new WebSocket(wsUrl);
  }
  return new WebSocket(wsUrl, { headers });
}

function resolveMessage(message) {
  return typeof message === 'function' ? message() : message;
}
