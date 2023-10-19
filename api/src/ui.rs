use ratatui::style::{Style, Color};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;
use std::collections::HashMap;

use crate::types::Item;


pub struct NavigableList<'a, T> {
    pub list: &'a Vec<T>,
    idx: usize,
}

impl<'a, T> NavigableList<'a, T> {
    pub fn from(list: &Vec<T>) -> NavigableList<T> {
        NavigableList {list, idx: 0}
    }

    pub fn move_next(&mut self) {
        if self.idx + 1 < self.list.len() {
            self.idx += 1;
        }
    }

    pub fn move_prev(&mut self) {
        if self.idx > 0 {
            self.idx -= 1;
        }
    }

    pub fn get_position(&self) -> usize {
        self.idx
    }

    pub fn get(&self) -> &T {
        self.list
            .get(self.idx)
            .expect(format!("Invalid position: {}", self.idx).as_str())
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.list.iter()
    }
}


pub fn make_item_store(items: Vec<Item>) -> HashMap<u32, Item> {
    items.into_iter()
        .map(|item| (*item.id(), item))
        .collect::<HashMap<u32, Item>>()
}


pub fn render_item_titles<'a>(
    item_id_list: &NavigableList<u32>,
    item_store: &'a std::collections::HashMap<u32, Item>,
) -> Paragraph<'a> {
    let position = item_id_list.get_position();
    let vec = item_id_list.iter()
        .enumerate()
        .map(|(idx, item_id)| {
            let item = item_store.get(item_id)
                .expect(format!("Missing item: {}", item_id).as_str());

            if idx == position {
                Span::styled(
                    format!("> {} {}", idx, item.title()),
                    Style::new().fg(Color::Red),
                )
            } else {
                Span::styled(
                    format!("  {} {}", idx, item.title()),
                    Style::new().fg(Color::White),
                )
            }
        })
        .map(|span| Line::from(span))
        .collect::<Vec<_>>();
    Paragraph::new(Text::from(vec))
}



#[test]
fn test_list_navigation() {
    let items = vec![1, 2, 3];
    let mut list = NavigableList::from(&items);
    assert_eq!(*list.get(), 1);

    list.move_prev();
    assert_eq!(*list.get(), 1);

    list.move_next();
    assert_eq!(*list.get(), 2);

    list.move_next();
    assert_eq!(*list.get(), 3);

    list.move_next();
    assert_eq!(*list.get(), 3);

    list.move_prev();
    assert_eq!(*list.get(), 2);
}