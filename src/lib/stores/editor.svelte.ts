import type { TabState, ViewMode, SplitDirection } from "../types";

const MAX_TABS = 20;

let tabs = $state<TabState[]>([]);
let activeTabId = $state<string | null>(null);
let viewMode = $state<ViewMode>("edit");
let splitDirection = $state<SplitDirection>("horizontal");
let theme = $state<"dark" | "light">("dark");

function getActiveTab(): TabState | null {
  if (!activeTabId) return null;
  return tabs.find((t) => t.id === activeTabId) ?? null;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  return `${(bytes / 1024).toFixed(1)} KB`;
}

export const editor = {
  get tabs() {
    return tabs;
  },
  get activeTabId() {
    return activeTabId;
  },
  set activeTabId(v: string | null) {
    activeTabId = v;
  },
  get activeTab(): TabState | null {
    return getActiveTab();
  },
  get viewMode() {
    return viewMode;
  },
  set viewMode(v: ViewMode) {
    viewMode = v;
  },
  get splitDirection() {
    return splitDirection;
  },
  set splitDirection(v: SplitDirection) {
    splitDirection = v;
  },
  get theme() {
    return theme;
  },
  set theme(v: "dark" | "light") {
    theme = v;
  },

  // Backward-compat delegating getters/setters
  get content() {
    return getActiveTab()?.content ?? "";
  },
  set content(v: string) {
    const tab = getActiveTab();
    if (tab) tab.content = v;
  },
  get previewHtml() {
    return getActiveTab()?.previewHtml ?? "";
  },
  set previewHtml(v: string) {
    const tab = getActiveTab();
    if (tab) tab.previewHtml = v;
  },
  get frontmatter() {
    return getActiveTab()?.frontmatter ?? null;
  },
  set frontmatter(v: string | null) {
    const tab = getActiveTab();
    if (tab) tab.frontmatter = v;
  },
  get line() {
    return getActiveTab()?.line ?? 1;
  },
  set line(v: number) {
    const tab = getActiveTab();
    if (tab) tab.line = v;
  },
  get column() {
    return getActiveTab()?.column ?? 1;
  },
  set column(v: number) {
    const tab = getActiveTab();
    if (tab) tab.column = v;
  },
  get dirty() {
    return getActiveTab()?.dirty ?? false;
  },
  set dirty(v: boolean) {
    const tab = getActiveTab();
    if (tab) tab.dirty = v;
  },
  get fileSize() {
    return getActiveTab()?.fileSize ?? "";
  },
  set fileSize(v: string) {
    const tab = getActiveTab();
    if (tab) tab.fileSize = v;
  },

  openTab(filePath: string, fileName: string, content: string) {
    // If tab already exists, switch to it
    const existing = tabs.find((t) => t.id === filePath);
    if (existing) {
      activeTabId = filePath;
      return;
    }

    // QA-003: Enforce max tabs — only close non-dirty tabs, never discard unsaved work
    if (tabs.length >= MAX_TABS) {
      const closeable = tabs.find((t) => t.id !== activeTabId && !t.dirty);
      if (closeable) {
        tabs = tabs.filter((t) => t.id !== closeable.id);
      } else {
        // All tabs are dirty — refuse to open instead of silently discarding work
        console.warn("Cannot open new tab: all tabs have unsaved changes. Save or close a tab first.");
        return;
      }
    }

    const newTab: TabState = {
      id: filePath,
      filePath,
      fileName,
      content,
      previewHtml: "",
      frontmatter: null,
      line: 1,
      column: 1,
      dirty: false,
      fileSize: formatSize(new Blob([content]).size),
      cursorPos: 0,
      scrollTop: 0,
    };

    tabs = [...tabs, newTab];
    activeTabId = filePath;
  },

  closeTab(id: string) {
    const index = tabs.findIndex((t) => t.id === id);
    if (index === -1) return;

    // If closing the active tab, pick a neighbor
    if (activeTabId === id) {
      if (tabs.length === 1) {
        activeTabId = null;
      } else if (index < tabs.length - 1) {
        // Prefer right neighbor
        activeTabId = tabs[index + 1].id;
      } else {
        // Left neighbor
        activeTabId = tabs[index - 1].id;
      }
    }

    tabs = tabs.filter((t) => t.id !== id);
  },

  switchTab(id: string) {
    if (tabs.some((t) => t.id === id)) {
      activeTabId = id;
    }
  },

  updateActiveContent(content: string) {
    const tab = getActiveTab();
    if (tab) {
      tab.content = content;
      tab.dirty = true;
    }
  },

  updateActivePreview(html: string, frontmatter: string | null) {
    const tab = getActiveTab();
    if (tab) {
      tab.previewHtml = html;
      tab.frontmatter = frontmatter;
    }
  },

  updateActiveCursor(line: number, col: number, cursorPos: number) {
    const tab = getActiveTab();
    if (tab) {
      tab.line = line;
      tab.column = col;
      tab.cursorPos = cursorPos;
    }
  },

  saveActiveScrollTop(scrollTop: number) {
    const tab = getActiveTab();
    if (tab) {
      tab.scrollTop = scrollTop;
    }
  },
};
