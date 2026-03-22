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

export interface TabState {
  id: string;
  filePath: string;
  fileName: string;
  content: string;
  previewHtml: string;
  frontmatter: string | null;
  line: number;
  column: number;
  dirty: boolean;
  fileSize: string;
  cursorPos: number;
  scrollTop: number;
}

export interface SearchMatch {
  filePath: string;
  fileName: string;
  lineNumber: number;
  lineContent: string;
  matchStart: number;
  matchEnd: number;
}

export interface WorkspaceEntry {
  root: string;
  name: string;
  entries: FileEntry[];
  collapsed: boolean;
}

export interface WorkspaceSelection {
  root: string;
  name: string;
  entries: FileEntry[];
}

export type ViewMode = "edit" | "preview" | "split";
export type SplitDirection = "horizontal" | "vertical";

export type Provider = "ollama" | "google" | "openai" | "anthropic" | "custom";

export interface CompletionConfig {
  provider: Provider;
  apiKey: string;
  model: string;
  baseUrl: string;
  autoCompletion?: boolean;
  custom: {
    baseUrl: string;
    model: string;
  };
}

export interface AppConfig {
  completion: CompletionConfig;
  workspaces?: string[];
  splitDirection?: SplitDirection;
  theme?: "dark" | "light";
}
