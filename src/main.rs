use crossterm::{
    cursor::SetCursorStyle,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{self, Rng};
use std::{error::Error, io, ops::Index};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
struct App {
    characters: [[Character; 5]; 6],
    position: [usize; 2],
    status: Gamestatus,
    word: String,
    message: String,
}
impl Default for App {
    fn default() -> Self {
        Self {
            characters: [[Character {
                character: ' ',
                status: CharacterStatus::NotGuessed,
            }; 5]; 6],
            status: Gamestatus::Ongoing,
            position: [0, 0],
            word: {
                let words = std::fs::read_to_string("words.txt")
                    .unwrap()
                    .split("\n")
                    .map(|f| f.trim().to_string())
                    .collect::<Vec<String>>();
                words[rand::thread_rng().gen_range(0..words.len())].clone()
            },
            message: String::new(),
        }
    }
}
#[derive(Copy, Clone)]

struct Character {
    character: char,
    status: CharacterStatus,
}
#[derive(Clone, Copy)]
enum CharacterStatus {
    NotGuessed,
    Correct,
    SomewhereElse,
    Incorrect,
}
#[derive(Debug)]
enum Gamestatus {
    Ongoing,
    Won,
    Lost,
}
fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match app.status {
                Gamestatus::Lost => match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => app = App::default(),
                    _ => {}
                },
                Gamestatus::Won => match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => app = App::default(),
                    _ => {}
                },
                Gamestatus::Ongoing => match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Backspace => {
                        if app.position[1] >= 1 {
                            app.characters[app.position[0]][app.position[1] - 1].character = ' ';
                            app.position[1] -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        if app.position[1] >= 5 {
                            if app.characters[app.position[0]]
                                .iter()
                                .map(|f| f.character.to_string())
                                .collect::<String>()
                                .to_lowercase()
                                == app.word.to_lowercase()
                            {
                                app.message = "jeromes".to_string();
                                app.status = Gamestatus::Won
                            } else if app.position[0] >= 5 {
                                app.status = Gamestatus::Lost
                            } else {
                                for i in 0..5 {
                                    app.characters[app.position[0]][i].status = value(
                                        app.characters[app.position[0]][i].character,
                                        app.word.clone(),
                                        app.word.chars().collect::<Vec<char>>()[i],
                                    )
                                }
                                app.position[0] += 1;
                                app.position[1] = 0;
                            }
                        }
                    }
                    KeyCode::Char(char) => {
                        if app.position[1] < 5 {
                            app.characters[app.position[0]][app.position[1]].character =
                                char.to_uppercase().collect::<Vec<char>>()[0];
                            app.position[1] += 1;
                        }
                    }
                    _ => {}
                },
            }
        }
    }
}
fn value(character: char, word: String, comparison: char) -> CharacterStatus {
    if comparison.to_string() == character.to_lowercase().to_string() {
        return CharacterStatus::Correct;
    } else if word.contains(character) {
        return CharacterStatus::SomewhereElse;
    } else {
        return CharacterStatus::Incorrect;
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let title = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(Rect {
            x: 0,
            y: 0,
            width: f.size().width,
            height: (f.size().height as f32 * 0.2).round() as u16,
        });
    f.render_widget(
        Paragraph::new(Text::from(format!(
            "FIMSHDLE WORD : {} message : {}",
            app.word, app.message
        )))
        .alignment(tui::layout::Alignment::Center),
        title[0],
    );
    match app.status {
        Gamestatus::Ongoing => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Ratio(1, 6),
                        Constraint::Ratio(1, 6),
                        Constraint::Ratio(1, 6),
                        Constraint::Ratio(1, 6),
                        Constraint::Ratio(1, 6),
                        Constraint::Ratio(1, 6),
                    ]
                    .as_ref(),
                )
                .split(Rect {
                    x: (f.size().width as f32 * 0.1).round() as u16,
                    y: (f.size().height as f32 * 0.2).round() as u16,
                    width: (f.size().width as f32 * 0.8).round() as u16,
                    height: (f.size().height as f32 * 0.8).round() as u16,
                });
            for i in 0..6 {
                let y = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                            Constraint::Ratio(1, 5),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[i]);
                for n in 0..5 {
                    f.render_widget(
                        Paragraph::new(Text::from(app.characters[i][n].character.to_string()))
                            .alignment(tui::layout::Alignment::Center)
                            .style(Style::default().fg(match app.characters[i][n].status {
                                CharacterStatus::Correct => Color::Green,
                                CharacterStatus::Incorrect => Color::Red,
                                CharacterStatus::SomewhereElse => Color::Yellow,
                                CharacterStatus::NotGuessed => Color::White,
                            }))
                            .block(Block::default().borders(Borders::ALL)),
                        y[n],
                    )
                }
            }
        }
        Gamestatus::Lost => f.render_widget(
            Paragraph::new(Text::from(format!(
                "You lost the word was  : {} press enter to restart",
                app.word
            ))),
            f.size(),
        ),
        Gamestatus::Won => f.render_widget(
            Paragraph::new(Text::from(format!(
                "You won with {} guesses press enter to restart",
                app.position[0] + 1
            ))),
            f.size(),
        ),
    }
}
