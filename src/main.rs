mod api;
mod app;
mod cache;
mod cli;
mod commands;
mod config;
mod models;
mod view;
use anyhow::Result;
use clap::Parser;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = app::App::new()?;
    app.run(&cli)
}
