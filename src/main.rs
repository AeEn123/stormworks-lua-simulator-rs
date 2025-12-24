#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

mod gui;
mod locale;
mod lua;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // #[clap(short, long)]
    // test: bool,
    script: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(script) = args.script {
        match std::fs::read_to_string(&script) {
            Ok(content) => lua::run_microcontroller(&content).unwrap(), // TODO: Error handling
            Err(err) => eprintln!("Error reading script: {}", err),
        }
    }

    // gui::run_gui().unwrap();
}
