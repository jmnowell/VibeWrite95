mod app;
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
use ratatui::prelude::*;
use ui::dialogs::file_dialog::FileDialogState;
use ui::dialogs::confirm::ConfirmDialogState;
use ui::outline_pane::OutlinePaneState;

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

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut app = App::new();

    if let Some(ref path) = cli.file {
        app.document = models::document::Document::from_path(path);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let mut buffer = EditorBuffer::new();
    let mut outline_state = OutlinePaneState::new();

    // Load file if specified
    if let Some(ref path) = app.document.file_path.clone() {
        if let Ok(content) = services::file_service::read_file(path) {
            buffer = EditorBuffer::from_text(&content);
        }
    }

    loop {
        // Sync cursor position from buffer to app
        app.cursor_line = buffer.cursor_line();
        app.cursor_col = buffer.cursor_col();

        // Update outline from current buffer
        if app.show_outline {
            let content = buffer.content();
            let items = services::markdown_parser::extract_outline(&content, None);
            outline_state.update(items);
        }

        terminal.draw(|f| ui::draw(f, app, &mut buffer, &mut outline_state))?;

        if let Event::Key(key) = event::read()? {
            // Handle outline navigation
            if matches!(app.mode, AppMode::OutlineNav) {
                handle_outline_key(app, &mut buffer, &mut outline_state, key.code);
                continue;
            }

            // Handle dialog input
            if let AppMode::Dialog(kind) = app.mode {
                handle_dialog_key(app, &mut buffer, kind, key.code, key.modifiers);
                continue;
            }

            app.clear_actions();
            match dispatch_key(app, key) {
                KeyAction::Quit => {
                    if app.is_dirty {
                        app.confirm_dialog_state = Some(ConfirmDialogState::new(
                            "Quit",
                            "Save changes before quitting?",
                        ));
                        app.confirm_action = Some(ConfirmAction::SaveBeforeQuit);
                        app.mode = AppMode::Dialog(DialogKind::Confirm);
                    } else {
                        app.should_quit = true;
                    }
                }
                KeyAction::ForwardToEditor(input) => {
                    buffer.textarea.input(input);
                    app.is_dirty = true;
                }
                KeyAction::Handled | KeyAction::None => {}
            }

            process_actions(app, &mut buffer)?;
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn process_actions(app: &mut App, buffer: &mut EditorBuffer) -> io::Result<()> {
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
        save_file(app, buffer)?;
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
                "New File",
                "Save changes before creating new file?",
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
    // Block operations
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
    Ok(())
}

fn save_file(app: &mut App, buffer: &EditorBuffer) -> io::Result<()> {
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
    outline_state: &mut OutlinePaneState,
    code: KeyCode,
) {
    match code {
        KeyCode::Esc | KeyCode::Left => {
            app.mode = AppMode::Editing;
        }
        KeyCode::Up => {
            outline_state.previous();
        }
        KeyCode::Down => {
            outline_state.next();
        }
        KeyCode::Enter => {
            if let Some(item) = outline_state.selected_item() {
                let target_line = item.line_number;
                // Navigate to the heading line
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

fn handle_dialog_key(
    app: &mut App,
    buffer: &mut EditorBuffer,
    kind: DialogKind,
    code: KeyCode,
    _modifiers: KeyModifiers,
) {
    match kind {
        DialogKind::FileOpen | DialogKind::FileSaveAs => {
            if let Some(ref mut state) = app.file_dialog_state {
                match code {
                    KeyCode::Esc => {
                        app.close_dialog();
                    }
                    KeyCode::Enter => {
                        let path = PathBuf::from(&state.input);
                        if kind == DialogKind::FileOpen {
                            if let Ok(content) = services::file_service::read_file(&path) {
                                *buffer = EditorBuffer::from_text(&content);
                                app.document = models::document::Document::from_path(&path);
                                app.is_dirty = false;
                            }
                        } else {
                            let content = buffer.content();
                            let _ = services::file_service::write_file(&path, &content);
                            app.document = models::document::Document::from_path(&path);
                            app.is_dirty = false;
                        }
                        app.close_dialog();
                    }
                    KeyCode::Tab => {
                        state.tab_complete();
                    }
                    KeyCode::Backspace => {
                        state.backspace();
                    }
                    KeyCode::Left => {
                        state.move_left();
                    }
                    KeyCode::Right => {
                        state.move_right();
                    }
                    KeyCode::Char(c) => {
                        state.insert_char(c);
                    }
                    _ => {}
                }
            }
        }
        DialogKind::Confirm => {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    let action = app.confirm_action;
                    let _ = save_file(app, buffer);
                    match action {
                        Some(ConfirmAction::SaveBeforeQuit) => {
                            app.should_quit = true;
                        }
                        Some(ConfirmAction::SaveBeforeNew) => {
                            new_file(app, buffer);
                        }
                        _ => {}
                    }
                    app.close_dialog();
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    let action = app.confirm_action;
                    match action {
                        Some(ConfirmAction::SaveBeforeQuit) => {
                            app.should_quit = true;
                        }
                        Some(ConfirmAction::SaveBeforeNew) => {
                            new_file(app, buffer);
                        }
                        _ => {}
                    }
                    app.close_dialog();
                }
                KeyCode::Char('c') | KeyCode::Char('C') | KeyCode::Esc => {
                    app.close_dialog();
                }
                _ => {}
            }
        }
        DialogKind::Input => {
            if let Some(ref mut state) = app.input_dialog_state {
                match code {
                    KeyCode::Esc => {
                        app.close_dialog();
                    }
                    KeyCode::Enter => {
                        app.dialog_input = state.input.clone();
                        app.close_dialog();
                    }
                    KeyCode::Backspace => {
                        state.backspace();
                    }
                    KeyCode::Char(c) => {
                        state.insert_char(c);
                    }
                    _ => {}
                }
            }
        }
        _ => {
            if code == KeyCode::Esc {
                app.close_dialog();
            }
        }
    }
}
