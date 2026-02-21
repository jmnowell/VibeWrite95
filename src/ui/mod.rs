pub mod layout;
pub mod help_menu;
pub mod status_line;
pub mod ruler;
pub mod editor_view;
pub mod outline_pane;
pub mod dialogs;

use ratatui::Frame;
use crate::app::{App, AppMode, DialogKind};
use crate::editor::buffer::EditorBuffer;
use crate::ui::outline_pane::OutlinePaneState;

pub fn draw(
    f: &mut Frame,
    app: &App,
    buffer: &mut EditorBuffer,
    outline_state: &mut OutlinePaneState,
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
        outline_pane::render(f, outline_area, outline_state, focused);
    }

    editor_view::render(f, areas.editor, app, buffer);

    // Render dialogs on top
    if let AppMode::Dialog(kind) = app.mode {
        match kind {
            DialogKind::FileOpen | DialogKind::FileSaveAs => {
                if let Some(ref state) = app.file_dialog_state {
                    dialogs::file_dialog::render(f, f.area(), state);
                }
            }
            DialogKind::Confirm => {
                if let Some(ref state) = app.confirm_dialog_state {
                    dialogs::confirm::render(f, f.area(), state);
                }
            }
            DialogKind::Input => {
                if let Some(ref state) = app.input_dialog_state {
                    dialogs::input::render(f, f.area(), state);
                }
            }
            _ => {}
        }
    }
}
