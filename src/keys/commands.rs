use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{Input, Key};
use crate::app::{App, AppMode, PrefixKey};

pub enum KeyAction {
    Quit,
    ForwardToEditor(Input),
    Handled,
    None,
}

pub fn dispatch_key(app: &mut App, key: KeyEvent) -> KeyAction {
    match app.mode {
        AppMode::CommandPrefix(prefix) => dispatch_prefix(app, prefix, key),
        AppMode::Editing => dispatch_editing(app, key),
        AppMode::Dialog(_) => dispatch_dialog(app, key),
        AppMode::OutlineNav => dispatch_outline(app, key),
        AppMode::ProjectNav => dispatch_project(app, key),
    }
}

fn dispatch_prefix(app: &mut App, prefix: PrefixKey, key: KeyEvent) -> KeyAction {
    app.mode = AppMode::Editing;

    if key.code == KeyCode::Esc {
        return KeyAction::Handled;
    }

    match prefix {
        PrefixKey::CtrlK => dispatch_ctrl_k(app, key),
        PrefixKey::CtrlQ => dispatch_ctrl_q(app, key),
        PrefixKey::CtrlO => dispatch_ctrl_o(app, key),
    }
}

fn dispatch_ctrl_k(app: &mut App, key: KeyEvent) -> KeyAction {
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => KeyAction::Quit,
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.save_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('b') | KeyCode::Char('B') => {
            app.block_begin = Some((app.cursor_line, app.cursor_col));
            KeyAction::Handled
        }
        KeyCode::Char('k') | KeyCode::Char('K') => {
            app.block_end = Some((app.cursor_line, app.cursor_col));
            KeyAction::Handled
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.show_block = !app.show_block;
            KeyAction::Handled
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.new_project_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('o') | KeyCode::Char('O') => {
            app.open_project_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.print_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('x') | KeyCode::Char('X') => {
            app.export_pdf_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('v') | KeyCode::Char('V') => {
            app.block_move_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.block_copy_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.block_delete_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            app.tab_switch_requested = Some(c.to_digit(10).unwrap() as usize - 1);
            KeyAction::Handled
        }
        _ => KeyAction::Handled,
    }
}

fn dispatch_ctrl_q(app: &mut App, key: KeyEvent) -> KeyAction {
    match key.code {
        KeyCode::Char('f') | KeyCode::Char('F') => {
            app.find_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('a') | KeyCode::Char('A') => {
            app.replace_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            app.commit_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            app.history_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('v') | KeyCode::Char('V') => {
            app.revert_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.goto_start = true;
            KeyAction::Handled
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            app.goto_end = true;
            KeyAction::Handled
        }
        _ => KeyAction::Handled,
    }
}

fn dispatch_ctrl_o(app: &mut App, key: KeyEvent) -> KeyAction {
    match key.code {
        KeyCode::Char('l') | KeyCode::Char('L') => {
            app.show_outline = !app.show_outline;
            KeyAction::Handled
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.show_project = !app.show_project;
            KeyAction::Handled
        }
        KeyCode::Char('w') | KeyCode::Char('W') => {
            app.word_wrap = !app.word_wrap;
            KeyAction::Handled
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.show_ruler = !app.show_ruler;
            KeyAction::Handled
        }
        KeyCode::Char('t') | KeyCode::Char('T') => {
            app.next_tab_requested = true;
            KeyAction::Handled
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            app.show_line_numbers = !app.show_line_numbers;
            KeyAction::Handled
        }
        _ => KeyAction::Handled,
    }
}

fn dispatch_editing(app: &mut App, key: KeyEvent) -> KeyAction {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') => return KeyAction::Quit,
            KeyCode::Char('k') => {
                app.mode = AppMode::CommandPrefix(PrefixKey::CtrlK);
                return KeyAction::Handled;
            }
            KeyCode::Char('q') => {
                app.mode = AppMode::CommandPrefix(PrefixKey::CtrlQ);
                return KeyAction::Handled;
            }
            KeyCode::Char('o') => {
                app.mode = AppMode::CommandPrefix(PrefixKey::CtrlO);
                return KeyAction::Handled;
            }
            KeyCode::Char('j') => {
                app.show_help = !app.show_help;
                return KeyAction::Handled;
            }
            KeyCode::Char('s') => {
                app.save_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('n') => {
                app.new_file_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('l') => {
                app.show_outline = !app.show_outline;
                return KeyAction::Handled;
            }
            KeyCode::Char('r') => {
                app.show_project = !app.show_project;
                return KeyAction::Handled;
            }
            KeyCode::Char('b') => {
                app.bold_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('i') => {
                app.italic_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('u') => {
                app.underline_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('t') => {
                app.heading_cycle_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('w') => {
                app.close_tab_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('p') => {
                app.print_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('a') => {
                app.save_as_requested = true;
                return KeyAction::Handled;
            }
            KeyCode::Char('g') => {
                // Delete char at cursor - forward to editor
                return KeyAction::ForwardToEditor(Input { key: Key::Delete, ctrl: false, alt: false, shift: false });
            }
            KeyCode::Char('y') => {
                app.delete_line_requested = true;
                return KeyAction::Handled;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::F(7) => {
            app.spell_check_requested = true;
            KeyAction::Handled
        }
        KeyCode::Insert => {
            app.insert_mode = !app.insert_mode;
            KeyAction::Handled
        }
        _ => {
            KeyAction::ForwardToEditor(key.into())
        }
    }
}

fn dispatch_dialog(app: &mut App, key: KeyEvent) -> KeyAction {
    if key.code == KeyCode::Esc {
        app.mode = AppMode::Editing;
        app.close_dialog();
    }
    KeyAction::Handled
}

fn dispatch_outline(app: &mut App, key: KeyEvent) -> KeyAction {
    if key.code == KeyCode::Esc {
        app.mode = AppMode::Editing;
    }
    KeyAction::Handled
}

fn dispatch_project(app: &mut App, key: KeyEvent) -> KeyAction {
    if key.code == KeyCode::Esc {
        app.mode = AppMode::Editing;
    }
    KeyAction::Handled
}
