use clap::{Parser, Subcommand};
use std::{fs::File, io::Error, path::Path};

#[derive(Parser)]
#[command(name = "cli")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Touch { name: String },
}

fn create_file(name: &str) -> std::io::Result<File> {
    let path = Path::new(name);

    if !path.exists() {
        Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "File already exists",
        ))
    } else {
        let file = File::create(path)?;
        Ok(file)
    }
}

fn run_cmd() {
    let cmd = Cli::parse();
    match &cmd.command {
        Commands::Touch { name } => match create_file(name.as_str()) {
            Ok(_) => println!("File {} is succesfully created", name),
            Err(e) => eprintln!("Error creating file '{}': {}", name, e),
        },
    }
}

fn main() {
    run_cmd();
}
