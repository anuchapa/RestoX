use super::{Subcommand};

#[derive(Subcommand)]
pub enum Commands{
    Create{
    #[arg(long)]
        name : String
    },
    Delete{
        #[arg(long)]
        id : usize
    },
    Rename{
        #[arg(long)]
        id : usize,
        #[arg(long)]
        name: String
    },
    List
}