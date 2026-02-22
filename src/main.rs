mod app;
mod config;
mod editor;
mod keys;
mod models;
mod services;
mod ui;

use std::io;
use std::path::PathBuf;

use app::{App, AppMode, DialogKind, ConfirmAction};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use editor::buffer::EditorBuffer;
use keys::commands::{dispatch_key, KeyAction};
use models::editor_tab::EditorTab;
use models::project::Project;
use models::project_file::ProjectFile;
use ratatui::prelude::*;
use services::spell_check::SpellChecker;
use ui::UiState;
use ui::dialogs::file_dialog::FileDialogState;
use ui::dialogs::confirm::ConfirmDialogState;
use ui::dialogs::input::InputDialogState;
use ui::dialogs::history::HistoryDialogState;
use ui::dialogs::diff_view::DiffViewState;
use ui::dialogs::spell_check::SpellCheckDialogState;
use ui::dialogs::find::FindDialogState;

#[derive(Parser)]
#[command(name = "vibewrite95", version, about = "A WordStar-inspired terminal markdown editor")]
struct Cli {
    /// File to open for editing
    file: Option<String>,

    /// Open a project directory
    #[arg(long)]
    project: Option<String>,

    /// Use custom config file
    #[arg(long)]
    config: Option<String>,

    /// Disable colors (monochrome mode)
    #[arg(long)]
    no_color: bool,
}

struct AppState {
    app: App,
    buffer: EditorBuffer<'static>,
    ui_state: UiState,
    // Project state
    project: Option<Project>,
    tabs: Vec<EditorTab>,
    active_tab: usize,
    spell_checker: SpellChecker,
    // Track pending action for input dialog
    pending_input_action: PendingInputAction,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PendingInputAction {
    None,
    CommitVersion,
    NewProject,
    AddProjectFile,
}

impl AppState {
    fn new() -> Self {
        Self {
            app: App::new(),
            buffer: EditorBuffer::new(),
            ui_state: UiState::new(),
            project: None,
            tabs: Vec::new(),
            active_tab: 0,
            spell_checker: SpellChecker::new(),
            pending_input_action: PendingInputAction::None,
        }
    }

    #[allow(dead_code)]
    fn current_tab(&self) -> Option<&EditorTab> {
        self.tabs.get(self.active_tab)
    }

    fn current_tab_mut(&mut self) -> Option<&mut EditorTab> {
        self.tabs.get_mut(self.active_tab)
    }

    /// Sync buffer content back to current tab
    fn flush_buffer_to_tab(&mut self) {
        if !self.tabs.is_empty() {
            let content = self.buffer.content();
            let dirty = self.app.is_dirty;
            if let Some(tab) = self.current_tab_mut() {
                tab.content = content;
                tab.is_dirty = dirty;
            }
        }
    }

    /// Load the active tab into the buffer
    fn load_tab_to_buffer(&mut self) {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            let content = tab.content.clone();
            let is_dirty = tab.is_dirty;
            let abs_path = tab.project_file.absolute_path();
            self.buffer = EditorBuffer::from_text(&content);
            self.app.is_dirty = is_dirty;
            self.app.document = models::document::Document::from_path(abs_path);
        }
    }

    fn switch_tab(&mut self, index: usize) {
        if index < self.tabs.len() && index != self.active_tab {
            self.flush_buffer_to_tab();
            self.active_tab = index;
            self.load_tab_to_buffer();
            self.ui_state.project_state.select(index);
        }
    }

    fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            let next = (self.active_tab + 1) % self.tabs.len();
            self.switch_tab(next);
        }
    }

    fn open_project(&mut self, dir: PathBuf) -> io::Result<()> {
        let project = services::project_service::load_project(&dir)?;
        self.app.project_name = Some(project.name());
        self.app.show_project = true;
        let tabs = services::project_service::load_tabs(&project);
        // Open first tab if available
        if !tabs.is_empty() {
            self.active_tab = 0;
            let content = tabs[0].content.clone();
            self.buffer = EditorBuffer::from_text(&content);
            self.app.document = models::document::Document::from_path(
                tabs[0].project_file.absolute_path()
            );
        }
        self.project = Some(project);
        self.tabs = tabs;
        Ok(())
    }

    fn create_project(&mut self, dir: PathBuf) -> io::Result<()> {
        let project = services::project_service::create_project(dir)?;
        self.app.project_name = Some(project.name());
        self.app.show_project = true;
        self.project = Some(project);
        self.tabs = Vec::new();
        self.buffer = EditorBuffer::new();
        Ok(())
    }

    fn add_file_to_project(&mut self, relative_path: &str) -> io::Result<()> {
        if let Some(ref mut project) = self.project {
            services::project_service::add_file(project, relative_path)?;
            let pf = ProjectFile::new(relative_path, &project.directory_path);
            let abs_path = pf.absolute_path();
            let content = std::fs::read_to_string(&abs_path).unwrap_or_default();
            self.tabs.push(EditorTab::new(pf, content));
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut state = AppState::new();

    // Load config
    let cfg = config::Config::load(cli.config.as_deref().map(std::path::Path::new));
    state.app.show_help = cfg.ui.show_help_menu;
    state.app.show_ruler = cfg.editor.show_ruler;
    state.app.word_wrap = cfg.editor.word_wrap;
    state.app.show_line_numbers = cfg.editor.show_line_numbers;
    state.app.insert_mode = cfg.editor.insert_mode;

    if let Some(ref path) = cli.project {
        let _ = state.open_project(PathBuf::from(path));
    } else if let Some(ref path) = cli.file {
        state.app.document = models::document::Document::from_path(path);
        if let Ok(content) = services::file_service::read_file(PathBuf::from(path).as_path()) {
            state.buffer = EditorBuffer::from_text(&content);
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, state: &mut AppState) -> io::Result<()> {
    loop {
        // Sync cursor position from buffer
        state.app.cursor_line = state.buffer.cursor_line();
        state.app.cursor_col = state.buffer.cursor_col();

        // Update outline
        if state.app.show_outline {
            let content = state.buffer.content();
            let items = services::markdown_parser::extract_outline(&content, None);
            state.ui_state.outline_state.update(items);
        }

        let tabs_ref: &[EditorTab] = &state.tabs;
        let active = state.active_tab;

        terminal.draw(|f| {
            ui::draw(f, &state.app, &mut state.buffer, &mut state.ui_state, tabs_ref, active);
        })?;

        if let Event::Key(key) = event::read()? {
            if matches!(state.app.mode, AppMode::OutlineNav) {
                handle_outline_key(&mut state.app, &mut state.buffer, &mut state.ui_state, key.code);
                continue;
            }

            if matches!(state.app.mode, AppMode::ProjectNav) {
                handle_project_nav_key(state, key.code);
                continue;
            }

            if let AppMode::Dialog(kind) = state.app.mode {
                handle_dialog_key(state, kind, key.code, key.modifiers);
                continue;
            }

            state.app.clear_actions();
            match dispatch_key(&mut state.app, key) {
                KeyAction::Quit => {
                    if state.app.is_dirty {
                        state.app.confirm_dialog_state = Some(ConfirmDialogState::new(
                            "Quit", "Save changes before quitting?",
                        ));
                        state.app.confirm_action = Some(ConfirmAction::SaveBeforeQuit);
                        state.app.mode = AppMode::Dialog(DialogKind::Confirm);
                    } else {
                        state.app.should_quit = true;
                    }
                }
                KeyAction::ForwardToEditor(input) => {
                    state.buffer.textarea.input(input);
                    state.app.is_dirty = true;
                }
                KeyAction::Handled => {}
            }

            process_actions(state)?;
        }

        if state.app.should_quit {
            return Ok(());
        }
    }
}

fn process_actions(state: &mut AppState) -> io::Result<()> {
    let app = &mut state.app;
    let buffer = &mut state.buffer;

    if app.goto_start {
        buffer.textarea.move_cursor(tui_textarea::CursorMove::Top);
        buffer.textarea.move_cursor(tui_textarea::CursorMove::Head);
    }
    if app.goto_end {
        buffer.textarea.move_cursor(tui_textarea::CursorMove::Bottom);
        buffer.textarea.move_cursor(tui_textarea::CursorMove::End);
    }
    if app.delete_line_requested {
        buffer.textarea.move_cursor(tui_textarea::CursorMove::Head);
        buffer.textarea.delete_line_by_end();
        buffer.textarea.delete_next_char();
        app.is_dirty = true;
    }
    if app.save_requested {
        save_current_file(app, buffer)?;
    }
    if app.save_as_requested {
        let initial = app.document.file_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        app.file_dialog_state = Some(FileDialogState::with_initial("Save As", &initial));
        app.mode = AppMode::Dialog(DialogKind::FileSaveAs);
    }
    if app.open_file_requested {
        app.file_dialog_state = Some(FileDialogState::new("Open File"));
        app.mode = AppMode::Dialog(DialogKind::FileOpen);
    }
    if app.new_file_requested {
        if app.is_dirty {
            app.confirm_dialog_state = Some(ConfirmDialogState::new(
                "New File", "Save changes before creating new file?",
            ));
            app.confirm_action = Some(ConfirmAction::SaveBeforeNew);
            app.mode = AppMode::Dialog(DialogKind::Confirm);
        } else {
            new_file(app, buffer);
        }
    }
    if app.bold_requested {
        editor::formatting::toggle_bold(&mut buffer.textarea);
        app.is_dirty = true;
    }
    if app.italic_requested {
        editor::formatting::toggle_italic(&mut buffer.textarea);
        app.is_dirty = true;
    }
    if app.underline_requested {
        editor::formatting::toggle_underline(&mut buffer.textarea);
        app.is_dirty = true;
    }
    if app.heading_cycle_requested {
        editor::formatting::cycle_heading(&mut buffer.textarea);
        app.is_dirty = true;
    }
    if app.block_delete_requested {
        if let (Some(begin), Some(end)) = (app.block_begin, app.block_end) {
            editor::block::delete_block(&mut buffer.textarea, begin, end);
            app.block_begin = None;
            app.block_end = None;
            app.is_dirty = true;
        }
    }
    if app.block_copy_requested {
        if let (Some(begin), Some(end)) = (app.block_begin, app.block_end) {
            editor::block::copy_block(&mut buffer.textarea, begin, end);
            app.is_dirty = true;
        }
    }
    if app.block_move_requested {
        if let (Some(begin), Some(end)) = (app.block_begin, app.block_end) {
            editor::block::move_block(&mut buffer.textarea, begin, end);
            app.block_begin = None;
            app.block_end = None;
            app.is_dirty = true;
        }
    }

    let next_tab = app.next_tab_requested;
    let tab_switch = app.tab_switch_requested;
    let new_proj = app.new_project_requested;
    let open_proj = app.open_project_requested;
    let commit_req = app.commit_requested;
    let history_req = app.history_requested;
    let revert_req = app.revert_requested;
    let spell_req = app.spell_check_requested;
    let find_req = app.find_requested;
    let replace_req = app.replace_requested;
    let export_req = app.export_pdf_requested;

    if next_tab {
        state.next_tab();
    }
    if let Some(idx) = tab_switch {
        state.switch_tab(idx);
    }
    if new_proj {
        state.app.input_dialog_state = Some(InputDialogState::new("New Project", "Enter project directory path:"));
        state.app.mode = AppMode::Dialog(DialogKind::Input);
        state.pending_input_action = PendingInputAction::NewProject;
    }
    if open_proj {
        state.app.file_dialog_state = Some(FileDialogState::new("Open Project"));
        state.app.mode = AppMode::Dialog(DialogKind::FileOpen);
        state.app.open_project_requested = true;
    }
    if commit_req {
        if state.project.is_some() {
            state.app.input_dialog_state = Some(InputDialogState::new("Commit", "Enter commit message:"));
            state.app.mode = AppMode::Dialog(DialogKind::Input);
            state.pending_input_action = PendingInputAction::CommitVersion;
        }
    }
    if history_req {
        if let Some(ref project) = state.project {
            if let Some(ref tab) = state.tabs.get(state.active_tab) {
                let filename = tab.project_file.relative_path.clone();
                let history = services::versioning::load_history(&project.directory_path, &filename);
                state.ui_state.history_state = Some(HistoryDialogState::new(&filename, history.commits));
                state.app.mode = AppMode::Dialog(DialogKind::History);
            }
        }
    }
    if revert_req {
        if let Some(ref project) = state.project {
            if let Some(ref tab) = state.tabs.get(state.active_tab) {
                let filename = tab.project_file.relative_path.clone();
                let history = services::versioning::load_history(&project.directory_path, &filename);
                state.ui_state.history_state = Some(HistoryDialogState::new(&filename, history.commits));
                state.app.mode = AppMode::Dialog(DialogKind::History);
                // Will open DiffView after commit selected
            }
        }
    }
    if spell_req {
        let content = state.buffer.content();
        let words = state.spell_checker.check_text(&content);
        state.ui_state.spell_state = Some(SpellCheckDialogState::new(words));
        state.app.mode = AppMode::Dialog(DialogKind::SpellCheck);
    }
    if find_req {
        state.ui_state.find_state = Some(FindDialogState::new_find());
        state.app.mode = AppMode::Dialog(DialogKind::FindReplace);
    }
    if replace_req {
        state.ui_state.find_state = Some(FindDialogState::new_replace());
        state.app.mode = AppMode::Dialog(DialogKind::FindReplace);
    }
    if export_req {
        let default_name = state.app.document.file_path
            .as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("export");
        let default_path = std::env::current_dir()
            .unwrap_or_default()
            .join(format!("{}.pdf", default_name))
            .to_string_lossy()
            .to_string();
        state.app.file_dialog_state = Some(FileDialogState::with_initial("Export PDF", &default_path));
        state.app.mode = AppMode::Dialog(DialogKind::ExportPdf);
    }

    Ok(())
}

fn save_current_file(app: &mut App, buffer: &EditorBuffer) -> io::Result<()> {
    if let Some(ref path) = app.document.file_path {
        let content = buffer.content();
        services::file_service::write_file(path, &content)?;
        app.is_dirty = false;
        app.status_message = Some(format!("Saved: {}", app.document.file_name()));
    } else {
        app.file_dialog_state = Some(FileDialogState::new("Save As"));
        app.mode = AppMode::Dialog(DialogKind::FileSaveAs);
    }
    Ok(())
}

fn new_file(app: &mut App, buffer: &mut EditorBuffer) {
    *buffer = EditorBuffer::new();
    app.document = models::document::Document::new();
    app.is_dirty = false;
}

fn handle_outline_key(
    app: &mut App,
    buffer: &mut EditorBuffer,
    ui_state: &mut UiState,
    code: KeyCode,
) {
    match code {
        KeyCode::Esc | KeyCode::Left => {
            app.mode = AppMode::Editing;
        }
        KeyCode::Up => {
            ui_state.outline_state.previous();
        }
        KeyCode::Down => {
            ui_state.outline_state.next();
        }
        KeyCode::Enter => {
            if let Some(item) = ui_state.outline_state.selected_item() {
                let target_line = item.line_number;
                buffer.textarea.move_cursor(tui_textarea::CursorMove::Top);
                for _ in 0..target_line {
                    buffer.textarea.move_cursor(tui_textarea::CursorMove::Down);
                }
                buffer.textarea.move_cursor(tui_textarea::CursorMove::Head);
                app.mode = AppMode::Editing;
            }
        }
        _ => {}
    }
}

fn handle_project_nav_key(state: &mut AppState, code: KeyCode) {
    let count = state.tabs.len();
    match code {
        KeyCode::Esc | KeyCode::Right => {
            state.app.mode = AppMode::Editing;
        }
        KeyCode::Up => {
            state.ui_state.project_state.previous(count);
        }
        KeyCode::Down => {
            state.ui_state.project_state.next(count);
        }
        KeyCode::Enter => {
            if let Some(idx) = state.ui_state.project_state.list_state.selected() {
                state.switch_tab(idx);
                state.app.mode = AppMode::Editing;
            }
        }
        KeyCode::Char('u') | KeyCode::Char('U') => {
            if let Some(idx) = state.ui_state.project_state.list_state.selected() {
                if let Some(ref mut project) = state.project {
                    let _ = services::project_service::move_file_up(project, idx);
                    if idx > 0 {
                        state.tabs.swap(idx, idx - 1);
                        if state.active_tab == idx { state.active_tab = idx - 1; }
                        else if state.active_tab == idx - 1 { state.active_tab = idx; }
                        state.ui_state.project_state.select(idx - 1);
                    }
                }
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if let Some(idx) = state.ui_state.project_state.list_state.selected() {
                if let Some(ref mut project) = state.project {
                    let _ = services::project_service::move_file_down(project, idx);
                    if idx + 1 < state.tabs.len() {
                        state.tabs.swap(idx, idx + 1);
                        if state.active_tab == idx { state.active_tab = idx + 1; }
                        else if state.active_tab == idx + 1 { state.active_tab = idx; }
                        state.ui_state.project_state.select(idx + 1);
                    }
                }
            }
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            state.app.input_dialog_state = Some(InputDialogState::new(
                "Add File", "Enter relative file path to add:",
            ));
            state.app.mode = AppMode::Dialog(DialogKind::Input);
            state.pending_input_action = PendingInputAction::AddProjectFile;
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if let Some(idx) = state.ui_state.project_state.list_state.selected() {
                if let Some(ref mut project) = state.project {
                    if idx < state.tabs.len() {
                        let rel = state.tabs[idx].project_file.relative_path.clone();
                        let _ = services::project_service::remove_file(project, &rel);
                        state.tabs.remove(idx);
                        if state.active_tab >= state.tabs.len() && state.active_tab > 0 {
                            state.active_tab -= 1;
                        }
                        state.load_tab_to_buffer();
                    }
                }
            }
        }
        _ => {}
    }
}

fn handle_dialog_key(
    state: &mut AppState,
    kind: DialogKind,
    code: KeyCode,
    _modifiers: KeyModifiers,
) {
    match kind {
        DialogKind::FileOpen | DialogKind::FileSaveAs => {
            let open_proj = state.app.open_project_requested;
            if let Some(ref mut dialog) = state.app.file_dialog_state {
                match code {
                    KeyCode::Esc => {
                        state.app.close_dialog();
                    }
                    KeyCode::Enter => {
                        let path = PathBuf::from(&dialog.input);
                        if kind == DialogKind::FileOpen {
                            if open_proj {
                                let _ = state.open_project(path);
                            } else if let Ok(content) = services::file_service::read_file(&path) {
                                state.buffer = EditorBuffer::from_text(&content);
                                state.app.document = models::document::Document::from_path(&path);
                                state.app.is_dirty = false;
                            }
                        } else {
                            let content = state.buffer.content();
                            let _ = services::file_service::write_file(&path, &content);
                            state.app.document = models::document::Document::from_path(&path);
                            state.app.is_dirty = false;
                        }
                        state.app.close_dialog();
                    }
                    KeyCode::Tab => { dialog.tab_complete(); }
                    KeyCode::Backspace => { dialog.backspace(); }
                    KeyCode::Left => { dialog.move_left(); }
                    KeyCode::Right => { dialog.move_right(); }
                    KeyCode::Char(c) => { dialog.insert_char(c); }
                    _ => {}
                }
            }
        }
        DialogKind::Confirm => {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    let action = state.app.confirm_action;
                    let _ = save_current_file(&mut state.app, &state.buffer);
                    match action {
                        Some(ConfirmAction::SaveBeforeQuit) => state.app.should_quit = true,
                        Some(ConfirmAction::SaveBeforeNew) => new_file(&mut state.app, &mut state.buffer),
                        _ => {}
                    }
                    state.app.close_dialog();
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    let action = state.app.confirm_action;
                    match action {
                        Some(ConfirmAction::SaveBeforeQuit) => state.app.should_quit = true,
                        Some(ConfirmAction::SaveBeforeNew) => new_file(&mut state.app, &mut state.buffer),
                        _ => {}
                    }
                    state.app.close_dialog();
                }
                KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Esc => {
                    state.app.close_dialog();
                }
                _ => {}
            }
        }
        DialogKind::Input => {
            if let Some(ref mut input_state) = state.app.input_dialog_state {
                match code {
                    KeyCode::Esc => {
                        state.app.close_dialog();
                        state.pending_input_action = PendingInputAction::None;
                    }
                    KeyCode::Enter => {
                        let value = input_state.input.clone();
                        state.app.dialog_input = value.clone();
                        let action = state.pending_input_action;
                        state.app.close_dialog();
                        state.pending_input_action = PendingInputAction::None;
                        match action {
                            PendingInputAction::NewProject => {
                                let _ = state.create_project(PathBuf::from(&value));
                            }
                            PendingInputAction::AddProjectFile => {
                                let _ = state.add_file_to_project(&value);
                            }
                            PendingInputAction::CommitVersion => {
                                do_commit(state, &value);
                            }
                            PendingInputAction::None => {}
                        }
                    }
                    KeyCode::Backspace => { input_state.backspace(); }
                    KeyCode::Char(c) => { input_state.insert_char(c); }
                    _ => {}
                }
            }
        }
        DialogKind::History => {
            match code {
                KeyCode::Esc => {
                    state.app.mode = AppMode::Editing;
                    state.ui_state.history_state = None;
                }
                KeyCode::Up => {
                    if let Some(ref mut hs) = state.ui_state.history_state {
                        hs.previous();
                    }
                }
                KeyCode::Down => {
                    if let Some(ref mut hs) = state.ui_state.history_state {
                        hs.next();
                    }
                }
                KeyCode::Enter => {
                    // Open diff view for selected commit
                    if let Some(ref hs) = state.ui_state.history_state {
                        if let Some(commit) = hs.selected_commit() {
                            if let Some(ref project) = state.project {
                                if let Some(ref tab) = state.tabs.get(state.active_tab) {
                                    let filename = tab.project_file.relative_path.clone();
                                    if let Ok(snapshot) = services::versioning::load_snapshot(
                                        &project.directory_path, &filename, &commit.id
                                    ) {
                                        let current = state.buffer.content();
                                        state.ui_state.diff_state = Some(DiffViewState::new(
                                            commit.id.clone(),
                                            commit.message.clone(),
                                            filename,
                                            current,
                                            snapshot,
                                        ));
                                        state.ui_state.history_state = None;
                                        state.app.mode = AppMode::Dialog(DialogKind::DiffView);
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        DialogKind::DiffView => {
            match code {
                KeyCode::Esc => {
                    state.app.mode = AppMode::Editing;
                    state.ui_state.diff_state = None;
                }
                KeyCode::PageDown => {
                    if let Some(ref mut ds) = state.ui_state.diff_state {
                        ds.scroll_down();
                    }
                }
                KeyCode::PageUp => {
                    if let Some(ref mut ds) = state.ui_state.diff_state {
                        ds.scroll_up();
                    }
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    // Revert to snapshot
                    if let Some(ref ds) = state.ui_state.diff_state {
                        let snapshot = ds.snapshot_content.clone();
                        state.buffer = EditorBuffer::from_text(&snapshot);
                        state.app.is_dirty = true;
                    }
                    state.app.mode = AppMode::Editing;
                    state.ui_state.diff_state = None;
                }
                _ => {}
            }
        }
        DialogKind::SpellCheck => {
            match code {
                KeyCode::Esc => {
                    state.app.mode = AppMode::Editing;
                    state.ui_state.spell_state = None;
                }
                KeyCode::Up => {
                    if let Some(ref mut ss) = state.ui_state.spell_state {
                        ss.previous_word();
                    }
                }
                KeyCode::Down => {
                    if let Some(ref mut ss) = state.ui_state.spell_state {
                        ss.next_word();
                    }
                }
                KeyCode::Tab => {
                    if let Some(ref mut ss) = state.ui_state.spell_state {
                        use ui::dialogs::spell_check::SpellFocus;
                        ss.focus = if ss.focus == SpellFocus::Words {
                            SpellFocus::Suggestions
                        } else {
                            SpellFocus::Words
                        };
                    }
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    // Replace selected word with selected suggestion
                    if let Some(ref mut ss) = state.ui_state.spell_state {
                        if let (Some(word), Some(suggestion)) = (
                            ss.selected_word().map(|w| w.word.clone()),
                            ss.selected_suggestion(),
                        ) {
                            let content = state.buffer.content();
                            let new_content = content.replace(&word, &suggestion);
                            state.buffer = EditorBuffer::from_text(&new_content);
                            state.app.is_dirty = true;
                            ss.remove_selected_word();
                        }
                    }
                }
                KeyCode::Char('i') | KeyCode::Char('I') => {
                    // Ignore (skip this word)
                    if let Some(ref mut ss) = state.ui_state.spell_state {
                        ss.next_word();
                    }
                }
                KeyCode::Char('d') | KeyCode::Char('D') => {
                    // Add to user dictionary
                    if let Some(ref mut ss) = state.ui_state.spell_state {
                        if let Some(word) = ss.selected_word().map(|w| w.word.clone()) {
                            let _ = state.spell_checker.add_to_user_dictionary(&word);
                            ss.remove_selected_word();
                        }
                    }
                }
                _ => {}
            }
        }
        DialogKind::ExportPdf => {
            if let Some(ref mut dialog) = state.app.file_dialog_state {
                match code {
                    KeyCode::Esc => {
                        state.app.close_dialog();
                    }
                    KeyCode::Enter => {
                        let path = std::path::PathBuf::from(&dialog.input);
                        let content = state.buffer.content();
                        match services::print_service::export_pdf(&content, &path) {
                            Ok(()) => state.app.status_message = Some(format!("Exported: {}", path.display())),
                            Err(e) => state.app.status_message = Some(format!("Export failed: {}", e)),
                        }
                        state.app.close_dialog();
                    }
                    KeyCode::Tab => { dialog.tab_complete(); }
                    KeyCode::Backspace => { dialog.backspace(); }
                    KeyCode::Left => { dialog.move_left(); }
                    KeyCode::Right => { dialog.move_right(); }
                    KeyCode::Char(c) => { dialog.insert_char(c); }
                    _ => {}
                }
            }
        }
        DialogKind::FindReplace => {
            match code {
                KeyCode::Esc => {
                    state.app.mode = AppMode::Editing;
                    state.ui_state.find_state = None;
                }
                KeyCode::Tab => {
                    if let Some(ref mut fs) = state.ui_state.find_state {
                        use ui::dialogs::find::FindField;
                        fs.active_field = match fs.active_field {
                            FindField::Search => FindField::Replace,
                            FindField::Replace => FindField::Search,
                        };
                    }
                }
                KeyCode::Enter => {
                    let content = state.buffer.content();
                    if let Some(ref mut fs) = state.ui_state.find_state {
                        if let Some(offset) = fs.find_next(&content) {
                            let before = &content[..offset];
                            let target_line = before.lines().count().saturating_sub(1);
                            let target_col = before.lines().last().map_or(0, |l| l.len());
                            state.buffer.textarea.move_cursor(tui_textarea::CursorMove::Top);
                            for _ in 0..target_line {
                                state.buffer.textarea.move_cursor(tui_textarea::CursorMove::Down);
                            }
                            state.buffer.textarea.move_cursor(tui_textarea::CursorMove::Head);
                            for _ in 0..target_col {
                                state.buffer.textarea.move_cursor(tui_textarea::CursorMove::Forward);
                            }
                        }
                    }
                }
                KeyCode::Backspace => {
                    if let Some(ref mut fs) = state.ui_state.find_state {
                        fs.backspace();
                    }
                }
                KeyCode::Char(c) => {
                    if let Some(ref mut fs) = state.ui_state.find_state {
                        if c == 'a' || c == 'A' {
                            // Replace all
                            let content = state.buffer.content();
                            let new_content = fs.replace_all(&content);
                            state.buffer = EditorBuffer::from_text(&new_content);
                            state.app.is_dirty = true;
                        } else {
                            fs.insert_char(c);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn do_commit(state: &mut AppState, message: &str) {
    if let Some(ref project) = state.project {
        if let Some(ref tab) = state.tabs.get(state.active_tab) {
            let filename = tab.project_file.relative_path.clone();
            let content = state.buffer.content();
            let _ = services::versioning::commit(&project.directory_path, &filename, &content, message);
        }
    }
}
