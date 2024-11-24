mod encrypt;
mod decrypt;

use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use clap::{Arg, Command};
use std::fs::DirBuilder;
use std::env;


fn initialize() -> Result<(), io::Error> {
    initialize_directory()?;
    let current_path = env::current_dir()?;
    println!("Current directory: {:?}", current_path);
    let mut commit = String::new();
    let path = current_path.join("\\test");
    // let path = current_path;
    encrypt::visit_dirs(&path, &mut commit)?;
    encrypt::store_commit(&commit, "initial commit")?;
    println!("The commit looks like this: {}", commit);
    Ok(())

}

fn commit(message: &str) -> Result<(), io::Error> {
    let current_path = env::current_dir()?;
    let mut commit = String::new();
    let path = current_path.join("\\test");
    // let path = current_path;
    encrypt::visit_dirs(&path, &mut commit)?;
    encrypt::store_commit(&commit, &message)?;
    println!("The commit looks like this: {}", commit);
    Ok(())
}

fn initialize_directory() -> Result<(), io::Error> {
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    let path = Path::new(".vx");
    builder.create(path)?;
    builder.create(path.join("objects"))?;
    builder.create(path.join("tree"))?;
    builder.create(path.join("commits"))?;
    File::create(path.join("history.history"))?;

    Ok(())

}

fn main() {
    let cli = Command::new("versionix")
        .about("vx version control system")
        .subcommand(
            Command::new("init")
            .about("Initializes a .vx folder")
        )
        .subcommand(
            Command::new("log")
            .about("View commit history")
        )
        .subcommand(
            Command::new("rc")
            .about("Reads a commit")
            .arg(
                Arg::new("path")
                .help("path for reading")
                .required(true)
                .index(1)
            )
        )
        .subcommand(
            Command::new("commit")
            .about("Adds a new commit")
            .arg(
                Arg::new("message")
                .help("message of the commmit")
                .required(true)
                .index(1)
            )
        )
        .get_matches();
        if let Some(_) = cli.subcommand_matches("init") {
            match initialize() {
                Ok(_) => println!("Successfully initialized!"),
                Err(e) => eprintln!("Error creating directory! {}",e),
            }
        } else if let Some(_) = cli.subcommand_matches("log") {
            match decrypt::read_commit_history() {
                Ok(_) => println!("--End of history--"),
                Err(e) => eprintln!("Error: {}", e),
            }
        } else if let Some(matches) = cli.subcommand_matches("rc") {
            if let Some(path) = matches.get_one::<String>("path") {
                match decrypt::read_commit(path) {
                    Ok(_) => println!("--End of files--"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        } else if let Some(matches) = cli.subcommand_matches("commit") {
            if let Some(message) = matches.get_one::<String>("message") {
                match commit(&message){
                    Ok(_) => println!("Successfully commited"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
}
