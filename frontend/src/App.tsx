import { useCallback, useRef, useState } from "react";
import "./App.css";

function useStreamLog() {
  const [running, setRunning] = useState(false);
  const [log, setLog] = useState("");
  const abortRef = useRef<AbortController | null>(null);

  // Appends streamed text without losing previous output
  const append = useCallback((text: string) => {
    if (!text) return;
    setLog((prev) => prev + text);
  }, []);

  const start = useCallback(async () => {
    if (running) return;

    setRunning(true);
    setLog("");

    const controller = new AbortController();
    abortRef.current = controller;

    try {
      const r = await fetch("/api/run-brutehash-live", {
        signal: controller.signal,
        cache: "no-store",
      });

      if (!r.ok) throw new Error(`HTTP ${r.status}`);
      if (!r.body) throw new Error("No response body stream");

      const reader = r.body.getReader();
      const decoder = new TextDecoder();

      // Read the HTTP response as a live byte stream
      while (true) {
        const { value, done } = await reader.read();
        if (done) break;

        append(decoder.decode(value, { stream: true }));
      }

      append(decoder.decode());
    } catch (e: any) {
      append(
        e?.name === "AbortError"
          ? "Client -> stopped\n"
          : `\n Client -> ${e?.message ?? String(e)}\n` 
      );
    } finally {
      setRunning(false);
      abortRef.current = null;
    }
  }, [append, running]);

  // Cancels the active fetch and kills the backend process
  const stop = useCallback(() => {
    abortRef.current?.abort();
  }, []);

  const clear = useCallback(() => setLog(""), []);

  return { running, log, start, stop, clear };
}

export default function App() {
  const { running, log, start, stop, clear } = useStreamLog();

  return (
    <div className="app">
      <header className="header">
        <h2 className="title">Brutehash</h2>

        <div className="controls">
          <button className="btn" onClick={start} disabled={running}>
            {running ? "Running..." : "Start"}
          </button>

          <button className="btn" onClick={stop} disabled={!running}>
            Stop
          </button>

          <button className="btn btnSecondary" onClick={clear} disabled={running || !log}>
            Clear
          </button>
        </div>
      </header>

      <pre className="log">
        {log || (running ? "starting...\n" : "")}
      </pre>
    </div>
  );
}
