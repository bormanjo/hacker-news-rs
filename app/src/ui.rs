use tui::{backend::Backend, Frame};

use crate::app::{App, Action};

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples


    match app.action {
        Action::NavigateStories => {
            let (list, mut state) = app.get_navigate_stories_list();
            frame.render_stateful_widget(list, frame.size(), &mut state)
        },
        Action::ViewHelpDialogue => frame.render_widget(
            app.get_help_dialogue(),
            frame.size(),
        ),
        Action::ViewStory => frame.render_widget(
            app.get_story_info(),
            frame.size(),
        ),
    }
}
