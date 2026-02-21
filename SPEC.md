# VibeWrite2k - Application Specification

## 1. Overview

**Application Name:** Vibe Write 2k
**Type:** Desktop Markdown Editor and Writing Platform
**Platform:** Cross-platform (Avalonia UI)
**Technology Stack:**
- C# / .NET 8.0
- Avalonia UI 11.2.3 (cross-platform desktop framework)
- AvaloniaEdit (text editor component)
- Markdig (Markdown parser)
- QuestPDF (PDF generation)
- CommunityToolkit.Mvvm (MVVM framework)

## 2. Architecture

### 2.1 MVVM Pattern
- Single `MainWindowViewModel` as the primary view model
- `MainWindow` as the primary view (XAML + code-behind)
- Uses CommunityToolkit.Mvvm `ObservableObject` for property change notifications

### 2.2 Service-Oriented Design
- **FileService** - File I/O (open, save, save-as, folder picking)
- **ProjectService** - Project lifecycle and file management
- **VersioningService** - Snapshot-based version control
- **PrintService** - PDF generation and CUPS-based printing
- **SpellCheckService** - Dictionary-based spell checking with Levenshtein distance suggestions
- **MarkdownParserService** - Markdown-to-AST parsing via Markdig
- **LineDiffService** - Side-by-side line-level diff (LCS algorithm)

### 2.3 Editor Composition
Multiple transformers compose editor behavior:
- **MarkdownColorizingTransformer** - Syntax highlighting, heading sizing, emphasis rendering
- **FocusModeTransformer** - Dims non-focused lines
- **MarkdownSyntaxHider** - Hides raw markdown syntax characters
- **FormattingCommands** - Toggle bold/italic/underline, cycle heading levels

## 3. Data Models

### 3.1 Document
- `FilePath` - Full path to document file
- `FileName` - Computed from path

### 3.2 Project
- `Version` - Schema version (default: 1)
- `FileOrder` - Ordered list of relative file paths
- `DirectoryPath` - Absolute path to project root
- Persisted as `project.json` in the project directory

### 3.3 ProjectFile
- `RelativePath` - Path relative to project directory
- `FileName` - Computed from relative path
- `AbsolutePath` - Full filesystem path

### 3.4 EditorTab
- `ProjectFile` - Associated project file
- `Content` - Current text content
- `CaretOffset` - Cursor position
- `ScrollOffset` - Vertical scroll position
- `IsDirty` - Unsaved changes flag
- `DisplayName` - Filename with `*` suffix when dirty

### 3.5 OutlineItem
- `Title` - Heading text
- `Level` - Heading level (1-3)
- `LineNumber` - 0-based line number in source
- `SourceFilePath` - Source project file (null for single-file mode)
- `IndentMargin` - Computed indentation for visual hierarchy

### 3.6 Version Control Models

**VersionHistory:**
- `Version` - Schema version (default: 1)
- `Commits` - List of VersionCommit entries

**VersionCommit:**
- `Id` - Unique commit ID (format: `yyyyMMdd-HHmmss-<4-hex>`)
- `Timestamp` - UTC datetime
- `Message` - User-provided commit message
- `FileSizeBytes` - File size at commit time

### 3.7 Spell Check Models
- `WordLocation` - Record of (Offset, Length) in document
- `MisspelledWord` - Word text, all occurrence locations, and suggestions

## 4. Features

### 4.1 Single Document Editing
- Create, open, and save Markdown files (`.md`, `.markdown`, `.txt`)
- Rich text formatting via toolbar or keyboard shortcuts
- Real-time syntax highlighting with hidden markup characters
- Headings rendered at display size (H1: 28pt, H2: 22pt, H3: 18pt)
- Bold (`**`), Italic (`*`), and Underline (`++`) support
- 100ms debounce on text changes for re-parsing

### 4.2 Project Mode
- Multi-file writing projects stored as a directory with `project.json`
- File ordering with add/remove/move-up/move-down controls
- Tabbed editing with per-tab state preservation (content, caret, scroll)
- Project files pane (right side, 220px)
- Combined outline across all project files

### 4.3 Navigation Outline
- Extracts H1-H3 headings from Markdown AST
- Displayed in left pane (220px)
- Click-to-navigate with automatic file switching in project mode
- Real-time updates as document is edited
- Toggle visibility with Ctrl+Shift+O

### 4.4 Focus Mode
- Distraction-free writing environment
- Hides menu bar, toolbar, and tab bar
- Dims all non-focused lines (gray #999999)
- Centers editor horizontally (80-character max width)
- Enlarged font (1.4x normal size)
- Vertically centers the focused line on screen
- Enter: Ctrl+Shift+F | Exit: Escape

### 4.5 Version Control
- Available only in project mode
- Snapshot-based (full file copy per commit)
- Storage: `.vibe/<filename>/snapshots/<commitId>.md`
- History: `.vibe/<filename>/history.json`
- Commit with message dialog (Ctrl+K)
- View history with timestamp, message, and file size
- Revert with side-by-side diff view (current vs. snapshot)

### 4.6 Spell Checking
- Dictionary-based using embedded `english-words.txt`
- User dictionary support (`~/.vibe/user-dictionary.txt` on Mac/Linux, `%APPDATA%\Vibe\user-dictionary.txt` on Windows)
- Levenshtein distance suggestions (edit distance 0-2, up to 8 suggestions)
- Markdown-aware: skips code blocks, code inlines, link URLs, and HTML
- Minimum word length: 2 characters
- Supports contractions (e.g., `don't`)
- Interactive dialog with actions: Replace, Replace All, Ignore, Ignore All, Add to Dictionary

### 4.7 Print & PDF Export
- PDF generation via QuestPDF (Letter size, 1" margins)
- Markdown-aware rendering (headings, emphasis styles)
- Physical printing via CUPS (`lp` command) on macOS/Linux
- Export to PDF file with save dialog

## 5. User Interface

### 5.1 Window Properties
- Default size: 900x700
- Minimum size: 400x300
- Resizable
- Default title: "Vibe - Untitled"
- Title format: "Vibe - [filename]" (with `*` when dirty)

### 5.2 Layout Structure

```
┌────────────────────────────────────────────────┐
│ Menu Bar (File | View | Tools | Version)       │
├────────────────────────────────────────────────┤
│ Formatting Toolbar [B] [I] [U] [Header ▼]     │
├────────────────────────────────────────────────┤
│ Tab Bar (project mode only)                    │
├──────────┬────────────────────────┬────────────┤
│ Outline  │                        │ Project    │
│ Pane     │   Text Editor          │ Files Pane │
│ (220px)  │   (AvaloniaEdit)       │ (220px)    │
│          │   14pt, word wrap      │            │
│          │   line numbers         │            │
│          │                        │            │
└──────────┴────────────────────────┴────────────┘
```

### 5.3 Menu Structure

**File:**
- New (Ctrl+N)
- Open (Ctrl+O)
- New Project
- Open Project
- Save (Ctrl+S)
- Save As (Ctrl+Shift+S)
- Print (Ctrl+P)
- Export PDF
- Exit

**View:**
- Toggle Outline (Ctrl+Shift+O)
- Focus Mode (Ctrl+Shift+F)

**Tools:**
- Check Spelling (F7)

**Version** (project mode only):
- Commit (Ctrl+K)
- View History
- Revert to Commit

### 5.4 Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+N | New document |
| Ctrl+O | Open file |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save As |
| Ctrl+B | Bold |
| Ctrl+I | Italic |
| Ctrl+U | Underline |
| Ctrl+Shift+O | Toggle outline |
| Ctrl+Shift+F | Toggle focus mode |
| Ctrl+P | Print |
| Ctrl+K | Commit (project mode) |
| F7 | Spell check |
| Escape | Exit focus mode |

### 5.5 Formatting Toolbar
- Bold (B), Italic (I), Underline (U) toggle buttons
- Header level dropdown: Normal, H1, H2, H3

### 5.6 Dialogs
- Save prompt (Save / Don't Save / Cancel)
- Commit message input
- Version history list (chronological, newest first)
- Revert dialog with side-by-side diff (color-coded: added/deleted/unchanged)
- Spell check interactive dialog (word list, suggestions, action buttons)
- Add file choice (New / Existing)
- New file name input
- Information/confirmation dialogs

## 6. File Formats & Storage

### 6.1 Supported File Types
- Markdown: `.md`, `.markdown`
- Text: `.txt`
- All files: `*`

### 6.2 Project Structure
```
ProjectRoot/
├── project.json           # Project metadata
├── chapter1.md
├── chapter2.md
└── .vibe/                 # Version control
    ├── chapter1.md/
    │   ├── history.json
    │   └── snapshots/
    │       ├── 20250220-143025-a1b2.md
    │       └── ...
    └── chapter2.md/
        ├── history.json
        └── snapshots/
            └── ...
```

### 6.3 project.json Format
```json
{
  "version": 1,
  "fileOrder": [
    "chapter1.md",
    "chapter2.md"
  ]
}
```

### 6.4 history.json Format
```json
{
  "version": 1,
  "commits": [
    {
      "id": "20250220-143025-a1b2",
      "timestamp": "2025-02-20T14:30:25Z",
      "message": "Initial draft",
      "fileSizeBytes": 2048
    }
  ]
}
```

## 7. PDF Rendering

- Page size: Letter (8.5" x 11")
- Margins: 1 inch all sides
- Heading sizes: H1 = 28pt bold, H2 = 22pt bold, H3 = 18pt bold
- Body text: 14pt
- Supports bold, italic, and underline emphasis
- Underline uses Markdig's `++text++` (inserted text / EmphasisExtras)

## 8. Markdown Support

### Rendered Features
- Headings: H1, H2, H3 with visual sizing and hidden `#` characters
- Bold: `**text**` with hidden delimiters
- Italic: `*text*` with hidden delimiters
- Underline: `++text++` with hidden delimiters
- Nested emphasis (e.g., `***bold italic***`)

### Parser Configuration
- Markdig with PreciseSourceLocation enabled
- EmphasisExtras extension (strikethrough, inserted text)

## 9. State Management

- Document dirty flag tracking
- Per-tab state preservation (content, caret, scroll, dirty flag)
- Project and outline pane visibility state
- Focus mode state with full save/restore
- AST cached with 100ms debounce on text changes
- Project file contents and ASTs cached in memory dictionaries

## 10. Constraints & Limitations

1. **Printing** - Only CUPS-based printing (macOS/Linux); no Windows print support
2. **No auto-save** - Manual save required
3. **Outline depth** - Only H1-H3 headings shown
4. **Version control** - Per-file only; no global project-level history
5. **No search/replace** - No built-in find/replace functionality
6. **Single view model** - All UI state in one MainWindowViewModel
7. **Diff algorithm** - Simple LCS; not a full Git-style diff
8. **Spell check dictionary** - English only; scales with dictionary file size

## 11. Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| Avalonia | 11.2.3 | Cross-platform UI framework |
| Avalonia.Desktop | 11.2.3 | Desktop platform support |
| Avalonia.Themes.Fluent | 11.2.3 | Fluent design theme |
| Avalonia.AvaloniaEdit | 11.2.0 | Text editor component |
| Markdig | 0.38.0 | Markdown parser |
| CommunityToolkit.Mvvm | 8.4.0 | MVVM utilities |
| QuestPDF | 2025.12.4 | PDF generation |

## 12. Target Framework

- .NET 8.0
