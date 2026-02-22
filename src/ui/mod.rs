pub mod layout;
pub mod help_menu;
pub mod status_line;
pub mod ruler;
pub mod editor_view;
pub mod outline_pane;
pub mod project_pane;
pub mod tab_bar;
pub mod dialogs;

use ratatui::Frame;
use crate::app::{App, AppMode, DialogKind};
use crate::editor::buffer::EditorBuffer;
use crate::models::editor_tab::EditorTab;
use crate::ui::outline_pane::OutlinePaneState;
use crate::ui::project_pane::ProjectPaneState;

pub struct UiState {
    pub outline_state: OutlinePaneState,
    pub project_state: ProjectPaneState,
    pub history_state: Option<dialogs::history::HistoryDialogState>,
    pub diff_state: Option<dialogs::diff_view::DiffViewState>,
    pub spell_state: Option<dialogs::spell_check::SpellCheckDialogState>,
    pub find_state: Option<dialogs::find::FindDialogState>,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            outline_state: OutlinePaneState::new(),
            project_state: ProjectPaneState::new(),
            history_state: None,
            diff_state: None,
            spell_state: None,
            find_state: None,
        }
    }
}

pub fn draw(
    f: &mut Frame,
    app: &App,
    buffer: &mut EditorBuffer,
    ui_state: &mut UiState,
    tabs: &[EditorTab],
    active_tab: usize,
) {
    let areas = layout::compute_layout(f.area(), app);

    if app.show_help {
        help_menu::render(f, areas.help, app);
    }

    status_line::render(f, areas.status, app);

    if app.show_ruler {
        ruler::render(f, areas.ruler);
    }

    if let Some(outline_area) = areas.outline {
        let focused = matches!(app.mode, AppMode::OutlineNav);
        outline_pane::render(f, outline_area, &mut ui_state.outline_state, focused);
    }

    editor_view::render(f, areas.editor, app, buffer);

    if let Some(project_area) = areas.project {
        let focused = matches!(app.mode, AppMode::ProjectNav);
        project_pane::render(f, project_area, tabs, active_tab, &mut ui_state.project_state, focused);
    }

    if let Some(tab_area) = areas.tab_bar {
        tab_bar::render(f, tab_area, tabs, active_tab);
    }

    // Render dialogs on top
    if let AppMode::Dialog(kind) = app.mode {
        let full_area = f.area();
        match kind {
            DialogKind::FileOpen | DialogKind::FileSaveAs => {
                if let Some(ref state) = app.file_dialog_state {
                    dialogs::file_dialog::render(f, full_area, state);
                }
            }
            DialogKind::Confirm => {
                if let Some(ref state) = app.confirm_dialog_state {
                    dialogs::confirm::render(f, full_area, state);
                }
            }
            DialogKind::Input => {
                if let Some(ref state) = app.input_dialog_state {
                    dialogs::input::render(f, full_area, state);
                }
            }
            DialogKind::History => {
                if let Some(ref mut state) = ui_state.history_state {
                    dialogs::history::render(f, full_area, state);
                }
            }
            DialogKind::DiffView => {
                if let Some(ref state) = ui_state.diff_state {
                    dialogs::diff_view::render(f, full_area, state);
                }
            }
            DialogKind::SpellCheck => {
                if let Some(ref mut state) = ui_state.spell_state {
                    dialogs::spell_check::render(f, full_area, state);
                }
            }
            DialogKind::FindReplace => {
                if let Some(ref state) = ui_state.find_state {
                    dialogs::find::render(f, full_area, state);
                }
            }
            DialogKind::ExportPdf => {
                if let Some(ref state) = app.file_dialog_state {
                    dialogs::file_dialog::render(f, full_area, state);
                }
            }
        }
    }
}
