//! pcpulse command-line entry point.

mod view;

use pcpulse_core::collector::{collect_once, collect_snapshot_once};
use pcpulse_core::config::{Config, default_config, load_config};
use pcpulse_core::health::evaluate;

use ratatui::DefaultTerminal;
use ratatui::restore;
use ratatui::try_init;

use crossterm::event::{Event, KeyCode, poll, read};
use std::env::args;
use std::io::Result;
use std::time::Duration;

fn help_text() {
    println!("Usage: pcpulse [options]");
    println!("  --snapshot-once   collect one snapshot and print plain text");
    println!("  --help            show this help");
    println!("  --version         show version");
}

fn run_app(terminal: &mut DefaultTerminal, config: &Config) -> Result<()> {
    loop {
        let snapshot = collect_once(config);
        let report = evaluate(&snapshot, config);

        terminal.draw(|frame| view::draw(&snapshot, &report, frame))?;

        if poll(Duration::from_millis(250))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    },
                    KeyCode::Char('Q') => {
                        return Ok(());
                    },
                    KeyCode::Char('c') => {},
                    _ => {},
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let mut flag: String = "".to_string();

    for arg in args() {
        if arg == "--snapshot-once" || arg == "--help" || arg == "-h" || arg == "--version" {
            flag = arg;
        }
    }

    if flag == "--help" || flag == "-h" {
        help_text();
        return Ok(());
    }

    if flag == "--version" {
        println!("pcpulse 0.1.0");
        return Ok(());
    }

    let cfg: Config = match load_config() {
        Ok(cfg) => cfg,
        Err(_) => default_config(),
    };

    if flag == "--snapshot-once" {
        let snapshot = collect_snapshot_once();
        let report = evaluate(&snapshot, &cfg);
        println!("{}", view::text(&snapshot, &report));
        return Ok(());
    }

    match try_init() {
        Ok(terminal) => {
            let mut term = terminal;
            let result = run_app(&mut term, &cfg);
            restore();
            result
        },
        Err(_) => {
            let snapshot = collect_snapshot_once();
            let report = evaluate(&snapshot, &cfg);
            println!("{}", view::text(&snapshot, &report));
            Ok(())
        },
    }
}