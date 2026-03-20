export type ActiveTab = "edit" | "preview";

let content = $state("");
let previewHtml = $state("");
let frontmatter = $state<string | null>(null);
let activeTab = $state<ActiveTab>("edit");
let line = $state(1);
let column = $state(1);
let dirty = $state(false);
let fileSize = $state("");

export const editor = {
  get content() { return content; },
  set content(v: string) { content = v; },
  get previewHtml() { return previewHtml; },
  set previewHtml(v: string) { previewHtml = v; },
  get frontmatter() { return frontmatter; },
  set frontmatter(v: string | null) { frontmatter = v; },
  get activeTab() { return activeTab; },
  set activeTab(v: ActiveTab) { activeTab = v; },
  get line() { return line; },
  set line(v: number) { line = v; },
  get column() { return column; },
  set column(v: number) { column = v; },
  get dirty() { return dirty; },
  set dirty(v: boolean) { dirty = v; },
  get fileSize() { return fileSize; },
  set fileSize(v: string) { fileSize = v; },
};
