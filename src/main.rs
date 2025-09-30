mod ui_pages;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Color},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Terminal,
};

use std::{error::Error, io, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Screen {
    Main,
    TodoList,
    Calendar,
    Obsidian,
    WorkingOutPad,
    Configuration,
}

struct App {
    menu_items: Vec<&'static str>,
    selected: usize,
    screen: Screen,
}

impl App {
    fn new() -> Self {
        let menu_items = vec![
            "🐄  TODO list",
            "⚡  Calendar",
            "🚀  Obsidian",
            "🦐  Working out Pad",
        ];
        Self {
            menu_items,
            selected: 0,
            screen: Screen::Main,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.menu_items.len();
    }

    fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.menu_items.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn open_selected(&mut self) {
        self.screen = match self.selected {
            0 => Screen::TodoList,
            1 => Screen::Calendar,
            2 => Screen::Obsidian,
            3 => Screen::WorkingOutPad,
            4 => Screen::Configuration,
            _ => Screen::Main,
        };
    }

    fn back_to_main(&mut self) {
        self.screen = Screen::Main;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ensure we restore terminal on panic/exit
    let res = run_app(&mut terminal);

    // restore
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        // Poll for input with timeout so UI can remain responsive
        if event::poll(Duration::from_millis(200))? {
        match event::read()? {
    Event::Key(KeyEvent { code, .. }) => match (app.screen, code) {
        (Screen::Main, KeyCode::Char('q') | KeyCode::Esc) => return Ok(()),
        (Screen::Main, KeyCode::Down | KeyCode::Char('j')) => app.next(),
        (Screen::Main, KeyCode::Up | KeyCode::Char('k')) => app.previous(),
        (Screen::Main, KeyCode::Enter) => app.open_selected(),
        (screen, KeyCode::Esc) | (screen, KeyCode::Char('q')) if screen != Screen::Main => {
            app.back_to_main()
        }
        (_, KeyCode::Char('h')) => app.back_to_main(),
        _ => {}
    },
    Event::Mouse(_) | Event::Resize(_, _) => { /* ignore */ }
    Event::FocusGained | Event::FocusLost | Event::Paste(_) => { /* ignore */ }
}
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r)[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup)[1]
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    match app.screen {
        Screen::Main => draw_main_menu(f, app),
        Screen::TodoList => ui_pages::draw_fullscreen_page(f, "TodoList"),
        Screen::Calendar => ui_pages::draw_fullscreen_page(f, "Calendar"),
        Screen::Obsidian => ui_pages::draw_fullscreen_page(f, "Obsidian"),
        Screen::WorkingOutPad => ui_pages::draw_fullscreen_page(f, "WorkingOutPad"),
        Screen::Configuration => ui_pages::draw_fullscreen_page(f, "Configuration"),
    }
}

fn draw_main_menu(f: &mut ratatui::Frame, app: &App) {
    let size = f.size();

    // HEADER
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(25), // header
                Constraint::Min(6),     // menu
                Constraint::Length(3),  // footer
            ]
            .as_ref(),
        )
        .split(size);

    let header = Paragraph::new(ascii_art_cached("Ducky", "alligator", Color::Red))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    f.render_widget(header, Rect {
        x: 0,
        y: vertical_chunks[0].y,
        width: size.width,
        height: vertical_chunks[0].height,
    });

    // MENU
    let menu_area = centered_rect(40, 40, vertical_chunks[1]);
    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, it)| {
            let line = if i == app.selected {
                Line::from(Span::styled(
                    it.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::raw(*it))
            };
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED | Modifier::BOLD)
                .fg(Color::Yellow),
        )
        .highlight_symbol("> ");

    f.render_widget(list, menu_area);

    // FOOTER
    let footer = Paragraph::new(vec![
        Line::from(Span::raw("Ducky Loaded 0 AI Features")),
        Line::from(Span::styled(
            "Jack Baker -> Make Beautiful things | press q to quit, j/k up and down, enter to select screen",
            Style::default().add_modifier(Modifier::DIM),
        )),
    ])
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    f.render_widget(footer, Rect {
        x: 0,
        y: vertical_chunks[2].y,
        width: size.width,
        height: vertical_chunks[2].height,
    });
}



// Key for caching: (text, font, color)
type AsciiKey = (String, String, Color);

// Global cache
static ASCII_CACHE: Lazy<Mutex<HashMap<AsciiKey, Vec<Line<'static>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn ascii_art_cached(text: &str, font: &str, colour: Color) -> Vec<Line<'static>> {
    let key = (text.to_string(), font.to_string(), colour);

    // Check cache
    if let Some(cached) = ASCII_CACHE.lock().unwrap().get(&key) {
        return cached.clone();
    }

    // Generate ASCII art
    let output = std::process::Command::new("figlet")
        .arg("-f")
        .arg(font)
        .arg(text)
        .output();

    let ascii_lines = match output {
        Ok(out) if out.status.success() => {
            let ascii = String::from_utf8_lossy(&out.stdout);
            ascii
                .lines()
                .map(|line| Line::from(Span::styled(line.to_string(), Style::default().fg(colour))))
                .collect::<Vec<Line<'static>>>()
        }
        _ => {
            // fallback
            let fallback = format!(
                "+{line}+\n| {word} |\n+{line}+",
                line = "-".repeat(text.len() + 2),
                word = text
            );
            fallback
                .lines()
                .map(|line| Line::from(Span::styled(line.to_string(), Style::default().fg(colour))))
                .collect::<Vec<Line<'static>>>()
        }
    };

    // Store in cache
    ASCII_CACHE.lock().unwrap().insert(key, ascii_lines.clone());
    ascii_lines
}
