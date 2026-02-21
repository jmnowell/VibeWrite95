use ratatui::prelude::*;
use crate::app::App;
use crate::editor::buffer::EditorBuffer;

pub fn render(f: &mut Frame, area: Rect, _app: &App, buffer: &mut EditorBuffer) {
    f.render_widget(&buffer.textarea, area);
}
