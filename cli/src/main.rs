use clap::{Parser, Subcommand};

#[derive(Parser)]
struct  Cli {
    #[command(subcommand)]
    command : Option<Commands>,
}

#[derive(Subcommand)]
enum  Commands {
    #[command(subcommand)]
    Store (StoreCommands)
}

#[derive(Subcommand)]
enum StoreCommands{
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

fn main(){
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Store(store_cmd)) => {
            println!("store function.");
            match  store_cmd {
                StoreCommands::Create{name  } =>{ println!("store name {} is created.",name)},
                StoreCommands::Delete{id} => { println!("store id {} is delete.",id)},
                StoreCommands::Rename{id,name} =>  {println!("store id {} is renamed to {}.",id,name)},
                StoreCommands::List => println!("store listig."),
            }
        },
        None => {
            println!("There was no subcommand given");
        }
    }
}
// use std::env;

// fn main() {
//     let args:Vec<String> = env::args().collect();
//     println!("{:?}",args);

//      if args.len() < 2 {
//         eprintln!("No command provided.");
//         return;
//     }

//     let pars1 = &args[1];
//     match pars1.as_str() {
//         "store" => {
//             println!("this is store function.");
//             let pars2 = &args[2];
//             match pars2.as_str() {
//                 "create" =>println!("this is store add function."),
//                 "delete" =>println!("this is store delete function."),
//                 "rename" =>println!("this is store rename function."),
//                 "list"   =>println!("this is store list function."),
//                 other => println!("unknown command {}.",other), 
//             } 
//         },
//         other => println!("unknown command {}.",other),
//     }
// }
