// @ts-nocheck
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const workflowPath = resolve(process.cwd(), ".github/workflows/release.yml");
const workflow = readFileSync(workflowPath, "utf8");

describe("release workflow", () => {
  it("builds both Apple Silicon and Intel macOS artifacts", () => {
    expect(workflow).toContain("aarch64-apple-darwin");
    expect(workflow).toContain("x86_64-apple-darwin");
  });

  it("passes Apple signing and notarization secrets to the Tauri action", () => {
    expect(workflow).toContain("APPLE_CERTIFICATE");
    expect(workflow).toContain("APPLE_CERTIFICATE_PASSWORD");
    expect(workflow).toContain("APPLE_SIGNING_IDENTITY");
    expect(workflow).toContain("APPLE_ID");
    expect(workflow).toContain("APPLE_PASSWORD");
    expect(workflow).toContain("APPLE_TEAM_ID");
  });

  it("prepares an App Store Connect API key file for macOS notarization", () => {
    expect(workflow).toContain("APPLE_API_KEY");
    expect(workflow).toContain("APPLE_API_KEY_P8");
    expect(workflow).toContain("APPLE_API_KEY_PATH");
  });

  it("does not reference secrets directly inside step conditions", () => {
    expect(workflow).not.toMatch(/if:\s.*secrets\./);
  });
});
