import express from "express";
import path from "path";
import { spawn } from "child_process";

const app = express();

app.get("/api/run-brutehash-live", (req, res) => {
  res.setHeader("Content-Type", "text/plain; charset=utf-8");
  res.setHeader("Cache-Control", "no-cache, no-transform");
  res.setHeader("Connection", "keep-alive");
  res.setHeader("X-Accel-Buffering", "no");
  (res as any).flushHeaders?.();

  const exePath = path.resolve(
    process.cwd(),
    "..",
    "brutehash",
    "target",
    "release",
    "brutehash.exe"
  );

  const child = spawn(exePath, [], {
    windowsHide: true,
    stdio: ["ignore", "pipe", "pipe"],
  });

  child.stderr.on("data", (d) => res.write(d));
  child.stdout.on("data", (d) => res.write(d));

  child.on("error", (e) => {
    res.write(`{"type":"server_error","msg":${JSON.stringify(String(e))}}\n`);
    res.end();
  });

  child.on("close", (code) => {
    res.write(`{"type":"exit","code":${code ?? -1}}\n`);
    res.end();
  });

  req.on("close", () => {
    if (process.platform === "win32") {
      spawn("taskkill", ["/pid", String(child.pid), "/t", "/f"]);
    } else {
      child.kill("SIGKILL");
    }
  });
});

app.listen(3000, () => console.log("Backend on http://localhost:3000"));
