use clap::{Parser, ValueEnum};
use rudb::{cli::{commands::{AnyCommand, Command}, tokens::token_stream}, database::{AnyDatabase, Database, DatabaseKey}};
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

fn handle_db<K: DatabaseKey>(input: &str, database: &mut Database<K>, history: &mut Vec<String>) {
    let mut tokens = token_stream::<SplitWhitespace>(input);
    // println!("tokens:\n{:#?}", tokens.collect::<Vec<_>>());

    match AnyCommand::parse_from(&mut tokens, database).as_mut() {
        Ok(command) => {
            match command.exec(history) {
                Ok(output) => println!("{output}"),
                Err(err) => println!("Database error: {err}"),
            }
        },
        Err(err) => println!("Parse error: {err}"),
    }
}

fn handle_line(line: &str, database: &mut AnyDatabase, history: &mut Vec<String>) {
    match database {
        AnyDatabase::StringDatabase(database) => handle_db(line, database, history),
        AnyDatabase::IntDatabase(database) => handle_db(line, database, history),
    }
    history.push(line.to_string());
}


fn main() {
    let args = Args::parse();

    let mut database: AnyDatabase = match args.key_type {
        KeyType::Int => AnyDatabase::IntDatabase(Database::<i64>::new()),
        KeyType::String => AnyDatabase::StringDatabase(Database::<String>::new()),
    };

    let mut history = Vec::<String>::new();

    io::stdin()
        .lines()
        .map_while(Result::ok)
        .for_each(|l| handle_line(&l, &mut database, &mut history));
}
