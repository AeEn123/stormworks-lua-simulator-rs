#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;

mod gui;
mod locale;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    test: bool,
}

fn main() {
    let args = Args::parse();
    if args.test {
        println!("test");
    }

    gui::run_gui().unwrap();
}
