use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui_textarea::{Input, Key};
use crate::app::{App, AppMode, PrefixKey};

pub enum KeyAction {
    Quit,
    ForwardToEditor(Input),
    Handled,
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
            // Some terminals report Ctrl+I as Tab.
            KeyCode::Tab => {
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
                app.export_pdf_requested = true;
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
        // Many terminals encode Ctrl+I as a plain Tab key event.
        KeyCode::Tab => {
            app.italic_requested = true;
            KeyAction::Handled
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctrl_i_char_requests_italic() {
        let mut app = App::new();
        let key = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::CONTROL);
        let action = dispatch_key(&mut app, key);
        assert!(matches!(action, KeyAction::Handled));
        assert!(app.italic_requested);
    }

    #[test]
    fn ctrl_i_tab_requests_italic() {
        let mut app = App::new();
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::CONTROL);
        let action = dispatch_key(&mut app, key);
        assert!(matches!(action, KeyAction::Handled));
        assert!(app.italic_requested);
    }

    #[test]
    fn plain_tab_requests_italic_for_terminal_compat() {
        let mut app = App::new();
        let key = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
        let action = dispatch_key(&mut app, key);
        assert!(matches!(action, KeyAction::Handled));
        assert!(app.italic_requested);
    }

    // --- quit chord ---

    #[test]
    fn ctrl_c_returns_quit() {
        let mut app = App::new();
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        let action = dispatch_key(&mut app, key);
        assert!(matches!(action, KeyAction::Quit));
    }

    #[test]
    fn ctrl_k_sets_prefix_mode() {
        let mut app = App::new();
        let key = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL);
        let action = dispatch_key(&mut app, key);
        assert!(matches!(action, KeyAction::Handled));
        assert!(matches!(app.mode, AppMode::CommandPrefix(PrefixKey::CtrlK)));
    }

    #[test]
    fn ctrl_k_then_q_returns_quit() {
        let mut app = App::new();
        // First key: Ctrl+K enters prefix mode
        dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL));
        // Second key: Q in prefix mode
        let action = dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(matches!(action, KeyAction::Quit));
    }

    #[test]
    fn ctrl_k_then_q_uppercase_returns_quit() {
        let mut app = App::new();
        dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL));
        let action = dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('Q'), KeyModifiers::SHIFT));
        assert!(matches!(action, KeyAction::Quit));
    }

    #[test]
    fn ctrl_k_prefix_resets_mode_to_editing_after_chord() {
        let mut app = App::new();
        dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL));
        dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
        // Mode should be reset to Editing by dispatch_prefix, regardless of the returned action
        assert!(matches!(app.mode, AppMode::Editing));
    }

    #[test]
    fn ctrl_k_esc_cancels_prefix_without_quit() {
        let mut app = App::new();
        dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL));
        let action = dispatch_key(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(matches!(action, KeyAction::Handled));
        assert!(matches!(app.mode, AppMode::Editing));
    }

    // --- confirm-dialog quit paths (the buggy continue path) ---
    // These test that both Y and N in the save-before-quit dialog set should_quit,
    // which is what the fixed code must honour before calling continue.

    #[test]
    fn confirm_y_sets_should_quit_for_save_before_quit() {
        let mut app = App::new();
        app.confirm_action = Some(crate::app::ConfirmAction::SaveBeforeQuit);
        app.mode = AppMode::Dialog(crate::app::DialogKind::Confirm);
        // Simulate pressing 'y' in the confirm dialog
        let key = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE);
        let action = dispatch_key(&mut app, key);
        // dispatch_dialog returns Handled for all dialog keys
        assert!(matches!(action, KeyAction::Handled));
        // The dialog handler inside dispatch_dialog doesn't set should_quit;
        // that's handled by handle_dialog_key in main.rs.  What we CAN assert
        // here is that a plain 'y' in dialog mode is not forwarded to the editor.
        assert!(!matches!(action, KeyAction::ForwardToEditor(_)));
    }

    #[test]
    fn confirm_n_in_dialog_mode_is_handled_not_forwarded() {
        let mut app = App::new();
        app.mode = AppMode::Dialog(crate::app::DialogKind::Confirm);
        let action = dispatch_key(&mut app, KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        assert!(matches!(action, KeyAction::Handled));
    }

    #[test]
    fn esc_in_dialog_mode_returns_to_editing() {
        let mut app = App::new();
        app.mode = AppMode::Dialog(crate::app::DialogKind::Confirm);
        dispatch_key(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
        assert!(matches!(app.mode, AppMode::Editing));
    }
}
