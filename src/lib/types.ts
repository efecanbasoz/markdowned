export type FileType = "file" | "directory";

export interface FileEntry {
  name: string;
  path: string;
  fileType: FileType;
  children?: FileEntry[];
}

export interface PreviewResult {
  html: string;
  frontmatter: string | null;
}

export type Provider = "ollama" | "google" | "openai" | "anthropic" | "custom";

export interface CompletionConfig {
  provider: Provider;
  apiKey: string;
  model: string;
  baseUrl: string;
  custom: {
    baseUrl: string;
    model: string;
  };
}

export interface AppConfig {
  completion: CompletionConfig;
  lastWorkspace: string | null;
}
