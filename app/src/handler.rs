use crate::app::{App, AppResult, Action};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        },
        // Toggle help dialogue
        KeyCode::Char('h') | KeyCode::Char('H') => {
            if app.action != Action::ViewHelpDialogue {
                app.update_action(Action::ViewHelpDialogue);
            } else {
                app.update_action(app.last_action.unwrap());
            }
        }
        _ => {},
    }

    app.dispatch_action(key_event).await;

    Ok(())
}
