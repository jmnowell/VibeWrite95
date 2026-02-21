# VibeWrite95 - TUI Application Specification

A terminal-based Markdown editor with a WordStar-inspired interface, built in Rust.

## 1. Overview

**Application Name:** VibeWrite95
**Type:** Terminal User Interface (TUI) Markdown Editor and Writing Platform
**Platform:** Cross-platform (any terminal supporting ANSI/xterm)
**Language:** Rust
**UI Paradigm:** WordStar-style command-driven TUI with help menu overlay

### 1.1 Technology Stack

| Crate | Purpose |
|-------|---------|
| [ratatui](https://ratatui.rs/) | TUI framework (immediate-mode rendering, layouts, widgets) |
| [crossterm](https://crates.io/crates/crossterm) | Cross-platform terminal backend (input, raw mode, alternate screen) |
| [tui-textarea](https://github.com/rhysd/tui-textarea) | Multi-line text editor widget for ratatui |
| [pulldown-cmark](https://crates.io/crates/pulldown-cmark) | CommonMark Markdown parser (event/pull-based, efficient) |
| [comrak](https://crates.io/crates/comrak) | GFM-compatible Markdown parser (AST-based, for outline extraction) |
| [printpdf](https://crates.io/crates/printpdf) | PDF generation |
| [levenshtein](https://crates.io/crates/levenshtein) | Edit distance for spell check suggestions |
| [serde](https://crates.io/crates/serde) / [serde_json](https://crates.io/crates/serde_json) | JSON serialization for project.json, history.json |
| [chrono](https://crates.io/crates/chrono) | Timestamp handling for version control |
| [dirs](https://crates.io/crates/dirs) | Cross-platform user directory resolution |
| [rand](https://crates.io/crates/rand) | Random hex for commit IDs |

### 1.2 Design Philosophy

The application emulates the look and feel of WordStar from the early 1990s DOS era:

- **Full-screen terminal application** using the alternate screen buffer
- **Command-prefix keyboard system** (^K, ^Q, ^O prefixes like WordStar)
- **Persistent help menu area** at the top of the screen showing available commands
- **Status line** showing file info, cursor position, and mode
- **Ruler line** showing column positions and margins
- **No mouse required** - entirely keyboard-driven (mouse support optional)
- **Monochrome-friendly** color scheme with high-contrast text (white/cyan on blue or black)

## 2. Architecture

### 2.1 Application Structure

```
vibe-write-95/
├── Cargo.toml
├── src/
│   ├── main.rs                  # Entry point, terminal setup, main loop
│   ├── app.rs                   # Application state machine
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── layout.rs            # Screen layout (help area, ruler, status, editor)
│   │   ├── help_menu.rs         # WordStar-style help menu rendering
│   │   ├── status_line.rs       # Status bar rendering
│   │   ├── ruler.rs             # Ruler line widget
│   │   ├── editor_view.rs       # Main text editor area
│   │   ├── outline_pane.rs      # Navigation outline (left pane)
│   │   ├── project_pane.rs      # Project files list (right pane)
│   │   ├── tab_bar.rs           # Tab bar for project mode
│   │   └── dialogs/
│   │       ├── mod.rs
│   │       ├── file_dialog.rs   # File open/save path input
│   │       ├── confirm.rs       # Yes/No/Cancel confirmation
│   │       ├── input.rs         # Single-line text input (commit msg, filename)
│   │       ├── spell_check.rs   # Spell check interactive dialog
│   │       ├── history.rs       # Version history list
│   │       └── diff_view.rs     # Side-by-side diff for revert
│   ├── editor/
│   │   ├── mod.rs
│   │   ├── buffer.rs            # Text buffer with gap buffer or rope
│   │   ├── cursor.rs            # Cursor position and movement
│   │   ├── syntax.rs            # Markdown syntax highlighting for TUI
│   │   └── formatting.rs        # Bold/italic/underline/heading toggle commands
│   ├── services/
│   │   ├── mod.rs
│   │   ├── file_service.rs      # File I/O operations
│   │   ├── project_service.rs   # Project management (project.json)
│   │   ├── versioning.rs        # Snapshot-based version control (.vibe/)
│   │   ├── print_service.rs     # PDF generation and CUPS printing
│   │   ├── spell_check.rs       # Dictionary + Levenshtein suggestions
│   │   ├── markdown_parser.rs   # Markdown parsing and outline extraction
│   │   ├── line_diff.rs         # LCS-based side-by-side diff
│   │   └── word_extractor.rs    # Markdown-aware word extraction for spell check
│   ├── models/
│   │   ├── mod.rs
│   │   ├── document.rs          # Document model
│   │   ├── project.rs           # Project model
│   │   ├── project_file.rs      # ProjectFile model
│   │   ├── editor_tab.rs        # EditorTab model
│   │   ├── outline_item.rs      # OutlineItem model
│   │   ├── version.rs           # VersionHistory, VersionCommit
│   │   └── spell.rs             # WordLocation, MisspelledWord
│   └── keys/
│       ├── mod.rs
│       └── commands.rs          # WordStar-style keybinding map and dispatch
├── resources/
│   └── english-words.txt        # Spell check dictionary
└── tests/
    ├── services/
    ├── editor/
    └── integration/
```

### 2.2 Application Loop

The application follows ratatui's standard immediate-mode rendering loop:

```
1. Initialize terminal (alternate screen, raw mode)
2. Loop:
   a. Render UI (help menu, ruler, status, editor, panes, any open dialog)
   b. Poll for input events (crossterm)
   c. Dispatch input through command prefix system
   d. Update application state
   e. If state changed, trigger re-render
3. Restore terminal on exit
```

### 2.3 State Machine

The application has these top-level modes:

- **Editing** - Normal text editing (default)
- **CommandPrefix** - Waiting for second key after ^K, ^Q, or ^O prefix
- **Dialog** - A modal dialog is open (file path, confirm, spell check, etc.)
- **OutlineNav** - Navigating the outline pane
- **ProjectNav** - Navigating the project files pane

## 3. Data Models

All models are identical to the original spec. They are Rust structs with `serde::Serialize` / `serde::Deserialize` derives where persistence is needed.

### 3.1 Document
```rust
struct Document {
    file_path: Option<PathBuf>,
}
// file_name() -> computed from file_path
```

### 3.2 Project
```rust
#[derive(Serialize, Deserialize)]
struct Project {
    version: u32,           // default: 1
    file_order: Vec<String>, // relative paths
    #[serde(skip)]
    directory_path: PathBuf,
}
```

### 3.3 ProjectFile
```rust
struct ProjectFile {
    relative_path: String,
    // absolute_path() and file_name() are computed
}
```

### 3.4 EditorTab
```rust
struct EditorTab {
    project_file: ProjectFile,
    content: String,
    caret_offset: usize,
    scroll_offset: usize,
    is_dirty: bool,
    // display_name() -> filename + "*" if dirty
}
```

### 3.5 OutlineItem
```rust
struct OutlineItem {
    title: String,
    level: u8,             // 1-3
    line_number: usize,    // 0-based
    source_file_path: Option<String>,
}
```

### 3.6 Version Control Models
```rust
#[derive(Serialize, Deserialize)]
struct VersionHistory {
    version: u32,
    commits: Vec<VersionCommit>,
}

#[derive(Serialize, Deserialize)]
struct VersionCommit {
    id: String,             // yyyyMMdd-HHmmss-<4hex>
    timestamp: String,      // ISO 8601 UTC
    message: String,
    file_size_bytes: u64,
}
```

### 3.7 Spell Check Models
```rust
struct WordLocation {
    offset: usize,
    length: usize,
}

struct MisspelledWord {
    word: String,
    locations: Vec<WordLocation>,
    suggestions: Vec<String>,
}
```

## 4. WordStar-Style User Interface

### 4.1 Screen Layout

The terminal screen is divided into fixed zones from top to bottom:

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                        HELP MENU AREA (4-6 lines)                          │
│  Shows context-sensitive command reference, toggleable with ^J             │
├──────────────────────────────────────────────────────────────────────────────┤
│ L1 C1  Insert  Wrap  FILE.MD *  [Project: MyBook]          VibeWrite95     │ ← Status Line
├──────────────────────────────────────────────────────────────────────────────┤
│ |----+----|----+----|----+----|----+----|----+----|----+----|----+----|----+  │ ← Ruler Line
├──────────┬───────────────────────────────────────────────────┬──────────────┤
│ OUTLINE  │               EDITING AREA                       │ PROJECT      │
│          │                                                   │ FILES        │
│ # Intro  │  The quick brown fox jumps over the lazy dog.    │              │
│ # Ch 1   │  **Bold** and *italic* text with headings.       │ intro.md     │
│ ## 1.1   │                                                   │ chapter1.md* │
│ ## 1.2   │  ## Section Heading                               │ chapter2.md  │
│ # Ch 2   │                                                   │              │
│          │  More body text here...                           │ [A]dd        │
│          │                                                   │ [R]emove     │
│          │                                                   │ [U]p / [D]own│
│          │                                                   │              │
├──────────┴───────────────────────────────────────────────────┴──────────────┤
│ Tab: intro.md | chapter1.md* | chapter2.md                                 │ ← Tab Bar (project mode)
└──────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 Help Menu Area

The help menu occupies the top 4-6 lines and displays context-sensitive command shortcuts, styled like WordStar's classic help display. Toggle with **^J** (Ctrl+J).

**Main Editing Help Menu:**
```
 ^N New   ^O Open   ^S Save   ^A Save As  │  ^B Bold   ^I Italic   ^U Underline
 ^K Block/File      ^Q Quick/Search        │  ^P Print  F7 Spell    ^W Close
─── ^K Block/File: S Save  Q Quit  P Project  N New Project  O Open Project ───
─── ^Q Quick:      F Find   R Replace   H History   V Revert   C Commit     ───
```

When a prefix key is pressed (e.g., ^K), the help menu updates to show only the commands available under that prefix, with the prefix highlighted.

**^K Prefix Menu (Block/File):**
```
 ^K  Block & File Commands:
 B Begin block   K End block   V Move block   C Copy block   Y Delete block
 S Save file     D Save & Done   Q Quit (abandon)   N New project
 O Open project  P Print to PDF   X Export PDF
```

**^Q Prefix Menu (Quick/Search):**
```
 ^Q  Quick Commands:
 F Find text     A Find & Replace    R Repeat last find
 C Commit        H View history      V Revert to version
```

### 4.3 Status Line

Single line showing editor state, styled with reverse video (inverted colors):

```
 L:42 C:15  Insert  Wrap  chapter1.md *  [Project: MyBook]          VibeWrite95
```

Fields:
- **L:nn C:nn** - Current line and column (1-based)
- **Insert/Overwrite** - Input mode
- **Wrap** - Word wrap indicator
- **filename** - Current file name
- **\*** - Dirty indicator (unsaved changes)
- **[Project: name]** - Project name (project mode only)
- **VibeWrite95** - Application name (right-aligned)

### 4.4 Ruler Line

Shows column positions with markers every 10 columns:

```
|----+----|----+----|----+----|----+----|----+----|----+----|----+----|----+----|
1   5   10   15   20   25   30   35   40   45   50   55   60   65   70   75  80
```

### 4.5 Color Scheme

Emulating the classic DOS word processor aesthetic:

| Element | Foreground | Background |
|---------|-----------|------------|
| Help menu area | White / Cyan | Blue |
| Help menu keys | Yellow (bright) | Blue |
| Status line | Black | White (reverse video) |
| Ruler line | DarkGray | Black |
| Editor text | White | Black |
| Editor - headings | Cyan, Bold | Black |
| Editor - bold text | White, Bold | Black |
| Editor - italic text | Green | Black |
| Editor - underline | Yellow, Underline | Black |
| Editor - markdown syntax chars | DarkGray | Black |
| Outline pane | Cyan | DarkBlue |
| Outline heading | White, Bold | DarkBlue |
| Project pane | Cyan | DarkBlue |
| Project active file | Black | Cyan (highlight) |
| Tab bar | White | DarkGray |
| Tab active | Black | White |
| Dialog box | White | DarkBlue |
| Dialog border | Cyan | DarkBlue |
| Dialog highlight | Black | Cyan |

## 5. Keyboard Command System

### 5.1 Direct Commands (Single Key)

| Key | Action |
|-----|--------|
| ^N | New document |
| ^O | Open file (prompts for path) |
| ^S | Save file |
| ^A | Save As (prompts for path) |
| ^B | Toggle bold on selection/word |
| ^I | Toggle italic on selection/word |
| ^U | Toggle underline on selection/word |
| ^W | Close current tab (project mode) |
| ^P | Print (CUPS) |
| ^J | Toggle help menu visibility |
| ^T | Cycle heading level (Normal → H1 → H2 → H3 → Normal) |
| ^L | Toggle outline pane |
| ^R | Toggle project files pane |
| ^G | Delete character at cursor |
| ^Y | Delete current line |
| ^E | Cursor up |
| ^X | Cursor down |
| ^D | Cursor right |
| ^F | Cursor left (WordStar diamond) |
| F7 | Spell check |
| Escape | Cancel prefix / close dialog |

### 5.2 ^K Prefix Commands (Block & File)

Press ^K, then the second key:

| Second Key | Action |
|------------|--------|
| B | Mark block begin |
| K | Mark block end |
| V | Move block |
| C | Copy block |
| Y | Delete block |
| H | Hide/show block |
| S | Save file |
| D | Save and close file |
| Q | Quit without saving |
| N | New project |
| O | Open project |
| P | Print to printer |
| X | Export to PDF |
| 1-3 | Switch to tab 1-9 (project mode) |

### 5.3 ^Q Prefix Commands (Quick/Search)

Press ^Q, then the second key:

| Second Key | Action |
|------------|--------|
| F | Find text (forward) |
| A | Find and replace |
| R | Repeat last find |
| C | Commit current file (project mode) |
| H | View version history |
| V | Revert to version |
| S | Go to start of file |
| D | Go to end of file |
| E | Go to top of screen |
| X | Go to bottom of screen |
| L | Go to line number |

### 5.4 ^O Prefix Commands (Onscreen/View)

Press ^O, then the second key:

| Second Key | Action |
|------------|--------|
| L | Toggle outline pane |
| P | Toggle project pane |
| W | Toggle word wrap |
| N | Toggle line numbers |
| R | Toggle ruler line |
| T | Switch to next tab (project mode) |

### 5.5 Navigation Keys

| Key | Action |
|-----|--------|
| Arrow keys | Cursor movement |
| Home / End | Start/end of line |
| PgUp / PgDn | Page up/down |
| ^E / ^X / ^D / ^S | WordStar diamond (up/down/right/left) |
| ^A | Word left |
| ^F | Word right |

> **Note:** Some direct commands (^A, ^F, ^S) overlap with WordStar navigation. The application resolves this by context: ^S in editing mode saves; the WordStar diamond navigation uses ^E/^X/^D/^S only when the WordStar diamond mode is enabled via config. By default, modern shortcuts take priority, with WordStar diamond available as an option.

## 6. Features

### 6.1 Single Document Editing
- Create, open, and save Markdown files (`.md`, `.markdown`, `.txt`, `*`)
- Markdown syntax highlighting in the terminal (headings, bold, italic, underline, dimmed syntax characters)
- Headings shown with color differentiation and bold (no font size in TUI, so use color + indentation)
- Bold (`**`), Italic (`*`), and Underline (`++`) markup with visual rendering
- Word wrap at terminal width or configurable column (default: 80)
- Insert and overwrite modes (toggle with Insert key)
- 100ms debounce on text changes for re-parsing

### 6.2 Project Mode
- Multi-file writing projects stored as a directory with `project.json`
- File ordering with add/remove/move-up/move-down controls in project pane
- Tabbed editing with per-tab state preservation (content, caret, scroll)
- Project files pane (right side)
- Combined outline across all project files
- Tab bar at bottom of screen showing open files
- Switch tabs with ^K1-^K9 or ^OT (next tab)

### 6.3 Navigation Outline
- Extracts H1-H3 headings from Markdown AST
- Displayed in left pane
- Navigate with arrow keys when pane is focused (^L to toggle/focus)
- Enter key jumps to heading in editor, switching files in project mode
- Real-time updates as document is edited

### 6.4 Version Control
- Available only in project mode
- Snapshot-based (full file copy per commit)
- Storage: `.vibe/<filename>/snapshots/<commitId>.md`
- History: `.vibe/<filename>/history.json`
- Commit: ^QC (prompts for commit message)
- View history: ^QH (scrollable list dialog)
- Revert: ^QV (side-by-side diff dialog with commit selection)

### 6.5 Spell Checking
- Dictionary-based using bundled `english-words.txt`
- User dictionary: `~/.vibe/user-dictionary.txt` (Mac/Linux), `%APPDATA%\Vibe\user-dictionary.txt` (Windows)
- Levenshtein distance suggestions (edit distance 0-2, up to 8 suggestions)
- Markdown-aware: skips code blocks, code inlines, link URLs, HTML
- Minimum word length: 2 characters
- Supports contractions
- Interactive TUI dialog:
  - Left panel: scrollable list of misspelled words with occurrence count
  - Right panel: word detail, suggestions list, action keys
  - Actions: (R)eplace, (A)ll replace, (I)gnore, (G)ignore all, (D)add to dictionary
  - Navigate between words with arrow keys / Tab

### 6.6 Print & PDF Export
- PDF generation via `printpdf` crate
- Letter size (8.5" x 11"), 1" margins
- Markdown-aware rendering (headings, emphasis)
- Heading sizes: H1 = 24pt bold, H2 = 18pt bold, H3 = 14pt bold
- Body text: 12pt
- Physical printing via CUPS (`lp` command) on macOS/Linux
- ^KP for print, ^KX for export to PDF

### 6.7 Block Operations (WordStar-style)
- Mark block begin: ^KB
- Mark block end: ^KK
- Block is visually highlighted (reverse video or distinct background)
- Move block: ^KV
- Copy block: ^KC
- Delete block: ^KY
- Hide/show block markers: ^KH

## 7. Dialogs

All dialogs are modal overlays rendered as bordered boxes centered on screen with a WordStar-aesthetic (cyan border on dark blue background).

### 7.1 File Path Input
- Prompt: "Enter file path:"
- Single-line text input with basic editing (backspace, delete, home, end)
- Tab completion for file paths
- Enter to confirm, Escape to cancel

### 7.2 Confirm Dialog
- Question text centered
- Options shown as highlighted keys: `(Y)es  (N)o  (C)ancel`
- Single keypress to select

### 7.3 Text Input Dialog
- Used for: commit message, new file name
- Single-line input with prompt label
- Enter to confirm, Escape to cancel

### 7.4 Spell Check Dialog
```
┌─── Spell Check ──────────────────────────────────────────────────┐
│                                                                   │
│  Misspelled Words:          │  Word: "teh"                       │
│  ─────────────────          │  Occurrences: 3                    │
│  > teh (3)                  │                                     │
│    recieve (1)              │  Suggestions:                      │
│    seperate (2)             │  > the                             │
│    definately (1)           │    tea                              │
│                             │    ten                              │
│                             │                                     │
│                             │  (R)eplace  (A)ll  (I)gnore        │
│                             │  (G)nore All  (D)ictionary         │
│                                                                   │
│  Esc: Close                                                       │
└───────────────────────────────────────────────────────────────────┘
```

### 7.5 Version History Dialog
```
┌─── Version History: chapter1.md ─────────────────────────────────┐
│                                                                   │
│  ID                    Date                Message       Size     │
│  ─────────────────────────────────────────────────────────────    │
│  > 20250220-1430-a1b2  2025-02-20 14:30   Initial draft  2.1 KB │
│    20250221-0915-c3d4  2025-02-21 09:15   Ch1 revision   3.4 KB │
│    20250222-1100-e5f6  2025-02-22 11:00   Final edits    3.8 KB │
│                                                                   │
│  Enter: Select    Esc: Close                                      │
└───────────────────────────────────────────────────────────────────┘
```

### 7.6 Diff/Revert Dialog
```
┌─── Revert: chapter1.md ─────────────────────────────────────────┐
│  Commit: 20250220-1430-a1b2  "Initial draft"                    │
│                                                                   │
│  Current                     │  Snapshot                         │
│  ──────────                  │  ────────                         │
│  The quick brown fox         │  The quick brown fox              │
│  jumps over the lazy dog.    │  jumps over the lazy dog.         │
│ -This line was changed.      │ +This was the original line.      │
│  More text here.             │  More text here.                  │
│                                                                   │
│  (R)evert to this version    PgUp/PgDn: Scroll    Esc: Cancel   │
└───────────────────────────────────────────────────────────────────┘
```

## 8. Markdown Rendering in Terminal

Since TUI cannot use variable font sizes, Markdown is rendered with color and style attributes:

| Markdown Element | TUI Rendering |
|-----------------|---------------|
| `# Heading 1` | Cyan, Bold, ALL CAPS, preceded by blank line |
| `## Heading 2` | Cyan, Bold, preceded by blank line |
| `### Heading 3` | Cyan, preceded by blank line |
| `**bold**` | White, Bold attribute |
| `*italic*` | Green, Italic attribute (if terminal supports) |
| `++underline++` | Yellow, Underline attribute |
| `***bold italic***` | Green, Bold + Italic |
| Syntax characters (`**`, `*`, `++`, `#`) | DarkGray (dimmed, not hidden) |
| Code blocks | DarkGray background, White text |
| Links | Blue, Underline |

## 9. File Formats & Storage

Identical to the original spec:

### 9.1 Supported File Types
- Markdown: `.md`, `.markdown`
- Text: `.txt`
- All files: `*`

### 9.2 Project Structure
```
ProjectRoot/
├── project.json
├── chapter1.md
├── chapter2.md
└── .vibe/
    ├── chapter1.md/
    │   ├── history.json
    │   └── snapshots/
    │       └── 20250220-143025-a1b2.md
    └── chapter2.md/
        ├── history.json
        └── snapshots/
            └── ...
```

### 9.3 project.json Format
```json
{
  "version": 1,
  "fileOrder": [
    "chapter1.md",
    "chapter2.md"
  ]
}
```

### 9.4 history.json Format
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

## 10. PDF Rendering

- Page size: Letter (8.5" x 11")
- Margins: 1 inch all sides
- Heading sizes: H1 = 24pt bold, H2 = 18pt bold, H3 = 14pt bold
- Body text: 12pt
- Supports bold, italic, and underline emphasis
- Underline uses `++text++` (EmphasisExtras / inserted text)

## 11. State Management

- Document dirty flag tracking
- Per-tab state preservation (content, caret, scroll, dirty flag)
- Outline and project pane visibility state
- AST cached with 100ms debounce on text changes
- Project file contents and ASTs cached in memory (HashMap)
- Block markers (begin/end positions) for block operations
- Command prefix state (None, ^K pending, ^Q pending, ^O pending)
- Current dialog state (if any dialog is open)

## 12. Configuration

Optional config file at `~/.vibe/config.toml`:

```toml
[editor]
word_wrap = true
wrap_column = 80
tab_width = 4
show_line_numbers = false
show_ruler = true
insert_mode = true         # true = insert, false = overwrite

[ui]
show_help_menu = true      # toggle with ^J
color_scheme = "classic"   # "classic" (blue/white) or "dark" (black/white)
wordstar_diamond = false   # enable ^E/^X/^D/^S navigation

[spell]
dictionary_path = ""       # custom dictionary path (default: bundled)
user_dictionary_path = ""  # custom user dictionary path
```

## 13. Constraints & Limitations

1. **Printing** - Only CUPS-based printing (macOS/Linux); no native Windows print support
2. **No auto-save** - Manual save required (^S or ^KS)
3. **Outline depth** - Only H1-H3 headings shown
4. **Version control** - Per-file only; no global project-level history
5. **No font size variation** - TUI uses color/style differentiation for headings instead
6. **Terminal dependency** - Requires a modern terminal with ANSI color support
7. **Diff algorithm** - Simple LCS; not a full Git-style diff
8. **Spell check dictionary** - English only
9. **No Focus Mode** - Removed (the TUI is inherently distraction-free)
10. **Block operations** - Limited to contiguous text ranges (no multi-cursor)

## 14. Build & Distribution

```bash
# Development
cargo build

# Release
cargo build --release

# Run
cargo run

# Run with file
cargo run -- path/to/file.md

# Run with project
cargo run -- --project path/to/project/dir

# Install
cargo install --path .
```

### 14.1 CLI Arguments

| Argument | Description |
|----------|-------------|
| `<file>` | Open a file for editing |
| `--project <dir>` | Open a project directory |
| `--config <path>` | Use custom config file |
| `--no-color` | Disable colors (monochrome mode) |
| `--help` | Show usage information |
| `--version` | Show version |

## 15. Testing Strategy

- **Unit tests** for all services (file, project, versioning, spell check, diff, word extractor)
- **Unit tests** for editor buffer operations (insert, delete, block ops, cursor movement)
- **Unit tests** for command dispatch and prefix system
- **Integration tests** for version control workflow (commit, history, revert)
- **Integration tests** for PDF generation (verify output is valid PDF)
- **Snapshot tests** for UI rendering using `ratatui`'s built-in test backend
