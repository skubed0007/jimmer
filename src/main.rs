use std::{
    env, fs::File, io::BufReader, path::{Path, PathBuf}, thread, time::{Duration, Instant}
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use colored::Colorize;
use rand::{rng, seq::IndexedRandom};
use rodio::{Decoder, OutputStream, Sink};

const QUOTES: &[&str] = &[
    "Later the coffee gets cold, later interest gets old",
    "Later the dream shatters, later the truth matters",
    "Later the illusion fades, later the real pervades",
    "Later the comfort breaks, later the challenge awakes",
    "Later the sleep departs, later the journey starts",
    "Later the mask falls, later the self stands tall",
    "Later the chains loosen, later the spirit's chosen",
    "Later the fear subsides, later the courage guides",
    "Later the doubt recedes, later the will proceeds",
    "Later the past releases, later the future increases",
    "Later the lies unravel, later the answers travel",
    "Later the world awakens, later the heart strengthens",
    "Later the vision clears, later the purpose nears",
    "Later the voice calls, later the destiny enthralls",
    "Later the time is now, later the power you endow",
    "Later the choice is made, later the path is laid",
    "Later the battle starts, later the victory imparts",
    "Later the wound heals, later the spirit reveals",
    "Later the scar remains, later the wisdom it sustains",
    "Later the fall occurs, later the lesson endures",
    "Later the rise begins, later the true self wins",
    "Later the change arrives, later the new life thrives",
    "Later the old self dies, later the true self flies",
    "Later the cage breaks open, later the spirit is spoken",
    "Later the blindfold lifts, later the true sight gifts",
    "Later the illusion shatters, later reality matters",
    "Later the dream dissolves, later the truth evolves",
    "Later the false hope fades, later true strength pervades",
    "Later the shallow sleep ends, later the deep knowing transcends",
    "Later the painted smile cracks, later the authentic self attacks",
    "Later the sugar-coated lie decays, later the bitter truth sways",
    "Later the fleeting pleasure departs, later lasting purpose imparts",
    "Later the fleeting moment slips, later eternal presence grips",
    "Later the borrowed identity cracks, later the true self enacts",
    "Later the whispered promise breaks, later the silent vow awakes",
    "Later the fleeting beauty decays, later inner strength displays",
    "Later the comforting lie unwinds, later truth's harsh light blinds",
    "Later the sweet delusion ends, later reality transcends",
    "Later the painted world cracks, later the true world attacks",
    "Later the soft illusion shatters, later the stark truth matters",
    "Later the fleeting joy subsides, later deep purpose guides",
    "Later the borrowed strength wanes, later true power sustains",
    "Later the hollow praise rings hollow, later self-belief will follow",
    "Later the fleeting glimpse fades, later lasting vision pervades",
    "Later the sugar-coated dream sours, later true potential flowers",
    "Later the borrowed time runs out, later true self cries out",
    "Later the painted smile fades, later authentic strength pervades",
    "Later the comforting lie deceives, later harsh truth relieves",
    "Later the sweet illusion shatters, later reality truly matters",
    "Later the fleeting pleasure wanes, later lasting purpose sustains",
    "Later the borrowed identity cracks, later true self enacts",
    "Later the whispered promise breaks, later the silent vow awakes",
    "Later the fleeting beauty decays, later inner strength displays",
    "Later the comforting lie unwinds, later truth's harsh light blinds",
    "Later the sweet delusion ends, later reality transcends",
    "Later the painted world cracks, later the true world attacks",
    "Later the soft illusion shatters, later the stark truth matters",
    "Later the fleeting joy subsides, later deep purpose guides",
    "Later the borrowed strength wanes, later true power sustains",
    "Later the hollow praise rings hollow, later self-belief will follow",
    "Later the fleeting glimpse fades, later lasting vision pervades",
    "Later the sugar-coated dream sours, later true potential flowers",
    "Later the borrowed time runs out, later true self cries out",
    "Later the painted smile fades, later authentic strength pervades",
    "Later the comforting lie deceives, later harsh truth relieves",
    "Later the sweet illusion shatters, later reality truly matters",
    "Later the fleeting pleasure wanes, later lasting purpose sustains",
    "Later the borrowed identity cracks, later true self enacts",
    "Later the whispered promise breaks, later the silent vow awakes",
    "Later the fleeting beauty decays, later inner strength displays",
    "Later the comforting lie unwinds, later truth's harsh light blinds",
    "Later the sweet delusion ends, later reality transcends",
    "Later the painted world cracks, later the true world attacks",
    "Later the soft illusion shatters, later the stark truth matters",
    "Later the fleeting joy subsides, later deep purpose guides",
    "Later the borrowed strength wanes, later true power sustains",
    "Later the hollow praise rings hollow, later self-belief will follow",
    "Later the fleeting glimpse fades, later lasting vision pervades",
    "Later the sugar-coated dream sours, later true potential flowers",
    "Later the borrowed time runs out, later true self cries out",
    "Later the painted smile fades, later authentic strength pervades",
    "Later the comforting lie deceives, later harsh truth relieves",
    "Later the sweet illusion shatters, later reality truly matters",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return Ok(());
    }

    let total_time = parse_arguments(&args)?;
    if total_time == 0 {
        eprintln!("{}", "Error: No valid time arguments provided!".red().bold());
        return Ok(());
    }

    let theme = get_theme_from_args(&args)?;
    let audio_path = "audio.mp3";
let end_path = "end.mp3";

let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
let exe_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
let audio_exists = Path::new(&current_dir).join(audio_path).exists() || Path::new(&exe_dir).join(audio_path).exists();
let end_exists = Path::new(&current_dir).join(end_path).exists() || Path::new(&exe_dir).join(end_path).exists();

if !audio_exists && !end_exists {
    eprintln!("{}", "Warning: neither 'audio.mp3' nor 'end.mp3' found, using 'audio.mp3' for both.".yellow().bold());
    thread::sleep(Duration::from_secs(2));
    run_timer(total_time, audio_path, audio_path, theme)?;
} else if !audio_exists {
    eprintln!("{}", "Error: 'audio.mp3' not found!".red().bold());
    return Err("audio.mp3 not found".into());
} else if !end_exists {
    eprintln!("{}", "Warning: 'end.mp3' not found, using 'audio.mp3' for end sound.".yellow().bold());
    thread::sleep(Duration::from_secs(2));
    run_timer(total_time, audio_path, audio_path, theme)?;
} else {
    run_timer(total_time, audio_path, end_path, theme)?;
}

    Ok(())
}

fn parse_arguments(args: &[String]) -> Result<u64, Box<dyn std::error::Error>> {
    let mut total_time = 0u64;
    for arg in args.iter().skip(1) {
        if let Some(value) = arg.strip_prefix("--minute=") {
            total_time += value.parse::<u64>()? * 60_000;
        } else if let Some(value) = arg.strip_prefix("--second=") {
            total_time += value.parse::<u64>()? * 1_000;
        }
    }
    Ok(total_time)
}

fn get_theme_from_args(args: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    for arg in args.iter() {
        if let Some(value) = arg.strip_prefix("--theme=") {
            let theme = value.parse::<usize>()?;
            if theme >= 1 && theme <= 10 {
                return Ok(theme - 1); // Convert to 0-based index
            }
        }
    }
    Ok(0) // Default to theme 0
}

fn run_timer(total_time: u64, loop_sound: &str, end_sound: &str, theme: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let start_time = Instant::now();
    let mut paused = false;
    let mut pause_start = Instant::now();
    let mut total_paused = 0u64;
    let mut remaining = total_time;
    let mut last_quote_change = Instant::now();
    let mut current_quote = QUOTES.choose(&mut rng()).unwrap();
    
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(loop_sound)?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.set_volume(0.5);
    sink.play();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3), Constraint::Min(1), Constraint::Length(3)].as_ref())
                .split(size);

            let timer = Paragraph::new(Spans::from(vec![Span::styled(
                format_time(remaining),
                get_timer_style(theme),
            )]))
            .block(Block::default().borders(Borders::ALL).title("Timer"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

            let quote_widget = Paragraph::new(Spans::from(vec![Span::styled(
                current_quote.to_string(),
                get_quote_style(theme),
            )]))
            .block(Block::default().borders(Borders::ALL).title("Quote"))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
            let help = Paragraph::new(Spans::from(vec![Span::styled(
                "Press P to pause, Q to quit",
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )]));

            f.render_widget(timer, chunks[0]);
            f.render_widget(quote_widget, chunks[1]);
            f.render_widget(help, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('p') => {
                        // Pause or resume the timer
                        if paused {
                            total_paused += pause_start.elapsed().as_millis() as u64;
                        } else {
                            pause_start = Instant::now();
                        }
                        paused = !paused;
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if !paused {
            let elapsed = start_time.elapsed().as_millis() as u64;
            remaining = total_time.saturating_sub(elapsed - total_paused);
            if remaining == 0 {
                sink.stop();
                let sink = Sink::try_new(&stream_handle)?;
                let file = BufReader::new(File::open(end_sound)?);
                let source = Decoder::new(file)?;
                sink.append(source);
                sink.set_volume(2.0);
                sink.play();

                terminal.draw(|f| {
                    let size = f.size();
                    let block = Block::default().borders(Borders::ALL).title("TIME IS UP!");
                    let paragraph = Paragraph::new(Spans::from(vec![Span::styled(
                        "Press Q to exit...",
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                    )]))
                    .block(block)
                    .alignment(Alignment::Center);
                    f.render_widget(paragraph, size);
                })?;
                let _ = event::read()?;
                break;
            }
        }

        if last_quote_change.elapsed() >= Duration::from_secs(5) {
            current_quote = QUOTES.choose(&mut rng()).unwrap();
            last_quote_change = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn format_time(millis: u64) -> String {
    let minutes = millis / 60_000;
    let seconds = (millis % 60_000) / 1_000;
    format!("{:02}:{:02}", minutes, seconds)
}

fn get_timer_style(theme: usize) -> Style {
    match theme {
        0 => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        1 => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        2 => Style::default().fg(Color::Blue).add_modifier(Modifier::ITALIC),
        3 => Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED),
        4 => Style::default().fg(Color::Magenta).add_modifier(Modifier::DIM),
        5 => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        6 => Style::default().fg(Color::White).add_modifier(Modifier::ITALIC),
        7 => Style::default().fg(Color::Black).add_modifier(Modifier::UNDERLINED),
        8 => Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
        9 => Style::default().fg(Color::White).add_modifier(Modifier::REVERSED),
        _ => Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    }
}

fn get_quote_style(theme: usize) -> Style {
    match theme {
        0 => Style::default().fg(Color::White).add_modifier(Modifier::ITALIC),
        1 => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        2 => Style::default().fg(Color::Blue).add_modifier(Modifier::ITALIC),
        3 => Style::default().fg(Color::Magenta).add_modifier(Modifier::DIM),
        4 => Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED),
        5 => Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        6 => Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC),
        7 => Style::default().fg(Color::Black).add_modifier(Modifier::UNDERLINED),
        8 => Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD),
        9 => Style::default().fg(Color::Rgb(100, 200, 100)).add_modifier(Modifier::BOLD),
        _ => Style::default().fg(Color::White).add_modifier(Modifier::ITALIC),
    }
}
fn print_help() {
    println!("Usage: jimmer [options]");
    println!("\nOptions:");
    println!("  --minute=<value>   Set the timer duration in minutes");
    println!("  --second=<value>   Set the timer duration in seconds");
    println!("  --theme=<number>   Choose a theme for the timer and quotes (1-10)");
    println!("  -h, --help         Show this help message");
    println!("\nExample Usage:");
    println!("  timer --minute=5 --second=30 --theme=3");
    println!("\nIn this example:");
    println!("  - The timer will run for 5 minutes and 30 seconds.");
    println!("  - Theme 3 will be applied to the timer and quote widgets.");
    println!("  - 'audio.mp3' will be played in a loop during the timer.");
    println!("  - 'end.mp3' will play when the timer ends.");
    println!("\nNote:");
    println!("  - the timer will look for \"audio.mp3\" for loop and \"end.mp3\" for end sound");
}
