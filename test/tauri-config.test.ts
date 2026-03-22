// @ts-nocheck
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

describe("tauri desktop security config", () => {
  it("allows Tauri IPC on desktop custom protocols", () => {
    const configPath = resolve(process.cwd(), "src-tauri/tauri.conf.json");
    const config = JSON.parse(readFileSync(configPath, "utf8")) as {
      app?: {
        security?: {
          csp?: string;
        };
      };
    };

    const csp = config.app?.security?.csp ?? "";

    expect(csp).toContain("connect-src");
    expect(csp).toContain("ipc:");
    expect(csp).toContain("http://ipc.localhost");
    expect(csp).toContain("https://ipc.localhost");
  });
});
