use std::collections::HashMap;



use std::fs::OpenOptions;

use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};


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

#[derive(Debug)]
enum ParseError {
    UnknownCommand,
    InvalidArguments,
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

    fn set(&mut self, key: &str, val: &str){
        self.data.insert(key.to_string(), val.to_string());
    }
    
    fn delete(&mut self, key: &str) -> Option<String>{
        self.data.remove(key)

    }
    fn exists(&self, key: &str)->bool{
        self.data.contains_key(key)

    }
}




fn main() {
    println!("Simple Rust KV Store CLI");
    println!("Type EXIT to close the program");
    



    let file1 = File::open("store.log").unwrap();
    
    let reader = BufReader::new(file1);
    for line in reader.lines(){ 
            let actual_line =line.unwrap();
            let parts: Vec<&str> = actual_line.split_whitespace().collect();
            let command = parse_command(&parts).unwrap();

        }

    fn apply_command(store: &mut Store, ){


    }

    



    let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("store.log")
            .unwrap();
    
    
    
    let mut store= Store::new();




    loop {


        

        print!("kv> ");

        io::stdout()
        .flush()
        .expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
        
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        


        let parts: Vec<&str> = input.split_whitespace().collect();

        
        let command = match parse_command(&parts) {
            Ok(command) => command,
            Err(ParseError::UnknownCommand) => {
        println!("Unknown command. Type HELP for available commands.");
        continue;
    }
    

    Err(ParseError::InvalidArguments) => {
        println!("Invalid arguments.");
        continue;
        }
    };

        
        if !handle_commands(&mut store,command,&mut file ){
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


fn parse_command(parts: &[&str]) -> Result<Command,ParseError>{

    if parts.is_empty() {
        print!("Empty");
        return Err(ParseError::InvalidArguments);
        
    }

    let command = parts[0].to_ascii_uppercase();
  

    match command.as_str(){
        "GET" => {
            if parts.len() !=2{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Get(parts[1].to_string()))
        }
        "SET" => {
            if parts.len() !=3{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Set(parts[1].to_string(),parts[2].to_string(),))
        }

        "DELETE" => {
            if parts.len() !=2{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Delete(parts[1].to_string()))
        }

        "EXISTS" => {
            if parts.len() !=2{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Exists(parts[1].to_string()))
        }

        "COUNT" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Count)}

        "CLEAR" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Clear)}

        "HELP" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Help)}

        "EXIT" => {
            if parts.len() !=1{
                return Err(ParseError::InvalidArguments)
            }
            Ok(Command::Exit)}

        _ => Err(ParseError::UnknownCommand),
    }

    
}


fn handle_commands(store: &mut Store, command: Command, file: &mut File) -> bool{
match command {
            Command::Set(key, value) => {
                store.set(&key, &value);
                writeln!(file,"SET {} {}", key, value).unwrap();
                println!("OK");
                true
            }

            Command::Get(key) => {
                match store.get(&key) {
                Some(value) => println!("Found:{}", value),
                None => println!("Key does not exist"),
                    }
                true
            }

            Command::Count => {
                let count = store.count();
                println!("{count}");
                true
            }

            Command::Clear => {
                store.clear();
                writeln!(file,"Clear").unwrap();
                println!("OK");
                true
            }

            Command::Delete(key) => {
                match store.delete(&key) {
                Some(_) => {writeln!(file,"Delete {}", key).unwrap();
                            println!("Ok removed");}
                None => println!("Key does not exist"),
                }
                
                true
            }

            Command::Exists(key) => {
                let exist = store.exists(&key);
                println!("{}",exist);
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_store_is_empty() {
        let store = Store::new();

        assert_eq!(store.count(), 0);
    }

    #[test]
    fn get_set_store(){
        let mut store = Store::new();

        store.set("Language", "Rust");
        
        match store.get("Language") {
            Some(value) => {
                assert_eq!(value,"Rust");
            }
            None => {
                panic!("Expected key to exist")
            }
        }

    }
}   