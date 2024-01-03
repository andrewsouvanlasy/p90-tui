use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::Direction,
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::{BarChart, Block, Borders},
};

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

// App state
struct App {
    magazine: u32,
    magazine_capacity: u32,
    should_quit: bool,
}

// App UI render function
fn ui(app: &App, f: &mut Frame) {
    let mut block = Block::default().title("P90").borders(Borders::ALL);

    // Change title depending on state
    if app.magazine == 0 {
        block = Block::default().title("P90 (empty)").borders(Borders::ALL);
    } else if app.magazine == 51 {
        block = Block::default()
            .title("P90 (one in the chamber)")
            .borders(Borders::ALL);
    }

    let bar_chart = BarChart::default()
        .block(block)
        .bar_width(3)
        .direction(Direction::Horizontal)
        .data(&[("Magazine", app.magazine as u64)])
        .max(app.magazine_capacity as u64);

    f.render_widget(bar_chart, f.size());
}

// App update function
// Checks for keyboard events every 250ms
fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('f') => fire(app),
                    Char('r') => reload(app),
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

// Subtracts one round from the magazine
fn fire(app: &mut App) {
    if app.magazine > 0 {
        app.magazine -= 1;
    }
}

// Load a full magazine into the gun
fn reload(app: &mut App) {
    if (app.magazine) > 0 {
        app.magazine = 51;
    } else {
        app.magazine = 50;
    }
}

fn run() -> Result<()> {
    // Ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Application state
    let mut app = App {
        magazine: 50,
        magazine_capacity: 50,
        should_quit: false,
    };

    loop {
        // Application update
        update(&mut app)?;

        // Application render
        t.draw(|f| {
            ui(&app, f);
        })?;

        // Application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // Setup terminal
    startup()?;

    let result = run();

    // Teardown terminal before unwrapping Result of app run
    shutdown()?;

    result?;

    Ok(())
}
