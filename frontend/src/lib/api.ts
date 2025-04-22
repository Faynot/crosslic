type CommandHandler<T = any> = (response: {
  result?: T;
  error?: string;
}) => void;

class CrosslicAPI {
  private static instance: CrosslicAPI;
  private ws: WebSocket | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;
  private handlers = new Map<string, CommandHandler>();

  public call = new Proxy(
    {} as { [key: string]: (...args: any[]) => Promise<any> },
    {
      get: (_, command: string) => {
        return (...args: any[]) => this.sendCommand(command, args);
      },
    },
  );

  private constructor() {
    this.initWebSocket();
  }

  static getInstance(): CrosslicAPI {
    if (!CrosslicAPI.instance) {
      CrosslicAPI.instance = new CrosslicAPI();
    }
    return CrosslicAPI.instance;
  }

  private initWebSocket() {
    this.ws = new WebSocket("ws://localhost:3030/ws");

    this.ws.onopen = () => {
      this.reconnectAttempts = 0;
      console.log("WebSocket connected");
    };

    this.ws.onmessage = (event) => {
      const { id, result, error } = JSON.parse(event.data);
      const handler = this.handlers.get(id);
      if (handler) {
        error ? handler({ error }) : handler({ result });
        this.handlers.delete(id);
      }
    };

    this.ws.onclose = () => {
      if (this.reconnectAttempts < this.maxReconnectAttempts) {
        setTimeout(
          () => {
            this.reconnectAttempts++;
            this.initWebSocket();
          },
          1000 * Math.pow(2, this.reconnectAttempts),
        );
      }
    };
  }

  private async sendCommand<T>(command: string, args: any[]): Promise<T> {
    return new Promise((resolve, reject) => {
      const id = crypto.randomUUID();
      this.handlers.set(id, (response: any) => {
        if (response.error) {
          reject(response.error);
        } else {
          resolve(response.result as T);
        }
      });

      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({ id, command, data: args }));
      } else {
        reject(new Error("WebSocket connection not ready"));
      }
    });
  }
}

export const api = CrosslicAPI.getInstance();
