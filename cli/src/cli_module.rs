pub use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct  Cli {
    #[command(subcommand)]
    pub command : Option<Commands>,
}

#[derive(Subcommand)]
pub enum  Commands {
    #[command(subcommand)]
    Store (store::Commands)
}

pub mod store;

