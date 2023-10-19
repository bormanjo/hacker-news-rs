use std::{error, sync::Arc};

use api::{types::{Item, StoryKind, STORY_KINDS}, client::HNClient};
use crossterm::event::{KeyEvent, KeyCode};
use tui::{widgets::{Table, Row, Cell, List, ListItem, Block, Borders, BorderType, Paragraph, ListState}, style::{Style, Color}, layout::Alignment};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;


#[derive(Debug)]
pub struct Context {
    pub story_kind_selector: Selector<StoryKind>,
    pub story_selector: Selector<Item>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    ViewHelpDialogue,
    NavigateStories,
    ViewStory,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            story_kind_selector: Selector::new(STORY_KINDS.to_vec()),
            story_selector: Selector::new(Vec::new()),
        }
    }
}


#[derive(Debug)]
pub struct Selector<T> {
    index: usize,
    items: Vec<T>,
}

impl<T> Selector<T> {
    fn new(items: Vec<T>) -> Self {
        Self { index: 0, items }
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn get(&self) -> Option<&T> {
        self.items.get(self.index)
    }
    pub fn next(&mut self) {
        if self.index + 1 < self.len() { self.index += 1; }
    }
    pub fn previous(&mut self) {
        if self.index > 0 { self.index -= 1; }
    }
}


/// Application.
#[derive(Debug)]
pub struct App {
    pub client: Arc<HNClient>,

    /// Is the application running?
    pub running: bool,
    /// counter
    pub counter: u8,

    pub context: Context,
    pub last_action: Option<Action>,
    pub action: Action,
}

impl Default for App {
    fn default() -> Self {
        Self {
            client: HNClient::new(),
            running: true,
            counter: 0,
            context: Context::default(),
            action: Action::NavigateStories,
            last_action: None,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Self {
        let mut app = Self::default();
        app.refresh_stories().await;
        app
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn dispatch_action(&mut self, key_event: KeyEvent) {
        match self.action {
            Action::NavigateStories => self.navigate_stories(key_event).await,
            Action::ViewHelpDialogue => {},
            Action::ViewStory => self.navigate_story(key_event).await,
        }
    }

    async fn navigate_stories(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => {
                self.context.story_kind_selector.previous();
                self.refresh_stories().await;
            },
            KeyCode::Right => {
                self.context.story_kind_selector.next();
                self.refresh_stories().await;
            },
            KeyCode::Up => self.context.story_selector.previous(),
            KeyCode::Down => self.context.story_selector.next(),
            KeyCode::Enter => self.update_action(Action::ViewStory),
            _ => (),
        }
    }

    async fn navigate_story(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Backspace => self.update_action(self.last_action.unwrap()),
            _ => {},
        }
    }

    async fn refresh_stories(&mut self) {
        let story_kind = self.context.story_kind_selector.get().unwrap();
        let stories = self.client.get_stories(story_kind).await.unwrap();
        let items = self.client.get_items(&stories).await;

        self.context.story_selector = Selector::new(items);
    }

    pub fn update_action(&mut self, action: Action) {
        self.last_action = Some(self.action);
        self.action = action;
    }

    pub fn get_help_dialogue(&self) -> Paragraph {
        Paragraph::new(format!(
            "\n\
            Controls:\n\
            - `Esc`/`Ctrl+C`/`q` to exit the app at any time\n\
            - `h` toggles this message at any time\n\
            - `Left`/`Right` to navigate between story feeds (Top, New, Best)\n\
            - `Up`/`Down` to navigate between stories/comments\n\
            - `Enter`/`Backspace` to jump in/out of stories, comments\n\
            "
        ))
        .style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .block(
            Block::default()
                .title("About This Program")
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        )
    }

    pub fn get_story_info(&self) -> Paragraph {
        let story = self.context.story_selector.get().unwrap();
        let text = match (story.text(), story.url()) {
            (Some(text), _) => text.to_owned(),
            (None, Some(url)) => url.to_owned(),
            _ => format!("<No text available for item id: {}>", story.id()).to_string(),
        };
        Paragraph::new(text)
            .style(Style::default().fg(Color::Magenta).bg(Color::Black))
            .block(
                Block::default()
                    .title(format!(
                        "{} by {} @ {}",
                        story.title(),
                        story.by(),
                        story.time(),
                    ))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            )
    }

    pub fn get_navigate_stories_list(&self) -> (List, ListState) {
        let items: Vec<ListItem> = self.context.story_selector.items
            .iter()
            .enumerate()
            .map(|(i, item)| 
                ListItem::new(
                    format!(
                        "{}. {}",
                        i,
                        item.title().clone(),
                    )
                )
            )
            .collect();
        let list = List::new(items)
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::Magenta).bg(Color::Black))
            .block(
                Block::default()
                    .title(
                        format!(
                            "{:?} Stories",
                            self.context.story_kind_selector.get().unwrap(),
                        )
                    )
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
            );
        let mut state = ListState::default();
        state.select(Some(self.context.story_selector.index));

        (list, state)
    }

    pub fn get_navigate_stories_table(&self) -> Table {
        let mut rows = Vec::new();
        for (i, story) in self.context.story_selector.items.iter().enumerate() {
            let cells = vec![
                Cell::from(format!("{i:}. {}", story.title()).to_string()),
            ];
            let row = Row::new(cells);
            rows.push(row);
        }
        Table::new(rows)
    }
}
