use lf_preview_poc::render;
use ratatui::style::{Color, Modifier};
use ratatui::text::Line;
use std::env;
use std::io::{self, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "Usage: lf-preview-poc [--plain | --ansi] [--width COLUMNS] [--cwd PATH]\nReads Markdown from standard input. Defaults to plain output and 80 columns.";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os().skip(1);
    let mut ansi = false;
    let mut width = 80;
    let mut cwd = None;
    while let Some(arg) = args.next() {
        match arg.to_str() {
            Some("--ansi") => ansi = true,
            Some("--plain") => ansi = false,
            Some("--width") => {
                width = args
                    .next()
                    .and_then(|arg| arg.to_str()?.parse::<usize>().ok())
                    .filter(|number| *number > 0)
                    .ok_or("--width requires a positive column count")?;
            }
            Some("--cwd") => cwd = Some(PathBuf::from(args.next().ok_or("--cwd requires a path")?)),
            Some("--help" | "-h") => {
                println!("{USAGE}");
                return Ok(());
            }
            _ => return Err(USAGE.into()),
        }
    }
    let cwd = cwd.map_or_else(env::current_dir, Ok)?;
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for line in render(&input, width, &cwd) {
        write_line(&mut out, &line, ansi)?;
    }
    out.flush()?;
    Ok(())
}

fn write_line(out: &mut impl Write, line: &Line<'_>, ansi: bool) -> io::Result<()> {
    for span in &line.spans {
        let content: String = span.content.chars().filter(|ch| !ch.is_control() || *ch == '\t').collect();
        if content.is_empty() {
            continue;
        }
        let mut codes = Vec::new();
        if ansi {
            let style = line.style.patch(span.style);
            if let Some(color) = style.fg {
                codes.push(sgr_color(color, false));
            }
            if let Some(color) = style.bg {
                codes.push(sgr_color(color, true));
            }
            for (modifier, code) in [
                (Modifier::BOLD, "1"),
                (Modifier::DIM, "2"),
                (Modifier::ITALIC, "3"),
                (Modifier::UNDERLINED, "4"),
                (Modifier::SLOW_BLINK, "5"),
                (Modifier::RAPID_BLINK, "6"),
                (Modifier::REVERSED, "7"),
                (Modifier::HIDDEN, "8"),
                (Modifier::CROSSED_OUT, "9"),
            ] {
                if style.add_modifier.contains(modifier) {
                    codes.push(code.to_owned());
                }
            }
        }
        if codes.is_empty() {
            write!(out, "{content}")?;
        } else {
            write!(out, "\x1b[{}m{content}\x1b[0m", codes.join(";"))?;
        }
    }
    writeln!(out)
}

fn sgr_color(color: Color, background: bool) -> String {
    let offset = if background { 10 } else { 0 };
    let extended = 38 + offset;
    let code = match color {
        Color::Reset => 39,
        Color::Black => 30,
        Color::Red => 31,
        Color::Green => 32,
        Color::Yellow => 33,
        Color::Blue => 34,
        Color::Magenta => 35,
        Color::Cyan => 36,
        Color::Gray => 37,
        Color::DarkGray => 90,
        Color::LightRed => 91,
        Color::LightGreen => 92,
        Color::LightYellow => 93,
        Color::LightBlue => 94,
        Color::LightMagenta => 95,
        Color::LightCyan => 96,
        Color::White => 97,
        Color::Indexed(index) => return format!("{extended};5;{index}"),
        Color::Rgb(r, g, b) => return format!("{extended};2;{r};{g};{b}"),
    };
    (code + offset).to_string()
}
