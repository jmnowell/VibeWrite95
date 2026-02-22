use crate::models::document::Document;
use crate::ui::dialogs::file_dialog::FileDialogState;
use crate::ui::dialogs::confirm::ConfirmDialogState;
use crate::ui::dialogs::input::InputDialogState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Editing,
    CommandPrefix(PrefixKey),
    Dialog(DialogKind),
    #[allow(dead_code)]
    OutlineNav,
    #[allow(dead_code)]
    ProjectNav,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixKey {
    CtrlK,
    CtrlQ,
    CtrlO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogKind {
    FileOpen,
    FileSaveAs,
    Confirm,
    Input,
    SpellCheck,
    History,
    DiffView,
    FindReplace,
    ExportPdf,
}

pub struct App {
    pub mode: AppMode,
    pub document: Document,
    pub should_quit: bool,
    pub show_help: bool,
    pub insert_mode: bool,
    pub show_outline: bool,
    pub show_project: bool,
    pub show_ruler: bool,
    pub show_line_numbers: bool,
    pub word_wrap: bool,
    pub is_dirty: bool,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub project_name: Option<String>,
    pub status_message: Option<String>,

    // Block markers
    pub block_begin: Option<(usize, usize)>,
    pub block_end: Option<(usize, usize)>,
    pub show_block: bool,

    // Action requests (consumed by main loop)
    pub save_requested: bool,
    pub save_as_requested: bool,
    pub new_file_requested: bool,
    pub open_file_requested: bool,
    pub close_tab_requested: bool,
    pub export_pdf_requested: bool,
    pub spell_check_requested: bool,
    pub bold_requested: bool,
    pub italic_requested: bool,
    pub underline_requested: bool,
    pub heading_cycle_requested: bool,
    pub delete_line_requested: bool,
    pub find_requested: bool,
    pub replace_requested: bool,
    pub commit_requested: bool,
    pub history_requested: bool,
    pub revert_requested: bool,
    pub goto_start: bool,
    pub goto_end: bool,
    pub new_project_requested: bool,
    pub open_project_requested: bool,
    pub block_move_requested: bool,
    pub block_copy_requested: bool,
    pub block_delete_requested: bool,
    pub next_tab_requested: bool,
    pub tab_switch_requested: Option<usize>,

    // Dialog state
    pub dialog_input: String,
    pub dialog_title: String,
    pub dialog_message: String,
    pub confirm_action: Option<ConfirmAction>,
    pub file_dialog_state: Option<FileDialogState>,
    pub confirm_dialog_state: Option<ConfirmDialogState>,
    pub input_dialog_state: Option<InputDialogState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmAction {
    SaveBeforeQuit,
    SaveBeforeNew,
    #[allow(dead_code)]
    SaveBeforeOpen,
    #[allow(dead_code)]
    SaveBeforeClose,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: AppMode::Editing,
            document: Document::new(),
            should_quit: false,
            show_help: true,
            insert_mode: true,
            show_outline: false,
            show_project: false,
            show_ruler: true,
            show_line_numbers: false,
            word_wrap: true,
            is_dirty: false,
            cursor_line: 0,
            cursor_col: 0,
            project_name: None,
            status_message: None,
            block_begin: None,
            block_end: None,
            show_block: true,
            save_requested: false,
            save_as_requested: false,
            new_file_requested: false,
            open_file_requested: false,
            close_tab_requested: false,
            export_pdf_requested: false,
            spell_check_requested: false,
            bold_requested: false,
            italic_requested: false,
            underline_requested: false,
            heading_cycle_requested: false,
            delete_line_requested: false,
            find_requested: false,
            replace_requested: false,
            commit_requested: false,
            history_requested: false,
            revert_requested: false,
            goto_start: false,
            goto_end: false,
            new_project_requested: false,
            open_project_requested: false,
            block_move_requested: false,
            block_copy_requested: false,
            block_delete_requested: false,
            next_tab_requested: false,
            tab_switch_requested: None,
            dialog_input: String::new(),
            dialog_title: String::new(),
            dialog_message: String::new(),
            confirm_action: None,
            file_dialog_state: None,
            confirm_dialog_state: None,
            input_dialog_state: None,
        }
    }

    pub fn clear_actions(&mut self) {
        self.save_requested = false;
        self.save_as_requested = false;
        self.new_file_requested = false;
        self.open_file_requested = false;
        self.close_tab_requested = false;
        self.export_pdf_requested = false;
        self.spell_check_requested = false;
        self.bold_requested = false;
        self.italic_requested = false;
        self.underline_requested = false;
        self.heading_cycle_requested = false;
        self.delete_line_requested = false;
        self.find_requested = false;
        self.replace_requested = false;
        self.commit_requested = false;
        self.history_requested = false;
        self.revert_requested = false;
        self.goto_start = false;
        self.goto_end = false;
        self.new_project_requested = false;
        self.open_project_requested = false;
        self.block_move_requested = false;
        self.block_copy_requested = false;
        self.block_delete_requested = false;
        self.next_tab_requested = false;
        self.tab_switch_requested = None;
    }

    pub fn close_dialog(&mut self) {
        self.mode = AppMode::Editing;
        self.dialog_input.clear();
        self.dialog_title.clear();
        self.dialog_message.clear();
        self.confirm_action = None;
        self.file_dialog_state = None;
        self.confirm_dialog_state = None;
        self.input_dialog_state = None;
    }
}
