use clap::{Parser, Subcommand};
use once_cell::sync::Lazy;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Worker
}

pub static ARGS: Lazy<Args> = Lazy::new(Args::parse);