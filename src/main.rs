use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    fmt,
    io,
};
use rand::{Rng};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Paragraph, Wrap},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

// Stores and fetches colors for the entire program
fn color_palette(color: &str) -> tui::style::Color {
    match color {
        "bg" => Color::Black,
        "fg" => Color::White,
        "barbg" => Color::Blue,
        "red" => Color::Red,
        _ => Color::Cyan,
    }
}

enum Mode {
    Normal,
    Input,
}
impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::Normal => write!(f, "normal"),
            Mode::Input => write!(f, "input"),
        }
    }
}

// App holds the state of the application
struct App {
    // Stores whether pride mode is enabled or not
    pride_mode: bool,
    // Stores the current page
    mode: Mode,
    // Stores the current contents of the input box
    input_box: String,
    // Stores the submitted input
    submission: String,
    // The potential choices for the answers
    months: [String; 12],
    // Stores the answer
    answer: String,
    // Holds if the previous answer was correct or not
    correct: bool
}

impl Default for App {
    fn default() -> App {
        App {
            pride_mode: true,
            mode: Mode::Normal,
            input_box: String::new(),
            submission: String::new(),
            months: [String::from("January"), String::from("February"), String::from("March"), String::from("April"), String::from("May"), String::from("Pride"), String::from("July"), String::from("August"), String::from("September"), String::from("October"), String::from("November"), String::from("December")],
            answer: String::new(),
            correct: true,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // Restore terminal
    // This is used if the app stops without the shortcut being
    //  pressed for some reason. Allowing the default flow to
    //  handle the restoration of the terminal.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut rng = rand::thread_rng();
    let mut month = rng.gen_range(0, 12);
    loop {
        terminal.draw(|f| ui(f, &app, &month))?;
        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => { return Ok(()); }
                    KeyCode::Char('i') => { app.mode = Mode::Input; }
                    KeyCode::Char('p') => { app.pride_mode ^= true; }
                    // Make this highlight/flash the mode signifier
                    _ => {}
                },
               Mode::Input => match key.code {
                    KeyCode::Esc => { app.mode = Mode::Normal; }
                    KeyCode::Enter => {
                        app.submission = String::from(&app.input_box);
                        app.answer = String::from(&app.months[month]);
                        if app.submission.to_uppercase() == app.answer.to_uppercase() {
                            app.correct = true;
                        } else {
                            app.correct = false;
                        }
                        app.input_box.clear();
                        month = rng.gen_range(0, 12);
                    }
                    KeyCode::Char(c) => { app.input_box.push(c); }
                    KeyCode::Backspace => { app.input_box.pop(); }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App, month: &usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let question = Paragraph::new(format!(
            "{if_correct}\nWhat is the month: {question}"
            // If app.correct is true then no message needs to be displayed
            // Consider changing this to a bright green "Correct"
            , if_correct = if app.correct { String::new() } else { format!("WRONG! The answer is: {answer}", answer=app.answer) }
            // +1 to offset array index
            , question=month + 1
            )
        )
        .style(Style::default().bg(color_palette("bg")).fg(color_palette("fg")))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(question, chunks[0]);

    // Create text to put into the widget
    // Allows for easier formatting
    let text = format!(
        "Mode: {mode} | Pride mode: {pridemode}"
        , mode=app.mode
        , pridemode=if app.pride_mode == true { "enabled " } else { "disabled" }
    );

    let bar = Paragraph::new(text.clone())
        .style(Style::default().bg(color_palette("barbg")).fg(color_palette("fg")))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(bar, chunks[1]);

    match app.mode {
        Mode::Normal => {}
        Mode::Input => {
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + app.input_box.width() as u16,
                chunks[2].y,
            )
        }
    }

    let input_box = Paragraph::new(app.input_box.as_ref())
        .style(Style::default().bg(color_palette("bg")).fg(color_palette("fg")));
        //.block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input_box, chunks[2]);
}
