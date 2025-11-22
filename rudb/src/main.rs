use clap::{Parser, ValueEnum};
use rudb::{cli::commands::{AnyCommand, Command, token_stream}, database::{AnyDatabase, Database, DatabaseKey}};
use std::{io::{self, Error}, str::SplitWhitespace};

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

fn handle_db<K: DatabaseKey>(input: &str, database: &mut Database<K>) {
    let mut tokens = token_stream::<SplitWhitespace>(input);

    match AnyCommand::parse_from(&mut tokens, database).as_mut() {
        Ok(command) => {
            match command.exec() {
                Ok(output) => println!("{output}"),
                Err(err) => println!("Database error: {err}"),
            }
        },
        Err(err) => println!("Parse error: {err}"),
    }
}

fn handle_line(line: &str, database: &mut AnyDatabase) {
    match database {
        AnyDatabase::StringDatabase(database) => handle_db(line, database),
        AnyDatabase::IntDatabase(database) => handle_db(line, database),
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
