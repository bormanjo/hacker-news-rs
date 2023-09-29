use client::{api, ui};

use ratatui::prelude::{CrosstermBackend, Terminal};

pub type Frame<'a> = ratatui::Frame<'a, CrosstermBackend<std::io::Stderr>>;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;
  
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // create the API client
    let client = api::HNClient::new();

    // query stories as a list of items
    let item_ids = client.get_stories(api::Story::Best).await?;
    let mut item_id_list = ui::NavigableList::from(&item_ids);

    // query each item's content
    let items = client.get_items(&item_ids).await;
    let item_store = ui::make_item_store(items);

    loop {
        let paragraph = ui::render_item_titles(&item_id_list, &item_store);
        terminal.draw(|f| {
                f.render_widget(paragraph, f.size());
            }
        )?;

        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Down => {item_id_list.move_next();},
                    crossterm::event::KeyCode::Up => {item_id_list.move_prev();},
                    _ => ()
                }
            }
        }
    }
  
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
  
    Ok(())
  }