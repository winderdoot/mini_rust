use clap::{Parser, ValueEnum};
use rudb::{cli::commands::{AnyCommand, Command}, database::{AnyDatabase, Database}};
use std::io::{self, Error};

/// CLI interface for an in-memory rust database program
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Datbase primary key type
    #[arg(value_enum, rename_all="lower", short, long)]
    key_type: KeyType,

}

#[derive(ValueEnum, Clone, Debug)]
enum KeyType {
    Int,
    String
}

/* Commands  */

fn handle_line(line: &str, database: &mut AnyDatabase) {
    match AnyCommand::parse_from(line, database).as_mut() {
        Ok(command) => command.exec(),
        Err(err) => println!("{err}"),
    }
}


fn main() {
    let args = Args::parse();

    let mut database: AnyDatabase = match args.key_type {
        KeyType::Int => AnyDatabase::IntDatabase(Database::<i64>::new()),
        KeyType::String => AnyDatabase::StringDatabase(Database::<String>::new()),
    };

    io::stdin()
        .lines()
        .map_while(Result::ok)
        .for_each(|l| handle_line(&l, &mut database));
}
