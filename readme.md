# Hacker News TUI, written in Rust

This is a pet project to produce a textual interface for browsing hackernews stories
using your favorite terminal.

## Features + ToDos

- UI
  - [X] Navigate a list of story titles
    - [ ] Scroll through list rendered beyond terminal height
  - [ ] Display comments on a selected story
- API
  - [X] Get item ids for stories from endpoints: best/new/top
  - [X] Get an item's data via id
  - [X] Get data for a list of item ids
    - 1 `tokio` task per item executed asynchronously
- Code
  - [X] Refactor HNClient into sub-module
  - [ ] Refactor UI into sub-module
    - Define a state machine for UI
