use std::io::{self, Read, Write};
use std::process;

use clap::Parser;

mod cli;

use cli::{Cli, ColorArg, TableBorderArg};
use mdansi::error::Error;
use mdansi::render::{RenderOptions, Renderer};
use mdansi::stream::StreamRenderer;
use mdansi::style::ColorLevel;
use mdansi::table::BorderStyle;
use mdansi::terminal::TerminalCaps;
use mdansi::theme::{self, Theme};

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        // Handle EPIPE gracefully (e.g., piping to `head`)
        if let Error::Io(ref io_err) = e {
            if io_err.kind() == io::ErrorKind::BrokenPipe {
                process::exit(0);
            }
        }
        eprintln!("mdansi: {}", e);
        process::exit(1);
    }
}

fn run(args: Cli) -> mdansi::error::Result<()> {
    // Handle --list-themes
    if args.list_themes {
        println!("Built-in themes:");
        for name in theme::builtin_theme_names() {
            println!("  {}", name);
        }
        println!("\nUse --theme-file <path.toml> for custom themes.");
        return Ok(());
    }

    // Resolve terminal capabilities
    let mut caps = TerminalCaps::detect();
    if let Some(width) = args.width {
        caps = caps.with_width(width);
    }
    if let Some(ref color_arg) = args.color {
        match color_arg {
            ColorArg::Always => caps = caps.with_color_level(ColorLevel::TrueColor),
            ColorArg::Never => caps = caps.with_color_level(ColorLevel::None),
            ColorArg::Auto => {} // Keep detected
        }
    }

    // Resolve theme
    let the_theme = if let Some(ref path) = args.theme_file {
        theme::ThemeFile::load(path)?
    } else {
        theme::builtin_theme(&args.theme).ok_or_else(|| Error::UnknownTheme {
            name: args.theme.clone(),
        })?
    };

    // Build render options
    let mut options = RenderOptions::from_terminal(&caps);
    if args.no_wrap {
        options.wrap = false;
    }
    if args.no_highlight {
        options.highlight = false;
    }
    if args.line_numbers {
        options.line_numbers = true;
    }
    if args.no_code_wrap {
        options.code_wrap = false;
    }
    if args.no_truncate {
        options.table_truncate = false;
    }
    if args.plain {
        options.plain = true;
        options.highlight = false;
    }
    options.table_border = match args.table_border {
        TableBorderArg::Unicode => BorderStyle::Unicode,
        TableBorderArg::Ascii => BorderStyle::Ascii,
        TableBorderArg::None => BorderStyle::None,
    };

    // Read input
    if args.stream {
        // Streaming mode: read from stdin incrementally
        return run_streaming(the_theme, options);
    }

    let input = read_input(&args)?;

    // Render and output
    let renderer = Renderer::new(the_theme, options);
    let output = renderer.render(&input);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(output.as_bytes())?;
    handle.flush()?;

    Ok(())
}

fn read_input(args: &Cli) -> mdansi::error::Result<String> {
    match &args.file {
        Some(path) => {
            if path.as_os_str() == "-" {
                read_stdin()
            } else {
                Ok(std::fs::read_to_string(path)?)
            }
        }
        None => read_stdin(),
    }
}

fn read_stdin() -> mdansi::error::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn run_streaming(the_theme: Theme, options: RenderOptions) -> mdansi::error::Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut stream = StreamRenderer::new(handle, the_theme, options);

    let stdin = io::stdin();
    let mut buf = [0u8; 4096];

    loop {
        let n = stdin.lock().read(&mut buf)?;
        if n == 0 {
            break;
        }
        let chunk = String::from_utf8_lossy(&buf[..n]);
        stream.push(&chunk)?;
    }

    stream.flush_remaining()?;
    Ok(())
}
