use std::collections::HashMap;

use std::io::{self, Write};

enum Command{
    Set(String, String),
    Get(String),
    Delete(String),
    Exists(String),
    Count,
    Clear,
    Help,
    Exit,

}

struct Store{
    data:HashMap<String,String>

}

impl Store{
    fn new() -> Self{
        Self{
            data:HashMap::new()
        }
    }
    fn count(&self) -> usize {
        self.data.len()
    }
    fn clear(&mut self) {
        self.data.clear();
    }
    fn get(&self, key:&str) -> Option<&String>{
        self.data.get(key)

    }
}




fn main() {
    println!("Simple Rust KV Store CLI");
    println!("Type EXIT to close the program");

    let mut store= Store::new();
    loop {
        print!("kv> ");

        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        
        let command = match parse_command(&parts) {
            Some(command) => command,
            None => {
        println!("Unkown command. Type Help for available commands.");
        continue;
    }
};
        if !handle_commands(&mut store,command){
            break;
        }

        

        

    }
}

fn print_help() {
    println!(
        "SET key value 
                GET key 
                DELETE key
                EXISTS key
                COUNT
                CLEAR
                EXIT"
    );
}

fn handle_count(store: &HashMap<String, String>) {
    let count = store.len();

    println!("{count}");
}

fn handle_clear(store: &mut HashMap<String, String>) {
    store.clear();
    println!("OK");
}

fn handle_get(store: &HashMap<String, String>, key: &str) {
    match store.get(key) {
        Some(value) => println!("Found:{}", value),
        None => println!("Key does not exist"),
    }
}

fn handle_delete(store: &mut HashMap<String, String>, key: &str) {
    match store.remove(key) {
        Some(_) => println!("Ok removed"),
        None => println!("Key does not exist"),
    }
}

fn handle_exists(store: &HashMap<String, String>, key: &str) {
    let exists = store.contains_key(key);

    println!("{exists}");
}

fn handle_set(store: &mut HashMap<String, String>, key: &str, value: &str) {
    store.insert(key.to_string(), value.to_string());
    println!("OK");
}


fn parse_command(parts: &[&str]) -> Option<Command>{

    if parts.is_empty() {
        return None;
    }

    let command = parts[0].to_ascii_uppercase();

    match (command.as_str(), parts) {
        ("GET", [_, key]) => {
            Some(Command::Get(key.to_string()))
        }
        ("SET", [_, key, value]) => {
            Some(Command::Set(key.to_string(),value.to_string(),))
        }

        ("DELETE", [_, key]) => {
            Some(Command::Delete(key.to_string()))
        }

        ("EXISTS", [_, key]) => {
            Some(Command::Exists(key.to_string()))
        }

        ("COUNT", [_]) => Some(Command::Count),
        ("CLEAR", [_]) => Some(Command::Clear),
        ("HELP", [_]) => Some(Command::Help),
        ("EXIT", [_]) => Some(Command::Exit),

        _ => None,
    }

    
}


fn handle_commands(store: &mut HashMap<String,String>, command: Command) -> bool{
match command {
            Command::Set(key, value) => {
                handle_set(store, &key, &value);
                true
            }

            Command::Get(key) => {
                handle_get(&store, &key);
                true
            }

            Command::Count => {
                handle_count(&store);
                true
            }

            Command::Clear => {
                handle_clear( store);
                true
            }

            Command::Delete(key) => {
                handle_delete(store, &key);
                true
            }

            Command::Exists(key) => {
                handle_exists(store, &key);
                true
            }


            Command::Exit => {
                println!("Goodbye");
                false
            }

            Command::Help =>{ 
                print_help();
                true
            }
        }
    }