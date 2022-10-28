mod stateful_table;
pub mod document;
mod document_viewer;

use std::path::Path;
use std::{io, time::Duration, thread};
use std::fs;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, read, Event, KeyCode}};
use document::{Document};
use document_viewer::DocumentViewer;
use stateful_table::StatefulTable;
use tui::layout::{Layout, Direction, Constraint, Alignment};
use tui::style::{Style, Color};
use tui::text::{Spans, Span};
use tui::widgets::{Block, Borders, Paragraph, Wrap, Table, Row};
use tui::{backend::CrosstermBackend, Terminal};

fn get_files(path: &Path) -> Vec<fs::DirEntry> {
    fs::read_dir(path)
        .unwrap()
        .map(|item| item.unwrap())
        .collect()
}

fn get_ascii_art() -> String {
    get_file_contents(Path::new("./ascii_art.txt")).expect("Could not read ASCII art file")
}

fn get_file_contents(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut stateful_table = StatefulTable::new(get_files(Path::new(".")));
    let mut document_viewer = DocumentViewer::new(String::default());
    let tickrate = Duration::ZERO;
    loop {
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Down => {
                    stateful_table.next();
                    document_viewer.get_mut_state().set_offset(0);
                },
                KeyCode::Up => {
                    stateful_table.prev();
                    document_viewer.get_mut_state().set_offset(0);
                },
                KeyCode::Enter => {
                    let is_dir = stateful_table.get_selected().path().is_dir();
                    if is_dir {
                        stateful_table.set_items(get_files(&stateful_table.get_selected().path()))
                    }
                },
                KeyCode::Backspace => {
                    let current_path = stateful_table.get_selected().path().canonicalize().unwrap();
                    /* 
                    We need to get the "grandparent" directory because `.parent()` actually
                    only returns the current directory the first time around.
                    For instance, if the list is pointed ./src/main.rs and we use `.parent()` it would
                    return `./src`, and getting the contents of that directory is the same as the
                    directory we're already in. So we need to get the parent of `./src/main.rs`, then
                    get the parent again, which results in `./`.
                    
                    */
                    if let Some(current_dir) = current_path.parent() {
                        if let Some(parent_dir) = current_dir.parent() {
                            let entries = get_files(parent_dir);
                            stateful_table.set_items(entries);
                        }
                    }
                },
                KeyCode::PageDown => {
                    document_viewer.scroll_down();
                },
                KeyCode::PageUp => {
                    document_viewer.scroll_up();
                }
                _ => {}
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(70),
                    Constraint::Percentage(10),
                ].as_ref())
                .split(f.size());

            let inner_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ].as_ref())
                .split(chunks[1]);
            
            let ascii_art = Paragraph::new(get_ascii_art())
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center)
                .wrap(Wrap {
                    trim: false
                });
            f.render_widget(ascii_art, chunks[0]);

            let mut rows: Vec<Row> = Vec::new();
            for entry in &stateful_table.items {
                if entry.path().is_dir() {
                    rows.push(Row::new(vec![
                        entry.file_name().into_string().unwrap(),
                        String::from("D"),
                    ]).style(Style::default().fg(Color::Green)));
                } else {
                    rows.push(Row::new(vec![
                        entry.file_name().into_string().unwrap(),
                        String::from("F"),
                    ]).style(Style::default()));
                }
            }

            let table = Table::new(rows)
                .header(Row::new(vec![
                    "File Name",
                    "File Type",
                ]))
                .block(Block::default().borders(Borders::ALL).title("Files"))
                .widths(&[Constraint::Percentage(30), Constraint::Length(5), ])
                .column_spacing(1)
                .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                .highlight_symbol(">>");
            f.render_stateful_widget(table, inner_chunks[0], &mut stateful_table.state);

            let text = get_file_contents(&stateful_table.get_selected().path()).unwrap_or_default();
            document_viewer.set_contents(text);
            let document_widget = Document::new(document_viewer.get_lines().to_vec())
                .block(Block::default().title("Document Viewer").borders(Borders::ALL))
                .style(Style::default());
            f.render_stateful_widget(document_widget, inner_chunks[1], document_viewer.get_mut_state());

            let info_paragraph = Paragraph::new(vec![
                Spans::from(vec![
                    Span::styled("[Up Arrow] ", Style::default().fg(Color::Cyan)),
                    Span::raw("Previous File"), 
                    Span::styled(" [Down Arrow] ", Style::default().fg(Color::Cyan)),
                    Span::raw("Next File"), 
                ]),
                Spans::from(vec![
                    Span::styled("[Enter] ", Style::default().fg(Color::Cyan)),
                    Span::raw("Enter Directory"), 
                    Span::styled(" [Backspace] ", Style::default().fg(Color::Cyan)),
                    Span::raw("Previous Directory"), 
                ]),
                Spans::from(vec![
                    Span::styled("[Page Up] ", Style::default().fg(Color::Cyan)),
                    Span::raw("Scroll document viewer up"), 
                    Span::styled(" [Page Down] ", Style::default().fg(Color::Cyan)),
                    Span::raw("Scroll document viewer down"), 
                ]),
            ])
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info_paragraph, chunks[2]);
        })?;
        
        thread::sleep(tickrate);
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    Ok(())
}
