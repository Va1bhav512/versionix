mod encrypt;

use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use clap::{Arg, Command};
use std::fs::DirBuilder;
use std::env;


fn initialize() -> io::Result<()> {
    initialize_directory()?;
    let current_path = env::current_dir()?;
    println!("Current directory: {:?}", current_path);
    let path = current_path.join("\\test");
    encrypt::visit_dirs(&path)?;
    Ok(())

}

fn initialize_directory() -> io::Result<()> {
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    let path = Path::new(".vx");
    builder.create(path)?;
    builder.create(path.join("objects"))?;
    builder.create(path.join("tree"))?;

    Ok(())

}

fn read(path: &Path) {
    let file = File::open(&path).expect("File not found!");
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Could not read file!");
        println!("{}", line);
    }
}

fn main() {
    let cli = Command::new("vx")
        .about("vx version control system")
        .subcommand(
            Command::new("read")
            .about("Reads a file")
            .arg(
                Arg::new("path")
                .help("path for reading")
                .required(true)
                .index(1)
            )
        )
        .subcommand(
            Command::new("init")
            .about("Initializes a .vx folder")
        )
        .get_matches();
        if let Some(matches) = cli.subcommand_matches("read") {
            if let Some(path) = matches.get_one::<String>("path") {
                read(Path::new(path));
            }
        } else if let Some(_) = cli.subcommand_matches("init") {
            match initialize() {
                Ok(_) => println!("Successfully initialized!"),
                Err(e) => eprintln!("Error creating directory! {}",e),
            }
        }
}
